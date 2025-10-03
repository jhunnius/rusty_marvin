//! # Deck Module
//!
//! This module provides the `Deck` struct for managing a standard 52-card poker deck.
//! The Deck struct supports shuffling, dealing cards, and various card manipulation
//! operations required for poker game simulation and testing.
//!
//! ## Deck Structure
//!
//! A `Deck` contains:
//! - A vector of remaining cards in the deck
//! - A vector of cards that have been dealt
//! - Methods for shuffling, dealing, and card management
//!
//! ## Key Features
//!
//! - **Standard Deck**: Creates a proper 52-card poker deck
//! - **Shuffling**: Multiple shuffle methods including seeded randomization
//! - **Dealing**: Deal cards sequentially or randomly
//! - **Card Management**: Extract specific cards, check card availability
//! - **Reset Functionality**: Return all cards to the deck
//! - **Integration**: Works seamlessly with `Hand` and `Card` structs
//!
//! ## Examples
//!
//! ### Basic Deck Operations
//!
//! ```rust
//! use poker_api::api::deck::Deck;
//! use poker_api::api::card::Card;
//!
//! // Create and shuffle a new deck
//! let mut deck = Deck::new();
//! deck.shuffle();
//!
//! // Deal some cards
//! if let Some(card) = deck.deal() {
//!     println!("Dealt: {}", card);
//! }
//!
//! println!("Cards left: {}", deck.cards_left());
//! ```
//!
//! ### Seeded Shuffling for Testing
//!
//! ```rust
//! use poker_api::api::deck::Deck;
//!
//! // Create deck with reproducible shuffle for testing
//! let mut deck = Deck::with_seed(12345);
//!
//! // Deal cards in predictable order
//! let card1 = deck.deal().unwrap();
//! let card2 = deck.deal().unwrap();
//! ```
//!
//! ### Card Management
//!
//! ```rust
//! use poker_api::api::deck::Deck;
//! use poker_api::api::hand::Hand;
//! use poker_api::api::card::Card;
//!
//! let mut deck = Deck::new();
//!
//! // Extract specific cards
//! let ace_spades = Card::from_string("As").unwrap();
//! deck.extract_card(ace_spades);
//!
//! // Extract entire hand
//! let mut hand = Hand::from_string("Kh Qh Jh").unwrap();
//! deck.extract_hand(&hand);
//!
//! // Reset deck to restore all cards
//! deck.reset();
//! ```
//!
//! ## Design Decisions
//!
//! - **Two-Vector Design**: Separate active and dealt cards for efficient operations
//! - **Immutable Access**: Most getter methods take `&self` for safety
//! - **Standard Ordering**: Cards ordered by suit (Spades, Hearts, Diamonds, Clubs)
//! - **Error Handling**: Graceful handling of empty deck conditions
//! - **Random Access**: Support for both sequential and random dealing
//!
//! ## Performance Characteristics
//!
//! - **Memory**: ~104 bytes overhead + 52 bytes for cards
//! - **Shuffling**: O(n) where n is number of cards
//! - **Dealing**: O(1) for sequential, O(n) for random
//! - **Card Lookup**: O(n) linear search
//! - **Reset**: O(1) vector extension

use crate::api::card::Card;
use crate::api::hand::Hand;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    cards: Vec<Card>,
    dealt: Vec<Card>, // Cards that have been dealt
}

impl Deck {
    pub const NUM_CARDS: usize = 52;

    // Create a new, unshuffled deck
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(Self::NUM_CARDS);
        for suit in 0..4 {
            for rank in Card::TWO..=Card::ACE {
                cards.push(Card::from_rank_suit(rank, suit).unwrap());
            }
        }
        Self {
            cards,
            dealt: Vec::new(),
        }
    }

    // Create a new deck with a specific shuffle seed
    pub fn with_seed(seed: u64) -> Self {
        let mut deck = Self::new();
        deck.shuffle_with_seed(seed);
        deck
    }

    // Shuffle the deck using a random seed
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }

    // Shuffle the deck using a specific seed
    pub fn shuffle_with_seed(&mut self, seed: u64) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        self.cards.shuffle(&mut rng);
    }

    // Deal the next card from the deck
    pub fn deal(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            None
        } else {
            let card = self.cards.pop().unwrap();
            self.dealt.push(card);
            Some(card)
        }
    }

    // Deal a random card from the deck
    pub fn deal_random(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            None
        } else {
            let index = rand::random::<u8>() as usize % self.cards.len();
            let card = self.cards.remove(index);
            self.dealt.push(card);
            Some(card)
        }
    }

    // Reset the deck (put all cards back)
    pub fn reset(&mut self) {
        self.cards.extend(self.dealt.drain(..));
    }

    // Check if a card is still in the deck
    pub fn in_deck(&self, card: Card) -> bool {
        self.cards.contains(&card)
    }

    // Remove a specific card from the deck
    pub fn extract_card(&mut self, card: Card) -> Option<Card> {
        if let Some(index) = self.cards.iter().position(|&c| c == card) {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    // Remove a specific card by index from the deck
    pub fn extract_card_by_index(&mut self, index: u8) -> Option<Card> {
        if let Some(pos) = self.cards.iter().position(|&c| c.index() == index) {
            Some(self.cards.remove(pos))
        } else {
            None
        }
    }

    // Remove all cards in a hand from the deck
    pub fn extract_hand(&mut self, hand: &Hand) {
        for card in hand.get_card_array() {
            self.extract_card_by_index(card);
        }
    }

    // Get the number of cards left in the deck
    pub fn cards_left(&self) -> usize {
        self.cards.len()
    }

    // Get the top card (next card to be dealt)
    pub fn get_top_card(&self) -> Option<Card> {
        self.cards.last().copied()
    }

    // Get the index of the top card
    pub fn get_top_card_index(&self) -> Option<u8> {
        self.get_top_card().map(|card| card.index())
    }

    // Get a card at a specific index in the deck
    pub fn get_card(&self, index: usize) -> Option<Card> {
        self.cards.get(index).copied()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Deck ({} cards left)", self.cards_left())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.cards_left(), Deck::NUM_CARDS);
    }

    #[test]
    fn test_deck_shuffle() {
        let mut deck = Deck::new();
        deck.shuffle();
        assert_eq!(deck.cards_left(), Deck::NUM_CARDS);
    }

    #[test]
    fn test_deck_deal() {
        let mut deck = Deck::new();
        let card = deck.deal().unwrap();
        assert_eq!(deck.cards_left(), Deck::NUM_CARDS - 1);
        assert!(!deck.in_deck(card));
    }

    #[test]
    fn test_deck_reset() {
        let mut deck = Deck::new();
        let card = deck.deal().unwrap();
        deck.reset();
        assert_eq!(deck.cards_left(), Deck::NUM_CARDS);
        assert!(deck.in_deck(card));
    }

    #[test]
    fn test_deck_extract_card() {
        let mut deck = Deck::new();
        let card = Card::from_string("As").unwrap();
        deck.extract_card(card).unwrap();
        assert_eq!(deck.cards_left(), Deck::NUM_CARDS - 1);
        assert!(!deck.in_deck(card));
    }
}
