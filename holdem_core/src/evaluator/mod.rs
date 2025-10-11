//! # Evaluator Module
//!
//! High-performance poker hand evaluation system with lookup tables

/// Core evaluator functionality
pub mod evaluator;

/// Lookup table structures and algorithms
pub mod tables;

/// Thread-safe singleton pattern implementation
pub mod singleton;

/// Comprehensive error handling
pub mod errors;

/// Integration utilities for holdem_core types
pub mod integration;

/// Atomic file I/O operations with checksum validation
pub mod file_io;

/// Property-based testing framework
#[cfg(test)]
pub mod property_tests;

/// Usage examples and demonstrations
pub mod examples;

/// Performance benchmarks and profiling
pub mod benchmarks;

// Re-export key types for convenience
pub use evaluator::{Evaluator, HandRank, HandValue};
pub use file_io::{LutFileManager, TableInfo, TableType};
pub use integration::{HandEvaluation, HandEvaluator, HoleCardsEvaluation};
pub use singleton::EvaluatorSingleton;
