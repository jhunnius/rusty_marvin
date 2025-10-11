//! # Performance Benchmarks for Jump Table Evaluator
//!
//! This module provides comprehensive performance benchmarks to verify speed improvements
//! and ensure the jump table evaluator meets performance requirements.

use super::tables::{JumpTable, CanonicalMapping};
use holdem_core::card::PackedCard;
use holdem_core::evaluator::{HandRank, HandValue};
use holdem_core::{Card, Hand};
use std::time::{Duration, Instant};

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of iterations for each test
    pub iterations: usize,
    /// Warm-up iterations before measurement
    pub warmup_iterations: usize,
    /// Whether to include memory usage measurements
    pub measure_memory: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 1_000_000,
            warmup_iterations: 100_000,
            measure_memory: true,
        }
    }
}

/// Benchmark results for a single test
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Test name
    pub name: String,
    /// Total time taken
    pub total_time: Duration,
    /// Average time per operation
    pub avg_time_per_op: Duration,
    /// Operations per second
    pub ops_per_second: f64,
    /// Memory usage in bytes (if measured)
    pub memory_usage: Option<usize>,
}

impl BenchmarkResult {
    /// Create a new benchmark result
    pub fn new(name: String, total_time: Duration, iterations: usize, memory_usage: Option<usize>) -> Self {
        let avg_time_per_op = total_time / iterations as u32;
        let ops_per_second = 1_000_000_000f64 / avg_time_per_op.as_nanos() as f64 * iterations as f64;

        Self {
            name,
            total_time,
            avg_time_per_op,
            ops_per_second,
            memory_usage,
        }
    }
}

/// Performance benchmark suite for jump table evaluator
pub struct JumpTableBenchmark {
    config: BenchmarkConfig,
    table: JumpTable,
}

impl JumpTableBenchmark {
    /// Create a new benchmark suite
    pub fn new(config: BenchmarkConfig) -> Result<Self, String> {
        let mut table = JumpTable::with_target_memory();

        // Build the table for benchmarking
        table.build().map_err(|e| format!("Failed to build jump table: {:?}", e))?;

        Ok(Self { config, table })
    }

    /// Run all benchmarks
    pub fn run_all_benchmarks(&self) -> Result<Vec<BenchmarkResult>, String> {
        let mut results = Vec::new();

        // Benchmark 5-card evaluation
        results.push(self.benchmark_5_card_evaluation()?);

        // Benchmark 6-card evaluation
        results.push(self.benchmark_6_card_evaluation()?);

        // Benchmark 7-card evaluation
        results.push(self.benchmark_7_card_evaluation()?);

        // Benchmark canonicalization
        results.push(self.benchmark_canonicalization()?);

        // Benchmark memory usage
        if self.config.measure_memory {
            results.push(self.benchmark_memory_usage()?);
        }

        Ok(results)
    }

    /// Benchmark 5-card hand evaluation performance
    fn benchmark_5_card_evaluation(&self) -> Result<BenchmarkResult, String> {
        // Create test hands
        let test_hands = self.generate_test_5_card_hands(1000);

        // Warm-up
        for _ in 0..self.config.warmup_iterations {
            let hand = &test_hands[0];
            let _ = self.table.evaluate_5_card(hand);
        }

        // Actual benchmark
        let start_time = Instant::now();

        for _ in 0..self.config.iterations {
            let hand_index = 0; // Use first hand for consistent testing
            let _ = self.table.evaluate_5_card(&test_hands[hand_index]);
        }

        let total_time = start_time.elapsed();
        let memory_usage = if self.config.measure_memory {
            Some(self.table.memory_usage())
        } else {
            None
        };

        Ok(BenchmarkResult::new(
            "5-Card Evaluation".to_string(),
            total_time,
            self.config.iterations,
            memory_usage,
        ))
    }

    /// Benchmark 6-card hand evaluation performance
    fn benchmark_6_card_evaluation(&self) -> Result<BenchmarkResult, String> {
        // Create test hands
        let test_hands = self.generate_test_6_card_hands(1000);

        // Warm-up
        for _ in 0..self.config.warmup_iterations {
            let hand = &test_hands[0];
            let _ = self.table.evaluate_6_card(hand);
        }

        // Actual benchmark
        let start_time = Instant::now();

        for _ in 0..self.config.iterations {
            let hand_index = 0; // Use first hand for consistent testing
            let _ = self.table.evaluate_6_card(&test_hands[hand_index]);
        }

        let total_time = start_time.elapsed();

        Ok(BenchmarkResult::new(
            "6-Card Evaluation".to_string(),
            total_time,
            self.config.iterations,
            None,
        ))
    }

