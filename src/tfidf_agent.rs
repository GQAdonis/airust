// src/tfidf_agent.rs - Optimized TF-IDF/BM25 Agent
use crate::agent::{
    text_utils, Agent, ConfidenceAgent, PredictionResult, ResponseFormat, TrainableAgent,
    TrainingExample,
};
use indexmap::IndexMap;
use std::collections::HashSet;

/// TF-IDF Agent using BM25 scoring for intelligent text matching
pub struct TfidfAgent {
    /// Stored training documents
    docs: Vec<TrainingExample>,

    /// Document frequency for each term (in how many documents a term appears)
    term_df: IndexMap<String, f32>,

    /// Term frequencies for each document
    doc_term_freq: Vec<IndexMap<String, f32>>,

    /// Total number of documents
    doc_count: f32,

    /// BM25 parameter k1 (controls term frequency scaling)
    bm25_k1: f32,

    /// BM25 parameter b (controls document length normalization)
    bm25_b: f32,
}

impl TfidfAgent {
    /// Creates a new TF-IDF agent with default BM25 parameters
    pub fn new() -> Self {
        Self {
            docs: Vec::new(),
            term_df: IndexMap::new(),
            doc_term_freq: Vec::new(),
            doc_count: 0.0,
            bm25_k1: 1.2, // Default term frequency scaling
            bm25_b: 0.75, // Default length normalization
        }
    }

    /// Configures custom BM25 parameters for fine-tuned matching
    pub fn with_bm25_params(mut self, k1: f32, b: f32) -> Self {
        self.bm25_k1 = k1;
        self.bm25_b = b;
        self
    }

    /// Returns scored documents sorted descending by score
    fn scored_docs(&self, query_terms: &[String]) -> Vec<(usize, f32)> {
        let mut scores: Vec<(usize, f32)> = self
            .docs
            .iter()
            .enumerate()
            .map(|(i, doc)| {
                let score = self.bm25_score(query_terms, i) * doc.weight;
                (i, score)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores
    }

    /// Rebuilds the internal term frequency index from stored documents
    fn rebuild_index(&mut self) {
        self.doc_count = self.docs.len() as f32;
        self.term_df.clear();
        self.doc_term_freq.clear();

        for doc in &self.docs {
            let mut doc_terms: IndexMap<String, f32> = IndexMap::new();
            let terms = text_utils::tokenize(&doc.input);

            for term in &terms {
                *doc_terms.entry(term.clone()).or_insert(0.0) += 1.0;
            }

            let unique_terms: HashSet<String> = terms.into_iter().collect();
            for term in unique_terms {
                *self.term_df.entry(term).or_insert(0.0) += 1.0;
            }

            self.doc_term_freq.push(doc_terms);
        }
    }

    /// Calculates BM25 score between query terms and a specific document
    fn bm25_score(&self, query_terms: &[String], doc_idx: usize) -> f32 {
        // Calculate average document length
        let avg_doc_len: f32 = self
            .doc_term_freq
            .iter()
            .map(|doc| doc.values().sum::<f32>())
            .sum::<f32>()
            / self.doc_count;

        // Length of the current document
        let doc_len: f32 = self.doc_term_freq[doc_idx].values().sum();

        query_terms
            .iter()
            .map(|term| {
                // Check if term exists in the document frequency index
                if let Some(&df) = self.term_df.get(term) {
                    // Inverse Document Frequency (IDF) component
                    let idf = (self.doc_count - df + 0.5) / (df + 0.5);
                    let idf = (1.0 + idf).ln();

                    // Term Frequency (TF) with BM25 normalization
                    let tf = self.doc_term_freq[doc_idx]
                        .get(term)
                        .cloned()
                        .unwrap_or(0.0);

                    // BM25 scoring formula
                    let numerator = tf * (self.bm25_k1 + 1.0);
                    let denominator = tf
                        + self.bm25_k1 * (1.0 - self.bm25_b + self.bm25_b * doc_len / avg_doc_len);

                    idf * numerator / denominator
                } else {
                    0.0
                }
            })
            .sum()
    }
}

impl Agent for TfidfAgent {
    /// Predicts the most relevant response using BM25 scoring
    fn predict(&self, input: &str) -> ResponseFormat {
        if self.docs.is_empty() {
            return ResponseFormat::Text("No training data available.".to_string());
        }

        let query_terms = text_utils::tokenize(input);
        let scores = self.scored_docs(&query_terms);

        if let Some(&(best_idx, score)) = scores.first() {
            if score > 0.0 {
                return self.docs[best_idx].output.clone();
            }
        }

        ResponseFormat::Text("No matching answer found.".to_string())
    }
}

impl ConfidenceAgent for TfidfAgent {
    /// Calculates a confidence score for the best-matching document (0.0 - 1.0)
    fn calculate_confidence(&self, input: &str) -> f32 {
        if self.docs.is_empty() {
            return 0.0;
        }

        let query_terms = text_utils::tokenize(input);
        if query_terms.is_empty() {
            return 0.0;
        }

        let scores = self.scored_docs(&query_terms);
        match scores.first() {
            Some(&(_, score)) if score > 0.0 => (1.0_f32).min(score / (query_terms.len() as f32)),
            _ => 0.0,
        }
    }

    /// Returns the top N predictions with confidence scores
    fn predict_top_n(&self, input: &str, n: usize) -> Vec<PredictionResult> {
        if self.docs.is_empty() || n == 0 {
            return Vec::new();
        }

        let query_terms = text_utils::tokenize(input);
        let scores = self.scored_docs(&query_terms);
        let normalizer = (query_terms.len() as f32).max(1.0);

        scores
            .into_iter()
            .filter(|&(_, score)| score > 0.0)
            .take(n)
            .map(|(idx, score)| PredictionResult {
                response: self.docs[idx].output.clone(),
                confidence: (1.0_f32).min(score / normalizer),
                metadata: None,
            })
            .collect()
    }
}

impl TrainableAgent for TfidfAgent {
    /// Trains the agent by replacing all training documents
    fn train(&mut self, data: &[TrainingExample]) {
        self.docs = data.to_vec();
        self.rebuild_index();
    }

    /// Appends training documents without replacing existing data
    fn append(&mut self, data: &[TrainingExample]) {
        self.docs.extend_from_slice(data);
        self.rebuild_index();
    }
}

// Default implementation for creating a new TF-IDF agent
impl Default for TfidfAgent {
    fn default() -> Self {
        Self::new()
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

    fn tech_data() -> Vec<TrainingExample> {
        vec![
            make_example("rust programming language", "Rust is a systems programming language"),
            make_example("python scripting language", "Python is great for scripting"),
            make_example("javascript web development", "JavaScript runs in the browser"),
        ]
    }

    // === Basic Query Tests ===

    #[test]
    fn test_exact_query() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let result = String::from(agent.predict("rust programming language"));
        assert!(result.contains("Rust"));
    }

    #[test]
    fn test_partial_query() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let result = String::from(agent.predict("rust"));
        assert!(result.contains("Rust"));
    }

    #[test]
    fn test_no_match() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let result = String::from(agent.predict("quantum physics"));
        assert!(result.contains("No matching answer found"));
    }

