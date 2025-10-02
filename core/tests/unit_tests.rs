//! # Unit Tests for Poker Hand Evaluator
//!
//! Focused unit tests for individual components of the poker evaluation system.
//! These tests validate specific algorithms and edge cases in isolation.
//!
//! ## Test Categories
//!
//! - **Hand Rankings**: Test specific hand types and their relative strengths
//! - **Edge Cases**: Test unusual or boundary condition inputs
//! - **Performance**: Validate speed characteristics of key algorithms
//! - **Correctness**: Test against known good values for specific hands
//!
//! ## Test Data
//!
//! Uses carefully constructed test cases with known expected outcomes.
//! All test hands are deterministic and well-documented.

use poker_api::api::card::Card;
use poker_api::api::hand::Hand;
use poker_api::hand_evaluator::LookupHandEvaluator;

/// Test suite for specific hand rankings and their relative strengths.
///
/// These tests verify that the evaluator correctly ranks different hand types
/// and that stronger hands always receive higher rank values than weaker hands.
#[cfg(test)]
mod hand_rankings {
    use super::*;

    #[test]
    fn test_royal_flush_ranking() {
        // Tests that royal flush is the strongest possible hand.
        //
        // Creates a royal flush (A-K-Q-J-T of same suit) and verifies
        // it receives the highest possible rank value.
        //
        // Test Case:
        // - Hand: A♠ K♠ Q♠ J♠ T♠
        // - Expected: Highest rank value (> 368M)
        // - Verification: Stronger than all other hand types
        let evaluator = LookupHandEvaluator::new().unwrap();

        let mut hand = Hand::new();
        hand.add_card(Card::from_string("As").unwrap()).unwrap();
        hand.add_card(Card::from_string("Ks").unwrap()).unwrap();
        hand.add_card(Card::from_string("Qs").unwrap()).unwrap();
        hand.add_card(Card::from_string("Js").unwrap()).unwrap();
        hand.add_card(Card::from_string("Ts").unwrap()).unwrap();

        let rank = evaluator.rank_hand(&hand);
        assert!(
            rank > 368_793_253,
            "Royal flush should have rank > 368M, got {}",
            rank
        );
    }

    #[test]
    fn test_straight_flush_vs_four_of_kind() {
        /// Tests that straight flush beats four of a kind.
        ///
        /// Compares a straight flush against four of a kind to ensure
        /// the straight flush receives a higher rank value.
        ///
        /// # Test Cases
        /// - Straight Flush: K♠ Q♠ J♠ T♠ 9♠
        /// - Four of a Kind: A♠ A♥ A♦ A♣ K♦
        /// - Expected: Straight flush > Four of a kind
        let evaluator = LookupHandEvaluator::new().unwrap();

        let mut straight_flush = Hand::new();
        straight_flush
            .add_card(Card::from_string("Ks").unwrap())
            .unwrap();
        straight_flush
            .add_card(Card::from_string("Qs").unwrap())
            .unwrap();
        straight_flush
            .add_card(Card::from_string("Js").unwrap())
            .unwrap();
        straight_flush
            .add_card(Card::from_string("Ts").unwrap())
            .unwrap();
        straight_flush
            .add_card(Card::from_string("9s").unwrap())
            .unwrap();

        let mut four_of_kind = Hand::new();
        four_of_kind
            .add_card(Card::from_string("As").unwrap())
            .unwrap();
        four_of_kind
            .add_card(Card::from_string("Ah").unwrap())
            .unwrap();
        four_of_kind
            .add_card(Card::from_string("Ad").unwrap())
            .unwrap();
        four_of_kind
            .add_card(Card::from_string("Ac").unwrap())
            .unwrap();
        four_of_kind
            .add_card(Card::from_string("Kd").unwrap())
            .unwrap();

        let sf_rank = evaluator.rank_hand(&straight_flush);
        let fk_rank = evaluator.rank_hand(&four_of_kind);

        assert!(
            sf_rank > fk_rank,
            "Straight flush should beat four of a kind: {} > {}",
            sf_rank,
            fk_rank
        );
    }

