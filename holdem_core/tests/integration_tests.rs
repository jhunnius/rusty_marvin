//! Integration tests for poker data structures
//!
//! These tests verify that the holdem_core poker structures work together correctly,
//! including serialization round-trips and performance benchmarks.

use holdem_core::{board, card, deck, hand, hole_cards};
use std::time::Instant;

#[test]
fn test_integration_hand_from_hole_cards_and_board() {
    // Test creating hands from various hole card and board combinations

    // Test with preflop (no board cards)
    let hole_cards = hole_cards::HoleCards::new(
        card::Card::new(12, 0).unwrap(), // As
        card::Card::new(11, 1).unwrap(), // Kh
    )
    .unwrap();
    let empty_board = board::Board::new();
    let preflop_hand = hand::Hand::from_hole_cards_and_board(&hole_cards, &empty_board).unwrap();
    assert_eq!(preflop_hand.len, 2);
    assert_eq!(preflop_hand.cards()[0], card::Card::new(12, 0).unwrap());
    assert_eq!(preflop_hand.cards()[1], card::Card::new(11, 1).unwrap());

    // Test with flop
    let mut flop_board = board::Board::new();
    flop_board
        .deal_flop(vec![
            card::Card::new(10, 2).unwrap(), // Qd
            card::Card::new(9, 3).unwrap(),  // Js
            card::Card::new(8, 0).unwrap(),  // Tc
        ])
        .unwrap();
    let flop_hand = hand::Hand::from_hole_cards_and_board(&hole_cards, &flop_board).unwrap();
    assert_eq!(flop_hand.len, 5);
    assert_eq!(flop_hand.cards()[0], card::Card::new(12, 0).unwrap()); // As
    assert_eq!(flop_hand.cards()[1], card::Card::new(11, 1).unwrap()); // Kh
    assert_eq!(flop_hand.cards()[2], card::Card::new(10, 2).unwrap()); // Qd
    assert_eq!(flop_hand.cards()[3], card::Card::new(9, 3).unwrap()); // Js
    assert_eq!(flop_hand.cards()[4], card::Card::new(8, 0).unwrap()); // Tc

    // Test with turn
    let mut turn_board = flop_board.clone();
    turn_board
        .deal_turn(card::Card::new(7, 1).unwrap())
        .unwrap(); // 9h
    let turn_hand = hand::Hand::from_hole_cards_and_board(&hole_cards, &turn_board).unwrap();
    assert_eq!(turn_hand.len, 6);

    // Test with river
    let mut river_board = turn_board.clone();
    river_board
        .deal_river(card::Card::new(6, 2).unwrap())
        .unwrap(); // 8d
    let river_hand = hand::Hand::from_hole_cards_and_board(&hole_cards, &river_board).unwrap();
    assert_eq!(river_hand.len, 7);
}

#[test]
fn test_integration_hand_from_hole_cards_and_board_edge_cases() {
    // Test edge cases for hand creation from hole cards and board

    // Test with pocket aces and board
    let pocket_aces = hole_cards::HoleCards::new(
        card::Card::new(12, 0).unwrap(), // As
        card::Card::new(12, 1).unwrap(), // Ah
    )
    .unwrap();
    let mut board = board::Board::new();
    board
        .deal_flop(vec![
            card::Card::new(10, 2).unwrap(), // Qd
            card::Card::new(9, 3).unwrap(),  // Js
            card::Card::new(8, 0).unwrap(),  // Tc
        ])
        .unwrap();
    let hand = hand::Hand::from_hole_cards_and_board(&pocket_aces, &board).unwrap();
    assert_eq!(hand.len, 5);

    // Test with suited connectors
    let suited_connectors = hole_cards::HoleCards::new(
        card::Card::new(11, 0).unwrap(), // Ks
        card::Card::new(10, 0).unwrap(), // Qs
    )
    .unwrap();
    let hand = hand::Hand::from_hole_cards_and_board(&suited_connectors, &board).unwrap();
    assert_eq!(hand.len, 5);

    // Test with empty board (should work)
    let empty_board = board::Board::new();
    let hand = hand::Hand::from_hole_cards_and_board(&pocket_aces, &empty_board).unwrap();
    assert_eq!(hand.len, 2);
}

