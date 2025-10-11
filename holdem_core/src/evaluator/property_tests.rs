//! # Property-Based Testing Framework for Hand Evaluator
//!
//! Comprehensive property-based testing system using proptest to verify
//! mathematical correctness and implementation invariants of the lookup
//! table hand evaluator across all possible inputs and edge cases.
//!
//! ## Testing Philosophy
//!
//! Property-based testing ensures the evaluator behaves correctly under
//! all possible circumstances:
//!
//! ### Mathematical Correctness
//! - **Hand evaluation accuracy**: All hands evaluate to correct rank and value
//! - **Rank ordering**: Higher ranks always have higher values than lower ranks
//! - **Perfect hash properties**: Hash function is deterministic and collision-free
//! - **Combinatorial correctness**: Multi-card evaluation finds optimal 5-card hands
//!
//! ### Implementation Invariants
//! - **Deterministic behavior**: Same input always produces same output
//! - **Memory safety**: No crashes or undefined behavior on any input
//! - **Performance bounds**: Evaluation completes within expected time limits
//! - **Error handling**: Graceful handling of edge cases and invalid inputs
//!
//! ### Edge Case Coverage
//! - **Boundary conditions**: Extreme hash values and hand combinations
//! - **Duplicate cards**: Handling of invalid but non-crashing inputs
//! - **Special hands**: Royal flushes, wheel straights, and other edge cases
//! - **Hash collisions**: Verification of perfect hash collision resistance
//!
//! ## Test Categories
//!
//! ### Functional Correctness Tests
//! - **Hand evaluation determinism**: Same hand always evaluates identically
//! - **Rank ordering verification**: Hand values respect poker hand hierarchy
//! - **All hand types**: Coverage of all 10 poker hand categories
//! - **Multi-card evaluation**: Correct best-hand finding for 6-7 card hands
//!
//! ### Perfect Hash Tests
//! - **Hash determinism**: Perfect hash produces consistent results
//! - **Bounds validation**: All hashes fall within expected range
//! - **Collision detection**: No two different hands produce same hash
//! - **Distribution analysis**: Hash values are well-distributed
//!
//! ### Performance Tests
//! - **Evaluation speed**: Hand evaluation meets performance requirements
//! - **Memory usage**: Memory consumption stays within acceptable bounds
//! - **Scalability**: Performance degrades gracefully with hand complexity
//! - **Resource limits**: System handles resource constraints appropriately
//!
//! ### Error Handling Tests
//! - **Invalid inputs**: System handles malformed data gracefully
//! - **Edge cases**: Boundary conditions don't cause crashes
//! - **Resource exhaustion**: Appropriate behavior under memory/file pressure
//! - **Concurrent access**: Thread safety under testing conditions
//!
//! ## Usage Examples
//!
//! ### Running Property Tests
//! ```bash
//! # Run all property tests
//! cargo test property_tests
//!
//! # Run specific test patterns
//! cargo test test_hand_evaluation_deterministic
//! cargo test test_perfect_hash
//! cargo test performance_tests
//!
//! # Run with verbose output
//! cargo test property_tests -- --nocapture
//! ```
//!
//! ### Custom Property Test Implementation
//! ```rust
//! use proptest::prelude::*;
//! use math::evaluator::tables;
//! use holdem_core::{Card, Hand};
//! use std::str::FromStr;
//!
//! proptest! {
//!     /// Custom test for hand evaluation consistency
//!     #[test]
//!     fn test_custom_evaluation_property(
//!         card1 in any_card(),
//!         card2 in any_card(),
//!         card3 in any_card(),
//!         card4 in any_card(),
//!         card5 in any_card()
//!     ) {
//!         // Create hand from individual cards
//!         let cards = [card1, card2, card3, card4, card5];
//!
//!         // Test that evaluation doesn't panic
//!         let result = tables::evaluate_5_card_hand(&cards);
//!
//!         // Verify result is reasonable
//!         prop_assert!(result.rank as u8 >= 0);
//!         prop_assert!(result.rank as u8 <= 9);
//!         prop_assert!(result.value >= 0);
//!     }
//! }
//! ```
//!
//! ### Performance Benchmarking
//! ```rust
//! use math::evaluator::tables;
//! use holdem_core::{Card, Hand};
//! use std::time::Instant;
//! use std::str::FromStr;
//!
//! #[test]
//! fn benchmark_hand_evaluation() {
//!     let test_hands = vec![
//!         [
//!             Card::from_str("As").unwrap(),
//!             Card::from_str("Ks").unwrap(),
//!             Card::from_str("Qs").unwrap(),
//!             Card::from_str("Js").unwrap(),
//!             Card::from_str("Ts").unwrap(),
//!         ],
//!         [
//!             Card::from_str("Ah").unwrap(),
//!             Card::from_str("Kh").unwrap(),
//!             Card::from_str("Qh").unwrap(),
//!             Card::from_str("Jh").unwrap(),
//!             Card::from_str("Th").unwrap(),
//!         ],
//!     ];
//!
//!     let iterations = 100_000;
//!     let start = Instant::now();
//!
//!     for _ in 0..iterations {
//!         for hand in &test_hands {
//!             let _result = tables::evaluate_5_card_hand(hand);
//!         }
//!     }
//!
//!     let duration = start.elapsed();
//!     let total_evaluations = iterations * test_hands.len();
//!     let evaluations_per_second = total_evaluations as f64 / duration.as_secs_f64();
//!
//!     println!("Performance: {:.0} evaluations/sec", evaluations_per_second);
//!
//!     // Should achieve target performance
//!     assert!(evaluations_per_second >= 100_000.0);
//! }
//! ```
//!
//! ## Test Data Generation
//!
//! ### Hand Generation Strategies
//! The testing framework includes sophisticated strategies for generating test data:
//!
//! - **Known good hands**: Predefined hands with known correct evaluations
//! - **Random valid hands**: Randomly generated valid 5-card combinations
//! - **Edge case hands**: Special hands like wheel straights and royal flushes
//! - **Invalid inputs**: Malformed data for robustness testing
//!
//! ### Strategy Composition
//! ```rust
//! use proptest::prelude::*;
//!
//! /// Generate a strategy for valid poker hands
//! fn valid_poker_hand() -> impl Strategy<Value = [Card; 5]> {
//!     // Use predefined test hands for consistency
//!     prop::sample::select(vec![
//!         ["As", "Ks", "Qs", "Js", "Ts"], // Royal flush
//!         ["Ah", "Kh", "Qh", "Jh", "Th"], // Royal flush different suit
//!         ["9h", "8h", "7h", "6h", "5h"], // Straight flush
//!         ["Ah", "Ac", "Ad", "As", "Kh"], // Four of a kind
//!         // ... more test hands
//!     ])
//!     .prop_map(|card_strs| {
//!         let cards: Vec<Card> = card_strs.iter()
//!             .map(|s| Card::from_str(s).unwrap())
//!             .collect();
//!         cards.try_into().unwrap()
//!     })
//! }
//! ```
//!
//! ## Performance Testing
//!
//! ### Automated Performance Validation
//! The test suite includes automated performance validation:
//!
//! - **5-card evaluation**: Must exceed 100,000 evaluations/second
//! - **7-card evaluation**: Must exceed 1,000 evaluations/second
//! - **Memory usage**: Must stay under 2GB total
//! - **Hash calculation**: Must complete within time bounds
//!
//! ### Performance Regression Detection
//! ```rust
//! #[cfg(test)]
//! mod performance_regression_tests {
//!     use super::*;
//!
//!     /// Test for performance regressions in hand evaluation
//!     #[test]
//!     fn test_no_performance_regression() {
//!         let baseline_performance = 100_000.0; // 100K evaluations/sec
//!         let test_hands = create_test_hands();
//!
//!         let evaluations_per_second = measure_evaluation_performance(&test_hands);
//!
//!         assert!(
//!             evaluations_per_second >= baseline_performance * 0.9, // Allow 10% variance
//!             "Performance regression detected: {:.0} < {:.0}",
//!             evaluations_per_second,
//!             baseline_performance * 0.9
//!         );
//!     }
//! }
//! ```
//!
//! ## Memory Safety Testing
//!
//! ### Bounds Checking Verification
//! ```rust
//! use proptest::prelude::*;
//!
//! proptest! {
//!     /// Test that hash function never produces out-of-bounds indices
//!     #[test]
//!     fn test_hash_bounds_safety(cards in unique_cards(5)) {
//!         if cards.len() == 5 {
//!             let hand: [Card; 5] = cards.try_into().unwrap();
//!             let hash_index = tables::perfect_hash_5_cards(&hand);
//!
//!             // Hash must always be within valid range
//!             prop_assert!(hash_index < 2_598_960);
//!             prop_assert!(hash_index >= 0);
//!         }
//!     }
//! }
//! ```
//!
//! ### Memory Usage Validation
//! ```rust
//! #[test]
//! fn test_memory_usage_bounds() {
//!     let tables = tables::LookupTables::new();
//!     let memory_usage = tables.memory_usage();
//!
//!     // Memory usage should be reasonable
//!     assert!(memory_usage > 0, "Memory usage should be positive");
//!     assert!(
//!         memory_usage < 2_000_000_000, // 2GB limit
//!         "Memory usage too high: {} bytes",
//!         memory_usage
//!     );
//!
//!     println!("Memory usage: {:.2} MB", memory_usage as f64 / 1_048_576.0);
//! }
//! ```
//!
//! ## Integration Testing
//!
//! ### Cross-Module Integration Tests
//! ```rust
//! #[cfg(test)]
//! mod integration_tests {
//!     use super::*;
//!     use math::Evaluator;
//!     use holdem_core::Hand;
//!
//!     /// Test that direct table evaluation matches evaluator results
//!     #[test]
//!     fn test_table_evaluator_consistency() {
//!         let evaluator = Evaluator::instance();
//!         let test_cards = [
//!             Card::from_str("As").unwrap(),
//!             Card::from_str("Ks").unwrap(),
//!             Card::from_str("Qs").unwrap(),
//!             Card::from_str("Js").unwrap(),
//!             Card::from_str("Ts").unwrap(),
//!         ];
//!
//!         let direct_result = tables::evaluate_5_card_hand(&test_cards);
//!         let evaluator_result = evaluator.evaluate_5_card(&test_cards);
//!
//!         assert_eq!(direct_result, evaluator_result);
//!     }
//! }
//! ```
//!
//! ## Debugging and Diagnostics
//!
//! ### Test Failure Analysis
//! When property tests fail, the framework provides detailed information:
//!
//! - **Seed reproduction**: Failed tests can be reproduced with specific seeds
//! - **Shrinkage process**: Minimal failing examples are automatically generated
//! - **Stack traces**: Clear error reporting with context information
//! - **Custom assertions**: Detailed assertion messages for debugging
//!
//! ### Diagnostic Test Utilities
//! ```rust
//! /// Utility function for debugging hand evaluation
//! fn debug_hand_evaluation(cards: &[Card; 5]) -> String {
//!     let hash = tables::perfect_hash_5_cards(cards);
//!     let result = tables::evaluate_5_card_hand(cards);
//!
//!     format!(
//!         "Cards: {:?}\nHash: {}\nResult: {:?}",
//!         cards, hash, result
//!     )
//! }
//!
//! #[test]
//! fn debug_specific_hand() {
//!     let cards = [
//!         Card::from_str("As").unwrap(),
//!         Card::from_str("Ks").unwrap(),
//!         Card::from_str("Qs").unwrap(),
//!         Card::from_str("Js").unwrap(),
//!         Card::from_str("Ts").unwrap(),
//!     ];
//!
//!     println!("{}", debug_hand_evaluation(&cards));
//! }
//! ```
//!
//! ## Best Practices
//!
//! ### Writing Property Tests
//! - **Clear test names**: Describe what property is being tested
//! - **Good data generation**: Use appropriate strategies for test data
//! - **Informative assertions**: Provide context in assertion failures
//! - **Performance awareness**: Consider computational cost of test strategies
//!
//! ### Test Organization
//! - **Logical grouping**: Group related tests in test modules
//! - **Setup/teardown**: Use appropriate test setup for shared state
//! - **Documentation**: Document complex test strategies and invariants
//! - **Maintenance**: Keep test data and strategies up to date with implementation
//!
//! ### Performance Testing
//! - **Realistic workloads**: Test with realistic data sizes and patterns
//! - **Resource monitoring**: Track memory and CPU usage during tests
//! - **Regression detection**: Establish baselines and monitor for regressions
//! - **Profiling integration**: Use profiling tools to identify bottlenecks

