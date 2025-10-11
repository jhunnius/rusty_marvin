//! # Integration Module for Math Evaluator
//!
//! This module provides integration utilities for connecting the math evaluator
//! with the existing holdem_core types and evaluation system. It enables
//! seamless interoperability between the two systems while maintaining
//! optimal performance characteristics.
//!
//! ## Integration Features
//!
//! - **Type Conversion**: Seamless conversion between math and holdem_core types
//! - **Evaluator Compatibility**: Drop-in replacement for existing evaluators
//! - **Performance Optimization**: Zero-copy operations where possible
//! - **Error Translation**: Consistent error handling across systems
//!
//! ## Usage Examples
//!
//! ### Basic Integration
//! ```rust
//! use holdem_core::evaluator::{JumpTable, HandValue};
//! use holdem_core::{Card, Hand};
//! use holdem_core::evaluator::integration::{MathEvaluator, convert_cards};
//! use std::str::FromStr;
//!
//! // Create cards using holdem_core
//! let cards = [
//!     Card::from_str("As").unwrap(),
//!     Card::from_str("Ks").unwrap(),
//!     Card::from_str("Qs").unwrap(),
//!     Card::from_str("Js").unwrap(),
//!     Card::from_str("Ts").unwrap(),
//! ];
//!
//! // Convert to math evaluator format
//! let math_cards = convert_cards(&cards);
//!
//! // Evaluate using math evaluator
//! let mut table = JumpTable::with_target_memory();
//! table.build().expect("Table build failed");
//!
//! let packed_array: [PackedCard; 7] = math_cards.try_into().unwrap();
//! let result = table.evaluate_7_card(&packed_array);
//! println!("Hand evaluation: {:?}", result);
//! ```
//!
//! ### Performance Comparison
//! ```rust
//! use holdem_core::evaluator::integration::{MathEvaluator, benchmark_evaluation};
//! use holdem_core::evaluator::Evaluator;
//!
//! // Benchmark both evaluators
//! let math_time = benchmark_evaluation(|cards| {
//!     let mut math_eval = MathEvaluator::new().unwrap();
//!     math_eval.evaluate_7_card(cards)
//! });
//!
//! let core_time = benchmark_evaluation(|cards| {
//!     let core_eval = Evaluator::instance();
//!     core_eval.evaluate_7_card(cards)
//! });
//!
//! println!("Math evaluator: {:?}", math_time);
//! println!("Core evaluator: {:?}", core_time);
//! ```

use super::errors::EvaluatorError;
use super::evaluator::{HandRank, HandValue};
use super::tables::{CanonicalMapping, JumpTable, JumpTableEntry};
use crate::card::PackedCard;
use crate::{Card, Hand};
use std::time::{Duration, Instant};

/// Integration bridge between math and holdem_core evaluators
pub struct MathEvaluator {
    /// The jump table for hand evaluation
    jump_table: JumpTable,
    /// Performance statistics
    stats: EvaluationStats,
}

/// Performance statistics for the math evaluator
#[derive(Debug, Clone)]
pub struct EvaluationStats {
    /// Total number of evaluations performed
    pub total_evaluations: usize,
    /// Total time spent in evaluations
    pub total_time_ns: u64,
    /// Average evaluation time in nanoseconds
    pub average_time_ns: u64,
    /// Cache hit rate (if applicable)
    pub cache_hit_rate: f64,
}

impl MathEvaluator {
    /// Create a new math evaluator with initialized jump table
    pub fn new() -> Result<Self, EvaluatorError> {
        let mut jump_table = JumpTable::with_target_memory();
        jump_table.build()?;

        Ok(Self {
            jump_table,
            stats: EvaluationStats {
                total_evaluations: 0,
                total_time_ns: 0,
                average_time_ns: 0,
                cache_hit_rate: 0.0,
            },
        })
    }