#[test]
fn test_integration_serialization_round_trips() {
    // Test that all structures can be serialized and deserialized correctly

    // Test Card serialization round-trip
    let original_card = card::Card::new(12, 3).unwrap(); // Ace of Spades
    let json = serde_json::to_string(&original_card).unwrap();
    let deserialized_card: card::Card = serde_json::from_str(&json).unwrap();
    assert_eq!(original_card, deserialized_card);

    // Test HoleCards serialization round-trip
    let original_hole_cards = hole_cards::HoleCards::new(
        card::Card::new(12, 0).unwrap(),
        card::Card::new(11, 1).unwrap(),
    )
    .unwrap();
    let json = serde_json::to_string(&original_hole_cards).unwrap();
    let deserialized_hole_cards: hole_cards::HoleCards = serde_json::from_str(&json).unwrap();
    assert_eq!(original_hole_cards, deserialized_hole_cards);

    // Test Board serialization round-trip
    let mut original_board = board::Board::new();
    original_board
        .deal_flop(vec![
            card::Card::new(10, 2).unwrap(),
            card::Card::new(9, 3).unwrap(),
            card::Card::new(8, 0).unwrap(),
        ])
        .unwrap();
    original_board
        .deal_turn(card::Card::new(7, 1).unwrap())
        .unwrap();
    original_board
        .deal_river(card::Card::new(6, 2).unwrap())
        .unwrap();

    let json = serde_json::to_string(&original_board).unwrap();
    let deserialized_board: board::Board = serde_json::from_str(&json).unwrap();
    assert_eq!(original_board, deserialized_board);

    // Test Hand serialization round-trip
    let original_hand =
        hand::Hand::from_hole_cards_and_board(&original_hole_cards, &original_board).unwrap();
    let json = serde_json::to_string(&original_hand).unwrap();
    let deserialized_hand: hand::Hand = serde_json::from_str(&json).unwrap();
    assert_eq!(original_hand, deserialized_hand);

    // Test Deck serialization round-trip
    let original_deck = deck::Deck::new();
    let json = serde_json::to_string(&original_deck).unwrap();
    let deserialized_deck: deck::Deck = serde_json::from_str(&json).unwrap();
    assert_eq!(original_deck.cards(), deserialized_deck.cards());
}

#[test]
fn test_integration_serialization_round_trips_comprehensive() {
    // Test serialization round-trips with various data formats and edge cases

    // Test with TOML serialization
    let card = card::Card::new(12, 3).unwrap();
    let toml = toml::to_string(&card).unwrap();
    let deserialized: card::Card = toml::from_str(&toml).unwrap();
    assert_eq!(card, deserialized);

    // Test complex hand serialization
    let hole_cards = hole_cards::HoleCards::new(
        card::Card::new(12, 0).unwrap(),
        card::Card::new(11, 1).unwrap(),
    )
    .unwrap();
    let mut board = board::Board::new();
    board
        .deal_flop(vec![
            card::Card::new(10, 2).unwrap(),
            card::Card::new(9, 3).unwrap(),
            card::Card::new(8, 0).unwrap(),
        ])
        .unwrap();

    let hand = hand::Hand::from_hole_cards_and_board(&hole_cards, &board).unwrap();

    // JSON round-trip
    let json = serde_json::to_string(&hand).unwrap();
    let deserialized: hand::Hand = serde_json::from_str(&json).unwrap();
    assert_eq!(hand, deserialized);

    // TOML round-trip
    let toml = toml::to_string(&hand).unwrap();
    let deserialized: hand::Hand = toml::from_str(&toml).unwrap();
    assert_eq!(hand, deserialized);

    // Test empty structures
    let empty_hand = hand::Hand::new(vec![]).unwrap();
    let json = serde_json::to_string(&empty_hand).unwrap();
    let deserialized: hand::Hand = serde_json::from_str(&json).unwrap();
    assert_eq!(empty_hand, deserialized);
}