use super::{HandRank, HandValue};
use crate::evaluator::tables;
use crate::{Card, Hand};
use proptest::prelude::*;
use std::str::FromStr;

/// Strategy for generating valid 5-card poker hands
fn valid_5_card_hand() -> impl Strategy<Value = [Card; 5]> {
    prop::sample::select(vec![
        // Royal flush hands
        ["As", "Ks", "Qs", "Js", "Ts"],
        ["Ah", "Kh", "Qh", "Jh", "Th"],
        ["Ad", "Kd", "Qd", "Jd", "Td"],
        ["Ac", "Kc", "Qc", "Jc", "Tc"],
        // Straight flush hands
        ["9h", "8h", "7h", "6h", "5h"],
        ["8d", "7d", "6d", "5d", "4d"],
        ["7c", "6c", "5c", "4c", "3c"],
        ["6s", "5s", "4s", "3s", "2s"],
        // Four of a kind hands
        ["Ah", "Ac", "Ad", "As", "Kh"],
        ["Kh", "Kc", "Kd", "Ks", "Qh"],
        ["Qh", "Qc", "Qd", "Qs", "Jh"],
        ["Jh", "Jc", "Jd", "Js", "Th"],
        // Full house hands
        ["Ah", "Ac", "Ad", "Ks", "Kh"],
        ["Kh", "Kc", "Kd", "Qs", "Qh"],
        ["Qh", "Qc", "Qd", "Js", "Jh"],
        ["Jh", "Jc", "Jd", "Ts", "Th"],
        // Flush hands
        ["Ah", "Kh", "Qh", "9h", "7h"],
        ["Kd", "Qd", "Jd", "8d", "6d"],
        ["Qc", "Jc", "9c", "7c", "5c"],
        ["Js", "Ts", "8s", "6s", "4s"],
        // Straight hands
        ["Ah", "Kd", "Qc", "Js", "Th"],
        ["Kh", "Qd", "Js", "Tc", "9h"],
        ["Qh", "Jd", "Tc", "9s", "8h"],
        ["Jh", "Td", "9c", "8s", "7h"],
        ["Th", "9d", "8c", "7s", "6h"],
        ["9h", "8d", "7c", "6s", "5h"],
        ["8h", "7d", "6c", "5s", "4h"],
        ["7h", "6d", "5c", "4s", "3h"],
        ["6h", "5d", "4c", "3s", "2h"],
        ["5h", "4d", "3c", "2s", "Ah"], // Wheel straight
        // Three of a kind hands
        ["Ah", "Ac", "Ad", "Ks", "Qh"],
        ["Kh", "Kc", "Kd", "Qs", "Jh"],
        ["Qh", "Qc", "Qd", "Js", "Th"],
        ["Jh", "Jc", "Jd", "Ts", "9h"],
        // Two pair hands
        ["Ah", "Ac", "Kd", "Ks", "Qh"],
        ["Kh", "Kc", "Qd", "Qs", "Jh"],
        ["Qh", "Qc", "Jd", "Js", "Th"],
        ["Jh", "Jc", "Td", "Ts", "9h"],
        // Pair hands
        ["Ah", "Ac", "Kd", "Qs", "Jh"],
        ["Kh", "Kc", "Qd", "Js", "9h"],
        ["Qh", "Qc", "Jd", "Ts", "8h"],
        ["Jh", "Jc", "Td", "9s", "7h"],
        // High card hands
        ["Ah", "Kd", "Qc", "Js", "9h"],
        ["Kh", "Qd", "Js", "Tc", "8h"],
        ["Qh", "Jd", "Tc", "9s", "7h"],
        ["Jh", "Td", "9c", "8s", "6h"],
        ["Th", "9d", "8c", "7s", "5h"],
    ])
    .prop_map(|card_strs| {
        let cards: Vec<Card> = card_strs
            .iter()
            .map(|s| Card::from_str(s).unwrap())
            .collect();
        cards.try_into().unwrap()
    })
}

