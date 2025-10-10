//! # Hand Module
//!
//! This module provides the `Hand` struct for representing a complete poker hand
//! consisting of 5-7 cards for evaluation in Texas Hold'em poker. The Hand struct
//! supports evaluation of the best 5-card poker hand from available cards.
//!
//! ## Hand Representation
//!
//! A Hand consists of 0-7 distinct cards representing the complete hand available
//! to a player (hole cards + board cards).
//!
//! ## Examples
//!
//! ### Creating Hands
//!
//! ```rust
//! use holdem_core::{Hand, HoleCards, Board, Card};
//! use std::str::FromStr;
//!
//! // Create from hole cards and board
//! let hole_cards = HoleCards::from_notation("AKs").unwrap();
//! let mut board = Board::new();
//! // ... add board cards
//! let hand = Hand::from_hole_cards_and_board(&hole_cards, &board).unwrap();
//!
//! // Create from cards directly
//! let cards = vec![
//!     Card::from_str("As").unwrap(),
//!     Card::from_str("Kd").unwrap(),
//!     Card::from_str("Qh").unwrap(),
//!     Card::from_str("Js").unwrap(),
//!     Card::from_str("Tc").unwrap(),
//! ];
//! let hand = Hand::new(cards).unwrap();
//! ```
//!
//! ### Hand Evaluation
//!
//! ```rust
//! use holdem_core::{Hand, Card};
//! use std::str::FromStr;
//!
//! let cards = vec![
//!     Card::from_str("As").unwrap(),
//!     Card::from_str("Kd").unwrap(),
//!     Card::from_str("Qh").unwrap(),
//!     Card::from_str("Js").unwrap(),
//!     Card::from_str("Tc").unwrap(),
//! ];
//! let hand = Hand::new(cards).unwrap();
//! let strength = hand.strength(); // Placeholder for future evaluator
//! let best_five = hand.best_five_cards(); // Get best 5-card hand
//! ```

use crate::card::Card;
use crate::errors::PokerError;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// Represents a complete poker hand consisting of 5-7 cards for evaluation
/// This includes hole cards combined with board cards for hand strength calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hand {
    /// All cards in the hand (5-7 cards), sorted by rank descending
    pub cards: [Card; 7],
    /// Number of valid cards (5-7)
    pub len: usize,
}

impl Hand {
    /// Creates a new hand from a vector of 0-7 distinct cards
    ///
    /// Cards are sorted in rank-descending order for consistent evaluation.
    /// Validates that all cards are distinct and the total count doesn't exceed 7.
    ///
    /// # Arguments
    /// * `cards` - A vector of distinct cards (0-7 cards)
    ///
    /// # Returns
    /// * `Ok(Hand)` - A valid hand with sorted cards
    /// * `Err(PokerError)` - Error if validation fails
    ///
    /// # Panics
    ///
    /// This method does not panic, but returns an error if cards are invalid or duplicated.
    ///
    /// # Examples
    /// ```
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qh").unwrap(),
    /// ];
    /// let hand = Hand::new(cards).unwrap();
    /// assert_eq!(hand.len, 3);
    /// ```
    pub fn new(cards: Vec<Card>) -> Result<Self, PokerError> {
        let len = cards.len();
        if len > 7 {
            return Err(PokerError::InvalidHandSize { size: len });
        }

        // Check for duplicates
        let mut unique_cards = std::collections::HashSet::new();
        for &card in &cards {
            if !unique_cards.insert(card) {
                return Err(PokerError::DuplicateCard(card));
            }
        }

        let mut sorted_cards = cards;
        // Sort cards with higher rank first, then higher suit
        sorted_cards.sort_by(|a, b| b.cmp(a));

        let mut hand_cards = [Card::new(0, 0).unwrap(); 7]; // dummy initialization
        for (i, card) in sorted_cards.into_iter().enumerate() {
            hand_cards[i] = card;
        }

        Ok(Self {
            cards: hand_cards,
            len,
        })
    }

