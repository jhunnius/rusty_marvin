//! # Hand Module
//!
//! This module provides the `Hand` struct for managing collections of playing cards.
//! The Hand struct supports various poker hand sizes from 0 to 7 cards and provides
//! methods for card manipulation, sorting, and conversion to different formats.
//!
//! ## Hand Structure
//!
//! A `Hand` contains:
//! - An array of up to 7 optional `Card` values
//! - A size counter tracking the number of valid cards
//! - Methods for adding, removing, and accessing cards
//!
//! ## Key Features
//!
//! - **Dynamic Sizing**: Supports hands from 0 to 7 cards
//! - **Card Management**: Add, remove, and access cards by position
//! - **String Conversion**: Parse hands from string notation (e.g., "As Ks Qs")
//! - **Sorting**: Sort cards in descending order by rank
//! - **Validation**: Bounds checking and error handling
//! - **Multiple Formats**: Support for different string representations
//!
//! ## Examples
//!
//! ### Creating and Manipulating Hands
//!
//! ```rust
//! use poker_api::api::hand::Hand;
//! use poker_api::api::card::Card;
//!
//! // Create empty hand
//! let mut hand = Hand::new();
//!
//! // Add cards individually
//! let ace_spades = Card::from_string("As").unwrap();
//! hand.add_card(ace_spades).unwrap();
//!
//! // Create from string notation
//! let texas_holdem_hand = Hand::from_string("As Ks Qs Js Ts").unwrap();
//! ```
//!
//! ### Hand Operations
//!
//! ```rust
//! use poker_api::api::hand::Hand;
//!
//! let mut hand = Hand::from_string("2s As Ks").unwrap();
//!
//! // Access cards
//! println!("First card: {:?}", hand.get_card(1));
//! println!("Hand size: {}", hand.size());
//!
//! // Sort in descending order
//! hand.sort();
//!
//! // Convert to string
//! println!("Sorted hand: {}", hand.to_string());
//! ```
//!
//! ### Integration with Evaluators
//!
//! ```rust
//! use poker_api::api::hand::Hand;
//! use poker_api::hand_evaluator::LookupHandEvaluator;
//!
//! let hand = Hand::from_string("As Ks Qs Js Ts").unwrap();
//! let evaluator = LookupHandEvaluator::new().unwrap();
//! let rank = evaluator.rank_hand(&hand);
//! println!("Hand rank: {}", rank);
//! ```
//!
//! ## Design Decisions
//!
//! - **Fixed Array Size**: Uses 7-element array for Texas Hold'em compatibility
//! - **Optional Cards**: Uses `Option<Card>` to handle variable hand sizes
//! - **1-based Indexing**: Public API uses 1-based indexing for user convenience
//! - **Immutable Access**: Most getter methods take `&self` for safety
//! - **Error Handling**: Comprehensive error messages for invalid operations
//!
//! ## Performance Characteristics
//!
//! - **Memory**: Fixed 7-element array (56 bytes) + size field
//! - **Card Access**: O(1) array access for valid indices
//! - **Sorting**: O(n log n) comparison sort
//! - **String Conversion**: O(n) where n is hand size
//! - **Validation**: O(1) bounds checking

use crate::api::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    pub(crate) cards: [Option<Card>; 7], // Up to 7 cards
    size: usize,                         // Number of cards in the hand
}

impl Hand {
    pub const MAX_CARDS: usize = 7;

    // Create a new empty hand
    pub fn new() -> Self {
        Self {
            cards: [None; Self::MAX_CARDS],
            size: 0,
        }
    }

