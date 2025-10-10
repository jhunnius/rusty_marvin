//! # Hole Cards Module
//!
//! This module provides the `HoleCards` struct for representing a player's two private cards
//! in poker. Hole cards are distinct from complete hands and provide specialized methods
//! for analyzing preflop hand strength, suitedness, connectivity, and standard notation.
//!
//! ## Hole Cards Structure
//!
//! A `HoleCards` contains:
//! - An array of exactly two `Card` instances
//! - Methods for analyzing card relationships and properties
//! - Standard poker notation generation
//! - Efficient comparison and hashing
//!
//! ## Key Features
//!
//! - **Validation**: Ensures no duplicate cards during construction
//! - **Properties**: Pair detection, suitedness, connectivity analysis
//! - **Notation**: Standard poker abbreviations ("AKs", "QQ", "T9o")
//! - **Serialization**: Full serde support for persistence
//! - **Performance**: Optimized comparison and hashing for lookups
//! - **Access**: Methods for retrieving individual cards
//!
//! ## Examples
//!
//! ### Creating and Analyzing Hole Cards
//!
//! ```rust
//! use holdem_core::hole_cards::HoleCards;
//! use holdem_core::card::Card;
//!
//! // Create hole cards from strings
//! let hole_cards = HoleCards::from_notation("AKs").unwrap();
//!
//! // Check properties
//! assert!(hole_cards.is_suited());
//! assert!(!hole_cards.is_pair());
//! assert_eq!(hole_cards.connectivity(), 0); // No gap between A and K
//!
//! // Get standard notation
//! assert_eq!(hole_cards.notation(), "AKs");
//! ```
//!
//! ### Working with Individual Cards
//!
//! ```rust
//! use holdem_core::hole_cards::HoleCards;
//!
//! let hole_cards = HoleCards::from_notation("QQ").unwrap();
//!
//! // Access individual cards
//! let card1 = hole_cards.first_card();
//! let card2 = hole_cards.second_card();
//!
//! // Both cards are Queens
//! assert_eq!(card1.rank(), card2.rank());
//! assert!(hole_cards.is_pair());
//! ```
//!
//! ## Design Decisions
//!
//! - **Fixed Size**: Always exactly two cards for hole cards
//! - **Validation**: Constructor validates no duplicates
//! - **Ordering**: Cards stored in rank-descending order for consistency
//! - **Notation**: Follows standard poker conventions (high-low, s/o for suited/offsuit)
//! - **Performance**: Implements Hash and efficient comparison
//! - **Safety**: Comprehensive error handling and validation

use crate::card::Card;
use crate::errors::PokerError;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

/// Represents a player's two private hole cards in poker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HoleCards {
    /// The two cards, stored in rank-descending order
    pub cards: [Card; 2],
}

impl HoleCards {
    /// Create new hole cards from two individual cards
    ///
    /// # Errors
    /// Returns an error if the cards are identical (same rank and suit)
    ///
    /// # Examples
    /// ```
    /// use holdem_core::hole_cards::HoleCards;
    /// use holdem_core::card::Card;
    /// use std::str::FromStr;
    ///
    /// let card1 = Card::from_str("As").unwrap();
    /// let card2 = Card::from_str("Ks").unwrap();
    /// let hole_cards = HoleCards::new(card1, card2).unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic, but returns an error if cards are identical.
    pub fn new(card1: Card, card2: Card) -> Result<Self, PokerError> {
        if card1 == card2 {
            return Err(PokerError::DuplicateCard(card1));
        }

        // Store cards in rank-descending order
        let (high, low) = if card1.rank() >= card2.rank() {
            (card1, card2)
        } else {
            (card2, card1)
        };

        Ok(Self { cards: [high, low] })
    }