#[test]
fn test_integration_performance_benchmarks() {
    // Performance benchmarks for common operations

    let iterations = 1000;

    // Benchmark hand creation from hole cards and board
    let hole_cards = hole_cards::HoleCards::new(
        card::Card::new(12, 0).unwrap(),
        card::Card::new(11, 1).unwrap(),
    )
    .unwrap();
    let mut board = board::Board::new();
    board
        .deal_flop(vec![
            card::Card::new(10, 2).unwrap(),
            card::Card::new(9, 3).unwrap(),
            card::Card::new(8, 0).unwrap(),
        ])
        .unwrap();

    let start = Instant::now();
    for _ in 0..iterations {
        let _hand = hand::Hand::from_hole_cards_and_board(&hole_cards, &board).unwrap();
    }
    let duration = start.elapsed();
    println!("Hand creation ({} iterations): {:?}", iterations, duration);
    assert!(
        duration.as_millis() < 100,
        "Hand creation too slow: {:?}",
        duration
    );

    // Benchmark serialization round-trips
    let hand = hand::Hand::from_hole_cards_and_board(&hole_cards, &board).unwrap();
    let start = Instant::now();
    for _ in 0..iterations {
        let json = serde_json::to_string(&hand).unwrap();
        let _: hand::Hand = serde_json::from_str(&json).unwrap();
    }
    let duration = start.elapsed();
    println!(
        "Serialization round-trip ({} iterations): {:?}",
        iterations, duration
    );
    assert!(
        duration.as_millis() < 500,
        "Serialization too slow: {:?}",
        duration
    );

    // Benchmark card creation and operations
    let start = Instant::now();
    let mut cards = Vec::new();
    for i in 0..iterations {
        cards.push(card::Card::new((i % 13) as u8, (i % 4) as u8));
    }
    let duration = start.elapsed();
    println!("Card creation ({} iterations): {:?}", iterations, duration);
    assert!(
        duration.as_millis() < 50,
        "Card creation too slow: {:?}",
        duration
    );

    // Benchmark deck operations
    let start = Instant::now();
    for _ in 0..(iterations / 10) {
        // Fewer iterations for deck operations
        let mut deck = deck::Deck::new();
        let _ = deck.shuffle(&mut rand::rng());
        let _ = deck.deal(5);
    }
    let duration = start.elapsed();
    println!(
        "Deck shuffle and deal ({} iterations): {:?}",
        iterations / 10,
        duration
    );
    assert!(
        duration.as_millis() < 200,
        "Deck operations too slow: {:?}",
        duration
    );
}

#[test]
fn test_integration_complex_scenarios() {
    // Test complex real-world scenarios

    // Test multiple hands from same board
    let mut board = board::Board::new();
    board
        .deal_flop(vec![
            card::Card::new(12, 2).unwrap(), // Ad
            card::Card::new(11, 3).unwrap(), // Ks
            card::Card::new(10, 0).unwrap(), // Qc
        ])
        .unwrap();
    board.deal_turn(card::Card::new(9, 1).unwrap()).unwrap(); // Jh
    board.deal_river(card::Card::new(8, 2).unwrap()).unwrap(); // Td

    let hole_cards1 = hole_cards::HoleCards::new(
        card::Card::new(12, 0).unwrap(), // As
        card::Card::new(12, 1).unwrap(), // Ah
    )
    .unwrap(); // Pocket aces

    let hole_cards2 = hole_cards::HoleCards::new(
        card::Card::new(11, 0).unwrap(), // Kc
        card::Card::new(11, 1).unwrap(), // Kh
    )
    .unwrap(); // Pocket kings

    let hand1 = hand::Hand::from_hole_cards_and_board(&hole_cards1, &board).unwrap();
    let hand2 = hand::Hand::from_hole_cards_and_board(&hole_cards2, &board).unwrap();

    assert_eq!(hand1.len, 7);
    assert_eq!(hand2.len, 7);

    // Both hands should contain all board cards plus their hole cards
    assert!(hand1.cards().contains(&card::Card::new(12, 0).unwrap())); // As in hand1
    assert!(hand1.cards().contains(&card::Card::new(12, 1).unwrap())); // Ah in hand1
    assert!(hand2.cards().contains(&card::Card::new(11, 0).unwrap())); // Kc in hand2
    assert!(hand2.cards().contains(&card::Card::new(11, 1).unwrap())); // Kh in hand2

    // Both hands should contain all board cards
    for card in board.visible_cards() {
        assert!(hand1.cards().contains(card));
        assert!(hand2.cards().contains(card));
    }
}