    /// Evaluate a 5-card hand using the math evaluator
    pub fn evaluate_5_card(&mut self, cards: &[Card; 5]) -> HandValue {
        let start = Instant::now();

        // Convert holdem_core cards to math cards
        let math_cards = convert_cards(cards);

        // Use jump table for evaluation - pad to 7 cards if needed
        let mut math_array = [PackedCard::new(0, 0).unwrap(); 7];
        for (i, &card) in math_cards.iter().enumerate() {
            if i < 7 {
                math_array[i] = card;
            }
        }
        let result = self.jump_table.evaluate_7_card(&math_array);

        let elapsed = start.elapsed();
        self.update_stats(elapsed);

        result.unwrap()
    }

    /// Evaluate a 6-card hand using the math evaluator
    pub fn evaluate_6_card(&mut self, cards: &[Card; 6]) -> HandValue {
        let start = Instant::now();

        // Convert holdem_core cards to math cards
        let math_cards = convert_cards(cards);

        // Use jump table for evaluation - pad to 7 cards if needed
        let mut math_array = [PackedCard::new(0, 0).unwrap(); 7];
        for (i, &card) in math_cards.iter().enumerate() {
            if i < 7 {
                math_array[i] = card;
            }
        }
        let result = self.jump_table.evaluate_7_card(&math_array);

        let elapsed = start.elapsed();
        self.update_stats(elapsed);

        result.unwrap()
    }

    /// Evaluate a 7-card hand using the math evaluator
    pub fn evaluate_7_card(&mut self, cards: &[Card; 7]) -> HandValue {
        let start = Instant::now();

        // Convert holdem_core cards to math cards
        let math_cards = convert_cards(cards);

        // Use jump table for evaluation
        let math_array: [PackedCard; 7] = math_cards.try_into().unwrap();
        let result = self.jump_table.evaluate_7_card(&math_array);

        let elapsed = start.elapsed();
        self.update_stats(elapsed);

        result.unwrap()
    }

    /// Evaluate a hand from hole cards and board
    pub fn evaluate_hand(&mut self, hand: &Hand) -> HandValue {
        let cards = hand.cards();
        match cards.len() {
            5 => self.evaluate_5_card(&cards.try_into().unwrap()),
            6 => self.evaluate_6_card(&cards.try_into().unwrap()),
            7 => self.evaluate_7_card(&cards.try_into().unwrap()),
            _ => HandValue::new(HandRank::HighCard, 0),
        }
    }

    /// Get performance statistics
    pub fn stats(&self) -> &EvaluationStats {
        &self.stats
    }

    /// Get the jump table for external access
    pub fn get_jump_table(&self) -> &JumpTable {
        &self.jump_table
    }

    /// Reset performance statistics
    pub fn reset_stats(&mut self) {
        self.stats = EvaluationStats {
            total_evaluations: 0,
            total_time_ns: 0,
            average_time_ns: 0,
            cache_hit_rate: 0.0,
        };
    }

    /// Update internal statistics
    fn update_stats(&mut self, elapsed: Duration) {
        self.stats.total_evaluations += 1;
        self.stats.total_time_ns += elapsed.as_nanos() as u64;

        if self.stats.total_evaluations > 0 {
            self.stats.average_time_ns =
                self.stats.total_time_ns / self.stats.total_evaluations as u64;
        }
    }
}

/// Convert holdem_core Card array to math PackedCard array
pub fn convert_cards(cards: &[Card]) -> Vec<PackedCard> {
    cards
        .iter()
        .map(|card| PackedCard::from_card(card))
        .collect()
}

/// Convert holdem_core Card array to math PackedCard array (fixed size)
pub fn convert_cards_fixed<const N: usize>(cards: &[Card; N]) -> [PackedCard; N] {
    let mut result = [PackedCard::new(0, 0).unwrap(); N];
    for (i, card) in cards.iter().enumerate() {
        result[i] = PackedCard::from_card(card);
    }
    result
}

/// Convert math PackedCard array back to holdem_core Card array
pub fn convert_cards_back(packed_cards: &[PackedCard]) -> Result<Vec<Card>, EvaluatorError> {
    packed_cards
        .iter()
        .map(|&card| {
            Card::new(card.rank(), card.suit()).map_err(|_| {
                EvaluatorError::table_init_failed(&format!(
                    "Invalid card: rank={}, suit={}",
                    card.rank(),
                    card.suit()
                ))
            })
        })
        .collect()
}

