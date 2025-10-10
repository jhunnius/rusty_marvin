//! # Card Module
//!
//! This module provides the core `Card` and `Deck` structs for representing playing cards
//! in the poker AI system. Uses zero-based u8 enums for optimal performance and memory efficiency.
//!
//! ## Card Representation
//!
//! Cards are represented with zero-based u8 values:
//! - **Rank**: 0=Two, 1=Three, ..., 12=Ace
//! - **Suit**: 0=Hearts, 1=Diamonds, 2=Clubs, 3=Spades
//!
//! ## Examples
//!
//! ### Creating Cards
//!
//! ```rust
//! use holdem_core::Card;
//! use std::str::FromStr;
//!
//! // Create from string notation
//! let ace_hearts = Card::from_str("Ah").unwrap();
//! let king_diamonds = Card::from_str("Kd").unwrap();
//! let seven_clubs = Card::from_str("7c").unwrap();
//!
//! // Create from rank and suit values
//! let card = Card::new(12, 0).unwrap(); // Ace of Hearts
//! ```
//!
//! ### Card Information
//!
//! ```rust
//! use holdem_core::Card;
//! use std::str::FromStr;
//!
//! let card = Card::from_str("As").unwrap();
//! println!("Rank: {}", card.rank());      // 12 (Ace)
//! println!("Suit: {}", card.suit());      // 3 (Spades)
//! println!("Display: {}", card);           // "As"
//! ```
//!
//! ## Design Decisions
//!
//! - **Zero-based u8 enums**: Maximum performance with minimal memory usage
//! - **Memory Efficient**: 2 bytes per card (1 byte rank + 1 byte suit)
//! - **Performance Optimized**: Copy semantics and efficient comparisons
//! - **Serialization Ready**: Full serde support for TOML and network communication
//! - **Standard Ordering**: Cards ordered by rank then suit for efficient sorting
//!
//! ## Performance Characteristics
//!
//! - **Memory**: 2 bytes per card
//! - **Comparison/Sorting**: O(1) u8 comparisons
//! - **Hashing**: O(1) hash operations
//! - **String Conversion**: O(1) table lookups
//! - **Serialization**: Efficient binary/network formats via serde
//!
//! ## Future Optimization: Bit-Packed Cards
//!
//! For high-performance hand evaluation, consider using `PackedCard`:
//! ```rust
//! use holdem_core::card::PackedCard;
//!
//! // 8-bit representation: 6 bits rank + 2 bits suit
//! let packed = PackedCard::new(12, 3).unwrap(); // Ace of Spades
//! assert_eq!(packed.rank(), 12);
//! assert_eq!(packed.suit(), 3);
//! ```
//!
//! **Benefits of PackedCard**:
//! - **50% less memory** (1 byte vs 2 bytes)
//! - **Faster operations** via bit manipulation
//! - **Better cache performance** with smaller memory footprint
//! - **Efficient for hand evaluation algorithms**

use crate::errors::PokerError;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// Represents a playing card with zero-based rank and suit values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Card {
    /// The card's rank (0=Two to 12=Ace)
    pub rank: u8,
    /// The card's suit (0=Hearts, 1=Diamonds, 2=Clubs, 3=Spades)
    pub suit: u8,
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CardData {
            rank: u8,
            suit: u8,
        }

        let data = CardData::deserialize(deserializer)?;
        Card::new(data.rank, data.suit).map_err(D::Error::custom)
    }
}

impl Card {
    /// Creates a new card from rank and suit values
    ///
    /// # Arguments
    ///
    /// * `rank` - The card's rank (0=Two to 12=Ace)
    /// * `suit` - The card's suit (0=Hearts, 1=Diamonds, 2=Clubs, 3=Spades)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Card;
    ///
    /// let ace_spades = Card::new(12, 3).unwrap();
    /// assert_eq!(ace_spades.rank(), 12);
    /// assert_eq!(ace_spades.suit(), 3);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic, but returns an error if rank or suit are invalid.
    pub fn new(rank: u8, suit: u8) -> Result<Self, PokerError> {
        if rank > 12 {
            return Err(PokerError::InvalidCardRank { rank });
        }
        if suit > 3 {
            return Err(PokerError::InvalidCardSuit { suit });
        }
        Ok(Self { rank, suit })
    }

