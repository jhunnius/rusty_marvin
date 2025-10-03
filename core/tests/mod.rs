//! # Poker API Test Suite
//!
//! Comprehensive test suite for the poker hand evaluation library.
//! Tests are organized into logical groups covering different aspects
//! of the poker evaluation system.
//!
//! ## Test Categories
//!
//! - **Unit Tests**: Test individual components and algorithms
//! - **Integration Tests**: Test complete workflows and interactions
//! - **Performance Tests**: Validate speed and memory characteristics
//! - **Regression Tests**: Ensure compatibility with reference implementations
//!
//! ## Running Tests
//!
//! ```bash
//! # Run all tests
//! cargo test
//!
//! # Run specific test categories
//! cargo test api_integration
//! cargo test hand_eval_integration
//!
//! # Run with performance benchmarks
//! cargo test hand_eval_integration_test -- --nocapture
//! ```
//!
//! ## Test Data
//!
//! Tests use both deterministic test cases and randomized inputs to ensure
//! comprehensive coverage of the evaluation space.

/// Helper modules for test utilities and fixtures
mod helpers;

/// Integration tests for the poker API components
mod api_integration_test;

/// Comprehensive hand evaluation integration tests
mod hand_eval_integration_test;

/// Unit tests for individual components and algorithms
mod unit_tests;

/// Comprehensive tests for evaluator generator components
mod evaluator_generator_tests;
