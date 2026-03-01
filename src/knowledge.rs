// src/knowledge.rs - Unified Knowledge Base
use crate::agent::{AgentError, LegacyTrainingExample, ResponseFormat, TrainingExample};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

/// Supports both legacy and modern training data formats for backward compatibility
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum TrainingData {
    Legacy(Vec<LegacyTrainingExample>),
    Modern(Vec<TrainingExample>),
}

/// Represents a flexible knowledge base for storing and managing training examples
#[derive(Clone)]
pub struct KnowledgeBase {
    examples: Vec<TrainingExample>,
    file_path: Option<PathBuf>,
}

/// Compile-time embedded training data
pub static EMBEDDED_DATA: Lazy<Arc<Vec<TrainingExample>>> = Lazy::new(|| {
    let raw = include_str!(concat!(env!("OUT_DIR"), "/train.json"));

    match serde_json::from_str::<TrainingData>(raw) {
        Ok(TrainingData::Modern(examples)) => Arc::new(examples),
        Ok(TrainingData::Legacy(legacy)) => {
            // Converts legacy data into the modern format
            Arc::new(legacy.into_iter().map(|ex| ex.into()).collect())
        }
        Err(e) => {
            eprintln!("Error loading embedded training data: {}", e);
            Arc::new(Vec::new())
        }
    }
});

impl KnowledgeBase {
    /// Creates an empty knowledge base
    pub fn new() -> Self {
        Self {
            examples: Vec::new(),
            file_path: None,
        }
    }

    /// Creates a knowledge base from embedded data
    pub fn from_embedded() -> Self {
        Self {
            examples: EMBEDDED_DATA.to_vec(),
            file_path: None,
        }
    }

    /// Loads a knowledge base from a JSON file
    pub fn load(path: PathBuf) -> Result<Self, AgentError> {
        let data = fs::read_to_string(&path)?;

        match serde_json::from_str::<TrainingData>(&data) {
            Ok(TrainingData::Modern(examples)) => Ok(Self {
                examples,
                file_path: Some(path),
            }),
            Ok(TrainingData::Legacy(legacy)) => {
                // Converts legacy data
                Ok(Self {
                    examples: legacy.into_iter().map(|ex| ex.into()).collect(),
                    file_path: Some(path),
                })
            }
            Err(e) => Err(AgentError::SerializationError(e)),
        }
    }