    /// Creates a hand from hole cards and board cards
    ///
    /// Combines a player's private hole cards with the public board cards to form
    /// a complete hand for evaluation. The resulting hand will have 2-7 cards total.
    ///
    /// # Arguments
    /// * `hole_cards` - The player's two private cards
    /// * `board` - The current board state with community cards
    ///
    /// # Returns
    /// * `Ok(Hand)` - A valid hand combining hole cards and board cards
    /// * `Err(PokerError)` - Error if the combination would exceed 7 cards
    ///
    /// # Panics
    ///
    /// This method does not panic, but returns an error if the combination would exceed 7 cards.
    ///
    /// # Examples
    /// ```
    /// use holdem_core::{HoleCards, Board, Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let hole_cards = HoleCards::from_notation("AKs").unwrap();
    /// let mut board = Board::new();
    /// board.deal_flop(vec![
    ///     Card::from_str("Qh").unwrap(),
    ///     Card::from_str("Js").unwrap(),
    ///     Card::from_str("Tc").unwrap(),
    /// ]).unwrap();
    ///
    /// let hand = Hand::from_hole_cards_and_board(&hole_cards, &board).unwrap();
    /// assert_eq!(hand.len, 5);
    /// ```
    pub fn from_hole_cards_and_board(
        hole_cards: &crate::hole_cards::HoleCards,
        board: &crate::board::Board,
    ) -> Result<Self, PokerError> {
        let mut all_cards = Vec::new();

        // Add hole cards
        all_cards.extend_from_slice(&hole_cards.cards);

        // Add board cards
        all_cards.extend_from_slice(board.visible_cards());

        if all_cards.len() > 7 {
            return Err(PokerError::CombinedCardsExceedLimit {
                total: all_cards.len(),
            });
        }

        Self::new(all_cards)
    }

    /// Convenience constructor to create a hand from a slice of cards
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qh").unwrap(),
    ///     Card::from_str("Js").unwrap(),
    ///     Card::from_str("Ts").unwrap(),
    /// ];
    /// let hand = Hand::from_cards(&cards).unwrap();
    /// assert_eq!(hand.len, 5);
    /// ```
    pub fn from_cards(cards: &[Card]) -> Result<Self, PokerError> {
        Self::new(cards.to_vec())
    }

    /// Convenience constructor to create a hand from poker notation
    ///
    /// Supports formats like "As Ks Qs Js Ts" for a 5-card hand.
    /// Cards are separated by whitespace.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Hand;
    ///
    /// let hand = Hand::from_notation("As Ks Qs Js Ts").unwrap();
    /// assert_eq!(hand.len, 5);
    ///
    /// let hand = Hand::from_notation("Ah Kh Qh Jh").unwrap();
    /// assert_eq!(hand.len, 4);
    /// ```
    pub fn from_notation(notation: &str) -> Result<Self, PokerError> {
        let card_strings: Vec<&str> = notation.split_whitespace().collect();
        if card_strings.is_empty() {
            return Self::new(Vec::new());
        }

        let mut cards = Vec::new();
        for card_str in card_strings {
            if card_str.is_empty() {
                continue;
            }
            let card = Card::from_str(card_str)?;
            cards.push(card);
        }

        Self::new(cards)
    }

    /// Returns the cards as a slice (only valid cards)
    ///
    /// Returns only the valid cards in the hand, excluding any unused elements
    /// in the internal array. Cards are sorted in rank-descending order.
    ///
    /// # Returns
    /// A slice containing the valid cards in the hand
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    /// ];
    /// let hand = Hand::new(cards).unwrap();
    /// let card_slice = hand.cards();
    /// assert_eq!(card_slice.len(), 2);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Examples
    /// ```
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    /// ];
    /// let hand = Hand::new(cards).unwrap();
    /// let card_slice = hand.cards();
    /// assert_eq!(card_slice.len(), 2);
    /// ```
    pub fn cards(&self) -> &[Card] {
        &self.cards[0..self.len]
    }