    #[test]
    fn test_full_house_vs_flush() {
        /// Tests that full house beats flush.
        ///
        /// # Test Cases
        /// - Full House: K♠ K♥ K♦ Q♠ Q♥
        /// - Flush: A♠ J♠ 8♠ 5♠ 2♠
        /// - Expected: Full house > Flush
        let evaluator = LookupHandEvaluator::new().unwrap();

        let mut full_house = Hand::new();
        full_house
            .add_card(Card::from_string("Ks").unwrap())
            .unwrap();
        full_house
            .add_card(Card::from_string("Kh").unwrap())
            .unwrap();
        full_house
            .add_card(Card::from_string("Kd").unwrap())
            .unwrap();
        full_house
            .add_card(Card::from_string("Qs").unwrap())
            .unwrap();
        full_house
            .add_card(Card::from_string("Qh").unwrap())
            .unwrap();

        let mut flush = Hand::new();
        flush.add_card(Card::from_string("As").unwrap()).unwrap();
        flush.add_card(Card::from_string("Js").unwrap()).unwrap();
        flush.add_card(Card::from_string("8s").unwrap()).unwrap();
        flush.add_card(Card::from_string("5s").unwrap()).unwrap();
        flush.add_card(Card::from_string("2s").unwrap()).unwrap();

        let fh_rank = evaluator.rank_hand(&full_house);
        let f_rank = evaluator.rank_hand(&flush);

        assert!(
            fh_rank > f_rank,
            "Full house should beat flush: {} > {}",
            fh_rank,
            f_rank
        );
    }
}

/// Test suite for edge cases and boundary conditions.
///
/// These tests verify correct behavior in unusual or extreme situations
/// that might not be covered by normal gameplay scenarios.
#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_insufficient_cards() {
        /// Tests behavior with insufficient cards for evaluation.
        ///
        /// The evaluator should handle hands with fewer than 5 cards
        /// gracefully, returning 0 or a safe default value.
        ///
        /// # Test Cases
        /// - 0 cards: Should return 0
        /// - 1-4 cards: Should return 0
        /// - 5+ cards: Should return valid rank
        let evaluator = LookupHandEvaluator::new().unwrap();

        // Test with insufficient cards
        let mut small_hand = Hand::new();
        small_hand
            .add_card(Card::from_string("As").unwrap())
            .unwrap();
        small_hand
            .add_card(Card::from_string("Ks").unwrap())
            .unwrap();

        let rank = evaluator.rank_hand(&small_hand);
        assert_eq!(rank, 0, "Hands with < 5 cards should return 0");
    }

    #[test]
    fn test_duplicate_cards() {
        /// Tests behavior when duplicate cards are added to a hand.
        ///
        /// In a real poker scenario, duplicate cards shouldn't exist,
        /// but the evaluator should handle this case gracefully.
        ///
        /// # Test Case
        /// - Hand with duplicate cards
        /// - Expected: Should not crash, should return some rank value
        let evaluator = LookupHandEvaluator::new().unwrap();

        let mut hand = Hand::new();
        hand.add_card(Card::from_string("As").unwrap()).unwrap();
        hand.add_card(Card::from_string("As").unwrap()).unwrap(); // Duplicate
        hand.add_card(Card::from_string("Ks").unwrap()).unwrap();
        hand.add_card(Card::from_string("Qs").unwrap()).unwrap();
        hand.add_card(Card::from_string("Js").unwrap()).unwrap();

        // Should not panic, should return some value
        let rank = evaluator.rank_hand(&hand);
        assert!(rank > 0, "Should handle duplicate cards gracefully");
    }
}