    /// Create hole cards from standard poker notation
    ///
    /// # Supported Formats
    /// - "AKs" - suited Ace-King
    /// - "QQ" - pocket Queens
    /// - "T9o" - offsuit Ten-Nine
    ///
    /// # Errors
    /// Returns an error for invalid notation or duplicate ranks with same suit
    ///
    /// # Examples
    /// ```
    /// use holdem_core::hole_cards::HoleCards;
    ///
    /// let pocket_aces = HoleCards::from_notation("AA").unwrap();
    /// let pocket_kings = HoleCards::from_notation("KK").unwrap();
    /// let offsuit_jack_ten = HoleCards::from_notation("JTo").unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic, but returns an error for invalid notation.
    pub fn from_notation(s: &str) -> Result<Self, PokerError> {
        if s.len() < 2 || s.len() > 3 {
            return Err(PokerError::InvalidHoleCardsNotationLength { length: s.len() });
        }

        let chars: Vec<char> = s.chars().collect();

        // Parse ranks
        let rank1 = Card::rank_from_char(chars[0]).ok_or(PokerError::InvalidRankCharacter {
            character: chars[0],
        })?;
        let rank2 = Card::rank_from_char(chars[1]).ok_or(PokerError::InvalidRankCharacter {
            character: chars[1],
        })?;

        // Determine suitedness
        let suited = if s.len() == 3 {
            // Pairs cannot have suitedness indicators
            if rank1 == rank2 {
                return Err(PokerError::PairsCannotHaveSuitedness);
            }
            match chars[2] {
                's' => true,
                'o' => false,
                _ => {
                    return Err(PokerError::InvalidSuitednessIndicator {
                        indicator: chars[2],
                    })
                }
            }
        } else {
            // For pairs, always use different suits
            if rank1 != rank2 {
                return Err(PokerError::NonPairMustSpecifySuitedness);
            }
            false // pairs are neither suited nor offsuit in notation
        };

        // Create cards - for suited hands, use spades; for offsuit, different suits
        let suit1 = 3; // Spades
        let suit2 = if suited { 3 } else { 2 }; // Spades or hearts

        let card1 = Card::new(rank1, suit1).unwrap();
        let card2 = Card::new(rank2, suit2).unwrap();

        Self::new(card1, card2)
    }

    /// Get the first card (higher rank)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{HoleCards, Card};
    ///
    /// let hole_cards = HoleCards::new(Card::new(12, 0).unwrap(), Card::new(11, 1).unwrap()).unwrap();
    /// let first = hole_cards.first_card();
    /// assert_eq!(first.rank(), 12); // Ace
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn first_card(&self) -> Card {
        self.cards[0]
    }

    /// Get the second card (lower rank)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::{HoleCards, Card};
    ///
    /// let hole_cards = HoleCards::new(Card::new(12, 0).unwrap(), Card::new(11, 1).unwrap()).unwrap();
    /// let second = hole_cards.second_card();
    /// assert_eq!(second.rank(), 11); // King
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn second_card(&self) -> Card {
        self.cards[1]
    }

    /// Check if the hole cards form a pocket pair
    ///
    /// # Examples
    /// ```
    /// use holdem_core::hole_cards::HoleCards;
    ///
    /// let pocket_aces = HoleCards::from_notation("AA").unwrap();
    /// assert!(pocket_aces.is_pair());
    ///
    /// let suited_king_queen = HoleCards::from_notation("KQs").unwrap();
    /// assert!(!suited_king_queen.is_pair());
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn is_pair(&self) -> bool {
        self.cards[0].rank() == self.cards[1].rank()
    }

    /// Check if both cards have the same suit (suited)
    ///
    /// # Examples
    /// ```
    /// use holdem_core::hole_cards::HoleCards;
    ///
    /// let suited_ace_king = HoleCards::from_notation("AKs").unwrap();
    /// assert!(suited_ace_king.is_suited());
    ///
    /// let offsuit_queen_jack = HoleCards::from_notation("QJo").unwrap();
    /// assert!(!offsuit_queen_jack.is_suited());
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn is_suited(&self) -> bool {
        self.cards[0].suit() == self.cards[1].suit()
    }

