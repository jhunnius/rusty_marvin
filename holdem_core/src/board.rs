//! # Board Module
//!
//! This module provides the `Board` struct for representing community cards
//! in Texas Hold'em poker. The Board manages the progression through betting
//! streets (flop, turn, river) and ensures valid card sequencing.
//!
//! ## Board Representation
//!
//! A Board consists of up to 5 community cards revealed progressively:
//! - **Flop**: 3 cards revealed simultaneously
//! - **Turn**: 1 additional card (4th total)
//! - **River**: 1 additional card (5th total)
//!
//! ## Examples
//!
//! ### Creating and Progressing a Board
//!
//! ```rust
//! use holdem_core::{Board, Card};
//! use std::str::FromStr;
//!
//! let mut board = Board::new();
//!
//! // Deal flop
//! let flop_cards = vec![
//!     Card::from_str("As").unwrap(),
//!     Card::from_str("Kd").unwrap(),
//!     Card::from_str("Qh").unwrap(),
//! ];
//! board.deal_flop(flop_cards).unwrap();
//!
//! // Deal turn
//! let turn_card = Card::from_str("Jc").unwrap();
//! board.deal_turn(turn_card).unwrap();
//!
//! // Deal river
//! let river_card = Card::from_str("Ts").unwrap();
//! board.deal_river(river_card).unwrap();
//! ```
//!
//! ### Board State and Cards
//!
//! ```rust
//! use holdem_core::{Board, Street};
//!
//! let board = Board::new();
//! assert_eq!(board.street(), Street::Preflop);
//! assert_eq!(board.visible_cards().len(), 0);
//! ```

use crate::card::Card;
use crate::errors::PokerError;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the current betting street in Texas Hold'em
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Street {
    /// No community cards dealt yet
    Preflop,
    /// Flop cards dealt (3 cards)
    Flop,
    /// Turn card dealt (4 cards total)
    Turn,
    /// River card dealt (5 cards total)
    River,
}

impl fmt::Display for Street {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Street::Preflop => write!(f, "Preflop"),
            Street::Flop => write!(f, "Flop"),
            Street::Turn => write!(f, "Turn"),
            Street::River => write!(f, "River"),
        }
    }
}

/// Represents the community cards (board) in Texas Hold'em poker
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board {
    /// The community cards, in deal order
    cards: Vec<Card>,
    /// Current betting street
    street: Street,
}

