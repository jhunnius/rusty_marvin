//! # Deck Module
//!
//! This module provides the `Deck` struct for representing a deck of playing cards
//! in the poker AI system. Supports shuffling, dealing, and serialization.
//!
//! ## Deck Representation
//!
//! A Deck contains a vector of cards not yet dealt, starting with all 52 cards.
//!
//! ## Examples
//!
//! ### Creating and Using a Deck
//!
//! ```rust
//! use holdem_core::{Deck, Card};
//!
//! let mut deck = Deck::new();
//! assert_eq!(deck.remaining(), 52);
//!
//! // Shuffle the deck
//! use rand::SeedableRng;
//! let mut rng = rand::rngs::StdRng::from_seed([1; 32]);
//! deck.shuffle(&mut rng);
//!
//! // Deal some cards
//! let hole_cards = deck.deal(2);
//! let flop = deck.deal(3);
//! let turn = deck.deal(1);
//! let river = deck.deal(1);
//!
//! assert_eq!(hole_cards.len(), 2);
//! assert_eq!(flop.len(), 3);
//! assert_eq!(turn.len(), 1);
//! assert_eq!(river.len(), 1);
//! assert_eq!(deck.remaining(), 52 - 7);
//! ```
//!
//! ## Design Decisions
//!
//! - **Full Deck**: Always starts with 52 cards in standard poker order
//! - **Efficient Dealing**: Pop from end for O(1) deal operations
//! - **Serialization Ready**: Full serde support for persistence
//! - **Flexible Shuffling**: Uses rand crate for high-quality randomization

use crate::card::Card;
use serde::{Deserialize, Serialize};