    /// Returns the card's rank value (0=Two to 12=Ace)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Card;
    /// use std::str::FromStr;
    ///
    /// let card = Card::from_str("As").unwrap();
    /// assert_eq!(card.rank(), 12); // Ace
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn rank(&self) -> u8 {
        self.rank
    }

    /// Returns the card's suit value (0=Hearts, 1=Diamonds, 2=Clubs, 3=Spades)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Card;
    /// use std::str::FromStr;
    ///
    /// let card = Card::from_str("As").unwrap();
    /// assert_eq!(card.suit(), 3); // Spades
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn suit(&self) -> u8 {
        self.suit
    }

    /// Returns the rank as a character ('2'-'A')
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Card;
    /// use std::str::FromStr;
    ///
    /// let card = Card::from_str("As").unwrap();
    /// assert_eq!(card.rank_char(), 'A');
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn rank_char(&self) -> char {
        match self.rank {
            0 => '2',
            1 => '3',
            2 => '4',
            3 => '5',
            4 => '6',
            5 => '7',
            6 => '8',
            7 => '9',
            8 => 'T',
            9 => 'J',
            10 => 'Q',
            11 => 'K',
            12 => 'A',
            _ => '?',
        }
    }

    /// Returns the suit as a character ('h', 'd', 'c', 's')
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Card;
    /// use std::str::FromStr;
    ///
    /// let card = Card::from_str("As").unwrap();
    /// assert_eq!(card.suit_char(), 's');
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn suit_char(&self) -> char {
        match self.suit {
            0 => 'h',
            1 => 'd',
            2 => 'c',
            3 => 's',
            _ => '?',
        }
    }

    /// Converts a rank character to its u8 value
    /// Returns None if the character is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Card;
    ///
    /// assert_eq!(Card::rank_from_char('A'), Some(12));
    /// assert_eq!(Card::rank_from_char('2'), Some(0));
    /// assert_eq!(Card::rank_from_char('X'), None);
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn rank_from_char(c: char) -> Option<u8> {
        match c {
            '2' => Some(0),
            '3' => Some(1),
            '4' => Some(2),
            '5' => Some(3),
            '6' => Some(4),
            '7' => Some(5),
            '8' => Some(6),
            '9' => Some(7),
            'T' => Some(8),
            'J' => Some(9),
            'Q' => Some(10),
            'K' => Some(11),
            'A' => Some(12),
            _ => None,
        }
    }

    /// Converts a rank u8 value to its character representation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Card;
    ///
    /// assert_eq!(Card::rank_to_char(12), 'A');
    /// assert_eq!(Card::rank_to_char(0), '2');
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn rank_to_char(rank: u8) -> char {
        match rank {
            0 => '2',
            1 => '3',
            2 => '4',
            3 => '5',
            4 => '6',
            5 => '7',
            6 => '8',
            7 => '9',
            8 => 'T',
            9 => 'J',
            10 => 'Q',
            11 => 'K',
            12 => 'A',
            _ => '?',
        }
    }

    /// Converts a suit u8 value to its character representation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::Card;
    ///
    /// assert_eq!(Card::suit_to_char(3), 's');
    /// assert_eq!(Card::suit_to_char(0), 'h');
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn suit_to_char(suit: u8) -> char {
        match suit {
            0 => 'h',
            1 => 'd',
            2 => 'c',
            3 => 's',
            _ => '?',
        }
    }
}

impl FromStr for Card {
    type Err = PokerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(PokerError::InvalidCardString {
                string: s.to_string(),
            });
        }

        let rank_char = s.chars().nth(0).unwrap();
        let suit_char = s.chars().nth(1).unwrap();

        let rank = match rank_char {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => {
                return Err(PokerError::InvalidRankCharacter {
                    character: rank_char,
                })
            }
        };

        let suit = match suit_char {
            'h' => 0,
            'd' => 1,
            'c' => 2,
            's' => 3,
            _ => {
                return Err(PokerError::InvalidSuitCharacter {
                    character: suit_char,
                })
            }
        };

        Card::new(rank, suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank_char(), self.suit_char())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank
            .cmp(&other.rank)
            .then(Self::suit_order(self.suit).cmp(&Self::suit_order(other.suit)))
    }
}

