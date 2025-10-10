//! # Poker Error Types
//!
//! This module defines comprehensive error types for the poker library,
//! replacing string-based errors with structured, type-safe error handling.

use crate::card::Card;
use std::fmt;

/// Comprehensive error type for all poker-related operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PokerError {
    /// Invalid card rank (must be 0-12)
    InvalidCardRank { rank: u8 },
    /// Invalid card suit (must be 0-3)
    InvalidCardSuit { suit: u8 },
    /// Invalid card string format
    InvalidCardString { string: String },
    /// Invalid rank character in card string
    InvalidRankCharacter { character: char },
    /// Invalid suit character in card string
    InvalidSuitCharacter { character: char },
    /// Duplicate card found
    DuplicateCard(Card),
    /// Invalid hand size (must be 0-7 cards)
    InvalidHandSize { size: usize },
    /// Invalid hole cards notation length
    InvalidHoleCardsNotationLength { length: usize },
    /// Invalid suitedness indicator (must be 's' or 'o')
    InvalidSuitednessIndicator { indicator: char },
    /// Pairs cannot have suitedness indicators
    PairsCannotHaveSuitedness,
    /// Non-pair hole cards must specify suitedness
    NonPairMustSpecifySuitedness,
    /// Invalid street transition in board dealing
    InvalidStreetTransition,
    /// Flop must consist of exactly 3 cards
    FlopMustBeThreeCards { actual: usize },
    /// Must have exactly 3 cards before dealing turn
    MustHaveThreeCardsForTurn { actual: usize },
    /// Must have exactly 4 cards before dealing river
    MustHaveFourCardsForRiver { actual: usize },
    /// Cannot deal from current street
    CannotDealFromStreet { current_street: String },
    /// Combined hole cards and board exceed 7 cards
    CombinedCardsExceedLimit { total: usize },
    /// Duplicate cards in new deal
    DuplicateCardsInDeal,
    /// New card duplicates existing board card
    DuplicateWithExistingBoardCard(Card),
}

impl fmt::Display for PokerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PokerError::InvalidCardRank { rank } => {
                write!(f, "Invalid card rank: {}. Rank must be 0-12", rank)
            }
            PokerError::InvalidCardSuit { suit } => {
                write!(f, "Invalid card suit: {}. Suit must be 0-3", suit)
            }
            PokerError::InvalidCardString { string } => {
                write!(
                    f,
                    "Invalid card string: '{}'. Must be exactly 2 characters (rank + suit)",
                    string
                )
            }
            PokerError::InvalidRankCharacter { character } => {
                write!(
                    f,
                    "Invalid rank character: '{}'. Must be 2-9, T, J, Q, K, or A",
                    character
                )
            }
            PokerError::InvalidSuitCharacter { character } => {
                write!(
                    f,
                    "Invalid suit character: '{}'. Must be h, d, c, or s",
                    character
                )
            }
            PokerError::DuplicateCard(card) => {
                write!(f, "Duplicate card found: {}", card)
            }
            PokerError::InvalidHandSize { size } => {
                write!(
                    f,
                    "Invalid hand size: {}. Hand must contain 0-7 cards",
                    size
                )
            }
            PokerError::InvalidHoleCardsNotationLength { length } => {
                write!(
                    f,
                    "Invalid hole cards notation length: {}. Must be 2 or 3 characters",
                    length
                )
            }
            PokerError::InvalidSuitednessIndicator { indicator } => {
                write!(
                    f,
                    "Invalid suitedness indicator: '{}'. Must be 's' or 'o'",
                    indicator
                )
            }
            PokerError::PairsCannotHaveSuitedness => {
                write!(f, "Pairs cannot have suitedness indicators")
            }
            PokerError::NonPairMustSpecifySuitedness => {
                write!(
                    f,
                    "Non-pair hole cards must specify suitedness with 's' or 'o'"
                )
            }
            PokerError::InvalidStreetTransition => {
                write!(f, "Invalid street transition")
            }
            PokerError::FlopMustBeThreeCards { actual } => {
                write!(f, "Flop must consist of exactly 3 cards, got {}", actual)
            }
            PokerError::MustHaveThreeCardsForTurn { actual } => {
                write!(
                    f,
                    "Must have exactly 3 cards before dealing turn, got {}",
                    actual
                )
            }
            PokerError::MustHaveFourCardsForRiver { actual } => {
                write!(
                    f,
                    "Must have exactly 4 cards before dealing river, got {}",
                    actual
                )
            }
            PokerError::CannotDealFromStreet { current_street } => {
                write!(f, "Cannot deal from street: {}", current_street)
            }
            PokerError::CombinedCardsExceedLimit { total } => {
                write!(
                    f,
                    "Combined hole cards and board must result in at most 7 cards, got {}",
                    total
                )
            }
            PokerError::DuplicateCardsInDeal => {
                write!(f, "Duplicate cards in new deal")
            }
            PokerError::DuplicateWithExistingBoardCard(card) => {
                write!(f, "New card duplicates existing board card: {}", card)
            }
        }
    }
}

impl std::error::Error for PokerError {}

/// Convert PokerError to String for backward compatibility
impl From<PokerError> for String {
    fn from(error: PokerError) -> String {
        error.to_string()
    }
}

/// Convert &str to PokerError for backward compatibility
impl From<&str> for PokerError {
    fn from(s: &str) -> PokerError {
        PokerError::InvalidCardString {
            string: s.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;

    #[test]
    fn test_error_display() {
        let error = PokerError::InvalidCardRank { rank: 13 };
        assert_eq!(
            error.to_string(),
            "Invalid card rank: 13. Rank must be 0-12"
        );

        let error = PokerError::InvalidCardSuit { suit: 4 };
        assert_eq!(error.to_string(), "Invalid card suit: 4. Suit must be 0-3");

        let error = PokerError::DuplicateCard(Card::new(12, 3).unwrap());
        assert!(error.to_string().contains("Duplicate card found"));
    }

    #[test]
    fn test_error_conversion() {
        let error = PokerError::InvalidCardRank { rank: 13 };
        let string: String = error.into();
        assert!(string.contains("Invalid card rank"));

        let error: PokerError = "test string".into();
        assert!(matches!(error, PokerError::InvalidCardString { .. }));
    }
}
