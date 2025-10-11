//! Core poker hand evaluator implementation

use super::errors::EvaluatorError;
use super::tables::JumpTable;
use crate::{Card, Hand};
use std::sync::Arc;

/// Hand ranking enumeration
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum HandRank {
    /// High card
    HighCard = 0,
    /// One pair
    Pair = 1,
    /// Two pair
    TwoPair = 2,
    /// Three of a kind
    ThreeOfAKind = 3,
    /// Straight
    Straight = 4,
    /// Flush
    Flush = 5,
    /// Full house
    FullHouse = 6,
    /// Four of a kind
    FourOfAKind = 7,
    /// Straight flush
    StraightFlush = 8,
    /// Royal flush
    RoyalFlush = 9,
}

impl HandRank {
    /// Create a hand rank from a numeric value
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(HandRank::HighCard),
            1 => Some(HandRank::Pair),
            2 => Some(HandRank::TwoPair),
            3 => Some(HandRank::ThreeOfAKind),
            4 => Some(HandRank::Straight),
            5 => Some(HandRank::Flush),
            6 => Some(HandRank::FullHouse),
            7 => Some(HandRank::FourOfAKind),
            8 => Some(HandRank::StraightFlush),
            9 => Some(HandRank::RoyalFlush),
            _ => None,
        }
    }

    /// Convert to numeric value
    pub fn as_u8(&self) -> u8 {
        match self {
            HandRank::HighCard => 0,
            HandRank::Pair => 1,
            HandRank::TwoPair => 2,
            HandRank::ThreeOfAKind => 3,
            HandRank::Straight => 4,
            HandRank::Flush => 5,
            HandRank::FullHouse => 6,
            HandRank::FourOfAKind => 7,
            HandRank::StraightFlush => 8,
            HandRank::RoyalFlush => 9,
        }
    }
}

/// Hand value containing rank and strength
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct HandValue {
    /// The hand rank
    pub rank: HandRank,
    /// The strength value for comparison within the same rank
    pub value: u32,
}

impl HandValue {
    /// Create a new hand value
    pub fn new(rank: HandRank, value: u32) -> Self {
        Self { rank, value }
    }

    /// Create a hand value from a combined u32 value
    pub fn from_u32(combined: u32) -> Self {
        let rank_value = (combined >> 16) as u8;
        let strength = combined & 0xFFFF;

        let rank = HandRank::from_u8(rank_value).unwrap_or(HandRank::HighCard);
        Self::new(rank, strength)
    }

    /// Convert to a combined u32 value
    pub fn as_u32(&self) -> u32 {
        ((self.rank.as_u8() as u32) << 16) | self.value
    }
}

/// Main poker hand evaluator
#[derive(Debug, Clone)]
pub struct Evaluator {
    /// Jump table for hand evaluation
    tables: Arc<JumpTable>,
}

impl Evaluator {
    /// Create a new evaluator instance
    pub fn new() -> Result<Self, EvaluatorError> {
        let mut table = JumpTable::with_target_memory();
        table.build().map_err(|e| {
            EvaluatorError::table_init_failed(&format!("Failed to initialize lookup tables: {}", e))
        })?;

        Ok(Self {
            tables: Arc::new(table),
        })
    }

    /// Get the global evaluator instance (singleton pattern)
    pub fn instance() -> Arc<Evaluator> {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<Evaluator> = OnceLock::new();
        let evaluator =
            INSTANCE.get_or_init(|| Evaluator::new().expect("Failed to create evaluator instance"));
        Arc::new(evaluator.clone())
    }

    /// Evaluate a 5-card hand
    pub fn evaluate_5_card(&self, cards: &[Card; 5]) -> HandValue {
        // For now, return a placeholder implementation
        // In a full implementation, this would use the lookup tables
        HandValue::new(HandRank::HighCard, 0)
    }

    /// Evaluate a 6-card hand
    pub fn evaluate_6_card(&self, cards: &[Card; 6]) -> HandValue {
        // For now, return a placeholder implementation
        // In a full implementation, this would use the lookup tables
        HandValue::new(HandRank::HighCard, 0)
    }

    /// Evaluate a 7-card hand
    pub fn evaluate_7_card(&self, cards: &[Card; 7]) -> HandValue {
        // For now, return a placeholder implementation
        // In a full implementation, this would use the lookup tables
        HandValue::new(HandRank::HighCard, 0)
    }

    /// Evaluate a hand from hole cards and board
    pub fn evaluate_hand(&self, hand: &Hand) -> HandValue {
        let cards = hand.cards();
        match cards.len() {
            5 => {
                let card_array: [Card; 5] = cards
                    .try_into()
                    .unwrap_or_else(|_| panic!("Expected 5 cards, got {}", cards.len()));
                self.evaluate_5_card(&card_array)
            }
            6 => {
                let card_array: [Card; 6] = cards
                    .try_into()
                    .unwrap_or_else(|_| panic!("Expected 6 cards, got {}", cards.len()));
                self.evaluate_6_card(&card_array)
            }
            7 => {
                let card_array: [Card; 7] = cards
                    .try_into()
                    .unwrap_or_else(|_| panic!("Expected 7 cards, got {}", cards.len()));
                self.evaluate_7_card(&card_array)
            }
            _ => HandValue::new(HandRank::HighCard, 0),
        }
    }

    /// Get the jump table
    pub fn tables(&self) -> &JumpTable {
        &self.tables
    }

    /// Validate the evaluator state
    pub fn validate(&self) -> Result<(), EvaluatorError> {
        // Basic validation - check if tables exist and have content
        if self.tables.size > 0 {
            Ok(())
        } else {
            Err(EvaluatorError::table_init_failed(
                "Jump table not initialized",
            ))
        }
    }
}
