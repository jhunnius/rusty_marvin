//! # Poker API Library
//!
//! A high-performance poker hand evaluation library based on the Meerkat API algorithm.
//! This library provides lightning-fast poker hand strength evaluation using precomputed
//! lookup tables and perfect hashing.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use poker_api::hand_evaluator::LookupHandEvaluator;
//! use poker_api::api::hand::Hand;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let evaluator = LookupHandEvaluator::new()?;
//! let mut hand = Hand::new();
//! // ... add cards to hand
//! let rank = evaluator.rank_hand(&hand);
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - **Blazing Fast**: O(1) hand evaluation using perfect hash lookup tables
//! - **Memory Efficient**: ~128MB for complete evaluation tables
//! - **Standards Compliant**: Based on proven poker evaluation algorithms
//! - **Multi-Hand Support**: Evaluate 5, 6, or 7-card poker hands
//! - **Texas Hold'em Ready**: Optimized for Texas Hold'em hand evaluation

mod simple_player;
mod texas_holdem;
mod toml_preferences;

/// Core poker API types and traits
pub mod api;

/// High-performance poker hand evaluator
pub mod hand_evaluator;

/// Table generation algorithms for poker hand evaluation
pub mod evaluator_generator;

#[cfg(test)]
mod tests {}