    /// Benchmark 7-card hand evaluation performance
    fn benchmark_7_card_evaluation(&self) -> Result<BenchmarkResult, String> {
        // Create test hands
        let test_hands = self.generate_test_7_card_hands(1000);

        // Warm-up
        for _ in 0..self.config.warmup_iterations {
            let hand = &test_hands[0];
            let _ = self.table.evaluate_7_card(hand);
        }

        // Actual benchmark
        let start_time = Instant::now();

        for _ in 0..self.config.iterations {
            let hand_index = 0; // Use first hand for consistent testing
            let _ = self.table.evaluate_7_card(&test_hands[hand_index]);
        }

        let total_time = start_time.elapsed();

        Ok(BenchmarkResult::new(
            "7-Card Evaluation".to_string(),
            total_time,
            self.config.iterations,
            None,
        ))
    }

    /// Benchmark canonicalization performance
    fn benchmark_canonicalization(&self) -> Result<BenchmarkResult, String> {
        // Create test hands
        let test_hands = self.generate_test_7_card_hands(1000);

        // Warm-up
        for _ in 0..self.config.warmup_iterations {
            let hand = &test_hands[0];
            let _ = CanonicalMapping::from_cards(hand);
        }

        // Actual benchmark
        let start_time = Instant::now();

        for _ in 0..self.config.iterations {
            let hand_index = 0; // Use first hand for consistent testing
            let _ = CanonicalMapping::from_cards(&test_hands[hand_index]);
        }

        let total_time = start_time.elapsed();

        Ok(BenchmarkResult::new(
            "Canonicalization".to_string(),
            total_time,
            self.config.iterations,
            None,
        ))
    }

    /// Benchmark memory usage
    fn benchmark_memory_usage(&self) -> Result<BenchmarkResult, String> {
        let memory_usage = self.table.memory_usage();

        Ok(BenchmarkResult::new(
            "Memory Usage".to_string(),
            Duration::new(0, 0),
            1,
            Some(memory_usage),
        ))
    }

    /// Generate test 5-card hands for benchmarking
    fn generate_test_5_card_hands(&self, count: usize) -> Vec<[PackedCard; 5]> {
        let mut hands = Vec::new();

        // Generate a variety of test hands
        for i in 0..count.min(1000) {
            let mut hand = [PackedCard::new(0, 0).unwrap(); 5];

            // Create diverse hands for realistic benchmarking
            for j in 0..5 {
                let rank = (i * 7 + j * 13) % 13;
                let suit = (i * 11 + j * 17) % 4;
                hand[j] = PackedCard::new(rank as u8, suit as u8).unwrap_or_else(|_| {
                    PackedCard::new(0, 0).unwrap()
                });
            }

            hands.push(hand);
        }

        // Fill remaining slots if needed
        while hands.len() < count {
            hands.push(hands[hands.len() % hands.len()]);
        }

        hands
    }

    /// Generate test 6-card hands for benchmarking
    fn generate_test_6_card_hands(&self, count: usize) -> Vec<[PackedCard; 6]> {
        let mut hands = Vec::new();

        // Generate a variety of test hands
        for i in 0..count.min(1000) {
            let mut hand = [PackedCard::new(0, 0).unwrap(); 6];

            // Create diverse hands for realistic benchmarking
            for j in 0..6 {
                let rank = (i * 7 + j * 13) % 13;
                let suit = (i * 11 + j * 17) % 4;
                hand[j] = PackedCard::new(rank as u8, suit as u8).unwrap_or_else(|_| {
                    PackedCard::new(0, 0).unwrap()
                });
            }

            hands.push(hand);
        }

        // Fill remaining slots if needed
        while hands.len() < count {
            hands.push(hands[hands.len() % hands.len()]);
        }

        hands
    }

    /// Generate test 7-card hands for benchmarking
    fn generate_test_7_card_hands(&self, count: usize) -> Vec<[PackedCard; 7]> {
        let mut hands = Vec::new();

        // Generate a variety of test hands
        for i in 0..count.min(1000) {
            let mut hand = [PackedCard::new(0, 0).unwrap(); 7];

            // Create diverse hands for realistic benchmarking
            for j in 0..7 {
                let rank = (i * 7 + j * 13) % 13;
                let suit = (i * 11 + j * 17) % 4;
                hand[j] = PackedCard::new(rank as u8, suit as u8).unwrap_or_else(|_| {
                    PackedCard::new(0, 0).unwrap()
                });
            }

            hands.push(hand);
        }

        // Fill remaining slots if needed
        while hands.len() < count {
            hands.push(hands[hands.len() % hands.len()]);
        }

        hands
    }

    /// Print benchmark results in a formatted table
    pub fn print_results(&self, results: &[BenchmarkResult]) {
        println!("\n=== Jump Table Performance Benchmarks ===");
        println!("{:<20} {:<15} {:<15} {:<15} {}",
                 "Test", "Total Time", "Avg Time/Op", "Ops/Second", "Memory");

        for result in results {
            println!(
                "{:<20} {:<15} {:<15} {:<15.0} {}",
                result.name,
                format!("{:.2?}", result.total_time),
                format!("{:.2?}", result.avg_time_per_op),
                result.ops_per_second,
                match result.memory_usage {
                    Some(bytes) => format!("{} MB", bytes / 1_000_000),
                    None => "N/A".to_string(),
                }
            );
        }

        println!("\n=== Performance Requirements Check ===");
        self.check_performance_requirements(results);
    }

