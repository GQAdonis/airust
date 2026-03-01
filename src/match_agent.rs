// src/match_agent.rs - Unified matching agent replacing simple and fuzzy agents
use crate::agent::{
    Agent, ConfidenceAgent, PredictionResult, ResponseFormat, TrainableAgent, TrainingExample,
};
use strsim::levenshtein;

/// Defines different matching strategies for finding relevant training examples
pub enum MatchingStrategy {
    /// Exact match requiring full equality (case-insensitive)
    Exact,
    /// Fuzzy matching with configurable options
    Fuzzy(FuzzyOptions),
}

/// Configuration options for fuzzy matching
pub struct FuzzyOptions {
    /// Maximum allowed Levenshtein distance between input and training example
    /// None means no hard limit on distance
    pub max_distance: Option<usize>,

    /// Dynamic threshold factor based on input length
    /// Scales the maximum allowed distance as a fraction of input length
    pub threshold_factor: Option<f32>,
}

/// Default configuration for fuzzy matching
impl Default for FuzzyOptions {
    fn default() -> Self {
        Self {
            max_distance: None,
            threshold_factor: Some(0.3), // Default: 30% of input length as max distance
        }
    }
}

/// Default matching strategy (fuzzy with default options)
impl Default for MatchingStrategy {
    fn default() -> Self {
        MatchingStrategy::Fuzzy(FuzzyOptions::default())
    }
}

/// Unified agent capable of exact and fuzzy matching
pub struct MatchAgent {
    /// Stored training examples
    memory: Vec<TrainingExample>,

    /// Current matching strategy
    strategy: MatchingStrategy,
}

impl MatchAgent {
    /// Creates a new MatchAgent with a specific matching strategy
    pub fn new(strategy: MatchingStrategy) -> Self {
        Self {
            memory: Vec::new(),
            strategy,
        }
    }

    /// Creates an agent with exact matching strategy
    pub fn new_exact() -> Self {
        Self::new(MatchingStrategy::Exact)
    }

    /// Creates an agent with fuzzy matching strategy
    pub fn new_fuzzy() -> Self {
        Self::new(MatchingStrategy::Fuzzy(FuzzyOptions::default()))
    }

    /// Allows changing the matching strategy after agent creation
    pub fn with_strategy(mut self, strategy: MatchingStrategy) -> Self {
        self.strategy = strategy;
        self
    }
}

impl Agent for MatchAgent {
    /// Predicts the best matching response based on the current strategy
    fn predict(&self, input: &str) -> ResponseFormat {
        if self.memory.is_empty() {
            return ResponseFormat::Text("No training data available.".to_string());
        }

        match &self.strategy {
            MatchingStrategy::Exact => {
                // Exact match strategy
                for item in &self.memory {
                    if item.input.to_lowercase() == input.to_lowercase() {
                        return item.output.clone();
                    }
                }
                ResponseFormat::Text("No matching answer found.".to_string())
            }
            MatchingStrategy::Fuzzy(options) => {
                // Fuzzy matching strategy using Levenshtein distance
                let mut best_score = usize::MAX;
                let mut best_match = None;

                let input_lower = input.to_lowercase();

                // Calculate dynamic threshold based on input length
                let threshold = match options.threshold_factor {
                    Some(factor) => (input_lower.len() as f32 * factor) as usize,
                    None => usize::MAX,
                };

                for item in &self.memory {
                    let score = levenshtein(&item.input.to_lowercase(), &input_lower);

                    // Check max distance constraint
                    if let Some(max_dist) = options.max_distance {
                        if score > max_dist {
                            continue;
                        }
                    }

                    // Check dynamic threshold
                    if score > threshold {
                        continue;
                    }

                    // Find best match
                    if score < best_score {
                        best_score = score;
                        best_match = Some(item);
                    }
                }

                match best_match {
                    Some(item) => item.output.clone(),
                    None => ResponseFormat::Text("No matching answer found.".to_string()),
                }
            }
        }
    }
}

impl TrainableAgent for MatchAgent {
    /// Trains the agent by replacing all training examples
    fn train(&mut self, data: &[TrainingExample]) {
        self.memory = data.to_vec();
    }

    /// Appends training examples without replacing existing data
    fn append(&mut self, data: &[TrainingExample]) {
        self.memory.extend_from_slice(data);
    }
}