    #[test]
    fn test_empty_training() {
        let agent = TfidfAgent::new();
        let result = String::from(agent.predict("hello"));
        assert!(result.contains("No training data available"));
    }

    #[test]
    fn test_single_term() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let result = String::from(agent.predict("python"));
        assert!(result.contains("Python"));
    }

    #[test]
    fn test_stopwords_in_query() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        // "the" and "is" and "a" are filtered by tokenizer (alphabetic only)
        // but "rust" should still match
        let result = String::from(agent.predict("the rust language"));
        assert!(result.contains("Rust"));
    }

    #[test]
    fn test_case_insensitive() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let result = String::from(agent.predict("RUST PROGRAMMING"));
        assert!(result.contains("Rust"));
    }

    #[test]
    fn test_empty_query() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let result = String::from(agent.predict(""));
        assert!(result.contains("No matching answer found"));
    }

    #[test]
    fn test_punctuation_handling() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        // Punctuation gets stripped by tokenizer
        let result = String::from(agent.predict("rust!!! programming???"));
        assert!(result.contains("Rust"));
    }

    // === BM25 Parameter Tests ===

    #[test]
    fn test_bm25_default_params() {
        let agent = TfidfAgent::new();
        assert_eq!(agent.bm25_k1, 1.2);
        assert_eq!(agent.bm25_b, 0.75);
    }

    #[test]
    fn test_bm25_custom_params() {
        let agent = TfidfAgent::new().with_bm25_params(2.0, 0.5);
        assert_eq!(agent.bm25_k1, 2.0);
        assert_eq!(agent.bm25_b, 0.5);
    }

    #[test]
    fn test_bm25_b_zero_no_length_norm() {
        let mut agent = TfidfAgent::new().with_bm25_params(1.2, 0.0);
        agent.train(&tech_data());
        // Should still produce results (no length normalization)
        let result = String::from(agent.predict("rust"));
        assert!(result.contains("Rust"));
    }

    // === Weight Tests ===

    #[test]
    fn test_weight_affects_ranking() {
        let mut agent = TfidfAgent::new();
        let data = vec![
            TrainingExample {
                input: "programming language".to_string(),
                output: ResponseFormat::Text("low weight".to_string()),
                weight: 0.1,
                metadata: None,
            },
            TrainingExample {
                input: "programming language".to_string(),
                output: ResponseFormat::Text("high weight".to_string()),
                weight: 10.0,
                metadata: None,
            },
        ];
        agent.train(&data);
        let result = String::from(agent.predict("programming language"));
        assert_eq!(result, "high weight");
    }

    #[test]
    fn test_zero_weight_suppresses() {
        let mut agent = TfidfAgent::new();
        let data = vec![
            TrainingExample {
                input: "rust systems".to_string(),
                output: ResponseFormat::Text("suppressed".to_string()),
                weight: 0.0,
                metadata: None,
            },
            make_example("python scripting", "Python answer"),
        ];
        agent.train(&data);
        // "rust systems" has weight 0.0, so score * 0.0 = 0.0
        // "python scripting" shares no terms with query, so score = 0.0
        let result = String::from(agent.predict("rust systems"));
        assert!(result.contains("No matching answer found"));
    }

    // === Train Behavior Tests ===

    #[test]
    fn test_train_replaces_data() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        assert!(String::from(agent.predict("rust")).contains("Rust"));

        // Retrain with different data
        agent.train(&[make_example("golang concurrency", "Go uses goroutines")]);
        let result = String::from(agent.predict("rust"));
        assert!(result.contains("No matching answer found"));
        assert!(String::from(agent.predict("golang")).contains("goroutines"));
    }

    #[test]
    fn test_append_preserves_existing() {
        let mut agent = TfidfAgent::new();
        agent.train(&[make_example("rust programming", "Rust answer")]);
        agent.append(&[make_example("python scripting", "Python answer")]);
        assert!(String::from(agent.predict("rust")).contains("Rust"));
        assert!(String::from(agent.predict("python")).contains("Python"));
    }

    #[test]
    fn test_train_single_appends() {
        let mut agent = TfidfAgent::new();
        agent.train(&[make_example("rust programming", "Rust answer")]);
        agent.train_single(&make_example("golang concurrency", "Go answer"));
        assert!(String::from(agent.predict("rust")).contains("Rust"));
        assert!(String::from(agent.predict("golang")).contains("Go"));
    }

    #[test]
    fn test_single_document() {
        let mut agent = TfidfAgent::new();
        agent.train(&[make_example("hello world", "greeting")]);
        assert_eq!(String::from(agent.predict("hello")), "greeting");
    }

    #[test]
    fn test_default_trait() {
        let agent = TfidfAgent::default();
        assert_eq!(agent.bm25_k1, 1.2);
        assert_eq!(agent.bm25_b, 0.75);
    }

    // === ConfidenceAgent Tests ===

    #[test]
    fn test_calculate_confidence_match() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let conf = agent.calculate_confidence("rust programming language");
        assert!(conf > 0.0);
        assert!(conf <= 1.0);
    }

    #[test]
    fn test_calculate_confidence_no_match() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let conf = agent.calculate_confidence("quantum physics");
        assert_eq!(conf, 0.0);
    }

    #[test]
    fn test_calculate_confidence_empty_training() {
        let agent = TfidfAgent::new();
        assert_eq!(agent.calculate_confidence("anything"), 0.0);
    }

    #[test]
    fn test_calculate_confidence_empty_query() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        assert_eq!(agent.calculate_confidence(""), 0.0);
    }

    #[test]
    fn test_predict_top_n() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let results = agent.predict_top_n("programming language", 3);
        assert!(!results.is_empty());
        assert!(results.len() <= 3);
        // Results should be sorted by confidence descending
        for window in results.windows(2) {
            assert!(window[0].confidence >= window[1].confidence);
        }
    }

    #[test]
    fn test_predict_top_n_limits_results() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let results = agent.predict_top_n("programming language", 1);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_predict_top_n_empty() {
        let agent = TfidfAgent::new();
        let results = agent.predict_top_n("anything", 5);
        assert!(results.is_empty());
    }

    #[test]
    fn test_predict_top_n_zero() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let results = agent.predict_top_n("rust", 0);
        assert!(results.is_empty());
    }

    #[test]
    fn test_predict_top_n_no_match() {
        let mut agent = TfidfAgent::new();
        agent.train(&tech_data());
        let results = agent.predict_top_n("quantum physics", 5);
        assert!(results.is_empty());
    }
}
