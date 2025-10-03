//! # Card Module
//!
//! This module provides the core `Card` struct and associated functionality for representing
//! playing cards in the poker library. The Card struct uses a compact 8-bit representation
//! that efficiently stores both rank and suit information.
//!
//! ## Card Representation
//!
//! Cards are stored as a single `u8` byte with the following bit layout:
//! ```text
//! Bit:  76543210
//! Use:  rrrr-sss
//!       ++++----- Rank (1-14, where 1=Deuce, 2=Trey, ..., 13=King, 14=Ace)
//!           +++--- Suit (1=Clubs, 2=Diamonds, 3=Hearts, 4=Spades)
//! ```
//!
//! ## Examples
//!
//! ### Creating Cards
//!
//! ```rust
//! use poker_api::api::card::Card;
//!
//! // Create from rank and suit
//! let ace_spades = Card::from_rank_suit(Card::ACE, Card::SPADES).unwrap();
//!
//! // Create from string notation
//! let king_hearts = Card::from_string("Kh").unwrap();
//!
//! // Create from deck index (0-51)
//! let two_clubs = Card::from_index(39).unwrap();
//! ```
//!
//! ### Card Information
//!
//! ```rust
//! use poker_api::api::card::Card;
//!
//! let card = Card::from_string("As").unwrap();
//! println!("Rank: {}", card.rank());      // 14
//! println!("Suit: {}", card.suit());      // 0 (Spades)
//! println!("Index: {}", card.index());    // 12
//! println!("Display: {}", card);          // "As"
//! ```
//!
//! ## Design Decisions
//!
//! - **Compact Storage**: Single byte per card for memory efficiency
//! - **Fast Operations**: Bit operations for rank/suit extraction
//! - **Standard Ordering**: Cards ordered by suit (Spades, Hearts, Diamonds, Clubs)
//! - **Error Handling**: Comprehensive validation with descriptive error messages
//!
//! ## Performance Characteristics
//!
//! - **Memory**: 1 byte per card
//! - **Rank/Suit Access**: O(1) bit operations
//! - **String Conversion**: O(1) table lookups
//! - **Validation**: O(1) bounds checking

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card(u8);

impl Card {
    // Constants for suits (Java-style encoding: 1-4)
    pub const CLUBS: u8 = 1;
    pub const DIAMONDS: u8 = 2;
    pub const HEARTS: u8 = 3;
    pub const SPADES: u8 = 4;

    // Constants for ranks (Java-style encoding: 1-14)
    pub const DEUCE: u8 = 1;
    pub const TREY: u8 = 2;
    pub const FOUR: u8 = 3;
    pub const FIVE: u8 = 4;
    pub const SIX: u8 = 5;
    pub const SEVEN: u8 = 6;
    pub const EIGHT: u8 = 7;
    pub const NINE: u8 = 8;
    pub const TEN: u8 = 9;
    pub const JACK: u8 = 10;
    pub const QUEEN: u8 = 11;
    pub const KING: u8 = 12;
    pub const ACE: u8 = 13;

    // Special constants
    pub const BAD_CARD: u8 = 255;
    pub const NUM_SUITS: u8 = 4;
    pub const NUM_RANKS: u8 = 13;
    pub const NUM_CARDS: u8 = 52;

    // Constructor for an empty/invalid card
    pub fn new() -> Self {
        Self(Self::BAD_CARD)
    }