    /// Returns the cards as a slice (alias for cards())
    ///
    /// Convenience method that provides the same functionality as `cards()`.
    ///
    /// # Returns
    /// A slice containing the valid cards in the hand
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    /// ];
    /// let hand = Hand::new(cards).unwrap();
    /// let card_slice = hand.as_slice();
    /// assert_eq!(card_slice.len(), 2);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn as_slice(&self) -> &[Card] {
        self.cards()
    }

    /// Returns an iterator over the valid cards
    ///
    /// Provides an iterator that yields references to each valid card in the hand,
    /// in rank-descending order.
    ///
    /// # Returns
    /// An iterator over the valid cards in the hand
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    /// ];
    /// let hand = Hand::new(cards).unwrap();
    ///
    /// for card in hand.iter() {
    ///     println!("Card: {}", card);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Examples
    /// ```
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    /// ];
    /// let hand = Hand::new(cards).unwrap();
    ///
    /// for card in hand.iter() {
    ///     println!("Card: {}", card);
    /// }
    /// ```
    pub fn iter(&self) -> std::slice::Iter<Card> {
        self.cards[0..self.len].iter()
    }

    /// Placeholder for hand strength evaluation (to be implemented with fast evaluator)
    ///
    /// Returns a placeholder strength value. In the future, this will integrate with
    /// a high-performance hand evaluation system using lookup tables for instant
    /// poker hand ranking.
    ///
    /// # Returns
    /// Currently returns 0 (placeholder). Future implementation will return
    /// a strength value where higher values represent stronger hands.
    ///
    /// # Note
    /// This is a temporary implementation. The actual hand evaluation will be
    /// implemented in Phase 1 with optimized lookup tables.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qh").unwrap(),
    ///     Card::from_str("Js").unwrap(),
    ///     Card::from_str("Tc").unwrap(),
    /// ];
    /// let hand = Hand::new(cards).unwrap();
    /// let strength = hand.strength(); // Placeholder for future evaluator
    /// assert_eq!(strength, 0);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn strength(&self) -> u32 {
        // TODO: Integrate with high-performance hand evaluation system in Phase 1
        // For now, return a placeholder value
        0
    }

    /// Returns the best 5-card hand from the available cards
    ///
    /// For hands with 5 or fewer cards, returns all cards.
    /// For hands with 6-7 cards, returns the first 5 cards (sorted).
    /// In the future, this will implement proper poker hand selection algorithms.
    ///
    /// # Returns
    /// An array of 5 cards representing the best poker hand
    ///
    /// # Note
    /// This is a placeholder implementation. The actual algorithm will select
    /// the optimal 5-card combination from 6-7 cards for maximum poker strength.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qh").unwrap(),
    ///     Card::from_str("Js").unwrap(),
    ///     Card::from_str("Tc").unwrap(),
    /// ];
    /// let hand = Hand::new(cards).unwrap();
    /// let best_five = hand.best_five_cards();
    /// assert_eq!(best_five.len(), 5);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Examples
    /// ```
    /// use holdem_core::{Hand, Card};
    /// use std::str::FromStr;
    ///
    /// let cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qh").unwrap(),
    ///     Card::from_str("Js").unwrap(),
    ///     Card::from_str("Tc").unwrap(),
    /// ];
    /// let hand = Hand::new(cards).unwrap();
    /// let best_five = hand.best_five_cards();
    /// assert_eq!(best_five.len(), 5);
    /// ```
    pub fn best_five_cards(&self) -> [Card; 5] {
        // TODO: Implement proper 5-card selection algorithm
        // For now, return first 5 cards (sorted)
        let mut best = [Card::new(0, 0).unwrap(); 5];
        for i in 0..5 {
            best[i] = self.cards[i];
        }
        best
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hand({} cards: ", self.len)?;
        for (i, card) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", card)?;
        }
        write!(f, ")")
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by hand strength (higher strength is better)
        // TODO: Replace with proper poker hand ranking when evaluator is integrated
        self.strength().cmp(&other.strength()).reverse() // Reverse because higher strength should be greater
    }
}

impl IntoIterator for Hand {
    type Item = Card;
    type IntoIter = std::iter::Take<std::array::IntoIter<Card, 7>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter().take(self.len)
    }
}

