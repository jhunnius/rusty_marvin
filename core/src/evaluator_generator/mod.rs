//! # Poker Hand Evaluation Table Generator Module
//!
//! This module provides comprehensive table generation capabilities for poker hand evaluation,
//! specifically designed for poker bot testing frameworks. It implements the Java Meerkat
//! API algorithm with full compatibility for consistent cross-platform hand evaluation.
//!
//! ## Poker Testbed Purpose
//!
//! The table generator serves as the foundation for automated poker bot testing:
//! - **Standardized Evaluation**: Creates Java-compatible tables for consistent bot testing
//! - **Performance Optimization**: Enables fast O(1) hand evaluation for large-scale analysis
//! - **Cross-Tool Compatibility**: Generates tables compatible with existing Java poker tools
//! - **Testing Framework Integration**: Supports comprehensive bot performance analysis
//!
//! ## Module Structure
//!
//! - [`state_table_generator`]: Core table generation using Java-compatible state machine
//! - [`flushes`]: Specialized algorithms for flush hand detection and ranking
//! - [`pair`]: Pair and two-pair hand evaluation logic
//! - [`products`]: Mathematical utilities for hand strength calculation
//! - [`unique`]: Unique hand type detection algorithms
//! - [`values`]: Hand value computation and ranking utilities
//!
//! ## Java Compatibility
//!
//! All modules maintain strict compatibility with the Java Meerkat API:
//! - Card encoding uses Java-style 8-bit format (rrrr-sss)
//! - Ranking system matches Java values (1 = best, 7462 = worst)
//! - Algorithm implementations follow Java reference patterns
//! - Generated tables are binary-compatible with Java tools
//!
//! ## Bot Testing Integration
//!
//! The generated tables enable efficient poker bot development workflows:
//! - Fast hand evaluation for real-time bot decision making
//! - Deterministic results for reproducible bot testing
//! - Memory-efficient table loading for testing environments
//! - Incremental evaluation for complex multi-street analysis

pub mod state_table_generator;

pub mod flushes;
pub mod pair;
pub mod products;
pub mod unique;
pub mod values;
