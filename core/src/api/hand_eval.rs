//! # Poker Hand Evaluation API
//!
//! This module defines the core trait for poker hand evaluation implementations.
//! The `HandEval` trait provides a standardized interface for evaluating poker
//! hands and generating human-readable descriptions of hand strengths.
//!
//! ## Hand Ranking System
//!
//! The evaluation system uses a 32-bit integer ranking where:
//! - Bits 24-27: Hand category (straight flush, quads, etc.)
//! - Bits 0-23: Relative value within the category
//!
//! Higher values represent stronger hands, with the absolute strongest
//! hand (royal flush) having the highest value.
//!
//! ## Trait Design
//!
//! The trait is designed to support multiple evaluation implementations:
//! - Lookup table based (current implementation)
//! - Procedural evaluation algorithms
//! - Hardware-accelerated evaluators
//!
//! All implementations must produce identical rankings for fair comparison.

use crate::api::card::Card;
use crate::api::hand::Hand;

/// Core trait for poker hand evaluation implementations.
///
/// This trait defines the standard interface that all poker hand evaluators
/// must implement. It supports evaluation of 5, 6, and 7-card hands as well
/// as generation of human-readable hand descriptions.
///
/// # Hand Categories
/// The ranking system recognizes these hand categories in descending order:
/// - Straight Flush (9)
/// - Four of a Kind (8)
/// - Full House (7)
/// - Flush (6)
/// - Straight (5)
/// - Three of a Kind (4)
/// - Two Pair (3)
/// - One Pair (2)
/// - High Card (1)
pub trait HandEval {
    /// Bit shift for extracting hand category from rank value.
    /// Categories occupy bits 24-27 of the 32-bit rank.
    const HAND_CATEGORY_SHIFT: u32 = 24;

    /// Bit mask for extracting hand category from rank value.
    /// Uses bits 24-27: 0xF0000000
    const HAND_CATEGORY_MASK: u32 = 0xF << Self::HAND_CATEGORY_SHIFT;

    /// Bit mask for extracting relative value within hand category.
    /// Uses bits 0-23: 0x00FFFFFF
    const VALUE_MASK: u32 = 0x000FFFFF;

    /// Evaluates a poker hand of any size (5, 6, or 7 cards).
    ///
    /// This method automatically handles hands of different sizes by finding
    /// the best 5-card combination for hands with 6 or 7 cards.
    ///
    /// # Arguments
    /// * `hand` - The poker hand to evaluate (5-7 cards)
    ///
    /// # Returns
    /// * `u32` - Standardized hand rank value
    fn rank_hand(&self, hand: &Hand) -> u32;

    /// Evaluates a 7-card poker hand specifically.
    ///
    /// This method considers all possible 5-card combinations from the 7 cards
    /// and returns the rank of the best possible hand.
    ///
    /// # Arguments
    /// * `cards` - Array of exactly 7 cards to evaluate
    ///
    /// # Returns
    /// * `u32` - Standardized hand rank value of best 5-card combination
    fn rank_hand7(&self, cards: &[Card; 7]) -> u32;

    /// Generates a human-readable description of a hand rank value.
    ///
    /// Converts a numeric rank into a descriptive string showing the
    /// hand type and relevant card values.
    ///
    /// # Arguments
    /// * `hand_value` - Numeric rank value to describe
    ///
    /// # Returns
    /// * `String` - Human-readable hand description
    ///
    /// # Example
    /// ```text
    /// "Straight Flush (A-K-Q-J-T)"
    /// "Four of a Kind (Aces)"
    /// "Full House (Kings over Queens)"
    /// ```
    fn hand_description(&self, hand_value: u32) -> String;

    /// Generates a description of a 5-card hand.
    ///
    /// Convenience method that evaluates the hand and then describes it.
    ///
    /// # Arguments
    /// * `hand` - The 5-card hand to evaluate and describe
    ///
    /// # Returns
    /// * `String` - Human-readable description of the hand
    fn hand_description_hand(&self, hand: &Hand) -> String;

    /// Generates a description of a 7-card hand.
    ///
    /// Convenience method that evaluates the best 5-card combination
    /// from 7 cards and then describes it.
    ///
    /// # Arguments
    /// * `cards` - Array of 7 cards to evaluate and describe
    ///
    /// # Returns
    /// * `String` - Human-readable description of the best hand
    fn hand_description7(&self, cards: &[Card; 7]) -> String;
}