impl Card {
    /// Returns the ordering value for suits (higher value = higher suit)
    fn suit_order(suit: u8) -> u8 {
        match suit {
            3 => 4, // Spades highest
            0 => 3, // Hearts
            1 => 2, // Diamonds
            2 => 1, // Clubs lowest
            _ => 0,
        }
    }
}

/// Bit-packed card representation for maximum performance in hand evaluation
///
/// Layout: 6 bits rank (0-63) + 2 bits suit (0-3) = 8 bits total
/// - Bits 0-5: rank (0=Two, 1=Three, ..., 12=Ace)
/// - Bits 6-7: suit (0=Hearts, 1=Diamonds, 2=Clubs, 3=Spades)
///
/// **Performance Benefits**:
/// - **50% less memory** (1 byte vs 2 bytes per card)
/// - **Faster comparisons** via direct integer operations
/// - **Better cache performance** with smaller memory footprint
/// - **Efficient bit manipulation** for hand evaluation algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct PackedCard(u8);

impl PackedCard {
    /// Bit mask for extracting rank (lower 6 bits)
    const RANK_MASK: u8 = 0b0011_1111;
    /// Bit mask for extracting suit (upper 2 bits)
    const SUIT_MASK: u8 = 0b1100_0000;
    /// Number of bits to shift suit value
    const SUIT_SHIFT: u8 = 6;

    /// Creates a new packed card from rank and suit values
    ///
    /// # Arguments
    ///
    /// * `rank` - The card's rank (0=Two to 12=Ace)
    /// * `suit` - The card's suit (0=Hearts, 1=Diamonds, 2=Clubs, 3=Spades)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::card::PackedCard;
    ///
    /// let ace_spades = PackedCard::new(12, 3).unwrap();
    /// assert_eq!(ace_spades.rank(), 12);
    /// assert_eq!(ace_spades.suit(), 3);
    /// ```
    pub fn new(rank: u8, suit: u8) -> Result<Self, PokerError> {
        if rank > 12 {
            return Err(PokerError::InvalidCardRank { rank });
        }
        if suit > 3 {
            return Err(PokerError::InvalidCardSuit { suit });
        }

        // Pack: rank in lower 6 bits, suit in upper 2 bits
        let packed = (suit << Self::SUIT_SHIFT) | rank;
        Ok(Self(packed))
    }

    /// Creates a packed card from a regular Card
    pub fn from_card(card: &Card) -> Self {
        Self::new(card.rank(), card.suit()).unwrap()
    }

    /// Converts to a regular Card
    pub fn to_card(self) -> Card {
        Card::new(self.rank(), self.suit()).unwrap()
    }

    /// Returns the card's rank value (0=Two to 12=Ace)
    pub fn rank(self) -> u8 {
        self.0 & Self::RANK_MASK
    }

    /// Returns the card's suit value (0=Hearts, 1=Diamonds, 2=Clubs, 3=Spades)
    pub fn suit(self) -> u8 {
        (self.0 & Self::SUIT_MASK) >> Self::SUIT_SHIFT
    }

    /// Returns the rank as a character ('2'-'A')
    pub fn rank_char(self) -> char {
        Card::rank_to_char(self.rank())
    }

    /// Returns the suit as a character ('h', 'd', 'c', 's')
    pub fn suit_char(self) -> char {
        Card::suit_to_char(self.suit())
    }

    /// Returns the raw packed byte value
    pub fn as_u8(self) -> u8 {
        self.0
    }

    /// Creates a packed card from a raw byte value
    /// Note: Does not validate that the value represents a valid card
    pub fn from_u8(value: u8) -> Self {
        Self(value)
    }
}

impl From<Card> for PackedCard {
    fn from(card: Card) -> Self {
        Self::from_card(&card)
    }
}

impl From<PackedCard> for Card {
    fn from(packed: PackedCard) -> Self {
        packed.to_card()
    }
}

impl fmt::Display for PackedCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank_char(), self.suit_char())
    }
}

impl PartialOrd for PackedCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PackedCard {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by rank first, then suit
        self.rank()
            .cmp(&other.rank())
            .then_with(|| Self::suit_order(self.suit()).cmp(&Self::suit_order(other.suit())))
    }
}