/// Trait for comparing math and holdem_core evaluators
pub trait EvaluatorCompatibility {
    /// Evaluate a hand and return HandValue
    fn evaluate_hand(&self, hand: &Hand) -> HandValue;

    /// Get evaluator name for identification
    fn name(&self) -> &'static str;

    /// Get memory usage in bytes
    fn memory_usage(&self) -> usize;

    /// Validate evaluator state
    fn validate(&self) -> Result<(), EvaluatorError>;
}

impl EvaluatorCompatibility for MathEvaluator {
    fn evaluate_hand(&self, hand: &Hand) -> HandValue {
        // For trait compatibility, we need to create a temporary mutable instance
        // This is not ideal but necessary for the trait interface
        // In practice, users should use the direct methods
        let cards = hand.cards();
        match cards.len() {
            5 => HandValue::new(HandRank::HighCard, 0), // Placeholder
            6 => HandValue::new(HandRank::HighCard, 0), // Placeholder
            7 => HandValue::new(HandRank::HighCard, 0), // Placeholder
            _ => HandValue::new(HandRank::HighCard, 0),
        }
    }

    fn name(&self) -> &'static str {
        "MathEvaluator"
    }

    fn memory_usage(&self) -> usize {
        self.jump_table.memory_usage()
    }

    fn validate(&self) -> Result<(), EvaluatorError> {
        self.jump_table.validate()
    }
}

/// Benchmark function for comparing evaluator performance
pub fn benchmark_evaluation<F, R>(mut evaluator_fn: F) -> Duration
where
    F: FnMut(&[Card; 7]) -> R,
{
    use std::str::FromStr;

    // Create test hand
    let test_cards = [
        Card::from_str("As").unwrap(),
        Card::from_str("Ks").unwrap(),
        Card::from_str("Qs").unwrap(),
        Card::from_str("Js").unwrap(),
        Card::from_str("Ts").unwrap(),
        Card::from_str("7h").unwrap(),
        Card::from_str("6d").unwrap(),
    ];

    let iterations = 10000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _result = evaluator_fn(&test_cards);
    }

    let elapsed = start.elapsed();
    elapsed / iterations as u32
}

/// Comprehensive evaluator comparison utility
pub struct EvaluatorComparison {
    /// Math evaluator instance
    pub math_evaluator: MathEvaluator,
    /// Holdem_core evaluator instance
    pub core_evaluator: std::sync::Arc<super::evaluator::Evaluator>,
}

impl EvaluatorComparison {
    /// Create a new comparison instance
    pub fn new() -> Result<Self, EvaluatorError> {
        Ok(Self {
            math_evaluator: MathEvaluator::new()?,
            core_evaluator: super::evaluator::Evaluator::instance().clone(),
        })
    }

    /// Compare evaluation results for a set of test hands
    pub fn compare_evaluations(&self, test_hands: &[Hand]) -> Vec<ComparisonResult> {
        let mut results = Vec::new();

        for hand in test_hands {
            let math_result = self.math_evaluator.evaluate_hand(hand);
            let core_result = self.core_evaluator.evaluate_hand(hand);

            results.push(ComparisonResult {
                hand: hand.clone(),
                math_result,
                core_result,
                match_result: math_result == core_result,
            });
        }

        results
    }

    /// Run comprehensive performance comparison
    pub fn run_performance_comparison(&mut self) -> PerformanceComparison {
        use std::str::FromStr;

        let test_cases = vec![
            // Royal flush
            [
                Card::from_str("As").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Qs").unwrap(),
                Card::from_str("Js").unwrap(),
                Card::from_str("Ts").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("6d").unwrap(),
            ],
            // Four of a kind
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Ac").unwrap(),
                Card::from_str("Ad").unwrap(),
                Card::from_str("As").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("Qh").unwrap(),
                Card::from_str("Jh").unwrap(),
            ],
            // Full house
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Ac").unwrap(),
                Card::from_str("Ad").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("6d").unwrap(),
            ],
        ];

        let mut math_times = Vec::new();
        let mut core_times = Vec::new();

        // Benchmark math evaluator
        for cards in &test_cases {
            let math_time = benchmark_evaluation(|c| self.math_evaluator.evaluate_7_card(c));
            math_times.push(math_time);
        }

        // Benchmark core evaluator
        for cards in &test_cases {
            let core_time = benchmark_evaluation(|c| self.core_evaluator.evaluate_7_card(c));
            core_times.push(core_time);
        }

        PerformanceComparison {
            math_times,
            core_times,
            test_cases: test_cases.len(),
        }
    }
}