impl ConfidenceAgent for MatchAgent {
    /// Calculates confidence based on matching strategy
    fn calculate_confidence(&self, input: &str) -> f32 {
        if self.memory.is_empty() {
            return 0.0;
        }

        match &self.strategy {
            MatchingStrategy::Exact => {
                let input_lower = input.to_lowercase();
                if self.memory.iter().any(|item| item.input.to_lowercase() == input_lower) {
                    1.0
                } else {
                    0.0
                }
            }
            MatchingStrategy::Fuzzy(options) => {
                let input_lower = input.to_lowercase();
                let input_len = input_lower.len().max(1) as f32;

                let threshold = match options.threshold_factor {
                    Some(factor) => (input_lower.len() as f32 * factor) as usize,
                    None => usize::MAX,
                };

                let mut best_distance = usize::MAX;
                for item in &self.memory {
                    let dist = levenshtein(&item.input.to_lowercase(), &input_lower);
                    if let Some(max_dist) = options.max_distance {
                        if dist > max_dist {
                            continue;
                        }
                    }
                    if dist > threshold {
                        continue;
                    }
                    if dist < best_distance {
                        best_distance = dist;
                    }
                }

                if best_distance == usize::MAX {
                    0.0
                } else {
                    (1.0 - best_distance as f32 / input_len).max(0.0)
                }
            }
        }
    }

    /// Returns top N predictions with confidence scores
    fn predict_top_n(&self, input: &str, n: usize) -> Vec<PredictionResult> {
        if self.memory.is_empty() || n == 0 {
            return Vec::new();
        }

        match &self.strategy {
            MatchingStrategy::Exact => {
                let input_lower = input.to_lowercase();
                self.memory
                    .iter()
                    .filter(|item| item.input.to_lowercase() == input_lower)
                    .take(n)
                    .map(|item| PredictionResult {
                        response: item.output.clone(),
                        confidence: 1.0,
                        metadata: None,
                    })
                    .collect()
            }
            MatchingStrategy::Fuzzy(options) => {
                let input_lower = input.to_lowercase();
                let input_len = input_lower.len().max(1) as f32;

                let threshold = match options.threshold_factor {
                    Some(factor) => (input_lower.len() as f32 * factor) as usize,
                    None => usize::MAX,
                };

                let mut scored: Vec<(usize, usize)> = self
                    .memory
                    .iter()
                    .enumerate()
                    .filter_map(|(i, item)| {
                        let dist = levenshtein(&item.input.to_lowercase(), &input_lower);
                        if let Some(max_dist) = options.max_distance {
                            if dist > max_dist {
                                return None;
                            }
                        }
                        if dist > threshold {
                            return None;
                        }
                        Some((i, dist))
                    })
                    .collect();

                scored.sort_by_key(|&(_, dist)| dist);

                scored
                    .into_iter()
                    .take(n)
                    .map(|(idx, dist)| PredictionResult {
                        response: self.memory[idx].output.clone(),
                        confidence: (1.0 - dist as f32 / input_len).max(0.0),
                        metadata: None,
                    })
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_example(input: &str, output: &str) -> TrainingExample {
        TrainingExample {
            input: input.to_string(),
            output: ResponseFormat::Text(output.to_string()),
            weight: 1.0,
            metadata: None,
        }
    }

    fn sample_data() -> Vec<TrainingExample> {
        vec![
            make_example("hello", "Hi there!"),
            make_example("goodbye", "See you later!"),
            make_example("how are you", "I am fine, thanks!"),
        ]
    }

    // === Exact Matching Tests ===

    #[test]
    fn test_exact_basic_match() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        assert_eq!(String::from(agent.predict("hello")), "Hi there!");
    }

    #[test]
    fn test_exact_case_insensitive() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        assert_eq!(String::from(agent.predict("HELLO")), "Hi there!");
        assert_eq!(String::from(agent.predict("Hello")), "Hi there!");
    }

    #[test]
    fn test_exact_no_match_fallback() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        let result = String::from(agent.predict("unknown"));
        assert!(result.contains("No matching answer found"));
    }

    #[test]
    fn test_exact_empty_training() {
        let agent = MatchAgent::new_exact();
        let result = String::from(agent.predict("hello"));
        assert!(result.contains("No training data available"));
    }

