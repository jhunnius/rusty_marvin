//! # Hand Evaluation Module
//!
//! This module provides comprehensive poker hand evaluation capabilities,
//! including both traditional lookup table approaches and advanced jump table
//! implementations for optimal performance.
//!
//! ## Architecture Overview
//!
//! The evaluator module is organized into several sub-modules:
//!
//! - **`tables`**: Lookup table implementations and jump table structures
//! - **`integration`**: Integration utilities and compatibility layers
//! - **`property_tests`**: Property-based testing for evaluation correctness
//! - **`examples`**: Usage examples and performance benchmarks

pub mod errors;
pub mod evaluator;
pub mod examples;
pub mod file_io;
pub mod integration;
pub mod property_tests;
pub mod singleton;
pub mod tables;

// Re-export commonly used types from local modules
pub use errors::EvaluatorError;
pub use evaluator::{Evaluator, HandRank, HandValue};

// Re-export math-specific types
pub use tables::JumpTable;

// Module-level constants
pub const MAX_CARDS_IN_HAND: usize = 7;
pub const TOTAL_CARDS_IN_DECK: usize = 52;
pub const CARDS_PER_SUIT: usize = 13;

/// Version of the evaluator system
pub const EVALUATOR_VERSION: &str = "2.0.0";