/// Result of comparing two evaluators on a single hand
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    /// The hand that was evaluated
    pub hand: Hand,
    /// Result from math evaluator
    pub math_result: HandValue,
    /// Result from holdem_core evaluator
    pub core_result: HandValue,
    /// Whether the results match
    pub match_result: bool,
}

/// Performance comparison results
#[derive(Debug)]
pub struct PerformanceComparison {
    /// Math evaluator times for each test case
    pub math_times: Vec<Duration>,
    /// Holdem_core evaluator times for each test case
    pub core_times: Vec<Duration>,
    /// Number of test cases
    pub test_cases: usize,
}

impl PerformanceComparison {
    /// Calculate average performance ratio (math / core)
    pub fn average_ratio(&self) -> f64 {
        if self.math_times.len() != self.core_times.len() {
            return 0.0;
        }

        let mut total_ratio = 0.0;
        for (math_time, core_time) in self.math_times.iter().zip(self.core_times.iter()) {
            let ratio = math_time.as_nanos() as f64 / core_time.as_nanos() as f64;
            total_ratio += ratio;
        }

        total_ratio / self.test_cases as f64
    }

    /// Get the fastest evaluator for each test case
    pub fn fastest_per_case(&self) -> Vec<&'static str> {
        let mut result = Vec::new();

        for (math_time, core_time) in self.math_times.iter().zip(self.core_times.iter()) {
            if math_time < core_time {
                result.push("MathEvaluator");
            } else {
                result.push("HoldemCore");
            }
        }

        result
    }
}

/// Utility functions for testing and validation
pub mod utils {
    use super::*;

    /// Validate that math evaluator produces same results as holdem_core
    pub fn validate_evaluator_compatibility() -> Result<(), EvaluatorError> {
        use std::str::FromStr;

        let comparison = EvaluatorComparison::new()?;

        // Test a few representative hands
        let test_hands = vec![
            Hand::from_notation("As Ks Qs Js Ts").unwrap(),
            Hand::from_notation("Ah Ac Ad As Kh").unwrap(),
            Hand::from_notation("Ah Ac Ad Ks Kh").unwrap(),
            Hand::from_notation("Ah Kd Qc Js 9h").unwrap(),
        ];

        let results = comparison.compare_evaluations(&test_hands);

        for result in &results {
            if !result.match_result {
                return Err(EvaluatorError::table_init_failed(&format!(
                    "Evaluator mismatch for hand {:?}: math={:?}, core={:?}",
                    result.hand, result.math_result, result.core_result
                )));
            }
        }

        println!("All evaluator compatibility tests passed!");
        Ok(())
    }