#[test]
fn test_integration_error_handling() {
    // Test error handling in integration scenarios

    // Test duplicate cards between hole cards and board
    let hole_cards = hole_cards::HoleCards::new(
        card::Card::new(12, 0).unwrap(), // As
        card::Card::new(11, 1).unwrap(), // Kh
    )
    .unwrap();

    let mut board = board::Board::new();
    board
        .deal_flop(vec![
            card::Card::new(12, 0).unwrap(), // Duplicate As
            card::Card::new(9, 3).unwrap(),
            card::Card::new(8, 0).unwrap(),
        ])
        .unwrap();

    // This should fail due to duplicate cards
    assert!(hand::Hand::from_hole_cards_and_board(&hole_cards, &board).is_err());

    // Test serialization of invalid states (should still work)
    let invalid_hand = hand::Hand::new(vec![
        card::Card::new(12, 0).unwrap(),
        card::Card::new(12, 0).unwrap(), // Duplicate
    ]);
    assert!(invalid_hand.is_err());
}

#[test]
fn test_integration_data_consistency() {
    // Test that data remains consistent across operations

    // Create a complex scenario and verify all data stays consistent
    let hole_cards = hole_cards::HoleCards::new(
        card::Card::new(12, 0).unwrap(),
        card::Card::new(11, 1).unwrap(),
    )
    .unwrap();

    let mut board = board::Board::new();
    let flop_cards = vec![
        card::Card::new(10, 2).unwrap(),
        card::Card::new(9, 3).unwrap(),
        card::Card::new(8, 0).unwrap(),
    ];
    board.deal_flop(flop_cards.clone()).unwrap();

    let hand = hand::Hand::from_hole_cards_and_board(&hole_cards, &board).unwrap();

    // Verify hand contains exactly the expected cards
    let expected_cards = vec![
        card::Card::new(12, 0).unwrap(), // hole
        card::Card::new(11, 1).unwrap(), // hole
        card::Card::new(10, 2).unwrap(), // flop
        card::Card::new(9, 3).unwrap(),  // flop
        card::Card::new(8, 0).unwrap(),  // flop
    ];

    assert_eq!(hand.len, 5);
    for expected in expected_cards {
        assert!(hand.cards().contains(&expected));
    }

    // Test serialization preserves data
    let json = serde_json::to_string(&hand).unwrap();
    let deserialized: hand::Hand = serde_json::from_str(&json).unwrap();
    assert_eq!(hand, deserialized);
    assert_eq!(hand.len, deserialized.len);
    for i in 0..hand.len {
        assert_eq!(hand.cards()[i], deserialized.cards()[i]);
    }
}
// # Integration Tests for Math Evaluator
//
// This module provides integration tests for the advanced evaluator system,
// ensuring compatibility and correct interaction with the holdem_core evaluator.

use holdem_core::card::PackedCard;
use holdem_core::evaluator::integration::{EvaluatorComparison, MathEvaluator};
use holdem_core::evaluator::tables::JumpTable;
use holdem_core::evaluator::{HandRank, HandValue};
use holdem_core::{Card, Hand};
use std::str::FromStr;