/// Represents a deck of cards not yet dealt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a new full deck of 52 cards
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Deck;
    ///
    /// let deck = Deck::new();
    /// assert_eq!(deck.remaining(), 52);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in 0..4 {
            for rank in 0..13 {
                cards.push(Card::new(rank, suit).unwrap());
            }
        }
        Self { cards }
    }

    /// Shuffles the deck using the provided random number generator
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Deck;
    /// use rand::SeedableRng;
    ///
    /// let mut deck = Deck::new();
    /// let mut rng = rand::rngs::StdRng::from_seed([1; 32]);
    /// deck.shuffle(&mut rng);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn shuffle<R: rand::Rng>(&mut self, rng: &mut R) {
        use rand::seq::SliceRandom;
        self.cards.shuffle(rng);
    }

    /// Deals a single card from the top of the deck
    ///
    /// Returns `None` if the deck is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Deck;
    ///
    /// let mut deck = Deck::new();
    /// let card = deck.deal_one().unwrap();
    /// assert_eq!(deck.remaining(), 51);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic, but returns `None` if the deck is empty.
    pub fn deal_one(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Deals multiple cards from the top of the deck
    ///
    /// Returns a vector containing up to `count` cards. If fewer than `count` cards
    /// remain in the deck, returns all remaining cards.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Deck;
    ///
    /// let mut deck = Deck::new();
    /// let cards = deck.deal(5);
    /// assert_eq!(cards.len(), 5);
    /// assert_eq!(deck.remaining(), 47);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn deal(&mut self, count: usize) -> Vec<Card> {
        let mut dealt = Vec::new();
        for _ in 0..count {
            if let Some(card) = self.deal_one() {
                dealt.push(card);
            } else {
                break;
            }
        }
        dealt
    }

    /// Returns the number of cards remaining in the deck
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Deck;
    ///
    /// let mut deck = Deck::new();
    /// assert_eq!(deck.remaining(), 52);
    ///
    /// deck.deal(5);
    /// assert_eq!(deck.remaining(), 47);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    /// Returns true if the deck is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Deck;
    ///
    /// let mut deck = Deck::new();
    /// assert!(!deck.is_empty());
    ///
    /// // Deal all cards
    /// deck.deal(52);
    /// assert!(deck.is_empty());
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns a reference to the remaining cards
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Deck;
    ///
    /// let deck = Deck::new();
    /// let cards = deck.cards();
    /// assert_eq!(cards.len(), 52);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_deck_new() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
        assert!(!deck.is_empty());

        // Check all cards are present
        let cards = deck.cards();
        let mut card_set = HashSet::new();
        for &card in cards {
            assert!(card_set.insert(card), "Duplicate card in deck: {}", card);
        }
        assert_eq!(card_set.len(), 52);
    }

    #[test]
    fn test_deck_shuffle() {
        let mut deck1 = Deck::new();
        let deck2 = Deck::new();

        // Decks should be identical initially
        assert_eq!(deck1.cards(), deck2.cards());

        // Shuffle one deck
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::from_seed([1; 32]);
        deck1.shuffle(&mut rng);

        // After shuffle, order should be different (with very high probability)
        assert_ne!(deck1.cards(), deck2.cards());
        assert_eq!(deck1.remaining(), 52);
        assert_eq!(deck2.remaining(), 52);
    }

    #[test]
    fn test_deck_deal_one() {
        let mut deck = Deck::new();

        // Deal one card
        let card = deck.deal_one().unwrap();
        assert_eq!(deck.remaining(), 51);

        // Deal another
        let card2 = deck.deal_one().unwrap();
        assert_eq!(deck.remaining(), 50);

        // Cards should be different
        assert_ne!(card, card2);
    }

    #[test]
    fn test_deck_deal_multiple() {
        let mut deck = Deck::new();

        // Deal 5 cards
        let cards = deck.deal(5);
        assert_eq!(cards.len(), 5);
        assert_eq!(deck.remaining(), 47);

        // All cards should be unique
        let mut card_set = HashSet::new();
        for &card in &cards {
            assert!(card_set.insert(card), "Duplicate card dealt: {}", card);
        }
    }

    #[test]
    fn test_deck_deal_empty() {
        let mut deck = Deck::new();

        // Deal all cards
        let all_cards = deck.deal(52);
        assert_eq!(all_cards.len(), 52);
        assert_eq!(deck.remaining(), 0);
        assert!(deck.is_empty());

        // Try to deal one more
        assert!(deck.deal_one().is_none());

        // Try to deal multiple more
        let empty_deal = deck.deal(5);
        assert!(empty_deal.is_empty());
    }

    #[test]
    fn test_deck_serialization() {
        let deck = Deck::new();

        // Test JSON serialization
        let json = serde_json::to_string(&deck).unwrap();
        let deserialized: Deck = serde_json::from_str(&json).unwrap();
        assert_eq!(deck.cards(), deserialized.cards());

        // Test TOML serialization
        let toml = toml::to_string(&deck).unwrap();
        let deserialized: Deck = toml::from_str(&toml).unwrap();
        assert_eq!(deck.cards(), deserialized.cards());
    }

    #[test]
    fn test_deck_shuffle_randomness() {
        use rand::SeedableRng;

        let mut deck = Deck::new();
        let original_order: Vec<Card> = deck.cards().to_vec();

        // Shuffle with different seeds and verify randomness
        let mut rng1 = rand::rngs::StdRng::from_seed([1; 32]);
        deck.shuffle(&mut rng1);
        let shuffled1 = deck.cards().to_vec();

        // Reset deck
        deck = Deck::new();
        let mut rng2 = rand::rngs::StdRng::from_seed([2; 32]);
        deck.shuffle(&mut rng2);
        let shuffled2 = deck.cards().to_vec();

        // Different seeds should produce different shuffles
        assert_ne!(shuffled1, shuffled2);
        assert_ne!(shuffled1, original_order);
        assert_ne!(shuffled2, original_order);

        // All cards should still be present
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        for &card in &shuffled1 {
            set1.insert(card);
        }
        for &card in &shuffled2 {
            set2.insert(card);
        }
        assert_eq!(set1.len(), 52);
        assert_eq!(set2.len(), 52);
    }

    #[test]
    fn test_deck_deal_edge_cases() {
        let mut deck = Deck::new();

        // Deal more cards than available
        let cards = deck.deal(60);
        assert_eq!(cards.len(), 52);
        assert_eq!(deck.remaining(), 0);
        assert!(deck.is_empty());

        // Try dealing from empty deck
        let empty_deal = deck.deal(5);
        assert!(empty_deal.is_empty());

        // Deal one from empty deck
        assert!(deck.deal_one().is_none());
    }

    #[test]
    fn test_deck_remaining_cards() {
        let mut deck = Deck::new();
        assert_eq!(deck.remaining(), 52);

        // Deal various amounts and check remaining
        deck.deal(5);
        assert_eq!(deck.remaining(), 47);

        deck.deal_one();
        assert_eq!(deck.remaining(), 46);

        deck.deal(10);
        assert_eq!(deck.remaining(), 36);

        // Deal all remaining
        deck.deal(36);
        assert_eq!(deck.remaining(), 0);
        assert!(deck.is_empty());
    }

    #[test]
    fn test_deck_deal_zero() {
        let mut deck = Deck::new();

        // Deal zero cards
        let empty_deal = deck.deal(0);
        assert!(empty_deal.is_empty());
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_deck_default() {
        let deck = Deck::default();
        assert_eq!(deck.remaining(), 52);
        assert!(!deck.is_empty());
    }

    #[test]
    fn test_deck_clone() {
        let mut deck1 = Deck::new();
        deck1.deal(5);

        let deck2 = deck1.clone();
        assert_eq!(deck1.remaining(), deck2.remaining());
        assert_eq!(deck1.cards(), deck2.cards());
    }

    #[test]
    fn test_deck_serialization_partial() {
        let mut deck = Deck::new();
        deck.deal(10); // Remove 10 cards

        // Serialize partial deck
        let json = serde_json::to_string(&deck).unwrap();
        let deserialized: Deck = serde_json::from_str(&json).unwrap();

        assert_eq!(deck.remaining(), deserialized.remaining());
        assert_eq!(deck.cards(), deserialized.cards());

        // Should be able to continue dealing from deserialized deck
        let mut deserialized = deserialized;
        let more_cards = deserialized.deal(5);
        assert_eq!(more_cards.len(), 5);
    }

    #[test]
    fn test_deck_deal_uniqueness() {
        let mut deck = Deck::new();

        // Deal all cards and ensure uniqueness
        let mut all_cards = Vec::new();
        while let Some(card) = deck.deal_one() {
            all_cards.push(card);
        }

        assert_eq!(all_cards.len(), 52);

        let mut card_set = HashSet::new();
        for card in all_cards {
            assert!(card_set.insert(card), "Duplicate card found: {}", card);
        }
        assert_eq!(card_set.len(), 52);
    }

    #[test]
    fn test_deck_performance() {
        use std::time::Instant;

        // Test creation performance
        let start = Instant::now();
        for _ in 0..1000 {
            let _deck = Deck::new();
        }
        let creation_time = start.elapsed();

        // Test dealing performance
        let mut deck = Deck::new();
        let start = Instant::now();
        for _ in 0..52 {
            deck.deal_one();
        }
        let dealing_time = start.elapsed();

        // Should be very fast
        assert!(
            creation_time.as_millis() < 100,
            "Deck creation too slow: {:?}",
            creation_time
        );
        assert!(
            dealing_time.as_millis() < 10,
            "Dealing too slow: {:?}",
            dealing_time
        );
    }
}