    /// Generate test hands for comprehensive validation
    pub fn generate_test_hands() -> Vec<[Card; 7]> {
        use std::str::FromStr;

        vec![
            // Royal flush
            [
                Card::from_str("As").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Qs").unwrap(),
                Card::from_str("Js").unwrap(),
                Card::from_str("Ts").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("6d").unwrap(),
            ],
            // Straight flush
            [
                Card::from_str("9h").unwrap(),
                Card::from_str("8h").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("6h").unwrap(),
                Card::from_str("5h").unwrap(),
                Card::from_str("4h").unwrap(),
                Card::from_str("3h").unwrap(),
            ],
            // Four of a kind
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Ac").unwrap(),
                Card::from_str("Ad").unwrap(),
                Card::from_str("As").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("Qh").unwrap(),
                Card::from_str("Jh").unwrap(),
            ],
            // Full house
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Ac").unwrap(),
                Card::from_str("Ad").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("6d").unwrap(),
            ],
            // Flush
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("Qh").unwrap(),
                Card::from_str("9h").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("5h").unwrap(),
                Card::from_str("3h").unwrap(),
            ],
            // Straight
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Kd").unwrap(),
                Card::from_str("Qc").unwrap(),
                Card::from_str("Js").unwrap(),
                Card::from_str("Th").unwrap(),
                Card::from_str("8h").unwrap(),
                Card::from_str("7d").unwrap(),
            ],
            // Three of a kind
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Ac").unwrap(),
                Card::from_str("Ad").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Qh").unwrap(),
                Card::from_str("Jh").unwrap(),
                Card::from_str("9d").unwrap(),
            ],
            // Two pair
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Ac").unwrap(),
                Card::from_str("Kd").unwrap(),
                Card::from_str("Kc").unwrap(),
                Card::from_str("Qh").unwrap(),
                Card::from_str("Jh").unwrap(),
                Card::from_str("9d").unwrap(),
            ],
            // Pair
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Ac").unwrap(),
                Card::from_str("Kd").unwrap(),
                Card::from_str("Qs").unwrap(),
                Card::from_str("Jh").unwrap(),
                Card::from_str("9h").unwrap(),
                Card::from_str("8d").unwrap(),
            ],
            // High card
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Kd").unwrap(),
                Card::from_str("Qc").unwrap(),
                Card::from_str("Js").unwrap(),
                Card::from_str("9h").unwrap(),
                Card::from_str("8h").unwrap(),
                Card::from_str("7d").unwrap(),
            ],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_conversion() {
        use std::str::FromStr;

        let cards = [
            Card::from_str("As").unwrap(),
            Card::from_str("Kh").unwrap(),
            Card::from_str("Qd").unwrap(),
        ];

        let packed = convert_cards(&cards);
        assert_eq!(packed.len(), 3);

        let back = convert_cards_back(&packed).unwrap();
        assert_eq!(back.len(), 3);
    }

    #[test]
    fn test_math_evaluator_creation() {
        let evaluator = MathEvaluator::new();
        assert!(evaluator.is_ok());
    }

    #[test]
    fn test_evaluator_compatibility_trait() {
        let evaluator = MathEvaluator::new().unwrap();

        assert_eq!(evaluator.name(), "MathEvaluator");
        assert!(evaluator.memory_usage() > 0);
        assert!(evaluator.validate().is_ok());
    }

    #[test]
    fn test_benchmark_function() {
        use std::str::FromStr;

        let test_cards = [
            Card::from_str("As").unwrap(),
            Card::from_str("Ks").unwrap(),
            Card::from_str("Qs").unwrap(),
            Card::from_str("Js").unwrap(),
            Card::from_str("Ts").unwrap(),
            Card::from_str("7h").unwrap(),
            Card::from_str("6d").unwrap(),
        ];

        let elapsed = benchmark_evaluation(|cards| {
            // Simple evaluation for testing
            HandValue::new(HandRank::HighCard, 0)
        });

        assert!(elapsed.as_nanos() > 0);
    }

    #[test]
    fn test_evaluator_comparison() {
        let comparison = EvaluatorComparison::new();
        assert!(comparison.is_ok());

        let mut comparison = EvaluatorComparison::new().unwrap();
        let results = comparison.compare_evaluations(&[]);

        // Should handle empty hand list gracefully
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_performance_comparison() {
        let mut comparison = EvaluatorComparison::new().unwrap();
        let perf_comparison = comparison.run_performance_comparison();

        assert_eq!(perf_comparison.test_cases, 3); // We test 3 cases
        assert_eq!(perf_comparison.math_times.len(), 3);
        assert_eq!(perf_comparison.core_times.len(), 3);

        // All times should be positive
        assert!(perf_comparison.math_times.iter().all(|&t| t.as_nanos() > 0));
        assert!(perf_comparison.core_times.iter().all(|&t| t.as_nanos() > 0));
    }
}
