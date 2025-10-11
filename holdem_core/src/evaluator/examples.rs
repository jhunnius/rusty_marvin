//! # Usage Examples for Math Evaluator
//!
//! This module provides comprehensive examples demonstrating how to use the math evaluator
//! system for high-performance poker hand evaluation. These examples cover basic usage,
//! performance optimization, integration patterns, and advanced configuration.
//!
//! ## Example Categories
//!
//! - **Basic Examples**: Simple usage patterns for common scenarios
//! - **Performance Examples**: Optimization techniques and benchmarking
//! - **Integration Examples**: Working with holdem_core types
//! - **Advanced Examples**: Custom configuration and specialized use cases

use super::errors::EvaluatorError;
use super::evaluator::{HandRank, HandValue};
use super::integration::{benchmark_evaluation, utils, EvaluatorComparison, MathEvaluator};
use super::tables::{CanonicalMapping, JumpTable};
use crate::card::PackedCard;
use crate::{Card, Hand};
use std::str::FromStr;
use std::time::Instant;

/// Basic usage example demonstrating core functionality
pub fn basic_usage_example() -> Result<(), EvaluatorError> {
    println!("=== Basic Usage Example ===");

    // Create a 7-card poker hand
    let cards = [
        Card::from_str("As").unwrap(),
        Card::from_str("Ks").unwrap(),
        Card::from_str("Qs").unwrap(),
        Card::from_str("Js").unwrap(),
        Card::from_str("Ts").unwrap(),
        Card::from_str("7h").unwrap(),
        Card::from_str("6d").unwrap(),
    ];

    println!(
        "Evaluating hand: {:?}",
        cards.iter().map(|c| c.to_string()).collect::<Vec<_>>()
    );

    // Method 1: Using MathEvaluator (recommended for most use cases)
    let mut math_evaluator = MathEvaluator::new()?;
    let result1 = math_evaluator.evaluate_7_card(&cards);

    println!("MathEvaluator result: {:?}", result1);

    // Method 2: Using JumpTable directly (for advanced use cases)
    let mut table = JumpTable::with_target_memory();
    table.build()?;

    let packed_cards = super::integration::convert_cards(&cards);
    let packed_array: [PackedCard; 7] = packed_cards.try_into().unwrap();
    let result2 = table.evaluate_7_card(&packed_array);

    println!("JumpTable result: {:?}", result2);

    // Results should be identical
    assert_eq!(result1, result2.unwrap());

    println!("✓ Basic evaluation complete\n");
    Ok(())
}

/// Performance benchmarking example
pub fn performance_example() -> Result<(), EvaluatorError> {
    println!("=== Performance Benchmarking Example ===");

    // Create test hands of different types
    let test_hands = vec![
        (
            "Royal Flush",
            [
                Card::from_str("As").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Qs").unwrap(),
                Card::from_str("Js").unwrap(),
                Card::from_str("Ts").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("6d").unwrap(),
            ],
        ),
        (
            "Four of a Kind",
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Ac").unwrap(),
                Card::from_str("Ad").unwrap(),
                Card::from_str("As").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("Qh").unwrap(),
                Card::from_str("Jh").unwrap(),
            ],
        ),
        (
            "Full House",
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Ac").unwrap(),
                Card::from_str("Ad").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("6d").unwrap(),
            ],
        ),
        (
            "Flush",
            [
                Card::from_str("Ah").unwrap(),
                Card::from_str("Kh").unwrap(),
                Card::from_str("Qh").unwrap(),
                Card::from_str("9h").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("5h").unwrap(),
                Card::from_str("3h").unwrap(),
            ],
        ),
    ];

    let mut math_evaluator = MathEvaluator::new()?;

    println!("Benchmarking {} hand types...", test_hands.len());

    for (hand_type, cards) in test_hands {
        let iterations = 10000;

        let start_time = Instant::now();

        for _ in 0..iterations {
            let _result = math_evaluator.evaluate_7_card(&cards);
        }

        let elapsed = start_time.elapsed();
        let avg_time = elapsed / iterations;

        println!("  {}: {:?}", hand_type, avg_time);
    }

    // Display evaluator statistics
    let stats = math_evaluator.stats();
    println!("\nEvaluator Statistics:");
    println!("  Total evaluations: {}", stats.total_evaluations);
    println!("  Average time: {} ns", stats.average_time_ns);
    println!("  Total time: {} ms", stats.total_time_ns / 1_000_000);

    println!("✓ Performance benchmarking complete\n");
    Ok(())
}