    // Constructor from rank and suit
    pub fn from_rank_suit(rank: u8, suit: u8) -> Result<Self, &'static str> {
        if rank < Self::DEUCE || rank > Self::ACE {
            return Err("Invalid rank");
        }
        if suit < Self::CLUBS || suit > Self::SPADES {
            return Err("Invalid suit");
        }
        Ok(Self((rank << 3) | (suit & 0x7)))
    }

    // Constructor from index (0..51)
    pub fn from_index(index: u8) -> Result<Self, &'static str> {
        if index >= Self::NUM_CARDS {
            return Err("Invalid index");
        }
        let rank = (index % Self::NUM_RANKS) + Self::DEUCE;
        let suit = index / Self::NUM_RANKS + Self::CLUBS;
        Self::from_rank_suit(rank, suit)
    }

    // Constructor from string (e.g., "As" for Ace of Spades)
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        if s.len() != 2 {
            return Err("Invalid card string");
        }
        let rank_char = s.chars().nth(0).unwrap();
        let suit_char = s.chars().nth(1).unwrap();
        let rank = Self::rank_from_char(rank_char)?;
        let suit = Self::suit_from_char(suit_char)?;
        Self::from_rank_suit(rank, suit)
    }

    // Get the rank of the card
    pub fn rank(&self) -> u8 {
        (self.0 >> 3) & 0xF
    }

    // Get the suit of the card
    pub fn suit(&self) -> u8 {
        self.0 & 0x7
    }

    // Get the index of the card (0..51)
    pub fn index(&self) -> u8 {
        ((self.suit() - Self::CLUBS) * Self::NUM_RANKS) + (self.rank() - Self::DEUCE)
    }

    // Check if the card is valid
    pub fn is_valid(&self) -> bool {
        self.0 != Self::BAD_CARD
    }

    // Convert rank to character (e.g., 13 -> 'A')
    pub fn rank_to_char(rank: u8) -> Result<char, &'static str> {
        match rank {
            Self::DEUCE => Ok('2'),
            Self::TREY => Ok('3'),
            Self::FOUR => Ok('4'),
            Self::FIVE => Ok('5'),
            Self::SIX => Ok('6'),
            Self::SEVEN => Ok('7'),
            Self::EIGHT => Ok('8'),
            Self::NINE => Ok('9'),
            Self::TEN => Ok('T'),
            Self::JACK => Ok('J'),
            Self::QUEEN => Ok('Q'),
            Self::KING => Ok('K'),
            Self::ACE => Ok('A'),
            _ => Err("Invalid rank"),
        }
    }

    // Convert suit to character (e.g., 4 -> 's')
    pub fn suit_to_char(suit: u8) -> Result<char, &'static str> {
        match suit {
            Self::SPADES => Ok('s'),
            Self::HEARTS => Ok('h'),
            Self::DIAMONDS => Ok('d'),
            Self::CLUBS => Ok('c'),
            _ => Err("Invalid suit"),
        }
    }

    // Convert rank character to rank (e.g., 'A' -> 13)
    pub fn rank_from_char(rank_char: char) -> Result<u8, &'static str> {
        match rank_char {
            '2' => Ok(Self::DEUCE),
            '3' => Ok(Self::TREY),
            '4' => Ok(Self::FOUR),
            '5' => Ok(Self::FIVE),
            '6' => Ok(Self::SIX),
            '7' => Ok(Self::SEVEN),
            '8' => Ok(Self::EIGHT),
            '9' => Ok(Self::NINE),
            'T' => Ok(Self::TEN),
            'J' => Ok(Self::JACK),
            'Q' => Ok(Self::QUEEN),
            'K' => Ok(Self::KING),
            'A' => Ok(Self::ACE),
            _ => Err("Invalid rank character"),
        }
    }

    // Convert suit character to suit (e.g., 's' -> 0)
    pub fn suit_from_char(suit_char: char) -> Result<u8, &'static str> {
        match suit_char {
            's' => Ok(Self::SPADES),
            'h' => Ok(Self::HEARTS),
            'd' => Ok(Self::DIAMONDS),
            'c' => Ok(Self::CLUBS),
            _ => Err("Invalid suit character"),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.is_valid() {
            write!(f, "Invalid Card")
        } else {
            let rank_char = Self::rank_to_char(self.rank()).unwrap();
            let suit_char = Self::suit_to_char(self.suit()).unwrap();
            write!(f, "{}{}", rank_char, suit_char)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::from_rank_suit(Card::ACE, Card::SPADES).unwrap();
        assert_eq!(card.rank(), Card::ACE);
        assert_eq!(card.suit(), Card::SPADES);
        assert_eq!(card.index(), 51); // Ace of Spades is index 51 (last card)
        assert!(card.is_valid());
    }

    #[test]
    fn test_card_from_string() {
        let card = Card::from_string("As").unwrap();
        assert_eq!(card.rank(), Card::ACE);
        assert_eq!(card.suit(), Card::SPADES);
        assert_eq!(card.to_string(), "As");
    }

    #[test]
    fn test_card_from_index() {
        let card = Card::from_index(0).unwrap();
        assert_eq!(card.rank(), Card::DEUCE);
        assert_eq!(card.suit(), Card::CLUBS);
        assert_eq!(card.to_string(), "2c");
    }

    #[test]
    fn test_invalid_card() {
        let card = Card::new();
        assert!(!card.is_valid());
        assert_eq!(card.to_string(), "Invalid Card");
    }
}