    /// Check if performance meets requirements
    fn check_performance_requirements(&self, results: &[BenchmarkResult]) {
        for result in results {
            match result.name.as_str() {
                "5-Card Evaluation" => {
                    let target_ns = 100; // Target: 100 nanoseconds
                    let actual_ns = result.avg_time_per_op.as_nanos() as u64;
                    println!(
                        "5-Card Evaluation: {} ns (target: {} ns) - {}",
                        actual_ns,
                        target_ns,
                        if actual_ns <= target_ns { "✓ PASS" } else { "✗ FAIL" }
                    );
                }
                "6-Card Evaluation" => {
                    let target_ns = 500; // Target: 500 nanoseconds
                    let actual_ns = result.avg_time_per_op.as_nanos() as u64;
                    println!(
                        "6-Card Evaluation: {} ns (target: {} ns) - {}",
                        actual_ns,
                        target_ns,
                        if actual_ns <= target_ns { "✓ PASS" } else { "✗ FAIL" }
                    );
                }
                "7-Card Evaluation" => {
                    let target_us = 2; // Target: 2 microseconds
                    let actual_us = result.avg_time_per_op.as_micros() as u64;
                    println!(
                        "7-Card Evaluation: {} μs (target: {} μs) - {}",
                        actual_us,
                        target_us,
                        if actual_us <= target_us { "✓ PASS" } else { "✗ FAIL" }
                    );
                }
                "Memory Usage" => {
                    let target_mb = 130; // Target: 130 MB
                    if let Some(bytes) = result.memory_usage {
                        let actual_mb = bytes / 1_000_000;
                        println!(
                            "Memory Usage: {} MB (target: {} MB) - {}",
                            actual_mb,
                            target_mb,
                            if actual_mb <= target_mb { "✓ PASS" } else { "✗ FAIL" }
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

/// Run a quick performance test with default configuration
pub fn run_quick_benchmark() -> Result<Vec<BenchmarkResult>, String> {
    let config = BenchmarkConfig {
        iterations: 100_000,
        warmup_iterations: 10_000,
        measure_memory: true,
    };

    let benchmark = JumpTableBenchmark::new(config)?;
    benchmark.run_all_benchmarks()
}

/// Run a comprehensive performance test
pub fn run_comprehensive_benchmark() -> Result<Vec<BenchmarkResult>, String> {
    let config = BenchmarkConfig {
        iterations: 1_000_000,
        warmup_iterations: 100_000,
        measure_memory: true,
    };

    let benchmark = JumpTableBenchmark::new(config)?;
    benchmark.run_all_benchmarks()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_creation() {
        let config = BenchmarkConfig::default();
        let benchmark = JumpTableBenchmark::new(config);
        assert!(benchmark.is_ok());
    }

    #[test]
    fn test_quick_benchmark() {
        let results = run_quick_benchmark();
        assert!(results.is_ok());

        let results = results.unwrap();
        assert!(!results.is_empty());

        // Check that we have expected benchmark types
        let names: Vec<&str> = results.iter().map(|r| r.name.as_str()).collect();
        assert!(names.contains(&"5-Card Evaluation"));
        assert!(names.contains(&"6-Card Evaluation"));
        assert!(names.contains(&"7-Card Evaluation"));
    }

    #[test]
    fn test_benchmark_results_structure() {
        let results = run_quick_benchmark().unwrap();

        for result in results {
            // Check that timing values are reasonable
            assert!(result.total_time.as_nanos() > 0);
            assert!(result.avg_time_per_op.as_nanos() > 0);
            assert!(result.ops_per_second > 0.0);

            // Check that memory usage is reasonable (if measured)
            if let Some(memory) = result.memory_usage {
                assert!(memory > 0);
                assert!(memory < 200_000_000); // Less than 200MB
            }
        }
    }

    #[test]
    fn test_performance_requirements() {
        let results = run_quick_benchmark().unwrap();

        // Basic sanity checks for performance
        for result in results {
            match result.name.as_str() {
                "5-Card Evaluation" => {
                    // Should be very fast (< 1 microsecond)
                    assert!(result.avg_time_per_op.as_micros() < 1);
                }
                "6-Card Evaluation" => {
                    // Should be reasonably fast (< 5 microseconds)
                    assert!(result.avg_time_per_op.as_micros() < 5);
                }
                "7-Card Evaluation" => {
                    // Should be fast (< 10 microseconds)
                    assert!(result.avg_time_per_op.as_micros() < 10);
                }
                _ => {}
            }
        }
    }
}