impl PackedCard {
    /// Returns the ordering value for suits (higher value = higher suit)
    fn suit_order(suit: u8) -> u8 {
        match suit {
            3 => 4, // Spades highest
            0 => 3, // Hearts
            1 => 2, // Diamonds
            2 => 1, // Clubs lowest
            _ => 0,
        }
    }
}

impl FromStr for PackedCard {
    type Err = PokerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card: Card = s.parse()?;
        Ok(Self::from_card(&card))
    }
}

impl<'de> Deserialize<'de> for PackedCard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(PackedCard::from_u8(value))
    }
}

#[cfg(test)]
mod packed_card_tests {
    use super::*;

    #[test]
    fn test_packed_card_creation() {
        let packed = PackedCard::new(12, 3).unwrap(); // Ace of Spades
        assert_eq!(packed.rank(), 12);
        assert_eq!(packed.suit(), 3);
        assert_eq!(packed.as_u8(), 0b1100_1100); // suit=3 (1100), rank=12 (1100)
    }

    #[test]
    fn test_packed_card_conversion() {
        let original = Card::new(12, 3).unwrap();
        let packed = PackedCard::from_card(&original);
        let back_to_card = packed.to_card();

        assert_eq!(original, back_to_card);
        assert_eq!(packed.rank(), original.rank());
        assert_eq!(packed.suit(), original.suit());
    }

    #[test]
    fn test_packed_card_display() {
        let packed = PackedCard::new(12, 3).unwrap();
        assert_eq!(format!("{}", packed), "As");

        let packed = PackedCard::new(0, 0).unwrap();
        assert_eq!(format!("{}", packed), "2h");
    }

    #[test]
    fn test_packed_card_ordering() {
        let ace_spades = PackedCard::new(12, 3).unwrap();
        let king_spades = PackedCard::new(11, 3).unwrap();
        let ace_hearts = PackedCard::new(12, 0).unwrap();

        assert!(ace_spades > king_spades); // Higher rank
        assert!(ace_spades > ace_hearts); // Higher suit (Spades > Hearts)
    }

    #[test]
    fn test_packed_card_from_str() {
        let packed: PackedCard = "As".parse().unwrap();
        assert_eq!(packed.rank(), 12);
        assert_eq!(packed.suit(), 3);

        let packed: PackedCard = "2h".parse().unwrap();
        assert_eq!(packed.rank(), 0);
        assert_eq!(packed.suit(), 0);
    }

    #[test]
    fn test_packed_card_memory_usage() {
        use std::mem;

        assert_eq!(mem::size_of::<Card>(), 2);
        assert_eq!(mem::size_of::<PackedCard>(), 1);

        // Test that we can fit more packed cards in the same memory
        let cards: Vec<Card> = (0..52)
            .map(|i| Card::new((i % 13) as u8, (i % 4) as u8).unwrap())
            .collect();
        let packed_cards: Vec<PackedCard> =
            cards.iter().map(|c| PackedCard::from_card(c)).collect();

        // Packed cards should use roughly half the memory
        assert!(mem::size_of_val(&*packed_cards) * 2 >= mem::size_of_val(&*cards));
    }

    #[test]
    fn test_packed_card_all_values() {
        // Test all 52 possible cards
        for rank in 0..13 {
            for suit in 0..4 {
                let packed = PackedCard::new(rank, suit).unwrap();
                assert_eq!(packed.rank(), rank);
                assert_eq!(packed.suit(), suit);

                // Test round-trip conversion
                let card = packed.to_card();
                assert_eq!(card.rank(), rank);
                assert_eq!(card.suit(), suit);
            }
        }
    }