/// Integration test suite for the math evaluator
pub struct IntegrationTestSuite {
    /// Math evaluator instance
    math_evaluator: MathEvaluator,
    /// Holdem_core evaluator instance for comparison
    core_evaluator: std::sync::Arc<holdem_core::evaluator::Evaluator>,
}

/// Test results for integration testing
#[derive(Debug, Clone)]
pub struct IntegrationTestResults {
    /// Total number of tests run
    pub total_tests: usize,
    /// Number of passed tests
    pub passed_tests: usize,
    /// Number of failed tests
    pub failed_tests: usize,
    /// Test execution time in milliseconds
    pub execution_time_ms: u64,
}

impl IntegrationTestSuite {
    /// Create a new integration test suite
    pub fn new() -> Result<Self, holdem_core::evaluator::errors::EvaluatorError> {
        Ok(Self {
            math_evaluator: MathEvaluator::new()?,
            core_evaluator: holdem_core::evaluator::Evaluator::instance().clone(),
        })
    }

    /// Run all integration tests
    pub fn run_all_tests(
        &mut self,
    ) -> Result<IntegrationTestResults, holdem_core::evaluator::errors::EvaluatorError> {
        let start_time = std::time::Instant::now();

        println!("Running integration tests for math evaluator...");

        let mut total_tests = 0;
        let mut passed_tests = 0;
        let mut failed_tests = 0;

        // Test 5-card evaluation compatibility
        let (passed, failed) = self.test_5_card_compatibility()?;
        total_tests += passed + failed;
        passed_tests += passed;
        failed_tests += failed;

        // Test 6-card evaluation compatibility
        let (passed, failed) = self.test_6_card_compatibility()?;
        total_tests += passed + failed;
        passed_tests += passed;
        failed_tests += failed;

        // Test 7-card evaluation compatibility
        let (passed, failed) = self.test_7_card_compatibility()?;
        total_tests += passed + failed;
        passed_tests += passed;
        failed_tests += failed;

        // Test type conversion compatibility
        let (passed, failed) = self.test_type_conversion_compatibility()?;
        total_tests += passed + failed;
        passed_tests += passed;
        failed_tests += failed;

        let elapsed = start_time.elapsed();

        let results = IntegrationTestResults {
            total_tests,
            passed_tests,
            failed_tests,
            execution_time_ms: elapsed.as_millis() as u64,
        };

        println!("Integration tests completed: {:?}", results);
        Ok(results)
    }

    /// Test 5-card evaluation compatibility between math and core evaluators
    fn test_5_card_compatibility(
        &mut self,
    ) -> Result<(usize, usize), holdem_core::evaluator::errors::EvaluatorError> {
        println!("Testing 5-card evaluation compatibility...");

        let test_hands = generate_test_hands();
        let mut passed = 0;
        let mut failed = 0;

        for (i, cards) in test_hands.iter().enumerate() {
            if cards.len() >= 5 {
                let math_result = self
                    .math_evaluator
                    .evaluate_5_card(&cards[..5].try_into().unwrap());
                let core_result = self
                    .core_evaluator
                    .evaluate_5_card(&cards[..5].try_into().unwrap());

                if math_result == core_result {
                    passed += 1;
                } else {
                    failed += 1;
                    println!(
                        "5-card mismatch at test {}: math={:?}, core={:?}",
                        i, math_result, core_result
                    );
                }
            }
        }

        println!("5-card compatibility: {} passed, {} failed", passed, failed);
        Ok((passed, failed))
    }

    /// Test 6-card evaluation compatibility between math and core evaluators
    fn test_6_card_compatibility(
        &mut self,
    ) -> Result<(usize, usize), holdem_core::evaluator::errors::EvaluatorError> {
        println!("Testing 6-card evaluation compatibility...");

        let test_hands = generate_test_hands();
        let mut passed = 0;
        let mut failed = 0;

        for (i, cards) in test_hands.iter().enumerate() {
            if cards.len() >= 6 {
                let math_result = self
                    .math_evaluator
                    .evaluate_6_card(&cards[..6].try_into().unwrap());
                let core_result = self
                    .core_evaluator
                    .evaluate_6_card(&cards[..6].try_into().unwrap());

                if math_result == core_result {
                    passed += 1;
                } else {
                    failed += 1;
                    println!(
                        "6-card mismatch at test {}: math={:?}, core={:?}",
                        i, math_result, core_result
                    );
                }
            }
        }

        println!("6-card compatibility: {} passed, {} failed", passed, failed);
        Ok((passed, failed))
    }