    /// Calculate the connectivity between the two cards
    ///
    /// Returns the number of ranks between the cards (0 = connected, 1 = one gap, etc.)
    /// For pairs, returns 0 (perfectly connected).
    ///
    /// # Examples
    /// ```
    /// use holdem_core::hole_cards::HoleCards;
    ///
    /// let connected = HoleCards::from_notation("KQs").unwrap();
    /// assert_eq!(connected.connectivity(), 0); // K and Q are adjacent
    ///
    /// let one_gapper = HoleCards::from_notation("J9s").unwrap();
    /// assert_eq!(one_gapper.connectivity(), 1); // J and 9 have one rank between (T)
    ///
    /// let pocket_pair = HoleCards::from_notation("77").unwrap();
    /// assert_eq!(pocket_pair.connectivity(), 0); // Pairs are perfectly connected
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn connectivity(&self) -> u8 {
        if self.is_pair() {
            0
        } else {
            let rank1 = self.cards[0].rank() as i8;
            let rank2 = self.cards[1].rank() as i8;
            (rank1 - rank2 - 1) as u8
        }
    }

    /// Generate standard poker notation for the hole cards
    ///
    /// # Format
    /// - Pairs: "AA", "KK", "22"
    /// - Suited hands: "AKs", "QJs", "T9s"
    /// - Offsuit hands: "AKo", "QJo", "T9o"
    ///
    /// # Examples
    /// ```
    /// use holdem_core::hole_cards::HoleCards;
    ///
    /// let pocket_aces = HoleCards::from_notation("AA").unwrap();
    /// assert_eq!(pocket_aces.notation(), "AA");
    ///
    /// let suited_king_queen = HoleCards::from_notation("KQs").unwrap();
    /// assert_eq!(suited_king_queen.notation(), "KQs");
    ///
    /// let offsuit_jack_ten = HoleCards::from_notation("JTo").unwrap();
    /// assert_eq!(offsuit_jack_ten.notation(), "JTo");
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn notation(&self) -> String {
        let rank1_char = Card::rank_to_char(self.cards[0].rank());
        let rank2_char = Card::rank_to_char(self.cards[1].rank());

        if self.is_pair() {
            format!("{}{}", rank1_char, rank2_char)
        } else if self.is_suited() {
            format!("{}{}s", rank1_char, rank2_char)
        } else {
            format!("{}{}o", rank1_char, rank2_char)
        }
    }

    /// Get a display-friendly string representation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holdem_core::hole_cards::HoleCards;
    ///
    /// let aks = HoleCards::from_notation("AKs").unwrap();
    /// let string_repr = aks.to_string();
    /// assert!(string_repr.contains("A"));
    /// assert!(string_repr.contains("K"));
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.cards[0].to_string(),
            self.cards[1].to_string()
        )
    }
}

impl fmt::Display for HoleCards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.notation())
    }
}