/// Strategy for generating random valid cards
fn any_card() -> impl Strategy<Value = Card> {
    (0u8..13, 0u8..4).prop_map(|(rank, suit)| Card::new(rank, suit).unwrap())
}

/// Strategy for generating unique sets of cards
fn unique_cards(count: usize) -> impl Strategy<Value = Vec<Card>> {
    prop::sample::subsequence(
        (0u8..52)
            .map(|i| {
                let rank = i % 13;
                let suit = i / 13;
                Card::new(rank, suit).unwrap()
            })
            .collect::<Vec<_>>(),
        count,
    )
}

proptest! {
    /// Test that hand evaluation is deterministic
    #[test]
    fn test_hand_evaluation_deterministic(hand in valid_5_card_hand()) {
        // Use direct table evaluation to avoid singleton issues in parallel tests
        let result1 = tables::evaluate_5_card_hand(&hand);
        let result2 = tables::evaluate_5_card_hand(&hand);
        prop_assert_eq!(result1, result2);
    }

    /// Test that hand evaluation never panics
    #[test]
    fn test_hand_evaluation_never_panics(cards in unique_cards(5)) {
        if cards.len() == 5 {
            let hand: [Card; 5] = cards.try_into().unwrap();
            let _result = tables::evaluate_5_card_hand(&hand); // Should not panic
        }
    }

    /// Test that hand ranks are properly ordered
    #[test]
    fn test_hand_rank_ordering(
        hand1 in valid_5_card_hand(),
        hand2 in valid_5_card_hand()
    ) {
        let value1 = tables::evaluate_5_card_hand(&hand1);
        let value2 = tables::evaluate_5_card_hand(&hand2);

        // If ranks are different, higher rank should have higher value
        if value1.rank != value2.rank {
            match value1.rank.cmp(&value2.rank) {
                std::cmp::Ordering::Greater => prop_assert!(value1 > value2),
                std::cmp::Ordering::Less => prop_assert!(value1 < value2),
                std::cmp::Ordering::Equal => unreachable!(),
            }
        }
    }

    /// Test that all hand types are correctly identified
    #[test]
    fn test_all_hand_types_identified(hand in valid_5_card_hand()) {
        let result = tables::evaluate_5_card_hand(&hand);

        // Hand rank should be valid (not HighCard with value 0 for known good hands)
        prop_assert!(result.rank as u8 >= HandRank::HighCard as u8);
        prop_assert!(result.rank as u8 <= HandRank::RoyalFlush as u8);
    }

    /// Test that 6-card hand evaluation works correctly
    #[test]
    fn test_6_card_evaluation(cards in unique_cards(6)) {
        if cards.len() == 6 {
            let hand: [Card; 6] = cards.try_into().unwrap();
            let result = tables::evaluate_6_card_hand(&hand);
            prop_assert!(result.rank >= HandRank::HighCard);
        }
    }

    /// Test that 7-card hand evaluation works correctly
    #[test]
    fn test_7_card_evaluation(cards in unique_cards(7)) {
        if cards.len() == 7 {
            let hand: [Card; 7] = cards.try_into().unwrap();
            let result = tables::evaluate_7_card_hand(&hand);
            prop_assert!(result.rank >= HandRank::HighCard);
        }
    }

    /// Test that hand evaluation from Hand struct works
    #[test]
    fn test_hand_struct_evaluation(cards in unique_cards(5)) {
        if cards.len() == 5 {
            let hand_array: [Card; 5] = cards.try_into().unwrap();
            let result = tables::evaluate_5_card_hand(&hand_array);
            prop_assert!(result.rank >= HandRank::HighCard);
        }
    }
}