impl<'a> IntoIterator for &'a Hand {
    type Item = &'a Card;
    type IntoIter = std::iter::Take<std::slice::Iter<'a, Card>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter().take(self.len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_hand_creation_valid() {
        let cards = vec![
            Card::new(12, 2).unwrap(), // Ace of Spades
            Card::new(11, 0).unwrap(), // King of Hearts
            Card::new(10, 1).unwrap(), // Queen of Diamonds
            Card::new(9, 3).unwrap(),  // Jack of Clubs
            Card::new(8, 2).unwrap(),  // Ten of Spades
        ];
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.len, 5);
        assert_eq!(hand.cards()[0], Card::new(12, 2).unwrap());
        assert_eq!(hand.cards()[4], Card::new(8, 2).unwrap());
    }

    #[test]
    fn test_hand_creation_invalid_length() {
        let cards = vec![
            Card::new(12, 2).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
            Card::new(7, 0).unwrap(),
            Card::new(6, 1).unwrap(),
            Card::new(5, 3).unwrap(), // 8 cards
        ];
        assert!(Hand::new(cards).is_err());
    }

    #[test]
    fn test_hand_creation_empty() {
        let cards = vec![];
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.len, 0);
    }

    #[test]
    fn test_hand_creation_duplicate_cards() {
        let cards = vec![
            Card::new(12, 2).unwrap(),
            Card::new(12, 2).unwrap(), // Duplicate
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
        ];
        assert!(Hand::new(cards).is_err());
    }

    #[test]
    fn test_hand_from_hole_cards_and_board() {
        use crate::board::Board;
        use crate::hole_cards::HoleCards;

        let hole_cards =
            HoleCards::new(Card::new(12, 2).unwrap(), Card::new(11, 0).unwrap()).unwrap();
        let mut board = Board::new();

        // Add flop
        board
            .deal_flop(vec![
                Card::new(10, 1).unwrap(),
                Card::new(9, 3).unwrap(),
                Card::new(8, 2).unwrap(),
            ])
            .unwrap();

        let hand = Hand::from_hole_cards_and_board(&hole_cards, &board).unwrap();
        assert_eq!(hand.len, 5);
        assert_eq!(hand.cards()[0], Card::new(12, 2).unwrap()); // Ace first (sorted)
        assert_eq!(hand.cards()[1], Card::new(11, 0).unwrap()); // King
        assert_eq!(hand.cards()[2], Card::new(10, 1).unwrap()); // Queen
        assert_eq!(hand.cards()[3], Card::new(9, 3).unwrap()); // Jack
        assert_eq!(hand.cards()[4], Card::new(8, 2).unwrap()); // Ten
    }

    #[test]
    fn test_hand_strength_placeholder() {
        let cards = vec![
            Card::new(12, 2).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
        ];
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.strength(), 0); // Placeholder value
    }

    #[test]
    fn test_hand_best_five_cards() {
        let cards = vec![
            Card::new(12, 2).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
            Card::new(7, 0).unwrap(),
            Card::new(6, 1).unwrap(),
        ];
        let hand = Hand::new(cards).unwrap();
        let best_five = hand.best_five_cards();
        assert_eq!(best_five.len(), 5);
        // Should be sorted highest first
        assert_eq!(best_five[0], Card::new(12, 2).unwrap());
        assert_eq!(best_five[4], Card::new(8, 2).unwrap());
    }

    #[test]
    fn test_hand_comparison() {
        let cards1 = vec![
            Card::new(12, 2).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
        ];
        let hand1 = Hand::new(cards1).unwrap();

        let cards2 = vec![
            Card::new(11, 2).unwrap(),
            Card::new(10, 0).unwrap(),
            Card::new(9, 1).unwrap(),
            Card::new(8, 3).unwrap(),
            Card::new(7, 2).unwrap(),
        ];
        let hand2 = Hand::new(cards2).unwrap();

        // For now, all hands have same strength (0), so they're equal
        assert_eq!(hand1.cmp(&hand2), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_hand_serialization() {
        let cards = vec![
            Card::new(12, 2).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
        ];
        let hand = Hand::new(cards).unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&hand).unwrap();
        let deserialized: Hand = serde_json::from_str(&json).unwrap();
        assert_eq!(hand, deserialized);

        // Test TOML serialization
        let toml = toml::to_string(&hand).unwrap();
        let deserialized: Hand = toml::from_str(&toml).unwrap();
        assert_eq!(hand, deserialized);
    }

    #[test]
    fn test_hand_iteration() {
        let cards = vec![
            Card::new(12, 2).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
        ];
        let hand = Hand::new(cards).unwrap();

        // Test owned iteration
        let collected: Vec<Card> = hand.into_iter().collect();
        assert_eq!(collected.len(), 5);

        // Test borrowed iteration
        let hand = Hand::new(vec![
            Card::new(12, 2).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
        ])
        .unwrap();
        let mut iter = hand.iter();
        for _ in 0..5 {
            assert!(iter.next().is_some());
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_hand_display() {
        let cards = vec![
            Card::new(12, 2).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
        ];
        let hand = Hand::new(cards).unwrap();
        let display = format!("{}", hand);
        println!("Display: {}", display);
        assert!(display.starts_with("Hand(5 cards: "));
        assert!(display.contains("Ac"));
        assert!(display.contains("Kh"));
        assert!(display.contains("Qd"));
        assert!(display.contains("Js"));
        assert!(display.contains("Tc"));
    }

    #[test]
    fn test_hand_hashing() {
        let mut set = HashSet::new();

        let cards1 = vec![
            Card::new(12, 2).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 2).unwrap(),
        ];
        let hand1 = Hand::new(cards1).unwrap();

        let cards2 = vec![
            Card::new(11, 2).unwrap(),
            Card::new(10, 0).unwrap(),
            Card::new(9, 1).unwrap(),
            Card::new(8, 3).unwrap(),
            Card::new(7, 2).unwrap(),
        ];
        let hand2 = Hand::new(cards2).unwrap();

        assert!(set.insert(hand1));
        assert!(set.insert(hand2));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_performance_hand_creation() {
        use std::time::Instant;

        let start = Instant::now();
        let mut hands = Vec::new();

        // Create 10,000 hands
        for i in 0..10_000 {
            let cards = vec![
                Card::new(12, 2).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
                Card::new(9, 3).unwrap(),
                Card::new((7 - (i % 5)) as u8, 2).unwrap(), // Decreasing rank to avoid duplicates
            ];
            hands.push(Hand::new(cards).unwrap());
        }

        let duration = start.elapsed();
        assert_eq!(hands.len(), 10_000);

        // Should be very fast (< 100ms typically)
        assert!(
            duration.as_millis() < 500,
            "Hand creation took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_hand_construction_various_lengths() {
        // Test 5 cards
        let cards5 = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
        ];
        let hand5 = Hand::new(cards5).unwrap();
        assert_eq!(hand5.len, 5);

        // Test 6 cards
        let cards6 = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
            Card::new(7, 1).unwrap(),
        ];
        let hand6 = Hand::new(cards6).unwrap();
        assert_eq!(hand6.len, 6);

        // Test 7 cards
        let cards7 = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
            Card::new(7, 1).unwrap(),
            Card::new(6, 2).unwrap(),
        ];
        let hand7 = Hand::new(cards7).unwrap();
        assert_eq!(hand7.len, 7);
    }

    #[test]
    fn test_hand_construction_edge_cases() {
        // Test that invalid cards cannot be created
        assert!(Card::new(13, 0).is_err()); // Invalid rank
        assert!(Card::new(12, 4).is_err()); // Invalid suit

        // Test with all same suit
        let cards = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 0).unwrap(),
            Card::new(9, 0).unwrap(),
            Card::new(8, 0).unwrap(),
        ];
        let hand = Hand::new(cards).unwrap();
        assert_eq!(hand.len, 5);

        // Test with all same rank (impossible in poker, but test duplicate detection)
        let cards = vec![
            Card::new(12, 0).unwrap(),
            Card::new(12, 1).unwrap(),
            Card::new(12, 2).unwrap(),
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
        ];
        let hand = Hand::new(cards).unwrap(); // These are distinct cards
        assert_eq!(hand.len, 5);
    }

    #[test]
    fn test_hand_construction_duplicate_detection() {
        // Test exact duplicate cards
        let cards = vec![
            Card::new(12, 0).unwrap(),
            Card::new(12, 0).unwrap(), // Exact duplicate
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
        ];
        assert!(Hand::new(cards).is_err());

        // Test multiple duplicates
        let cards = vec![
            Card::new(12, 0).unwrap(),
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
        ];
        assert!(Hand::new(cards).is_err());
    }

    #[test]
    fn test_hand_strength_placeholder_comprehensive() {
        // Test various hand sizes
        for len in 5..=7 {
            let mut cards = Vec::new();
            for i in 0..len {
                cards.push(Card::new((12 - i) as u8, (i % 4) as u8).unwrap());
            }
            let hand = Hand::new(cards).unwrap();
            assert_eq!(
                hand.strength(),
                0,
                "Strength should be placeholder 0 for {} cards",
                len
            );
        }
    }

    #[test]
    fn test_hand_best_five_cards_comprehensive() {
        // Test with 5 cards (should return all)
        let cards5 = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
        ];
        let hand5 = Hand::new(cards5).unwrap();
        let best5 = hand5.best_five_cards();
        assert_eq!(
            best5,
            [
                Card::new(12, 0).unwrap(),
                Card::new(11, 1).unwrap(),
                Card::new(10, 2).unwrap(),
                Card::new(9, 3).unwrap(),
                Card::new(8, 0).unwrap()
            ]
        );

        // Test with 6 cards (should return first 5 sorted)
        let cards6 = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
            Card::new(7, 1).unwrap(),
        ];
        let hand6 = Hand::new(cards6).unwrap();
        let best6 = hand6.best_five_cards();
        assert_eq!(
            best6,
            [
                Card::new(12, 0).unwrap(),
                Card::new(11, 1).unwrap(),
                Card::new(10, 2).unwrap(),
                Card::new(9, 3).unwrap(),
                Card::new(8, 0).unwrap()
            ]
        );

        // Test with 7 cards
        let cards7 = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
            Card::new(7, 1).unwrap(),
            Card::new(6, 2).unwrap(),
        ];
        let hand7 = Hand::new(cards7).unwrap();
        let best7 = hand7.best_five_cards();
        assert_eq!(
            best7,
            [
                Card::new(12, 0).unwrap(),
                Card::new(11, 1).unwrap(),
                Card::new(10, 2).unwrap(),
                Card::new(9, 3).unwrap(),
                Card::new(8, 0).unwrap()
            ]
        );
    }

    #[test]
    fn test_hand_comparison_placeholder() {
        // Since strength is placeholder (0), hands with same strength should compare as equal
        let cards = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
        ];
        let hand1 = Hand::new(cards.clone()).unwrap();
        let hand2 = Hand::new(cards).unwrap();

        assert_eq!(hand1.cmp(&hand2), Ordering::Equal);
        assert_eq!(hand1, hand2); // Same cards
    }

    #[test]
    fn test_hand_serialization_edge_cases() {
        // Test empty hand
        let empty_hand = Hand::new(vec![]).unwrap();
        let json = serde_json::to_string(&empty_hand).unwrap();
        let deserialized: Hand = serde_json::from_str(&json).unwrap();
        assert_eq!(empty_hand, deserialized);

        // Test hand with 7 cards
        let cards7 = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
            Card::new(7, 1).unwrap(),
            Card::new(6, 2).unwrap(),
        ];
        let hand7 = Hand::new(cards7).unwrap();
        let json = serde_json::to_string(&hand7).unwrap();
        let deserialized: Hand = serde_json::from_str(&json).unwrap();
        assert_eq!(hand7, deserialized);

        // Test TOML with various cards
        let toml = toml::to_string(&hand7).unwrap();
        let deserialized: Hand = toml::from_str(&toml).unwrap();
        assert_eq!(hand7, deserialized);
    }

    #[test]
    fn test_hand_iteration_comprehensive() {
        // Test iteration with different lengths
        for len in 0..=7 {
            let mut cards = Vec::new();
            for i in 0..len {
                cards.push(Card::new((12 - i) as u8, (i % 4) as u8).unwrap());
            }
            let hand = Hand::new(cards).unwrap();

            // Test owned iteration
            let collected: Vec<Card> = hand.into_iter().collect();
            assert_eq!(collected.len(), len);

            // Test borrowed iteration
            let hand = Hand::new(
                (0..len)
                    .map(|i| Card::new((12 - i) as u8, (i % 4) as u8).unwrap())
                    .collect(),
            )
            .unwrap();
            let mut count = 0;
            for _ in &hand {
                count += 1;
            }
            assert_eq!(count, len);
        }
    }

    #[test]
    fn test_hand_display_comprehensive() {
        // Test empty hand
        let empty = Hand::new(vec![]).unwrap();
        assert_eq!(format!("{}", empty), "Hand(0 cards: )");

        // Test single card
        let single = Hand::new(vec![Card::new(12, 0).unwrap()]).unwrap();
        assert_eq!(format!("{}", single), "Hand(1 cards: Ah)");

        // Test multiple cards
        let cards = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
        ];
        let hand = Hand::new(cards).unwrap();
        let display = format!("{}", hand);
        assert!(display.starts_with("Hand(3 cards: "));
        assert!(display.contains("Ah"));
        assert!(display.contains("Kd"));
        assert!(display.contains("Qc"));
        assert!(display.ends_with(")"));
    }

    #[test]
    fn test_hand_hashing_consistency() {
        let mut set = HashSet::new();

        // Create same hand multiple times
        let cards = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(8, 0).unwrap(),
        ];

        let hand1 = Hand::new(cards.clone()).unwrap();
        let hand2 = Hand::new(cards).unwrap();

        // Same hands should hash the same
        assert_eq!(hand1, hand2);
        // Note: In practice, we'd use a proper hash function, but for test we check equality

        assert!(set.insert(hand1));
        assert!(!set.insert(hand2)); // Should not insert duplicate
        assert_eq!(set.len(), 1);

        // Different hands
        let diff_cards = vec![
            Card::new(7, 0).unwrap(),
            Card::new(6, 1).unwrap(),
            Card::new(5, 2).unwrap(),
            Card::new(4, 3).unwrap(),
            Card::new(3, 0).unwrap(),
        ];
        let hand3 = Hand::new(diff_cards).unwrap();
        assert!(set.insert(hand3));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_hand_boundary_conditions() {
        // Test maximum valid length
        let cards7 = (0..7)
            .map(|i| Card::new((12 - i) as u8, (i % 4) as u8).unwrap())
            .collect();
        let hand = Hand::new(cards7).unwrap();
        assert_eq!(hand.len, 7);

        // Test minimum valid length (0)
        let hand_empty = Hand::new(vec![]).unwrap();
        assert_eq!(hand_empty.len, 0);

        // Test that cards beyond len are not accessed
        let hand = Hand::new(vec![Card::new(12, 0).unwrap()]).unwrap();
        assert_eq!(hand.len, 1);
        assert_eq!(hand.cards()[0], Card::new(12, 0).unwrap());
        // The remaining cards in the array are uninitialized but shouldn't be accessed
    }

    #[test]
    fn test_hand_invalid_inputs() {
        // Test more than 7 cards
        let too_many = (0..8)
            .map(|i| Card::new((12 - i) as u8, (i % 4) as u8).unwrap())
            .collect();
        assert!(Hand::new(too_many).is_err());

        // Test duplicate detection with complex duplicates
        let cards = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
            Card::new(9, 3).unwrap(),
            Card::new(12, 0).unwrap(), // Duplicate
        ];
        assert!(Hand::new(cards).is_err());
    }
}
