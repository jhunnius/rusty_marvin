//! Error types for the poker evaluator system

use std::fmt;

/// Errors that can occur during hand evaluation
#[derive(Debug, Clone, PartialEq)]
pub enum EvaluatorError {
    /// Invalid card combination or hand
    InvalidHand(String),
    /// Table initialization failed
    TableInitFailed(String),
    /// File I/O error
    FileIoError(String),
    /// Invalid rank or suit value
    InvalidCardValue(String),
    /// Memory allocation error
    MemoryAllocationError(String),
    /// Evaluation algorithm error
    EvaluationError(String),
}

impl fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluatorError::InvalidHand(msg) => write!(f, "Invalid hand: {}", msg),
            EvaluatorError::TableInitFailed(msg) => {
                write!(f, "Table initialization failed: {}", msg)
            }
            EvaluatorError::FileIoError(msg) => write!(f, "File I/O error: {}", msg),
            EvaluatorError::InvalidCardValue(msg) => write!(f, "Invalid card value: {}", msg),
            EvaluatorError::MemoryAllocationError(msg) => {
                write!(f, "Memory allocation error: {}", msg)
            }
            EvaluatorError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
        }
    }
}

impl std::error::Error for EvaluatorError {}

impl EvaluatorError {
    /// Create a new invalid hand error
    pub fn invalid_hand(msg: &str) -> Self {
        EvaluatorError::InvalidHand(msg.to_string())
    }

    /// Create a new table initialization error
    pub fn table_init_failed(msg: &str) -> Self {
        EvaluatorError::TableInitFailed(msg.to_string())
    }

    /// Create a new file I/O error
    pub fn file_io_error(msg: &str) -> Self {
        EvaluatorError::FileIoError(msg.to_string())
    }

    /// Create a new invalid card value error
    pub fn invalid_card_value(msg: &str) -> Self {
        EvaluatorError::InvalidCardValue(msg.to_string())
    }

    /// Create a new memory allocation error
    pub fn memory_allocation_error(msg: &str) -> Self {
        EvaluatorError::MemoryAllocationError(msg.to_string())
    }

    /// Create a new evaluation error
    pub fn evaluation_error(msg: &str) -> Self {
        EvaluatorError::EvaluationError(msg.to_string())
    }
}

impl From<std::io::Error> for EvaluatorError {
    fn from(err: std::io::Error) -> Self {
        EvaluatorError::FileIoError(err.to_string())
    }
}