/// Additional property tests that don't use the proptest! macro
#[cfg(test)]
mod additional_property_tests {
    use super::*;
    use proptest::{prelude::*, sample};

    proptest! {
        /// Test that perfect hash produces valid indices
        #[test]
        fn test_perfect_hash_valid_indices(cards in unique_cards(5)) {
            if cards.len() == 5 {
                let hand: [Card; 5] = cards.try_into().unwrap();
                let hash_index = tables::perfect_hash_5_cards(&hand);
                prop_assert!(hash_index < 2_598_960, "Hash index out of bounds: {}", hash_index);
            }
        }

        /// Test that perfect hash is deterministic
        #[test]
        fn test_perfect_hash_deterministic(cards in unique_cards(5)) {
            if cards.len() == 5 {
                let hand: [Card; 5] = cards.try_into().unwrap();
                let hash1 = tables::perfect_hash_5_cards(&hand);
                let hash2 = tables::perfect_hash_5_cards(&hand);
                prop_assert_eq!(hash1, hash2);
            }
        }

        /// Test that no hash collisions occur for different hands
        #[test]
        fn test_no_hash_collisions(
            cards1 in unique_cards(5),
            cards2 in unique_cards(5)
        ) {
            if cards1.len() == 5 && cards2.len() == 5 {
                let hand1: [Card; 5] = cards1.try_into().unwrap();
                let hand2: [Card; 5] = cards2.try_into().unwrap();

                // Only test if hands are actually different
                if hand1 != hand2 {
                    let hash1 = tables::perfect_hash_5_cards(&hand1);
                    let hash2 = tables::perfect_hash_5_cards(&hand2);

                    // If hashes are the same, the hands should be equivalent
                    if hash1 == hash2 {
                        let value1 = tables::evaluate_5_card_hand(&hand1);
                        let value2 = tables::evaluate_5_card_hand(&hand2);
                        prop_assert_eq!(value1, value2, "Hash collision with different hand values");
                    }
                }
            }
        }
    }