impl Board {
    /// Creates a new empty board at preflop
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::board::Board;
    ///
    /// let board = Board::new();
    /// assert!(board.is_empty());
    /// assert_eq!(board.street().to_string(), "Preflop");
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            street: Street::Preflop,
        }
    }

    /// Returns the current betting street
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::board::Board;
    ///
    /// let board = Board::new();
    /// assert_eq!(board.street().to_string(), "Preflop");
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn street(&self) -> Street {
        self.street
    }

    /// Returns all cards currently visible on the board
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::board::Board;
    ///
    /// let board = Board::new();
    /// let cards = board.visible_cards();
    /// assert_eq!(cards.len(), 0);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn visible_cards(&self) -> &[Card] {
        &self.cards
    }

    /// Returns the number of visible cards
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::board::Board;
    ///
    /// let board = Board::new();
    /// assert_eq!(board.len(), 0);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns true if no cards have been dealt
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::board::Board;
    ///
    /// let board = Board::new();
    /// assert!(board.is_empty());
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Deals the flop (3 cards), advancing from preflop to flop
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{board::Board, Card};
    /// use std::str::FromStr;
    ///
    /// let mut board = Board::new();
    /// let flop_cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qh").unwrap(),
    /// ];
    /// board.deal_flop(flop_cards).unwrap();
    /// assert_eq!(board.len(), 3);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic, but returns an error if called at the wrong street or with invalid cards.
    pub fn deal_flop(&mut self, cards: Vec<Card>) -> Result<(), PokerError> {
        if self.street != Street::Preflop {
            return Err(PokerError::CannotDealFromStreet {
                current_street: self.street.to_string(),
            });
        }
        if cards.len() != 3 {
            return Err(PokerError::FlopMustBeThreeCards {
                actual: cards.len(),
            });
        }

        // Check for duplicates within flop and existing cards
        self.check_duplicates(&cards)?;

        self.cards.extend(cards);
        self.street = Street::Flop;
        Ok(())
    }

    /// Deals the turn card, advancing from flop to turn
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{board::Board, Card};
    /// use std::str::FromStr;
    ///
    /// let mut board = Board::new();
    /// // Deal flop first
    /// let flop_cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qh").unwrap(),
    /// ];
    /// board.deal_flop(flop_cards).unwrap();
    /// let turn_card = Card::from_str("Js").unwrap();
    /// board.deal_turn(turn_card).unwrap();
    /// assert_eq!(board.len(), 4);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic, but returns an error if called at the wrong street.
    pub fn deal_turn(&mut self, card: Card) -> Result<(), PokerError> {
        if self.street != Street::Flop {
            return Err(PokerError::CannotDealFromStreet {
                current_street: self.street.to_string(),
            });
        }
        if self.cards.len() != 3 {
            return Err(PokerError::MustHaveThreeCardsForTurn {
                actual: self.cards.len(),
            });
        }

        // Check for duplicates
        if self.cards.contains(&card) {
            return Err(PokerError::DuplicateWithExistingBoardCard(card));
        }

        self.cards.push(card);
        self.street = Street::Turn;
        Ok(())
    }

    /// Deals the river card, advancing from turn to river
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{board::Board, Card};
    /// use std::str::FromStr;
    ///
    /// let mut board = Board::new();
    /// // Deal flop and turn first
    /// let flop_cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qh").unwrap(),
    /// ];
    /// board.deal_flop(flop_cards).unwrap();
    /// let turn_card = Card::from_str("Js").unwrap();
    /// board.deal_turn(turn_card).unwrap();
    /// let river_card = Card::from_str("Tc").unwrap();
    /// board.deal_river(river_card).unwrap();
    /// assert_eq!(board.len(), 5);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic, but returns an error if called at the wrong street.
    pub fn deal_river(&mut self, card: Card) -> Result<(), PokerError> {
        if self.street != Street::Turn {
            return Err(PokerError::CannotDealFromStreet {
                current_street: self.street.to_string(),
            });
        }
        if self.cards.len() != 4 {
            return Err(PokerError::MustHaveFourCardsForRiver {
                actual: self.cards.len(),
            });
        }

        // Check for duplicates
        if self.cards.contains(&card) {
            return Err(PokerError::DuplicateWithExistingBoardCard(card));
        }

        self.cards.push(card);
        self.street = Street::River;
        Ok(())
    }

    /// Builder pattern method to deal the flop (3 cards) using method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{Board, Card, PokerError};
    /// use std::str::FromStr;
    ///
    /// let board = Board::new()
    ///     .with_flop([
    ///         Card::from_str("As").unwrap(),
    ///         Card::from_str("Kd").unwrap(),
    ///         Card::from_str("Qh").unwrap(),
    ///     ])?
    ///     .with_turn(Card::from_str("Js").unwrap())?
    ///     .with_river(Card::from_str("Ts").unwrap())?;
    /// # Ok::<(), PokerError>(())
    /// ```
    pub fn with_flop(mut self, cards: [Card; 3]) -> Result<Self, PokerError> {
        self.deal_flop(Vec::from(cards))?;
        Ok(self)
    }

    /// Builder pattern method to deal the turn card using method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{Board, Card, PokerError};
    /// use std::str::FromStr;
    ///
    /// let board = Board::new()
    ///     .with_flop([
    ///         Card::from_str("As").unwrap(),
    ///         Card::from_str("Kd").unwrap(),
    ///         Card::from_str("Qh").unwrap(),
    ///     ])?
    ///     .with_turn(Card::from_str("Js").unwrap())?;
    /// # Ok::<(), PokerError>(())
    /// ```
    pub fn with_turn(mut self, card: Card) -> Result<Self, PokerError> {
        self.deal_turn(card)?;
        Ok(self)
    }

    /// Builder pattern method to deal the river card using method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{Board, Card, PokerError};
    /// use std::str::FromStr;
    ///
    /// let board = Board::new()
    ///     .with_flop([
    ///         Card::from_str("As").unwrap(),
    ///         Card::from_str("Kd").unwrap(),
    ///         Card::from_str("Qh").unwrap(),
    ///     ])?
    ///     .with_turn(Card::from_str("Js").unwrap())?
    ///     .with_river(Card::from_str("Ts").unwrap())?;
    /// # Ok::<(), PokerError>(())
    /// ```
    pub fn with_river(mut self, card: Card) -> Result<Self, PokerError> {
        self.deal_river(card)?;
        Ok(self)
    }

    /// Returns cards visible at the specified street
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{board::{Board, Street}, Card};
    /// use std::str::FromStr;
    ///
    /// let mut board = Board::new();
    /// let flop_cards = vec![
    ///     Card::from_str("As").unwrap(),
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qh").unwrap(),
    /// ];
    /// board.deal_flop(flop_cards).unwrap();
    ///
    /// let flop_cards = board.cards_at_street(Street::Flop);
    /// assert_eq!(flop_cards.len(), 3);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn cards_at_street(&self, street: Street) -> &[Card] {
        match street {
            Street::Preflop => &[],
            Street::Flop => &self.cards[..3.min(self.cards.len())],
            Street::Turn => &self.cards[..4.min(self.cards.len())],
            Street::River => &self.cards,
        }
    }

    /// Checks for duplicate cards within the provided cards and existing board cards
    ///
    /// This method ensures that no card is dealt twice on the board, which would be
    /// invalid in poker. It checks both duplicates within the new cards being dealt
    /// and duplicates between new cards and existing board cards.
    ///
    /// # Arguments
    /// * `new_cards` - The cards being dealt
    ///
    /// # Returns
    /// * `Ok(())` if no duplicates found
    /// * `Err(PokerError)` with error if duplicates detected
    fn check_duplicates(&self, new_cards: &[Card]) -> Result<(), PokerError> {
        // Check duplicates within new cards
        for (i, &card1) in new_cards.iter().enumerate() {
            for &card2 in &new_cards[i + 1..] {
                if card1 == card2 {
                    return Err(PokerError::DuplicateCardsInDeal);
                }
            }
        }

        // Check duplicates with existing board cards
        for &new_card in new_cards {
            if self.cards.contains(&new_card) {
                return Err(PokerError::DuplicateWithExistingBoardCard(new_card));
            }
        }

        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cards.is_empty() {
            return write!(f, "Board: [empty] ({})", self.street);
        }

        write!(f, "Board: [")?;
        for (i, card) in self.cards.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", card)?;
        }
        write!(f, "] ({})", self.street)
    }
}

