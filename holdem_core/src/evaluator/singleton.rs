//! Singleton pattern implementation for the poker evaluator

use super::errors::EvaluatorError;
use super::Evaluator;
use std::sync::{Arc, OnceLock};

/// Singleton instance of the poker evaluator
#[derive(Debug)]
pub struct EvaluatorSingleton {
    /// The underlying evaluator instance
    evaluator: Arc<Evaluator>,
}

impl EvaluatorSingleton {
    /// Get the global evaluator instance
    pub fn instance() -> Arc<Evaluator> {
        static INSTANCE: OnceLock<EvaluatorSingleton> = OnceLock::new();
        let singleton = INSTANCE.get_or_init(|| EvaluatorSingleton {
            evaluator: Arc::new(Evaluator::new().expect("Failed to create evaluator")),
        });
        singleton.evaluator.clone()
    }

    /// Create a new singleton instance (mainly for testing)
    pub fn new() -> Result<Self, EvaluatorError> {
        Ok(EvaluatorSingleton {
            evaluator: Arc::new(Evaluator::new()?),
        })
    }

    /// Get a reference to the evaluator
    pub fn evaluator(&self) -> &Evaluator {
        &self.evaluator
    }
}