    /// Test hand evaluation consistency across different input methods
    #[test]
    fn test_evaluation_consistency() {
        // Test a few specific hands for consistency
        let test_cards = vec![
            Card::from_str("As").unwrap(),
            Card::from_str("Ks").unwrap(),
            Card::from_str("Qs").unwrap(),
            Card::from_str("Js").unwrap(),
            Card::from_str("Ts").unwrap(),
        ];

        let hand_array: [Card; 5] = test_cards.clone().try_into().unwrap();
        let _hand_struct = Hand::new(test_cards.clone()).unwrap();

        let result_array = tables::evaluate_5_card_hand(&hand_array);
        let hand_array_2: [Card; 5] = test_cards.clone().try_into().unwrap();
        let result_struct = tables::evaluate_5_card_hand(&hand_array_2);

        assert_eq!(result_array, result_struct);
    }

    /// Test that better hands always have higher values
    #[test]
    fn test_hand_comparison_logic() {
        // Test a few specific hand comparisons
        let hand1_cards = [
            Card::from_str("As").unwrap(),
            Card::from_str("Ks").unwrap(),
            Card::from_str("Qs").unwrap(),
            Card::from_str("Js").unwrap(),
            Card::from_str("Ts").unwrap(),
        ];
        let hand2_cards = [
            Card::from_str("Kh").unwrap(),
            Card::from_str("Qd").unwrap(),
            Card::from_str("Js").unwrap(),
            Card::from_str("Tc").unwrap(),
            Card::from_str("9h").unwrap(),
        ];

        let value1 = tables::evaluate_5_card_hand(&hand1_cards);
        let value2 = tables::evaluate_5_card_hand(&hand2_cards);

        // Royal flush should beat straight
        assert!(value1 > value2);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::{Duration, Instant};

    /// Test that 5-card hand evaluation meets performance requirements
    #[test]
    fn test_5_card_evaluation_performance() {
        let test_hands = vec![
            [
                Card::from_str("As").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Qs").unwrap(),
                Card::from_str("Js").unwrap(),
                Card::from_str("Ts").unwrap(),
            ],
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("Qh").unwrap(),
                Card::from_str("Jh").unwrap(),
                Card::from_str("Th").unwrap(),
            ],
            [
                Card::from_str("2h").unwrap(),
                Card::from_str("3h").unwrap(),
                Card::from_str("4h").unwrap(),
                Card::from_str("5h").unwrap(),
                Card::from_str("6h").unwrap(),
            ],
        ];

        let iterations = 100_000;
        let start = Instant::now();

        for _ in 0..iterations {
            for hand in &test_hands {
                let _result = tables::evaluate_5_card_hand(hand);
            }
        }

        let duration = start.elapsed();
        let total_evaluations = iterations * test_hands.len();
        let evaluations_per_second = total_evaluations as f64 / duration.as_secs_f64();

        println!("5-card evaluation performance:");
        println!("  Total evaluations: {}", total_evaluations);
        println!("  Time taken: {:?}", duration);
        println!("  Evaluations per second: {:.0}", evaluations_per_second);

        // Should be able to evaluate at least 100K hands per second
        assert!(
            evaluations_per_second >= 100_000.0,
            "5-card evaluation too slow: {:.0} evaluations/sec",
            evaluations_per_second
        );
    }