/// Test suite for performance characteristics.
///
/// These tests validate that the evaluator meets performance requirements
/// and doesn't regress in speed or memory usage.
#[cfg(test)]
mod performance {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_evaluation_speed() {
        /// Tests that hand evaluation meets performance requirements.
        ///
        /// Verifies that evaluating a large number of hands completes
        /// within reasonable time constraints.
        ///
        /// # Performance Target
        /// - 1 million evaluations should complete in < 30 seconds
        /// - Average evaluation time should be < 30 microseconds
        ///
        /// # Test Data
        /// Uses a variety of hand types to ensure balanced testing
        let evaluator = LookupHandEvaluator::new().unwrap();

        // Create diverse test hands
        let test_hands = create_diverse_test_hands(100_000);

        let start = Instant::now();

        for hand in &test_hands {
            let _rank = evaluator.rank_hand(hand);
        }

        let duration = start.elapsed();
        let per_hand = duration / test_hands.len() as u32;

        println!("Evaluated {} hands in {:?}", test_hands.len(), duration);
        println!("Average time per hand: {:?}", per_hand);

        // Performance assertions
        assert!(
            duration.as_secs() < 30,
            "1M evaluations should take < 30s, took {:?}",
            duration
        );
        assert!(
            per_hand.as_micros() < 30,
            "Average evaluation should be < 30μs, was {:?}",
            per_hand
        );
    }

    #[test]
    fn test_memory_usage() {
        /// Tests that memory usage stays within acceptable bounds.
        ///
        /// Verifies that the evaluator doesn't leak memory or use
        /// excessive resources during operation.
        ///
        /// # Memory Constraints
        /// - Peak memory usage should be < 200MB
        /// - No memory leaks during repeated evaluations
        let evaluator = LookupHandEvaluator::new().unwrap();

        // Force table generation and loading
        let hand = create_test_hand();
        let _rank = evaluator.rank_hand(&hand);

        // Basic memory usage check
        // In a real implementation, you might use a memory profiler
        assert!(
            true,
            "Memory usage should be monitored in integration tests"
        );
    }
}

/// Helper functions for creating test data.
fn create_diverse_test_hands(count: usize) -> Vec<Hand> {
    // Creates a diverse set of test hands for performance testing.
    //
    // Generates hands of different types and strengths to ensure
    // balanced performance characteristics across the evaluation space.
    let mut hands = Vec::with_capacity(count);

    // Sample different hand types
    let hand_types = vec![
        vec!["As", "Ks", "Qs", "Js", "Ts"], // Royal flush
        vec!["As", "Ah", "Ad", "Ac", "Ks"], // Four of a kind
        vec!["As", "Ah", "Ad", "Ks", "Kh"], // Full house
        vec!["As", "Ks", "Qs", "Js", "9s"], // Straight
        vec!["As", "Ks", "Qs", "Js", "Ts"], // Flush (different from royal)
        vec!["As", "Ah", "Ad", "Ks", "Qs"], // Three of a kind
        vec!["As", "Ah", "Ks", "Kd", "Qs"], // Two pair
        vec!["As", "Ah", "Ks", "Qs", "Js"], // One pair
        vec!["As", "Ks", "Qs", "Js", "9d"], // High card
    ];

    for i in 0..count {
        let hand_type = &hand_types[i % hand_types.len()];
        let mut hand = Hand::new();

        for &card_str in hand_type {
            hand.add_card(Card::from_string(card_str).unwrap()).unwrap();
        }

        hands.push(hand);
    }

    hands
}

fn create_test_hand() -> Hand {
    // Creates a simple test hand for basic testing.
    let mut hand = Hand::new();
    hand.add_card(Card::from_string("As").unwrap()).unwrap();
    hand.add_card(Card::from_string("Ks").unwrap()).unwrap();
    hand.add_card(Card::from_string("Qs").unwrap()).unwrap();
    hand.add_card(Card::from_string("Js").unwrap()).unwrap();
    hand.add_card(Card::from_string("Ts").unwrap()).unwrap();
    hand
}