impl PartialOrd for HoleCards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HoleCards {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by first card rank, then second card rank, then suitedness
        match self.cards[0].rank().cmp(&other.cards[0].rank()) {
            Ordering::Equal => match self.cards[1].rank().cmp(&other.cards[1].rank()) {
                Ordering::Equal => self.is_suited().cmp(&other.is_suited()),
                ord => ord,
            },
            ord => ord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_hole_cards_creation() {
        let card1 = Card::from_str("As").unwrap();
        let card2 = Card::from_str("Ks").unwrap();
        let hole_cards = HoleCards::new(card1, card2).unwrap();

        assert_eq!(hole_cards.first_card(), card1);
        assert_eq!(hole_cards.second_card(), card2);
    }

    #[test]
    fn test_duplicate_cards_error() {
        let card = Card::from_str("As").unwrap();
        let result = HoleCards::new(card, card);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PokerError::DuplicateCard(_)));
    }

    #[test]
    fn test_from_notation_pairs() {
        let aa = HoleCards::from_notation("AA").unwrap();
        assert!(aa.is_pair());
        assert_eq!(aa.notation(), "AA");

        let kk = HoleCards::from_notation("KK").unwrap();
        assert!(kk.is_pair());
        assert_eq!(kk.notation(), "KK");
    }

    #[test]
    fn test_from_notation_suited() {
        let aks = HoleCards::from_notation("AKs").unwrap();
        assert!(aks.is_suited());
        assert!(!aks.is_pair());
        assert_eq!(aks.notation(), "AKs");
        assert_eq!(aks.connectivity(), 0);
    }

    #[test]
    fn test_from_notation_offsuit() {
        let ako = HoleCards::from_notation("AKo").unwrap();
        assert!(!ako.is_suited());
        assert!(!ako.is_pair());
        assert_eq!(ako.notation(), "AKo");
        assert_eq!(ako.connectivity(), 0);
    }

    #[test]
    fn test_connectivity() {
        let connected = HoleCards::from_notation("KQs").unwrap();
        assert_eq!(connected.connectivity(), 0);

        let one_gapper = HoleCards::from_notation("J9s").unwrap();
        assert_eq!(one_gapper.connectivity(), 1);

        let two_gapper = HoleCards::from_notation("K8s").unwrap();
        assert_eq!(two_gapper.connectivity(), 4); // K to 8 has Q, J, T, 9 between - 4 gaps

        let pair = HoleCards::from_notation("77").unwrap();
        assert_eq!(pair.connectivity(), 0);
    }

    #[test]
    fn test_invalid_notation() {
        assert!(HoleCards::from_notation("").is_err());
        assert!(HoleCards::from_notation("A").is_err());
        assert!(HoleCards::from_notation("AKx").is_err());
        assert!(HoleCards::from_notation("AK").is_err()); // Missing suitedness for non-pair
        assert!(HoleCards::from_notation("1K").is_err()); // Invalid rank
    }

    #[test]
    fn test_ordering() {
        let aks = HoleCards::from_notation("AKs").unwrap();
        let ako = HoleCards::from_notation("AKo").unwrap();
        let aqs = HoleCards::from_notation("AQs").unwrap();

        assert!(aks > ako); // Suited beats offsuit for same ranks
        assert!(aqs < aks); // AK beats AQ
    }

    #[test]
    fn test_display() {
        let aks = HoleCards::from_notation("AKs").unwrap();
        assert_eq!(format!("{}", aks), "AKs");

        let qq = HoleCards::from_notation("QQ").unwrap();
        assert_eq!(format!("{}", qq), "QQ");
    }

    #[test]
    fn test_serialization() {
        let hole_cards = HoleCards::from_notation("AKs").unwrap();
        let serialized = serde_json::to_string(&hole_cards).unwrap();
        let deserialized: HoleCards = serde_json::from_str(&serialized).unwrap();
        assert_eq!(hole_cards, deserialized);
    }

    #[test]
    fn test_hole_cards_validation_edge_cases() {
        // Test with same rank different suits (should work)
        let card1 = Card::new(12, 3).unwrap(); // Ace of spades
        let card2 = Card::new(12, 0).unwrap(); // Ace of hearts
        let hole_cards = HoleCards::new(card1, card2).unwrap();
        assert!(hole_cards.is_pair());

        // Test with different ranks same suit (should work)
        let card1 = Card::new(12, 3).unwrap(); // Ace of spades
        let card2 = Card::new(11, 3).unwrap(); // King of spades
        let hole_cards = HoleCards::new(card1, card2).unwrap();
        assert!(hole_cards.is_suited());

        // Test duplicate cards (should fail)
        let card = Card::new(12, 3).unwrap();
        assert!(HoleCards::new(card, card).is_err());
    }

    #[test]
    fn test_hole_cards_properties_comprehensive() {
        // Test all possible pair combinations
        for rank in 0..13 {
            let notation = format!("{}{}", Card::rank_to_char(rank), Card::rank_to_char(rank));
            let hole_cards = HoleCards::from_notation(&notation).unwrap();
            assert!(hole_cards.is_pair());
            assert!(!hole_cards.is_suited()); // Pairs are neither suited nor offsuit
            assert_eq!(hole_cards.connectivity(), 0);
            assert_eq!(hole_cards.notation(), notation);
        }

        // Test suited hands
        let suited_hands = [
            "AKs", "AQs", "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s",
            "KQs", "KJs", "KTs", "K9s", "K8s", "K7s", "K6s", "K5s", "K4s", "K3s", "K2s", "QJs",
            "QTs", "Q9s", "Q8s", "Q7s", "Q6s", "Q5s", "Q4s", "Q3s", "Q2s",
        ];

        for hand in suited_hands.iter() {
            let hole_cards = HoleCards::from_notation(hand).unwrap();
            assert!(hole_cards.is_suited());
            assert!(!hole_cards.is_pair());
            assert_eq!(hole_cards.notation(), *hand);
        }

        // Test offsuit hands
        let offsuit_hands = [
            "AKo", "AQo", "AJo", "ATo", "A9o", "A8o", "A7o", "A6o", "A5o", "A4o", "A3o", "A2o",
        ];

        for hand in offsuit_hands.iter() {
            let hole_cards = HoleCards::from_notation(hand).unwrap();
            assert!(!hole_cards.is_suited());
            assert!(!hole_cards.is_pair());
            assert_eq!(hole_cards.notation(), *hand);
        }
    }

    #[test]
    fn test_hole_cards_connectivity_edge_cases() {
        // Test maximum gap (A2)
        let a2s = HoleCards::from_notation("A2s").unwrap();
        assert_eq!(a2s.connectivity(), 11); // A to 2 has K,Q,J,T,9,8,7,6,5,4,3 between - 11 gaps

        // Test minimum gap (AK, KQ, etc.)
        let aks = HoleCards::from_notation("AKs").unwrap();
        assert_eq!(aks.connectivity(), 0);

        let kqs = HoleCards::from_notation("KQs").unwrap();
        assert_eq!(kqs.connectivity(), 0);

        // Test various gaps
        let j9s = HoleCards::from_notation("J9s").unwrap();
        assert_eq!(j9s.connectivity(), 1); // J to 9 has T between

        let t7s = HoleCards::from_notation("T7s").unwrap();
        assert_eq!(t7s.connectivity(), 2); // T to 7 has 9,8 between

        let k8s = HoleCards::from_notation("K8s").unwrap();
        assert_eq!(k8s.connectivity(), 4); // K to 8 has Q,J,T,9 between
    }

    #[test]
    fn test_hole_cards_notation_comprehensive() {
        // Test all possible notations generate correctly
        let test_cases = [
            ("AA", "AA"),
            ("AKs", "AKs"),
            ("AKo", "AKo"),
            ("22", "22"),
            ("T9s", "T9s"),
            ("J8o", "J8o"),
        ];

        for (input, expected) in test_cases.iter() {
            let hole_cards = HoleCards::from_notation(input).unwrap();
            assert_eq!(hole_cards.notation(), *expected);
        }

        // Test round-trip: notation -> HoleCards -> notation
        let all_notations = [
            "AA", "KK", "QQ", "JJ", "TT", "99", "88", "77", "66", "55", "44", "33", "22", "AKs",
            "AQs", "AJs", "ATs", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s", "A2s", "AKo",
            "AQo", "AJo", "ATo", "A9o", "A8o", "A7o", "A6o", "A5o", "A4o", "A3o", "A2o", "KQs",
            "KJs", "KTs", "K9s", "K8s", "K7s", "K6s", "K5s", "K4s", "K3s", "K2s", "QJs", "QTs",
            "Q9s", "Q8s", "Q7s", "Q6s", "Q5s", "Q4s", "Q3s", "Q2s", "JTs", "J9s", "J8s", "J7s",
            "J6s", "J5s", "J4s", "J3s", "J2s", "T9s", "T8s", "T7s", "T6s", "T5s", "T4s", "T3s",
            "T2s", "98s", "97s", "96s", "95s", "94s", "93s", "92s", "87s", "86s", "85s", "84s",
            "83s", "82s", "76s", "75s", "74s", "73s", "72s", "65s", "64s", "63s", "62s", "54s",
            "53s", "52s", "43s", "42s", "32s",
        ];

        for notation in all_notations.iter() {
            let hole_cards = HoleCards::from_notation(notation).unwrap();
            assert_eq!(hole_cards.notation(), *notation);
        }
    }

    #[test]
    fn test_hole_cards_notation_invalid() {
        // Test various invalid notations
        let invalid_cases = [
            "", "A", "ABCD", "AKx", "AK", "AAs", "AAo", "AK", "1K", "AK1", "AKz", "ak", "AKS",
            "ako", "A K s", "AKss", "AA s",
        ];

        for invalid in invalid_cases.iter() {
            assert!(
                HoleCards::from_notation(invalid).is_err(),
                "Expected '{}' to be invalid",
                invalid
            );
        }

        // Test non-pair without suitedness indicator
        assert!(HoleCards::from_notation("AK").is_err());
        assert!(HoleCards::from_notation("QJ").is_err());
    }

    #[test]
    fn test_hole_cards_ordering_comprehensive() {
        // Test that ordering is consistent and transitive
        let hands = [
            HoleCards::from_notation("AA").unwrap(),
            HoleCards::from_notation("AKs").unwrap(),
            HoleCards::from_notation("AKo").unwrap(),
            HoleCards::from_notation("AQs").unwrap(),
            HoleCards::from_notation("AQo").unwrap(),
            HoleCards::from_notation("22").unwrap(),
        ];

        // Test transitivity
        for i in 0..hands.len() {
            for j in 0..hands.len() {
                for k in 0..hands.len() {
                    let cmp_ij = hands[i].cmp(&hands[j]);
                    let cmp_jk = hands[j].cmp(&hands[k]);
                    let cmp_ik = hands[i].cmp(&hands[k]);

                    // If i >= j and j >= k, then i >= k
                    if cmp_ij != Ordering::Less && cmp_jk != Ordering::Less {
                        assert!(
                            cmp_ik != Ordering::Less,
                            "Transitivity violated: {} >= {} >= {} but {} < {}",
                            hands[i],
                            hands[j],
                            hands[k],
                            hands[i],
                            hands[k]
                        );
                    }
                }
            }
        }

        // Test specific ordering expectations
        assert!(HoleCards::from_notation("AA").unwrap() > HoleCards::from_notation("AKs").unwrap());
        assert!(
            HoleCards::from_notation("AKs").unwrap() > HoleCards::from_notation("AKo").unwrap()
        );
        assert!(
            HoleCards::from_notation("AKs").unwrap() > HoleCards::from_notation("AQs").unwrap()
        );
    }

    #[test]
    fn test_hole_cards_serialization_comprehensive() {
        // Test JSON serialization round-trip for various hands
        let test_hands = ["AA", "AKs", "AKo", "22", "T9s", "J8o"];

        for hand in test_hands.iter() {
            let hole_cards = HoleCards::from_notation(hand).unwrap();

            // JSON round-trip
            let json = serde_json::to_string(&hole_cards).unwrap();
            let deserialized: HoleCards = serde_json::from_str(&json).unwrap();
            assert_eq!(hole_cards, deserialized);

            // TOML round-trip
            let toml = toml::to_string(&hole_cards).unwrap();
            let deserialized: HoleCards = toml::from_str(&toml).unwrap();
            assert_eq!(hole_cards, deserialized);
        }

        // Test serialization of edge cases
        let max_gap = HoleCards::from_notation("A2s").unwrap();
        let json = serde_json::to_string(&max_gap).unwrap();
        let deserialized: HoleCards = serde_json::from_str(&json).unwrap();
        assert_eq!(max_gap, deserialized);
    }

    #[test]
    fn test_hole_cards_hash_consistency() {
        use std::collections::HashSet;

        // Test that equal hole cards have equal hashes
        let hc1 = HoleCards::from_notation("AKs").unwrap();
        let hc2 = HoleCards::from_notation("AKs").unwrap();
        assert_eq!(hc1, hc2);

        // Test hash distribution (no major collisions for common hands)
        let mut hashes = HashSet::new();
        let common_hands = ["AA", "AKs", "AKo", "AQs", "AQo", "AJs", "AJo"];

        for hand in common_hands.iter() {
            let hc = HoleCards::from_notation(hand).unwrap();
            let hash = {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                hc.hash(&mut hasher);
                hasher.finish()
            };
            hashes.insert(hash);
        }

        // Should have no collisions for these distinct hands
        assert_eq!(hashes.len(), common_hands.len());
    }

    #[test]
    fn test_hole_cards_to_string() {
        let aks = HoleCards::from_notation("AKs").unwrap();
        let string_repr = aks.to_string();
        assert!(string_repr.contains("A"));
        assert!(string_repr.contains("K"));
        assert!(string_repr.len() > 3); // Should be more than just "AKs"
    }

    #[test]
    fn test_hole_cards_card_access() {
        let aks = HoleCards::from_notation("AKs").unwrap();
        let first = aks.first_card();
        let second = aks.second_card();

        assert_eq!(first.rank(), 12); // Ace
        assert_eq!(second.rank(), 11); // King
        assert_eq!(first.suit(), second.suit()); // Both spades
    }
}