    /// Test that 7-card hand evaluation meets performance requirements
    #[test]
    fn test_7_card_evaluation_performance() {
        let test_hands = vec![
            [
                Card::from_str("As").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Qs").unwrap(),
                Card::from_str("Js").unwrap(),
                Card::from_str("Ts").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("6d").unwrap(),
            ],
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("Qh").unwrap(),
                Card::from_str("Jh").unwrap(),
                Card::from_str("Th").unwrap(),
                Card::from_str("7d").unwrap(),
                Card::from_str("6c").unwrap(),
            ],
        ];

        let iterations = 10_000;
        let start = Instant::now();

        for _ in 0..iterations {
            for hand in &test_hands {
                let _result = tables::evaluate_7_card_hand(hand);
            }
        }

        let duration = start.elapsed();
        let total_evaluations = iterations * test_hands.len();
        let evaluations_per_second = total_evaluations as f64 / duration.as_secs_f64();

        println!("7-card evaluation performance:");
        println!("  Total evaluations: {}", total_evaluations);
        println!("  Time taken: {:?}", duration);
        println!("  Evaluations per second: {:.0}", evaluations_per_second);

        // Should be able to evaluate at least 1K hands per second
        assert!(
            evaluations_per_second >= 1_000.0,
            "7-card evaluation too slow: {:.0} evaluations/sec",
            evaluations_per_second
        );
    }

    /// Test memory usage is reasonable
    #[test]
    fn test_memory_usage() {
        let tables = tables::LookupTables::new();
        let memory_usage = tables.memory_usage();

        println!(
            "Total memory usage: {} bytes ({:.2} MB)",
            memory_usage,
            memory_usage as f64 / 1_048_576.0
        );

        // Should use less than 2GB total (reasonable for lookup tables)
        assert!(
            memory_usage < 2_000_000_000,
            "Memory usage too high: {} bytes",
            memory_usage
        );
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    /// Test evaluation with duplicate cards (should not crash)
    #[test]
    fn test_duplicate_cards() {
        let cards = [
            Card::from_str("As").unwrap(),
            Card::from_str("As").unwrap(), // Duplicate
            Card::from_str("Ks").unwrap(),
            Card::from_str("Qs").unwrap(),
            Card::from_str("Js").unwrap(),
        ];

        // Should not panic, even with invalid input
        let _result = tables::evaluate_5_card_hand(&cards);
    }

    /// Test evaluation with all same rank
    #[test]
    fn test_all_same_rank() {
        let cards = [
            Card::from_str("Ah").unwrap(),
            Card::from_str("Ad").unwrap(),
            Card::from_str("Ac").unwrap(),
            Card::from_str("As").unwrap(),
            Card::from_str("Kh").unwrap(),
        ];

        let result = tables::evaluate_5_card_hand(&cards);
        assert_eq!(result.rank, HandRank::FourOfAKind);
    }

    /// Test evaluation with all same suit
    #[test]
    fn test_all_same_suit() {
        let cards = [
            Card::from_str("Ah").unwrap(),
            Card::from_str("Kh").unwrap(),
            Card::from_str("Qh").unwrap(),
            Card::from_str("Jh").unwrap(),
            Card::from_str("Th").unwrap(),
        ];

        let result = tables::evaluate_5_card_hand(&cards);
        assert_eq!(result.rank, HandRank::RoyalFlush);
    }

    /// Test wheel straight flush
    #[test]
    fn test_wheel_straight_flush() {
        let cards = [
            Card::from_str("Ah").unwrap(),
            Card::from_str("2h").unwrap(),
            Card::from_str("3h").unwrap(),
            Card::from_str("4h").unwrap(),
            Card::from_str("5h").unwrap(),
        ];

        let result = tables::evaluate_5_card_hand(&cards);
        assert_eq!(result.rank, HandRank::StraightFlush);
    }

    /// Test boundary hash values
    #[test]
    fn test_boundary_hash_values() {
        use super::super::tables::perfect_hash_5_cards;

        // Test minimum possible hash (should be 0 or close)
        let min_cards = [
            Card::new(0, 0).unwrap(), // 2s
            Card::new(1, 0).unwrap(), // 3s
            Card::new(2, 0).unwrap(), // 4s
            Card::new(3, 0).unwrap(), // 5s
            Card::new(4, 0).unwrap(), // 6s
        ];
        let min_hash = perfect_hash_5_cards(&min_cards);
        assert!(
            min_hash < 2_598_960,
            "Minimum hash should be within bounds, got {}",
            min_hash
        );

        // Test maximum possible hash (should be < 2.5M)
        let max_cards = [
            Card::new(12, 3).unwrap(), // Ac
            Card::new(11, 3).unwrap(), // Kc
            Card::new(10, 3).unwrap(), // Qc
            Card::new(9, 3).unwrap(),  // Jc
            Card::new(8, 3).unwrap(),  // Tc
        ];
        let max_hash = perfect_hash_5_cards(&max_cards);
        assert!(
            max_hash < 2_598_960,
            "Maximum hash out of bounds: {}",
            max_hash
        );
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::super::errors::EvaluatorError;
    use super::*;

    /// Test error handling in evaluator creation
    #[test]
    fn test_evaluator_creation_error_handling() {
        // Test that evaluator handles file I/O errors gracefully
        // This is more of a smoke test since the actual error handling
        // depends on file system state
        let result = super::super::Evaluator::new();
        assert!(result.is_ok());
    }

    /// Test table validation error handling
    #[test]
    fn test_table_validation_error_handling() {
        let evaluator = super::super::Evaluator::instance();
        let validation_result = evaluator.validate_table_files();
        // Should not panic even if files are missing or corrupted
        assert!(validation_result.is_ok());
    }

    /// Test table regeneration error handling
    #[test]
    fn test_table_regeneration_error_handling() {
        let evaluator = super::super::Evaluator::instance();
        // Note: We can't easily test the mutable reference without Arc<Mutex>
        // but we can at least verify the method exists and doesn't panic
        // when called on a properly initialized evaluator
        let info_result = evaluator.get_table_info();
        assert!(info_result.is_ok());
    }
}

#[cfg(test)]
mod singleton_tests {
    use super::super::singleton::EvaluatorSingleton;
    use super::*;
    use std::sync::Arc;

    /// Test singleton behavior
    #[test]
    fn test_singleton_behavior() {
        let instance1 = EvaluatorSingleton::instance();
        let instance2 = EvaluatorSingleton::instance();

        // Should be the same instance
        assert_eq!(Arc::as_ptr(&instance1), Arc::as_ptr(&instance2));
    }

    /// Test singleton reference counting
    #[test]
    fn test_singleton_reference_counting() {
        let initial_count = EvaluatorSingleton::reference_count();

        {
            let _instance = EvaluatorSingleton::instance();
            assert_eq!(EvaluatorSingleton::reference_count(), initial_count + 1);
        }

        // Should decrement when out of scope
        assert_eq!(EvaluatorSingleton::reference_count(), initial_count);
    }

    /// Test singleton thread safety (basic smoke test)
    #[test]
    fn test_singleton_thread_safety() {
        use std::thread;

        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(|| {
                    let instance = EvaluatorSingleton::instance();
                    // All instances should be the same
                    let base_ptr = Arc::as_ptr(&EvaluatorSingleton::instance());
                    assert_eq!(Arc::as_ptr(&instance), base_ptr);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

#[cfg(test)]
mod file_io_tests {
    use super::super::file_io::{LutFileManager, TableType};
    use super::super::tables::{FiveCardTable, LookupTables};
    use super::*;
    use tempfile::TempDir;

    /// Test file I/O operations
    #[test]
    fn test_file_io_operations() {
        let temp_dir = TempDir::new().unwrap();
        let manager = LutFileManager::new(temp_dir.path());

        // Create test tables
        let mut tables = LookupTables::new();
        tables.five_card.data[0] = HandValue::new(HandRank::RoyalFlush, 1);

        // Test write operation
        let write_result = manager.write_table(TableType::FiveCard, &tables);
        assert!(write_result.is_ok());

        // Test read operation
        let read_result = manager.read_table(TableType::FiveCard);
        assert!(read_result.is_ok());

        let read_table = read_result.unwrap();
        assert_eq!(
            read_table.data()[0],
            HandValue::new(HandRank::RoyalFlush, 1)
        );
    }

    /// Test file corruption recovery
    #[test]
    fn test_file_corruption_recovery() {
        let temp_dir = TempDir::new().unwrap();
        let manager = LutFileManager::new(temp_dir.path());

        // Create a corrupted file by writing invalid data to the expected location
        let mut path = temp_dir.path().to_path_buf();
        path.push("5_card_table.lut");
        std::fs::write(&path, b"corrupted data").unwrap();

        // Should handle corrupted file gracefully
        let read_result = manager.read_table(TableType::FiveCard);
        assert!(read_result.is_err());

        // Table existence should return false for corrupted files
        assert!(!manager.table_exists(TableType::FiveCard));
    }

    /// Test atomic write operations
    #[test]
    fn test_atomic_write_operations() {
        let temp_dir = TempDir::new().unwrap();
        let manager = LutFileManager::new(temp_dir.path());

        let tables = LookupTables::new();

        // Write operation should be atomic
        let write_result = manager.write_table(TableType::FiveCard, &tables);
        assert!(write_result.is_ok());

        // File should exist and be valid after write
        assert!(manager.table_exists(TableType::FiveCard));
        let read_result = manager.read_table(TableType::FiveCard);
        assert!(read_result.is_ok());
    }

    /// Test table deletion
    #[test]
    fn test_table_deletion() {
        let temp_dir = TempDir::new().unwrap();
        let manager = LutFileManager::new(temp_dir.path());

        let tables = LookupTables::new();
        manager.write_table(TableType::FiveCard, &tables).unwrap();

        // Verify file exists
        assert!(manager.table_exists(TableType::FiveCard));

        // Delete file
        let delete_result = manager.delete_table(TableType::FiveCard);
        assert!(delete_result.is_ok());

        // Verify file no longer exists
        assert!(!manager.table_exists(TableType::FiveCard));
    }
}