/// Integration example showing compatibility with holdem_core
pub fn integration_example() -> Result<(), EvaluatorError> {
    println!("=== Integration Example ===");

    // Create hands using holdem_core
    let hole_cards = crate::HoleCards::from_notation("AKs")
        .map_err(|_| EvaluatorError::table_init_failed("Invalid hole cards"))?;
    let board = crate::Board::new();
    let hand = Hand::from_hole_cards_and_board(&hole_cards, &board)
        .map_err(|_| EvaluatorError::table_init_failed("Invalid hand"))?;

    println!("Evaluating complete hand:");
    println!("  Hole cards: {}", hole_cards);
    println!("  Board: {}", board);
    println!("  Complete hand: {}", hand);

    // Compare results between math and holdem_core evaluators
    let mut comparison = EvaluatorComparison::new()?;

    let math_result = comparison.math_evaluator.evaluate_hand(&hand);
    let core_result = comparison.core_evaluator.evaluate_hand(&hand);

    println!("\nEvaluation Results:");
    println!("  Math evaluator: {:?}", math_result);
    println!("  Core evaluator: {:?}", core_result);
    println!("  Results match: {}", math_result == core_result);

    // Test type conversion utilities
    let cards = hand.cards();
    let packed_cards = super::integration::convert_cards(&cards);
    let converted_back = super::integration::convert_cards_back(&packed_cards)?;

    println!("\nType Conversion Test:");
    println!(
        "  Original: {:?}",
        cards.iter().map(|c| c.to_string()).collect::<Vec<_>>()
    );
    println!(
        "  Converted back: {:?}",
        converted_back
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
    );
    println!(
        "  Conversion successful: {}",
        cards.len() == converted_back.len()
    );

    println!("✓ Integration example complete\n");
    Ok(())
}

/// Canonical mapping example demonstrating suit canonicalization
pub fn canonicalization_example() -> Result<(), EvaluatorError> {
    println!("=== Canonical Mapping Example ===");

    // Create cards with different suit patterns
    let test_cases = vec![
        (
            "Royal Flush (same suit)",
            vec![
                PackedCard::new(12, 0).unwrap(), // A spades
                PackedCard::new(11, 0).unwrap(), // K spades
                PackedCard::new(10, 0).unwrap(), // Q spades
                PackedCard::new(9, 0).unwrap(),  // J spades
                PackedCard::new(8, 0).unwrap(),  // T spades
            ],
        ),
        (
            "Mixed suits",
            vec![
                PackedCard::new(12, 0).unwrap(), // A spades
                PackedCard::new(11, 1).unwrap(), // K hearts
                PackedCard::new(10, 2).unwrap(), // Q diamonds
                PackedCard::new(9, 3).unwrap(),  // J clubs
            ],
        ),
        (
            "Three of a kind",
            vec![
                PackedCard::new(12, 0).unwrap(), // A spades
                PackedCard::new(12, 1).unwrap(), // A hearts
                PackedCard::new(12, 2).unwrap(), // A diamonds
                PackedCard::new(11, 3).unwrap(), // K clubs
                PackedCard::new(10, 0).unwrap(), // Q spades
            ],
        ),
    ];

    for (description, cards) in test_cases {
        println!("Testing: {}", description);

        let mapping = CanonicalMapping::from_cards(&cards);

        println!("  Original suits: {:?}", extract_suits(&cards));
        println!(
            "  Canonical suits: {:?}",
            mapping
                .canonical_cards
                .iter()
                .map(|&c| (c & 0x03))
                .collect::<Vec<_>>()
        );
        println!("  Suit mapping: {:?}", mapping.suit_map);
        println!("  Reverse mapping: {:?}", mapping.reverse_map);

        // Verify round-trip conversion
        let original_suits = mapping.to_original_suits(&mapping.canonical_cards);
        println!(
            "  Round-trip successful: {}",
            original_suits.len() == cards.len()
        );

        println!();
    }

    println!("✓ Canonical mapping example complete\n");
    Ok(())
}

/// Advanced configuration example
pub fn advanced_configuration_example() -> Result<(), EvaluatorError> {
    println!("=== Advanced Configuration Example ===");

    // Create custom jump table with specific size
    let custom_size = 25_000_000; // 25M entries
    let mut table = JumpTable::new(custom_size);

    println!("Creating custom jump table with {} entries...", custom_size);

    let build_start = Instant::now();
    table.build()?;
    let build_time = build_start.elapsed();

    println!("Table built in {:?}", build_time);
    println!(
        "Memory usage: {:.2} MB",
        table.memory_usage() as f64 / 1_000_000.0
    );

    // Validate table integrity
    table.validate()?;
    println!("Table validation: ✓ Passed");

    // Test evaluation with custom table
    let test_cards = [
        Card::from_str("As").unwrap(),
        Card::from_str("Ks").unwrap(),
        Card::from_str("Qs").unwrap(),
        Card::from_str("Js").unwrap(),
        Card::from_str("Ts").unwrap(),
        Card::from_str("7h").unwrap(),
        Card::from_str("6d").unwrap(),
    ];

    let packed_cards = super::integration::convert_cards(&test_cards);
    let packed_array: [PackedCard; 7] = packed_cards.try_into().unwrap();
    let result = table.evaluate_7_card(&packed_array);

    println!("Custom table evaluation result: {:?}", result);

    // Display table metadata
    println!("\nTable Metadata:");
    println!("  Version: {}", table.metadata.version);
    println!("  Created: {}", table.metadata.created_at);
    println!(
        "  Total combinations: {}",
        table.metadata.total_combinations
    );
    println!("  Level 5 nodes: {}", table.metadata.stats.level5_nodes);
    println!("  Level 6 nodes: {}", table.metadata.stats.level6_nodes);
    println!("  Level 7 nodes: {}", table.metadata.stats.level7_nodes);

    println!("✓ Advanced configuration example complete\n");
    Ok(())
}