impl IntoIterator for Board {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;
    use crate::hand::Hand;

    #[test]
    fn test_board_creation() {
        let board = Board::new();
        assert_eq!(board.street(), Street::Preflop);
        assert_eq!(board.len(), 0);
        assert!(board.is_empty());
        assert_eq!(board.visible_cards().len(), 0);
    }

    #[test]
    fn test_board_default() {
        let board = Board::default();
        assert_eq!(board.street(), Street::Preflop);
        assert_eq!(board.len(), 0);
        assert!(board.is_empty());
    }

    #[test]
    fn test_deal_flop_valid() {
        let mut board = Board::new();

        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];

        board.deal_flop(flop_cards.clone()).unwrap();

        assert_eq!(board.street(), Street::Flop);
        assert_eq!(board.len(), 3);
        assert!(!board.is_empty());
        assert_eq!(board.visible_cards(), flop_cards.as_slice());
    }

    #[test]
    fn test_deal_flop_invalid_street() {
        let mut board = Board::new();
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];

        // Deal flop
        board.deal_flop(flop_cards.clone()).unwrap();

        // Try to deal flop again
        assert!(board.deal_flop(flop_cards).is_err());
    }

    #[test]
    fn test_deal_flop_wrong_card_count() {
        let mut board = Board::new();

        // Too few cards
        let too_few = vec![Card::new(12, 3).unwrap()];
        assert!(board.deal_flop(too_few).is_err());

        // Too many cards
        let too_many = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
            Card::new(9, 2).unwrap(),
        ];
        assert!(board.deal_flop(too_many).is_err());
    }

    #[test]
    fn test_deal_flop_duplicates() {
        let mut board = Board::new();

        let duplicate_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(12, 3).unwrap(), // Duplicate
            Card::new(10, 1).unwrap(),
        ];

        assert!(board.deal_flop(duplicate_cards).is_err());
    }

    #[test]
    fn test_deal_turn_valid() {
        let mut board = Board::new();

        // First deal flop
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop_cards).unwrap();

        // Then deal turn
        let turn_card = Card::new(9, 2).unwrap();
        board.deal_turn(turn_card).unwrap();

        assert_eq!(board.street(), Street::Turn);
        assert_eq!(board.len(), 4);
        assert_eq!(board.visible_cards().len(), 4);
        assert_eq!(board.visible_cards()[3], turn_card);
    }

    #[test]
    fn test_deal_turn_invalid_street() {
        let mut board = Board::new();
        let turn_card = Card::new(9, 2).unwrap();

        // Try to deal turn before flop
        assert!(board.deal_turn(turn_card).is_err());
    }

    #[test]
    fn test_deal_turn_duplicate() {
        let mut board = Board::new();

        // Deal flop
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop_cards).unwrap();

        // Try to deal duplicate turn card
        let duplicate_turn = Card::new(12, 3).unwrap();
        assert!(board.deal_turn(duplicate_turn).is_err());
    }

    #[test]
    fn test_deal_river_valid() {
        let mut board = Board::new();

        // Deal flop
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop_cards).unwrap();

        // Deal turn
        let turn_card = Card::new(9, 2).unwrap();
        board.deal_turn(turn_card).unwrap();

        // Deal river
        let river_card = Card::new(8, 0).unwrap();
        board.deal_river(river_card).unwrap();

        assert_eq!(board.street(), Street::River);
        assert_eq!(board.len(), 5);
        assert_eq!(board.visible_cards().len(), 5);
        assert_eq!(board.visible_cards()[4], river_card);
    }

    #[test]
    fn test_deal_river_invalid_street() {
        let mut board = Board::new();
        let river_card = Card::new(8, 0).unwrap();

        // Try to deal river before turn
        assert!(board.deal_river(river_card).is_err());
    }

    #[test]
    fn test_deal_river_duplicate() {
        let mut board = Board::new();

        // Deal flop and turn
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop_cards).unwrap();
        board.deal_turn(Card::new(9, 2).unwrap()).unwrap();

        // Try to deal duplicate river card
        let duplicate_river = Card::new(12, 3).unwrap();
        assert!(board.deal_river(duplicate_river).is_err());
    }

    #[test]
    fn test_cards_at_street() {
        let mut board = Board::new();

        // Preflop
        assert_eq!(board.cards_at_street(Street::Preflop).len(), 0);
        assert_eq!(board.cards_at_street(Street::Flop).len(), 0);
        assert_eq!(board.cards_at_street(Street::Turn).len(), 0);
        assert_eq!(board.cards_at_street(Street::River).len(), 0);

        // Deal flop
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop_cards.clone()).unwrap();

        assert_eq!(board.cards_at_street(Street::Preflop).len(), 0);
        assert_eq!(board.cards_at_street(Street::Flop), flop_cards.as_slice());
        assert_eq!(board.cards_at_street(Street::Turn), flop_cards.as_slice());
        assert_eq!(board.cards_at_street(Street::River), flop_cards.as_slice());

        // Deal turn
        let turn_card = Card::new(9, 2).unwrap();
        board.deal_turn(turn_card).unwrap();

        let expected_turn_cards = &[flop_cards[0], flop_cards[1], flop_cards[2], turn_card];
        assert_eq!(board.cards_at_street(Street::Preflop).len(), 0);
        assert_eq!(board.cards_at_street(Street::Flop), flop_cards.as_slice());
        assert_eq!(board.cards_at_street(Street::Turn), expected_turn_cards);
        assert_eq!(board.cards_at_street(Street::River), expected_turn_cards);

        // Deal river
        let river_card = Card::new(8, 0).unwrap();
        board.deal_river(river_card).unwrap();

        let expected_river_cards = &[
            flop_cards[0],
            flop_cards[1],
            flop_cards[2],
            turn_card,
            river_card,
        ];
        assert_eq!(board.cards_at_street(Street::Preflop).len(), 0);
        assert_eq!(board.cards_at_street(Street::Flop), flop_cards.as_slice());
        assert_eq!(board.cards_at_street(Street::Turn), expected_turn_cards);
        assert_eq!(board.cards_at_street(Street::River), expected_river_cards);
    }

    #[test]
    fn test_combine_with_hand() {
        let mut board = Board::new();

        // Deal some cards
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop_cards).unwrap();

        // Create hole cards
        let hole_cards = crate::hole_cards::HoleCards::new(
            Card::new(9, 2).unwrap(), // Jack of Clubs
            Card::new(8, 0).unwrap(), // Ten of Hearts
        )
        .unwrap();

        // Create complete hand from hole cards and board
        let hand = Hand::from_hole_cards_and_board(&hole_cards, &board).unwrap();

        // Verify hand has correct number of cards
        assert_eq!(hand.len, 5); // 2 hole cards + 3 board cards

        // Check that hole cards are included
        assert!(hand.cards.iter().any(|&c| c == hole_cards.cards[0]));
        assert!(hand.cards.iter().any(|&c| c == hole_cards.cards[1]));

        // Check that board cards are included
        for card in &board.cards {
            assert!(hand.cards.iter().any(|&c| c == *card));
        }
    }

    #[test]
    fn test_board_display() {
        let mut board = Board::new();

        // Empty board
        assert_eq!(format!("{}", board), "Board: [empty] (Preflop)");

        // With cards
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop_cards).unwrap();

        let display = format!("{}", board);
        assert!(display.contains("Board: ["));
        assert!(display.contains("As"));
        assert!(display.contains("Kh"));
        assert!(display.contains("Qd"));
        assert!(display.contains("] (Flop)"));
    }

    #[test]
    fn test_street_display() {
        let test_cases = vec![
            (Street::Preflop, "Preflop"),
            (Street::Flop, "Flop"),
            (Street::Turn, "Turn"),
            (Street::River, "River"),
        ];

        for (street, expected) in test_cases {
            assert_eq!(format!("{}", street), expected);
        }
    }

    #[test]
    fn test_street_ordering() {
        assert!(Street::Preflop < Street::Flop);
        assert!(Street::Flop < Street::Turn);
        assert!(Street::Turn < Street::River);
        assert_eq!(Street::River, Street::River);
    }

    #[test]
    fn test_board_serialization() {
        let mut board = Board::new();

        // Deal some cards
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop_cards).unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&board).unwrap();
        let deserialized: Board = serde_json::from_str(&json).unwrap();
        assert_eq!(board, deserialized);

        // Test TOML serialization
        let toml = toml::to_string(&board).unwrap();
        let deserialized: Board = toml::from_str(&toml).unwrap();
        assert_eq!(board, deserialized);
    }

    #[test]
    fn test_board_iteration() {
        let mut board = Board::new();

        // Deal some cards
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop_cards.clone()).unwrap();

        // Test owned iteration
        let cards: Vec<Card> = board.into_iter().collect();
        assert_eq!(cards, flop_cards);

        // Test borrowed iteration
        let board = Board {
            cards: flop_cards.clone(),
            street: Street::Flop,
        };
        let mut iter = board.cards.iter();
        assert_eq!(iter.next(), Some(&flop_cards[0]));
        assert_eq!(iter.next(), Some(&flop_cards[1]));
        assert_eq!(iter.next(), Some(&flop_cards[2]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_board_equality() {
        let mut board1 = Board::new();
        let mut board2 = Board::new();

        // Both empty
        assert_eq!(board1, board2);

        // Deal same cards to both
        let flop_cards = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board1.deal_flop(flop_cards.clone()).unwrap();
        board2.deal_flop(flop_cards).unwrap();

        assert_eq!(board1, board2);

        // Different streets
        board1.deal_turn(Card::new(9, 2).unwrap()).unwrap();
        assert_ne!(board1, board2);
    }

    #[test]
    fn test_complete_board_progression() {
        let mut board = Board::new();

        // Start at preflop
        assert_eq!(board.street(), Street::Preflop);
        assert_eq!(board.len(), 0);

        // Deal flop
        let flop = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop).unwrap();
        assert_eq!(board.street(), Street::Flop);
        assert_eq!(board.len(), 3);

        // Deal turn
        let turn = Card::new(9, 2).unwrap();
        board.deal_turn(turn).unwrap();
        assert_eq!(board.street(), Street::Turn);
        assert_eq!(board.len(), 4);

        // Deal river
        let river = Card::new(8, 0).unwrap();
        board.deal_river(river).unwrap();
        assert_eq!(board.street(), Street::River);
        assert_eq!(board.len(), 5);

        // Verify all cards are present
        let visible = board.visible_cards();
        assert_eq!(visible.len(), 5);
        assert_eq!(visible[0], Card::new(12, 3).unwrap());
        assert_eq!(visible[1], Card::new(11, 0).unwrap());
        assert_eq!(visible[2], Card::new(10, 1).unwrap());
        assert_eq!(visible[3], Card::new(9, 2).unwrap());
        assert_eq!(visible[4], Card::new(8, 0).unwrap());
    }

    #[test]
    fn test_performance_board_operations() {
        use std::time::Instant;

        let start = Instant::now();
        let mut boards = Vec::new();

        // Create and progress 1,000 boards
        for _i in 0..1000 {
            let mut board = Board::new();

            let flop = vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
            ];
            board.deal_flop(flop).unwrap();

            let turn = Card::new(9, 2).unwrap();
            board.deal_turn(turn).unwrap();

            let river = Card::new(8, 0).unwrap();
            board.deal_river(river).unwrap();

            boards.push(board);
        }

        let duration = start.elapsed();
        assert_eq!(boards.len(), 1000);

        // Should be fast (< 50ms typically)
        assert!(
            duration.as_millis() < 200,
            "Board operations took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_performance_card_access() {
        use std::time::Instant;

        let mut board = Board::new();
        let flop = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop).unwrap();
        board.deal_turn(Card::new(9, 2).unwrap()).unwrap();
        board.deal_river(Card::new(8, 0).unwrap()).unwrap();

        let start = Instant::now();

        // Perform many card access operations
        for _ in 0..100_000 {
            let _ = board.len();
            let _ = board.is_empty();
            let _ = board.visible_cards();
            let _ = board.cards_at_street(Street::Flop);
            let _ = board.cards_at_street(Street::Turn);
            let _ = board.cards_at_street(Street::River);
        }

        let duration = start.elapsed();

        // Should be very fast (< 50ms typically)
        assert!(
            duration.as_millis() < 50,
            "Card access operations took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_board_street_progression_edge_cases() {
        let mut board = Board::new();

        // Test that we can't skip streets
        assert!(board.deal_turn(Card::new(9, 2).unwrap()).is_err());
        assert!(board.deal_river(Card::new(8, 0).unwrap()).is_err());

        // Deal flop
        let flop = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop).unwrap();

        // Still can't deal river
        assert!(board.deal_river(Card::new(8, 0).unwrap()).is_err());

        // Deal turn
        board.deal_turn(Card::new(9, 2).unwrap()).unwrap();

        // Now can deal river
        board.deal_river(Card::new(8, 0).unwrap()).unwrap();

        // Can't deal more cards
        assert!(board.deal_turn(Card::new(7, 1).unwrap()).is_err());
        assert!(board.deal_river(Card::new(6, 2).unwrap()).is_err());
    }

    #[test]
    fn test_board_card_validation_edge_cases() {
        let mut board = Board::new();

        // Test that invalid cards cannot be created
        assert!(Card::new(13, 0).is_err()); // Invalid rank
        assert!(Card::new(14, 1).is_err()); // Invalid rank
        assert!(Card::new(15, 2).is_err()); // Invalid rank
        assert!(Card::new(9, 4).is_err()); // Invalid suit
        assert!(Card::new(8, 5).is_err()); // Invalid suit

        // Test dealing with valid cards
        let flop = vec![
            Card::new(12, 0).unwrap(),
            Card::new(11, 1).unwrap(),
            Card::new(10, 2).unwrap(),
        ];
        board.deal_flop(flop).unwrap();
        assert_eq!(board.len(), 3);

        board.deal_turn(Card::new(9, 3).unwrap()).unwrap();
        assert_eq!(board.len(), 4);

        board.deal_river(Card::new(8, 0).unwrap()).unwrap();
        assert_eq!(board.len(), 5);
    }

    #[test]
    fn test_board_duplicate_detection_comprehensive() {
        let mut board = Board::new();

        // Deal flop
        let flop = vec![
            Card::new(12, 3).unwrap(),
            Card::new(11, 0).unwrap(),
            Card::new(10, 1).unwrap(),
        ];
        board.deal_flop(flop).unwrap();

        // Try turn card that duplicates flop
        assert!(board.deal_turn(Card::new(12, 3).unwrap()).is_err());
        assert!(board.deal_turn(Card::new(11, 0).unwrap()).is_err());
        assert!(board.deal_turn(Card::new(10, 1).unwrap()).is_err());

        // Valid turn
        board.deal_turn(Card::new(9, 2).unwrap()).unwrap();

        // Try river card that duplicates any previous
        assert!(board.deal_river(Card::new(12, 3).unwrap()).is_err());
        assert!(board.deal_river(Card::new(11, 0).unwrap()).is_err());
        assert!(board.deal_river(Card::new(10, 1).unwrap()).is_err());
        assert!(board.deal_river(Card::new(9, 2).unwrap()).is_err());

        // Valid river
        board.deal_river(Card::new(8, 0).unwrap()).unwrap();
    }

    #[test]
    fn test_board_serialization_edge_cases() {
        // Test empty board
        let empty_board = Board::new();
        let json = serde_json::to_string(&empty_board).unwrap();
        let deserialized: Board = serde_json::from_str(&json).unwrap();
        assert_eq!(empty_board, deserialized);

        // Test board with all streets
        let mut full_board = Board::new();
        full_board
            .deal_flop(vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
            ])
            .unwrap();
        full_board.deal_turn(Card::new(9, 2).unwrap()).unwrap();
        full_board.deal_river(Card::new(8, 0).unwrap()).unwrap();

        let json = serde_json::to_string(&full_board).unwrap();
        let deserialized: Board = serde_json::from_str(&json).unwrap();
        assert_eq!(full_board, deserialized);

        // Test TOML serialization
        let toml = toml::to_string(&full_board).unwrap();
        let deserialized: Board = toml::from_str(&toml).unwrap();
        assert_eq!(full_board, deserialized);
    }

    #[test]
    fn test_board_iteration_comprehensive() {
        // Test iteration with different board states
        let states = vec![
            (Street::Preflop, vec![]),
            (
                Street::Flop,
                vec![
                    Card::new(12, 3).unwrap(),
                    Card::new(11, 0).unwrap(),
                    Card::new(10, 1).unwrap(),
                ],
            ),
            (
                Street::Turn,
                vec![
                    Card::new(12, 3).unwrap(),
                    Card::new(11, 0).unwrap(),
                    Card::new(10, 1).unwrap(),
                    Card::new(9, 2).unwrap(),
                ],
            ),
            (
                Street::River,
                vec![
                    Card::new(12, 3).unwrap(),
                    Card::new(11, 0).unwrap(),
                    Card::new(10, 1).unwrap(),
                    Card::new(9, 2).unwrap(),
                    Card::new(8, 0).unwrap(),
                ],
            ),
        ];

        for (street, expected_cards) in states {
            let board = Board {
                cards: expected_cards.clone(),
                street,
            };

            // Test owned iteration
            let collected: Vec<Card> = board.into_iter().collect();
            assert_eq!(collected, expected_cards);

            // Test borrowed iteration
            let board = Board {
                cards: expected_cards.clone(),
                street,
            };
            let collected_ref: Vec<&Card> = (&board).into_iter().collect();
            assert_eq!(collected_ref.len(), expected_cards.len());
            for (i, &card) in collected_ref.iter().enumerate() {
                assert_eq!(card, &expected_cards[i]);
            }
        }
    }

    #[test]
    fn test_board_display_comprehensive() {
        // Test all streets
        let test_cases = vec![
            (Street::Preflop, "Board: [empty] (Preflop)"),
            (Street::Flop, "Board: [As Kh Qd] (Flop)"),
            (Street::Turn, "Board: [As Kh Qd Jc] (Turn)"),
            (Street::River, "Board: [As Kh Qd Jc Th] (River)"),
        ];

        for (street, expected_display) in test_cases {
            let cards = match street {
                Street::Preflop => vec![],
                Street::Flop => vec![
                    Card::new(12, 3).unwrap(),
                    Card::new(11, 0).unwrap(),
                    Card::new(10, 1).unwrap(),
                ],
                Street::Turn => vec![
                    Card::new(12, 3).unwrap(),
                    Card::new(11, 0).unwrap(),
                    Card::new(10, 1).unwrap(),
                    Card::new(9, 2).unwrap(),
                ],
                Street::River => vec![
                    Card::new(12, 3).unwrap(),
                    Card::new(11, 0).unwrap(),
                    Card::new(10, 1).unwrap(),
                    Card::new(9, 2).unwrap(),
                    Card::new(8, 0).unwrap(),
                ],
            };

            let board = Board { cards, street };
            assert_eq!(format!("{}", board), expected_display);
        }
    }

    #[test]
    fn test_board_hashing() {
        use std::collections::HashSet;

        let mut set = HashSet::new();

        // Create same board multiple times
        let mut board1 = Board::new();
        board1
            .deal_flop(vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
            ])
            .unwrap();

        let mut board2 = Board::new();
        board2
            .deal_flop(vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
            ])
            .unwrap();

        // Same boards should hash the same
        assert_eq!(board1, board2);
        assert!(set.insert(board1.clone()));
        assert!(!set.insert(board2.clone())); // Should not insert duplicate

        // Different boards
        let mut board3 = Board::new();
        board3
            .deal_flop(vec![
                Card::new(7, 0).unwrap(),
                Card::new(6, 1).unwrap(),
                Card::new(5, 2).unwrap(),
            ])
            .unwrap();
        assert!(set.insert(board3));

        // Different streets
        let mut board4 = board2.clone();
        board4.deal_turn(Card::new(9, 2).unwrap()).unwrap();
        assert!(set.insert(board4));

        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_board_boundary_conditions() {
        // Test maximum valid board (5 cards)
        let mut board = Board::new();
        board
            .deal_flop(vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
            ])
            .unwrap();
        board.deal_turn(Card::new(9, 2).unwrap()).unwrap();
        board.deal_river(Card::new(8, 0).unwrap()).unwrap();
        assert_eq!(board.len(), 5);
        assert_eq!(board.street(), Street::River);

        // Test minimum valid board (0 cards)
        let empty = Board::new();
        assert_eq!(empty.len(), 0);
        assert_eq!(empty.street(), Street::Preflop);
        assert!(empty.is_empty());

        // Test that cards_at_street handles partial boards correctly
        let mut partial_board = Board::new();
        partial_board
            .deal_flop(vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
            ])
            .unwrap();

        // Even though we're at flop, asking for turn/river should return flop cards
        assert_eq!(partial_board.cards_at_street(Street::Turn).len(), 3);
        assert_eq!(partial_board.cards_at_street(Street::River).len(), 3);
    }

    #[test]
    fn test_board_invalid_operations() {
        let mut board = Board::new();

        // Try to deal flop with wrong number of cards
        assert!(board.deal_flop(vec![]).is_err());
        assert!(board.deal_flop(vec![Card::new(12, 3).unwrap()]).is_err());
        assert!(board
            .deal_flop(vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
                Card::new(9, 2).unwrap()
            ])
            .is_err());

        // Deal valid flop
        board
            .deal_flop(vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
            ])
            .unwrap();

        // Try to deal turn when not exactly 3 cards (shouldn't happen in practice)
        // But test the validation
        let mut invalid_board = Board {
            cards: vec![Card::new(12, 3).unwrap(), Card::new(11, 0).unwrap()], // Only 2 cards
            street: Street::Flop,                                              // Incorrect state
        };
        assert!(invalid_board.deal_turn(Card::new(9, 2).unwrap()).is_err());

        // Try to deal river when not exactly 4 cards
        let mut invalid_board2 = Board {
            cards: vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
                Card::new(9, 2).unwrap(),
                Card::new(8, 0).unwrap(),
            ], // 5 cards
            street: Street::Turn, // Incorrect state
        };
        assert!(invalid_board2.deal_river(Card::new(7, 1).unwrap()).is_err());
    }

    #[test]
    fn test_street_serialization() {
        // Test that Street can be serialized/deserialized
        for &street in &[Street::Preflop, Street::Flop, Street::Turn, Street::River] {
            let json = serde_json::to_string(&street).unwrap();
            let deserialized: Street = serde_json::from_str(&json).unwrap();
            assert_eq!(street, deserialized);

            // Note: TOML serialization for enums may not be supported in all versions
            // JSON serialization is sufficient for testing serialization functionality
        }
    }

    #[test]
    fn test_board_clone_and_equality() {
        let mut original = Board::new();
        original
            .deal_flop(vec![
                Card::new(12, 3).unwrap(),
                Card::new(11, 0).unwrap(),
                Card::new(10, 1).unwrap(),
            ])
            .unwrap();
        original.deal_turn(Card::new(9, 2).unwrap()).unwrap();

        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.street(), cloned.street());
        assert_eq!(original.visible_cards(), cloned.visible_cards());

        // Modify clone and verify they're different
        let mut modified = cloned.clone();
        modified.deal_river(Card::new(8, 0).unwrap()).unwrap();
        assert_ne!(original, modified);
    }
}