    #[test]
    fn test_exact_whitespace_sensitivity() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        // " hello" != "hello" in exact match
        let result = String::from(agent.predict(" hello"));
        assert!(result.contains("No matching answer found"));
    }

    #[test]
    fn test_exact_unicode_input() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&[make_example("café", "coffee shop")]);
        assert_eq!(String::from(agent.predict("café")), "coffee shop");
        assert_eq!(String::from(agent.predict("CAFÉ")), "coffee shop");
    }

    #[test]
    fn test_exact_first_match_wins() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&[
            make_example("hello", "first"),
            make_example("hello", "second"),
        ]);
        assert_eq!(String::from(agent.predict("hello")), "first");
    }

    #[test]
    fn test_exact_empty_string() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&[make_example("", "empty match")]);
        assert_eq!(String::from(agent.predict("")), "empty match");
    }

    #[test]
    fn test_exact_markdown_response() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&[TrainingExample {
            input: "help".to_string(),
            output: ResponseFormat::Markdown("# Help\nUse /help".to_string()),
            weight: 1.0,
            metadata: None,
        }]);
        match agent.predict("help") {
            ResponseFormat::Markdown(md) => assert!(md.contains("# Help")),
            _ => panic!("Expected Markdown response"),
        }
    }

    #[test]
    fn test_exact_json_response() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&[TrainingExample {
            input: "status".to_string(),
            output: ResponseFormat::Json(serde_json::json!({"status": "ok"})),
            weight: 1.0,
            metadata: None,
        }]);
        match agent.predict("status") {
            ResponseFormat::Json(val) => assert_eq!(val["status"], "ok"),
            _ => panic!("Expected JSON response"),
        }
    }

    // === Fuzzy Matching Tests ===

    #[test]
    fn test_fuzzy_exact_input() {
        let mut agent = MatchAgent::new_fuzzy();
        agent.train(&sample_data());
        assert_eq!(String::from(agent.predict("hello")), "Hi there!");
    }

    #[test]
    fn test_fuzzy_close_match() {
        let mut agent = MatchAgent::new_fuzzy();
        agent.train(&sample_data());
        // "helo" is 1 edit from "hello"
        assert_eq!(String::from(agent.predict("helo")), "Hi there!");
    }

    #[test]
    fn test_fuzzy_threshold_rejection() {
        let mut agent = MatchAgent::new(MatchingStrategy::Fuzzy(FuzzyOptions {
            max_distance: None,
            threshold_factor: Some(0.1), // Very strict threshold
        }));
        agent.train(&sample_data());
        // "xxxxxxxxx" is very far from everything
        let result = String::from(agent.predict("xxxxxxxxx"));
        assert!(result.contains("No matching answer found"));
    }

    #[test]
    fn test_fuzzy_max_distance_constraint() {
        let mut agent = MatchAgent::new(MatchingStrategy::Fuzzy(FuzzyOptions {
            max_distance: Some(1),
            threshold_factor: None,
        }));
        agent.train(&sample_data());
        // "helo" is distance 1 from "hello" -> should match
        assert_eq!(String::from(agent.predict("helo")), "Hi there!");
        // "hxxxxxxx" is far from everything -> should not match
        let result = String::from(agent.predict("hxxxxxxx"));
        assert!(result.contains("No matching answer found"));
    }

    #[test]
    fn test_fuzzy_best_match_selection() {
        let mut agent = MatchAgent::new_fuzzy();
        agent.train(&[
            make_example("cat", "feline"),
            make_example("car", "vehicle"),
            make_example("bat", "animal"),
        ]);
        // "cat" exactly matches "cat" (distance 0)
        assert_eq!(String::from(agent.predict("cat")), "feline");
        // "cas" is distance 1 from "cat" and "car"
        // both are equally close, first one wins
    }

    #[test]
    fn test_fuzzy_case_insensitive() {
        let mut agent = MatchAgent::new_fuzzy();
        agent.train(&sample_data());
        assert_eq!(String::from(agent.predict("HELLO")), "Hi there!");
    }

    #[test]
    fn test_fuzzy_default_options() {
        let options = FuzzyOptions::default();
        assert!(options.max_distance.is_none());
        assert_eq!(options.threshold_factor, Some(0.3));
    }

    #[test]
    fn test_fuzzy_custom_options() {
        let options = FuzzyOptions {
            max_distance: Some(5),
            threshold_factor: Some(0.5),
        };
        assert_eq!(options.max_distance, Some(5));
        assert_eq!(options.threshold_factor, Some(0.5));
    }

    // === Constructor Tests ===

    #[test]
    fn test_new_exact_constructor() {
        let agent = MatchAgent::new_exact();
        let result = String::from(agent.predict("anything"));
        assert!(result.contains("No training data available"));
    }

    #[test]
    fn test_new_fuzzy_constructor() {
        let agent = MatchAgent::new_fuzzy();
        let result = String::from(agent.predict("anything"));
        assert!(result.contains("No training data available"));
    }

    #[test]
    fn test_with_strategy() {
        let agent = MatchAgent::new_fuzzy().with_strategy(MatchingStrategy::Exact);
        let result = String::from(agent.predict("anything"));
        assert!(result.contains("No training data available"));
    }

    #[test]
    fn test_default_strategy_is_fuzzy() {
        let strategy = MatchingStrategy::default();
        match strategy {
            MatchingStrategy::Fuzzy(_) => {} // expected
            MatchingStrategy::Exact => panic!("Default should be Fuzzy"),
        }
    }

    // === Train Behavior Tests ===

    #[test]
    fn test_train_replaces_all_data() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        assert_eq!(String::from(agent.predict("hello")), "Hi there!");

        // Train with new data replaces old
        agent.train(&[make_example("rust", "A language")]);
        let result = String::from(agent.predict("hello"));
        assert!(result.contains("No matching answer found"));
        assert_eq!(String::from(agent.predict("rust")), "A language");
    }

    #[test]
    fn test_train_single_appends() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());

        // train_single now appends (does not replace)
        agent.train_single(&make_example("new", "added"));
        assert_eq!(String::from(agent.predict("hello")), "Hi there!");
        assert_eq!(String::from(agent.predict("new")), "added");
    }

    #[test]
    fn test_append_preserves_existing() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&[make_example("hello", "Hi!")]);
        agent.append(&[make_example("bye", "Goodbye!")]);
        assert_eq!(String::from(agent.predict("hello")), "Hi!");
        assert_eq!(String::from(agent.predict("bye")), "Goodbye!");
    }

    #[test]
    fn test_add_example_appends() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        agent.add_example("rust", "A language", 1.0);
        // Old data still there
        assert_eq!(String::from(agent.predict("hello")), "Hi there!");
        assert_eq!(String::from(agent.predict("rust")), "A language");
    }

    // === Confidence / can_answer Tests ===

    #[test]
    fn test_confidence_match() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        assert_eq!(agent.confidence("hello"), 1.0);
    }

    #[test]
    fn test_confidence_no_match() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        assert_eq!(agent.confidence("unknown"), 0.0);
    }

    #[test]
    fn test_can_answer_true() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        assert!(agent.can_answer("hello"));
    }

    #[test]
    fn test_can_answer_false() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        assert!(!agent.can_answer("unknown"));
    }

    // === ConfidenceAgent Tests ===

    #[test]
    fn test_exact_calculate_confidence_match() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        assert_eq!(agent.calculate_confidence("hello"), 1.0);
    }

    #[test]
    fn test_exact_calculate_confidence_no_match() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        assert_eq!(agent.calculate_confidence("unknown"), 0.0);
    }

    #[test]
    fn test_exact_calculate_confidence_empty() {
        let agent = MatchAgent::new_exact();
        assert_eq!(agent.calculate_confidence("anything"), 0.0);
    }

    #[test]
    fn test_fuzzy_calculate_confidence_exact() {
        let mut agent = MatchAgent::new_fuzzy();
        agent.train(&sample_data());
        // Exact match -> distance 0 -> confidence 1.0
        assert_eq!(agent.calculate_confidence("hello"), 1.0);
    }

    #[test]
    fn test_fuzzy_calculate_confidence_close() {
        let mut agent = MatchAgent::new_fuzzy();
        agent.train(&sample_data());
        // "helo" is distance 1 from "hello" -> confidence = 1 - 1/4 = 0.75
        let conf = agent.calculate_confidence("helo");
        assert!(conf > 0.5 && conf < 1.0);
    }

    #[test]
    fn test_fuzzy_calculate_confidence_no_match() {
        let mut agent = MatchAgent::new(MatchingStrategy::Fuzzy(FuzzyOptions {
            max_distance: None,
            threshold_factor: Some(0.1),
        }));
        agent.train(&sample_data());
        assert_eq!(agent.calculate_confidence("xxxxxxxxxxx"), 0.0);
    }

    #[test]
    fn test_exact_predict_top_n() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&[
            make_example("hello", "first"),
            make_example("hello", "second"),
            make_example("bye", "later"),
        ]);
        let results = agent.predict_top_n("hello", 5);
        assert_eq!(results.len(), 2);
        assert_eq!(String::from(results[0].response.clone()), "first");
        assert_eq!(results[0].confidence, 1.0);
    }

    #[test]
    fn test_fuzzy_predict_top_n() {
        let mut agent = MatchAgent::new_fuzzy();
        agent.train(&sample_data());
        let results = agent.predict_top_n("hello", 2);
        assert!(!results.is_empty());
        assert!(results.len() <= 2);
        // First result should have highest confidence
        if results.len() > 1 {
            assert!(results[0].confidence >= results[1].confidence);
        }
    }

    #[test]
    fn test_predict_top_n_empty() {
        let agent = MatchAgent::new_exact();
        let results = agent.predict_top_n("hello", 5);
        assert!(results.is_empty());
    }

    #[test]
    fn test_predict_top_n_zero() {
        let mut agent = MatchAgent::new_exact();
        agent.train(&sample_data());
        let results = agent.predict_top_n("hello", 0);
        assert!(results.is_empty());
    }
}