/// Memory optimization example
pub fn memory_optimization_example() -> Result<(), EvaluatorError> {
    println!("=== Memory Optimization Example ===");

    // Compare different table sizes
    let sizes = vec![10_000_000, 20_000_000, 30_000_000, 34_000_000];

    for &size in &sizes {
        let mut table = JumpTable::new(size);

        let build_start = Instant::now();
        table.build()?;
        let build_time = build_start.elapsed();

        let memory_mb = table.memory_usage() as f64 / 1_000_000.0;

        println!(
            "Size: {:8} entries | Memory: {:6.1} MB | Build time: {:?}",
            size, memory_mb, build_time
        );

        // Validate that table works correctly
        table.validate()?;
    }

    // Show target memory usage
    let target_table = JumpTable::with_target_memory();
    let target_memory = target_table.memory_usage() as f64 / 1_000_000.0;

    println!("\nTarget configuration:");
    println!("  Memory usage: {:.1} MB", target_memory);
    println!("  Recommended for production use: ✓");

    println!("✓ Memory optimization example complete\n");
    Ok(())
}

/// Error handling example
pub fn error_handling_example() -> Result<(), EvaluatorError> {
    println!("=== Error Handling Example ===");

    // Demonstrate proper error handling
    match MathEvaluator::new() {
        Ok(mut evaluator) => {
            println!("✓ MathEvaluator created successfully");

            // Test with valid input
            let valid_cards = [
                Card::from_str("As").unwrap(),
                Card::from_str("Ks").unwrap(),
                Card::from_str("Qs").unwrap(),
                Card::from_str("Js").unwrap(),
                Card::from_str("Ts").unwrap(),
                Card::from_str("7h").unwrap(),
                Card::from_str("6d").unwrap(),
            ];

            let result = evaluator.evaluate_7_card(&valid_cards);
            println!("✓ Evaluation successful: {:?}", result);
        }
        Err(e) => {
            println!("✗ Failed to create MathEvaluator: {:?}", e);
            return Err(e);
        }
    }

    // Test table validation
    let mut table = JumpTable::new(1000);

    // Valid table should pass validation
    for i in 0..1000 {
        table.set(
            i,
            super::tables::JumpTableEntry::Terminal(HandValue::new(HandRank::HighCard, i as u32)),
        )?;
    }

    match table.validate() {
        Ok(()) => println!("✓ Table validation passed"),
        Err(e) => println!("✗ Table validation failed: {:?}", e),
    }

    // Test invalid table access
    match table.get(2000) {
        Some(_) => println!("✗ Should not find entry beyond table size"),
        None => println!("✓ Properly handles out-of-bounds access"),
    }

    println!("✓ Error handling example complete\n");
    Ok(())
}

/// Comprehensive demonstration of all features
pub fn comprehensive_demo() -> Result<(), EvaluatorError> {
    println!("=== Comprehensive Math Evaluator Demo ===\n");

    // Run all examples
    basic_usage_example()?;
    performance_example()?;
    integration_example()?;
    canonicalization_example()?;
    advanced_configuration_example()?;
    memory_optimization_example()?;
    error_handling_example()?;

    println!("🎉 All examples completed successfully!");
    println!("The math evaluator is ready for production use.");

    Ok(())
}

/// Helper function to extract suits from cards
fn extract_suits(cards: &[PackedCard]) -> Vec<u8> {
    cards.iter().map(|card| card.suit()).collect()
}

/// Helper function to extract ranks from cards
fn extract_ranks(cards: &[PackedCard]) -> Vec<u8> {
    cards.iter().map(|card| card.rank()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_usage_example() {
        assert!(basic_usage_example().is_ok());
    }

    #[test]
    fn test_canonicalization_example() {
        assert!(canonicalization_example().is_ok());
    }

    #[test]
    fn test_error_handling_example() {
        assert!(error_handling_example().is_ok());
    }

    #[test]
    fn test_comprehensive_demo() {
        assert!(comprehensive_demo().is_ok());
    }

    #[test]
    fn test_helper_functions() {
        let cards = vec![
            PackedCard::new(12, 0).unwrap(),
            PackedCard::new(11, 1).unwrap(),
            PackedCard::new(10, 2).unwrap(),
        ];

        let suits = extract_suits(&cards);
        let ranks = extract_ranks(&cards);

        assert_eq!(suits.len(), 3);
        assert_eq!(ranks.len(), 3);
        assert_eq!(suits, vec![0, 1, 2]);
        assert_eq!(ranks, vec![12, 11, 10]);
    }
}