    // Create a hand from a string representation (e.g., "As Ks Qs")
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        let mut hand = Self::new();
        for card_str in s.split_whitespace() {
            let card = Card::from_string(card_str)?;
            hand.add_card(card)?;
        }
        Ok(hand)
    }

    // Add a card to the hand
    pub fn add_card(&mut self, card: Card) -> Result<(), &'static str> {
        if self.size >= Self::MAX_CARDS {
            return Err("Hand is full");
        }
        self.cards[self.size] = Some(card);
        self.size += 1;
        Ok(())
    }

    // Remove the last card in the hand
    pub fn remove_card(&mut self) {
        if self.size > 0 {
            self.size -= 1;
            self.cards[self.size] = None;
        }
    }

    // Get the card at a specific position (1-based indexing)
    pub fn get_card(&self, pos: usize) -> Option<Card> {
        if pos == 0 || pos > self.size {
            return None;
        }
        self.cards[pos - 1]
    }

    // Get the card index at a specific position (1-based indexing)
    pub fn get_card_index(&self, pos: usize) -> Option<u8> {
        self.get_card(pos).map(|card| card.index())
    }

    // Get the last card in the hand
    pub fn get_last_card(&self) -> Option<Card> {
        if self.size == 0 {
            None
        } else {
            self.cards[self.size - 1]
        }
    }

    // Get the last card index in the hand
    pub fn get_last_card_index(&self) -> Option<u8> {
        self.get_last_card().map(|card| card.index())
    }

    // Clear the hand (remove all cards)
    pub fn clear(&mut self) {
        self.cards = [None; Self::MAX_CARDS];
        self.size = 0;
    }

    // Sort the hand in descending order by card index
    pub fn sort(&mut self) {
        self.cards[..self.size].sort_by(|a, b| {
            let a_index = a.map(|card| card.index()).unwrap_or(0);
            let b_index = b.map(|card| card.index()).unwrap_or(0);
            b_index.cmp(&a_index) // Sort in descending order
        });
    }

    // Check if the hand contains a specific card
    pub fn contains(&self, card: Card) -> bool {
        self.cards[..self.size].contains(&Some(card))
    }

    // Get a string representation of the hand (e.g., "As Ks Qs")
    pub fn to_string(&self) -> String {
        self.cards[..self.size]
            .iter()
            .filter_map(|&card| card.map(|c| c.to_string()))
            .collect::<Vec<String>>()
            .join(" ")
    }

    // Get a string representation for flashing purposes
    pub fn flashing_string(&self) -> String {
        self.to_string()
    }

    // Get the size of the hand
    pub fn size(&self) -> usize {
        self.size
    }

    // Get the array of card indexes (for LUT-based evaluators)
    pub fn get_card_array(&self) -> Vec<u8> {
        self.cards[..self.size]
            .iter()
            .filter_map(|&card| card.map(|c| c.index()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::card::Card;

    #[test]
    fn test_hand_creation() {
        let hand = Hand::new();
        assert_eq!(hand.size(), 0);
    }

    #[test]
    fn test_add_card() {
        let mut hand = Hand::new();
        let card = Card::from_string("As").unwrap();
        hand.add_card(card).unwrap();
        assert_eq!(hand.size(), 1);
        assert_eq!(hand.get_card(1), Some(card));
    }

    #[test]
    fn test_remove_card() {
        let mut hand = Hand::new();
        let card = Card::from_string("As").unwrap();
        hand.add_card(card).unwrap();
        hand.remove_card();
        assert_eq!(hand.size(), 0);
    }

    #[test]
    fn test_hand_from_string() {
        let hand = Hand::from_string("As Ks Qs").unwrap();
        assert_eq!(hand.size(), 3);
        assert_eq!(hand.get_card(1), Some(Card::from_string("As").unwrap()));
        assert_eq!(hand.get_card(2), Some(Card::from_string("Ks").unwrap()));
        assert_eq!(hand.get_card(3), Some(Card::from_string("Qs").unwrap()));
    }

    #[test]
    fn test_hand_sort() {
        let mut hand = Hand::from_string("2s As Ks").unwrap();
        hand.sort();
        assert_eq!(hand.get_card(1), Some(Card::from_string("As").unwrap()));
        assert_eq!(hand.get_card(2), Some(Card::from_string("Ks").unwrap()));
        assert_eq!(hand.get_card(3), Some(Card::from_string("2s").unwrap()));
    }

    #[test]
    fn test_hand_sort_with_empty_slots() {
        let mut hand = Hand::new();
        hand.add_card(Card::from_string("2s").unwrap()).unwrap();
        hand.add_card(Card::from_string("As").unwrap()).unwrap();
        hand.add_card(Card::from_string("Ks").unwrap()).unwrap();
        hand.sort();
        assert_eq!(hand.get_card(1), Some(Card::from_string("As").unwrap()));
        assert_eq!(hand.get_card(2), Some(Card::from_string("Ks").unwrap()));
        assert_eq!(hand.get_card(3), Some(Card::from_string("2s").unwrap()));
    }
}