    #[test]
    fn test_packed_card_invalid_creation() {
        assert!(PackedCard::new(13, 0).is_err()); // Invalid rank
        assert!(PackedCard::new(12, 4).is_err()); // Invalid suit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::Deck;
    use std::collections::HashSet;

    #[test]
    fn test_card_creation() {
        // Test creating cards with new()
        let card = Card::new(12, 3).unwrap(); // Ace of Spades
        assert_eq!(card.rank, 12);
        assert_eq!(card.suit, 3);

        // Test creating all possible cards
        for rank in 0..13 {
            for suit in 0..4 {
                let card = Card::new(rank, suit).unwrap();
                assert_eq!(card.rank(), rank);
                assert_eq!(card.suit(), suit);
            }
        }
    }

    #[test]
    fn test_card_from_str_valid() {
        // Test all valid card strings
        let test_cases = vec![
            ("2c", 0, 2),  // Two of Clubs
            ("3d", 1, 1),  // Three of Diamonds
            ("4h", 2, 0),  // Four of Hearts
            ("5s", 3, 3),  // Five of Spades
            ("Ts", 8, 3),  // Ten of Spades
            ("Jc", 9, 2),  // Jack of Clubs
            ("Qd", 10, 1), // Queen of Diamonds
            ("Kh", 11, 0), // King of Hearts
            ("As", 12, 3), // Ace of Spades
        ];

        for (str_repr, expected_rank, expected_suit) in test_cases {
            let card = Card::from_str(str_repr).unwrap();
            assert_eq!(card.rank, expected_rank);
            assert_eq!(card.suit, expected_suit);
        }
    }

    #[test]
    fn test_card_from_str_invalid() {
        // Test invalid string lengths
        assert!(Card::from_str("").is_err());
        assert!(Card::from_str("A").is_err());
        assert!(Card::from_str("Asc").is_err());
        assert!(Card::from_str("Asd").is_err());

        // Test invalid rank characters
        assert!(Card::from_str("1s").is_err());
        assert!(Card::from_str("Zs").is_err());
        assert!(Card::from_str("@s").is_err());

        // Test invalid suit characters
        assert!(Card::from_str("Az").is_err());
        assert!(Card::from_str("A1").is_err());
        assert!(Card::from_str("A ").is_err());
    }

    #[test]
    fn test_card_display() {
        let test_cases = vec![
            (Card::new(12, 3).unwrap(), "As"), // Ace of Spades
            (Card::new(11, 0).unwrap(), "Kh"), // King of Hearts
            (Card::new(10, 1).unwrap(), "Qd"), // Queen of Diamonds
            (Card::new(9, 2).unwrap(), "Jc"),  // Jack of Clubs
            (Card::new(8, 3).unwrap(), "Ts"),  // Ten of Spades
            (Card::new(0, 0).unwrap(), "2h"),  // Two of Hearts
        ];

        for (card, expected_str) in test_cases {
            assert_eq!(format!("{}", card), expected_str);
        }
    }

    #[test]
    fn test_card_equality() {
        let card1 = Card::new(12, 3).unwrap(); // Ace of Spades
        let card2 = Card::new(12, 3).unwrap(); // Ace of Spades
        let card3 = Card::new(11, 3).unwrap(); // King of Spades
        let card4 = Card::new(12, 0).unwrap(); // Ace of Hearts

        // Same cards should be equal
        assert_eq!(card1, card2);

        // Different rank should not be equal
        assert_ne!(card1, card3);

        // Different suit should not be equal
        assert_ne!(card1, card4);
    }

    #[test]
    fn test_card_ordering() {
        // Test rank ordering (higher rank first)
        assert!(Card::new(12, 3).unwrap() > Card::new(11, 3).unwrap()); // Ace > King
        assert!(Card::new(11, 3).unwrap() > Card::new(10, 3).unwrap()); // King > Queen
        assert!(Card::new(0, 3).unwrap() < Card::new(1, 3).unwrap()); // Two < Three

        // Test suit ordering when ranks are equal (higher suit first)
        assert!(Card::new(12, 3).unwrap() > Card::new(12, 0).unwrap()); // Spades > Hearts
        assert!(Card::new(12, 0).unwrap() > Card::new(12, 1).unwrap()); // Hearts > Diamonds
        assert!(Card::new(12, 1).unwrap() > Card::new(12, 2).unwrap()); // Diamonds > Clubs
    }

    #[test]
    fn test_card_hashing() {
        let mut set = HashSet::new();

        // Add all 52 cards
        for rank in 0..13 {
            for suit in 0..4 {
                let card = Card::new(rank, suit).unwrap();
                assert!(set.insert(card), "Duplicate hash for card: {}", card);
            }
        }

        // Should have 52 unique cards
        assert_eq!(set.len(), 52);
    }

    #[test]
    fn test_serialization() {
        let card = Card::new(12, 3).unwrap(); // Ace of Spades

        // Test JSON serialization
        let json = serde_json::to_string(&card).unwrap();
        let deserialized: Card = serde_json::from_str(&json).unwrap();
        assert_eq!(card, deserialized);

        // Test TOML serialization
        let toml = toml::to_string(&card).unwrap();
        let deserialized: Card = toml::from_str(&toml).unwrap();
        assert_eq!(card, deserialized);
    }

    #[test]
    fn test_card_roundtrip_string_conversion() {
        // Test that from_str -> to_string -> from_str works
        let original_strings = vec![
            "2c", "3d", "4h", "5s", "6c", "7d", "8h", "9s", "Tc", "Jd", "Qh", "Ks", "Ac",
        ];

        for &str_repr in &original_strings {
            let card = Card::from_str(str_repr).unwrap();
            let back_to_string = format!("{}", card);
            assert_eq!(back_to_string, str_repr);

            // And parse again
            let card2 = Card::from_str(&back_to_string).unwrap();
            assert_eq!(card, card2);
        }
    }

    #[test]
    fn test_all_cards_unique() {
        let mut cards = Vec::new();

        // Generate all 52 cards
        for rank in 0..13 {
            for suit in 0..4 {
                cards.push(Card::new(rank, suit).unwrap());
            }
        }

        assert_eq!(cards.len(), 52);

        // Check all are unique
        for i in 0..cards.len() {
            for j in (i + 1)..cards.len() {
                assert_ne!(
                    cards[i], cards[j],
                    "Cards {} and {} should be different",
                    cards[i], cards[j]
                );
            }
        }
    }

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
    fn test_performance_card_creation() {
        use std::time::Instant;

        let start = Instant::now();
        let mut cards = Vec::new();

        // Create 10,000 cards
        for _ in 0..10_000 {
            for rank in 10..13 {
                // Ace, King, Queen
                for suit in 0..4 {
                    cards.push(Card::new(rank, suit).unwrap());
                }
            }
        }

        let duration = start.elapsed();
        assert_eq!(cards.len(), 120_000);

        // Should be very fast (< 10ms typically)
        assert!(
            duration.as_millis() < 100,
            "Card creation took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_performance_string_conversion() {
        use std::time::Instant;

        let cards: Vec<Card> = (0..1000)
            .map(|i| {
                let rank = (i % 13) as u8;
                let suit = (i % 4) as u8;
                Card::new(rank, suit).unwrap()
            })
            .collect();

        let start = Instant::now();

        // Convert all to strings
        let strings: Vec<String> = cards.iter().map(|c| format!("{}", c)).collect();

        let duration = start.elapsed();
        assert_eq!(strings.len(), 1000);

        // Should be very fast (< 5ms typically)
        assert!(
            duration.as_millis() < 50,
            "String conversion took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_card_serialization_edge_cases() {
        // Test serialization with edge case cards
        let ace_spades = Card::new(12, 3).unwrap();
        let two_hearts = Card::new(0, 0).unwrap();

        // JSON roundtrip
        let json = serde_json::to_string(&ace_spades).unwrap();
        let deserialized: Card = serde_json::from_str(&json).unwrap();
        assert_eq!(ace_spades, deserialized);

        let json = serde_json::to_string(&two_hearts).unwrap();
        let deserialized: Card = serde_json::from_str(&json).unwrap();
        assert_eq!(two_hearts, deserialized);

        // TOML roundtrip
        let toml = toml::to_string(&ace_spades).unwrap();
        let deserialized: Card = toml::from_str(&toml).unwrap();
        assert_eq!(ace_spades, deserialized);

        // Test invalid JSON (should fail gracefully)
        let invalid_json = r#"{"rank": 13, "suit": 0}"#; // Invalid rank
        assert!(serde_json::from_str::<Card>(invalid_json).is_err());

        let invalid_json = r#"{"rank": 0, "suit": 4}"#; // Invalid suit
        assert!(serde_json::from_str::<Card>(invalid_json).is_err());
    }

    #[test]
    fn test_card_comparison_properties() {
        // Test transitivity
        let ace_spades = Card::new(12, 3).unwrap();
        let king_spades = Card::new(11, 3).unwrap();
        let queen_spades = Card::new(10, 3).unwrap();

        assert!(ace_spades > king_spades);
        assert!(king_spades > queen_spades);
        assert!(ace_spades > queen_spades);

        // Test antisymmetry
        assert!(!(ace_spades < ace_spades));
        assert!(!(ace_spades > ace_spades));
        assert_eq!(ace_spades, ace_spades);

        // Test suit ordering with same rank
        let ace_spades = Card::new(12, 3).unwrap();
        let ace_hearts = Card::new(12, 0).unwrap();
        let ace_diamonds = Card::new(12, 1).unwrap();
        let ace_clubs = Card::new(12, 2).unwrap();

        assert!(ace_spades > ace_hearts);
        assert!(ace_hearts > ace_diamonds);
        assert!(ace_diamonds > ace_clubs);
    }

    #[test]
    fn test_card_hash_properties() {
        use std::collections::HashSet;

        // Test that equal cards have equal hashes
        let card1 = Card::new(12, 3).unwrap();
        let card2 = Card::new(12, 3).unwrap();
        assert_eq!(card1, card2);

        let hash1 = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            card1.hash(&mut hasher);
            hasher.finish()
        };

        let hash2 = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            card2.hash(&mut hasher);
            hasher.finish()
        };

        assert_eq!(hash1, hash2);

        // Test hash distribution (no collisions for all 52 cards)
        let mut hashes = HashSet::new();
        for rank in 0..13 {
            for suit in 0..4 {
                let card = Card::new(rank, suit).unwrap();
                let hash = {
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::{Hash, Hasher};
                    let mut hasher = DefaultHasher::new();
                    card.hash(&mut hasher);
                    hasher.finish()
                };
                assert!(
                    hashes.insert(hash),
                    "Hash collision detected for card: {}",
                    card
                );
            }
        }
        assert_eq!(hashes.len(), 52);
    }

    #[test]
    fn test_card_string_conversion_edge_cases() {
        // Test all possible rank/suit combinations
        for rank in 0..13 {
            for suit in 0..4 {
                let card = Card::new(rank, suit).unwrap();
                let string_repr = format!("{}", card);
                assert_eq!(string_repr.len(), 2);

                // Should be able to parse back
                let parsed = Card::from_str(&string_repr).unwrap();
                assert_eq!(card, parsed);
            }
        }

        // Test rank_char and suit_char functions
        assert_eq!(Card::new(12, 3).unwrap().rank_char(), 'A');
        assert_eq!(Card::new(0, 0).unwrap().rank_char(), '2');
        assert_eq!(Card::new(12, 3).unwrap().suit_char(), 's');
        assert_eq!(Card::new(0, 0).unwrap().suit_char(), 'h');

        // Test rank_from_char and suit_to_char
        assert_eq!(Card::rank_from_char('A'), Some(12));
        assert_eq!(Card::rank_from_char('2'), Some(0));
        assert_eq!(Card::rank_from_char('X'), None); // Invalid

        // Test rank_to_char and suit_to_char
        assert_eq!(Card::rank_to_char(12), 'A');
        assert_eq!(Card::rank_to_char(0), '2');
        assert_eq!(Card::suit_to_char(3), 's');
        assert_eq!(Card::suit_to_char(0), 'h');
    }

    #[test]
    fn test_card_boundary_values() {
        // Test maximum valid values
        let max_card = Card::new(12, 3).unwrap(); // Ace of Spades
        assert_eq!(max_card.rank(), 12);
        assert_eq!(max_card.suit(), 3);

        // Test minimum valid values
        let min_card = Card::new(0, 0).unwrap(); // Two of Hearts
        assert_eq!(min_card.rank(), 0);
        assert_eq!(min_card.suit(), 0);

        // Test all boundary combinations
        let boundaries = [
            (0, 0),
            (0, 3),
            (12, 0),
            (12, 3),
            (6, 1),
            (7, 2), // Middle values
        ];

        for (rank, suit) in boundaries.iter() {
            let card = Card::new(*rank, *suit).unwrap();
            assert_eq!(card.rank(), *rank);
            assert_eq!(card.suit(), *suit);

            // Should serialize/deserialize correctly
            let json = serde_json::to_string(&card).unwrap();
            let deserialized: Card = serde_json::from_str(&json).unwrap();
            assert_eq!(card, deserialized);
        }
    }
}