    /// Test 7-card evaluation compatibility between math and core evaluators
    fn test_7_card_compatibility(
        &mut self,
    ) -> Result<(usize, usize), holdem_core::evaluator::errors::EvaluatorError> {
        println!("Testing 7-card evaluation compatibility...");

        let test_hands = generate_test_hands();
        let mut passed = 0;
        let mut failed = 0;

        for (i, cards) in test_hands.iter().enumerate() {
            let math_result = self.math_evaluator.evaluate_7_card(cards);
            let core_result = self.core_evaluator.evaluate_7_card(cards);

            if math_result == core_result {
                passed += 1;
            } else {
                failed += 1;
                println!(
                    "7-card mismatch at test {}: math={:?}, core={:?}",
                    i, math_result, core_result
                );
            }
        }

        println!("7-card compatibility: {} passed, {} failed", passed, failed);
        Ok((passed, failed))
    }

    /// Test type conversion compatibility
    fn test_type_conversion_compatibility(
        &mut self,
    ) -> Result<(usize, usize), holdem_core::evaluator::errors::EvaluatorError> {
        println!("Testing type conversion compatibility...");

        let mut passed = 0;
        let mut failed = 0;

        // Test conversion between Card and PackedCard
        let original_cards = [
            Card::from_str("As").unwrap(),
            Card::from_str("Ks").unwrap(),
            Card::from_str("Qs").unwrap(),
            Card::from_str("Js").unwrap(),
            Card::from_str("Ts").unwrap(),
        ];

        // Convert to PackedCard and back
        let packed_cards = holdem_core::evaluator::integration::convert_cards(&original_cards);
        let converted_back =
            holdem_core::evaluator::integration::convert_cards_back(&packed_cards)?;

        if original_cards.len() == converted_back.len() {
            passed += 1;
        } else {
            failed += 1;
            println!(
                "Type conversion length mismatch: {} vs {}",
                original_cards.len(),
                converted_back.len()
            );
        }

        // Test that evaluation works with converted cards
        let math_result_1 = self.math_evaluator.evaluate_5_card(&original_cards);
        let math_result_2 = self
            .math_evaluator
            .evaluate_5_card(&converted_back[..5].try_into().unwrap());

        if math_result_1 == math_result_2 {
            passed += 1;
        } else {
            failed += 1;
            println!(
                "Evaluation mismatch after conversion: {:?} vs {:?}",
                math_result_1, math_result_2
            );
        }

        println!(
            "Type conversion compatibility: {} passed, {} failed",
            passed, failed
        );
        Ok((passed, failed))
    }
}

/// Generate test hands for integration testing
fn generate_test_hands() -> Vec<[Card; 7]> {
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
    ]
}

/// Run integration tests
pub fn run_integration_tests(
) -> Result<IntegrationTestResults, holdem_core::evaluator::errors::EvaluatorError> {
    let mut suite = IntegrationTestSuite::new()?;
    suite.run_all_tests()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_test_suite_creation() {
        let suite = IntegrationTestSuite::new();
        assert!(suite.is_ok());
    }

    #[test]
    fn test_test_hands_generation() {
        let hands = generate_test_hands();
        assert_eq!(hands.len(), 5); // Should generate 5 test hands

        for hand in &hands {
            assert_eq!(hand.len(), 7); // All should be 7-card hands
        }
    }

    #[test]
    fn test_integration_test_runner() {
        let results = run_integration_tests();
        assert!(results.is_ok());

        let results = results.unwrap();
        assert!(results.total_tests > 0);
        assert!(results.execution_time_ms > 0);
    }
}