    /// Saves the knowledge base to a JSON file
    pub fn save(&self, path: Option<PathBuf>) -> Result<(), AgentError> {
        let path = path
            .or_else(|| self.file_path.clone())
            .ok_or_else(|| AgentError::InvalidInputError("No path provided".to_string()))?;

        let json = serde_json::to_string_pretty(&self.examples)?;

        let mut file = File::create(&path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    /// Adds a new training example to the knowledge base
    pub fn add_example(&mut self, input: String, output: impl Into<ResponseFormat>, weight: f32) {
        let example = TrainingExample {
            input,
            output: output.into(),
            weight,
            metadata: None, // Standardmäßig keine Metadaten
        };
        self.examples.push(example);
    }

    /// Removes a training example by its index
    pub fn remove_example(&mut self, index: usize) -> Result<TrainingExample, AgentError> {
        if index < self.examples.len() {
            Ok(self.examples.remove(index))
        } else {
            Err(AgentError::IndexOutOfBounds(index))
        }
    }

    /// Returns a reference to all training examples
    pub fn get_examples(&self) -> &[TrainingExample] {
        &self.examples
    }

    /// Merges another knowledge base into the current one
    pub fn merge(&mut self, other: &KnowledgeBase) {
        self.examples.extend_from_slice(&other.examples);
    }

    /// Merges embedded data into the current knowledge base
    pub fn merge_embedded(&mut self) {
        self.examples.extend_from_slice(&EMBEDDED_DATA);
    }
}

// Default implementation for creating a new knowledge base
impl Default for KnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // === Constructor Tests ===

    #[test]
    fn test_new_creates_empty() {
        let kb = KnowledgeBase::new();
        assert!(kb.get_examples().is_empty());
    }

    #[test]
    fn test_default_creates_empty() {
        let kb = KnowledgeBase::default();
        assert!(kb.get_examples().is_empty());
    }

    #[test]
    fn test_from_embedded() {
        let kb = KnowledgeBase::from_embedded();
        // from_embedded should load data (may be empty if build has no training data)
        // Just verify it doesn't panic and returns a valid KnowledgeBase
        let _ = kb.get_examples();
    }

    // === CRUD Tests ===

    #[test]
    fn test_add_example() {
        let mut kb = KnowledgeBase::new();
        kb.add_example("hello".to_string(), "world", 1.0);
        assert_eq!(kb.get_examples().len(), 1);
        assert_eq!(kb.get_examples()[0].input, "hello");
        assert_eq!(String::from(kb.get_examples()[0].output.clone()), "world");
    }

    #[test]
    fn test_add_multiple_examples() {
        let mut kb = KnowledgeBase::new();
        kb.add_example("q1".to_string(), "a1", 1.0);
        kb.add_example("q2".to_string(), "a2", 2.0);
        assert_eq!(kb.get_examples().len(), 2);
        assert_eq!(kb.get_examples()[1].weight, 2.0);
    }

    #[test]
    fn test_remove_valid_index() {
        let mut kb = KnowledgeBase::new();
        kb.add_example("q1".to_string(), "a1", 1.0);
        kb.add_example("q2".to_string(), "a2", 1.0);
        let removed = kb.remove_example(0).unwrap();
        assert_eq!(removed.input, "q1");
        assert_eq!(kb.get_examples().len(), 1);
        assert_eq!(kb.get_examples()[0].input, "q2");
    }

    #[test]
    fn test_remove_invalid_index() {
        let mut kb = KnowledgeBase::new();
        kb.add_example("q1".to_string(), "a1", 1.0);
        let result = kb.remove_example(5);
        assert!(result.is_err());
        match result.unwrap_err() {
            AgentError::IndexOutOfBounds(idx) => assert_eq!(idx, 5),
            other => panic!("Expected IndexOutOfBounds, got: {}", other),
        }
    }

    #[test]
    fn test_get_examples_order() {
        let mut kb = KnowledgeBase::new();
        kb.add_example("first".to_string(), "1", 1.0);
        kb.add_example("second".to_string(), "2", 1.0);
        kb.add_example("third".to_string(), "3", 1.0);
        assert_eq!(kb.get_examples()[0].input, "first");
        assert_eq!(kb.get_examples()[1].input, "second");
        assert_eq!(kb.get_examples()[2].input, "third");
    }

    // === Merge Tests ===

    #[test]
    fn test_merge_into_empty() {
        let mut kb1 = KnowledgeBase::new();
        let mut kb2 = KnowledgeBase::new();
        kb2.add_example("q1".to_string(), "a1", 1.0);

        kb1.merge(&kb2);
        assert_eq!(kb1.get_examples().len(), 1);
        assert_eq!(kb1.get_examples()[0].input, "q1");
    }

    #[test]
    fn test_merge_both_populated() {
        let mut kb1 = KnowledgeBase::new();
        kb1.add_example("q1".to_string(), "a1", 1.0);

        let mut kb2 = KnowledgeBase::new();
        kb2.add_example("q2".to_string(), "a2", 1.0);

        kb1.merge(&kb2);
        assert_eq!(kb1.get_examples().len(), 2);
        assert_eq!(kb1.get_examples()[0].input, "q1");
        assert_eq!(kb1.get_examples()[1].input, "q2");
    }

    #[test]
    fn test_merge_empty_into_populated() {
        let mut kb1 = KnowledgeBase::new();
        kb1.add_example("q1".to_string(), "a1", 1.0);
        let kb2 = KnowledgeBase::new();

        kb1.merge(&kb2);
        assert_eq!(kb1.get_examples().len(), 1);
    }

    #[test]
    fn test_merge_preserves_order() {
        let mut kb1 = KnowledgeBase::new();
        kb1.add_example("a".to_string(), "1", 1.0);
        kb1.add_example("b".to_string(), "2", 1.0);

        let mut kb2 = KnowledgeBase::new();
        kb2.add_example("c".to_string(), "3", 1.0);
        kb2.add_example("d".to_string(), "4", 1.0);

        kb1.merge(&kb2);
        let inputs: Vec<&str> = kb1.get_examples().iter().map(|e| e.input.as_str()).collect();
        assert_eq!(inputs, vec!["a", "b", "c", "d"]);
    }

    // === File I/O Tests ===

    #[test]
    fn test_save_load_roundtrip() {
        let mut kb = KnowledgeBase::new();
        kb.add_example("hello".to_string(), "world", 1.5);
        kb.add_example("foo".to_string(), "bar", 2.0);

        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_path_buf();

        kb.save(Some(path.clone())).unwrap();

        let loaded = KnowledgeBase::load(path).unwrap();
        assert_eq!(loaded.get_examples().len(), 2);
        assert_eq!(loaded.get_examples()[0].input, "hello");
        assert_eq!(loaded.get_examples()[1].weight, 2.0);
    }

    #[test]
    fn test_load_legacy_format() {
        let legacy_json = r#"[
            {"input": "hello", "output": "world", "weight": 1.0},
            {"input": "foo", "output": "bar", "weight": 2.0}
        ]"#;

        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_all(legacy_json.as_bytes()).unwrap();
        tmp.flush().unwrap();

        let kb = KnowledgeBase::load(tmp.path().to_path_buf()).unwrap();
        assert_eq!(kb.get_examples().len(), 2);
        assert_eq!(kb.get_examples()[0].input, "hello");
        assert_eq!(String::from(kb.get_examples()[0].output.clone()), "world");
    }

    #[test]
    fn test_load_invalid_json() {
        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_all(b"not valid json!!!").unwrap();
        tmp.flush().unwrap();

        let result = KnowledgeBase::load(tmp.path().to_path_buf());
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(matches!(err, AgentError::SerializationError(_)));
    }

    #[test]
    fn test_save_no_path() {
        let kb = KnowledgeBase::new();
        let result = kb.save(None);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(matches!(err, AgentError::InvalidInputError(_)));
    }
}
