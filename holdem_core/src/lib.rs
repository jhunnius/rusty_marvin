//! # Poker Core Library
//!
//! Core data structures for poker AI systems. This library provides the fundamental
//! representations for cards, decks, hole cards, hands, and community boards used
//! in poker games, optimized for performance with zero-based enumerations.
//!
//! ## Quick Start
//!
//! ```rust
//! use holdem_core::{Card, Deck, HoleCards, Hand, Board};
//! use std::str::FromStr;
//!
//! // Create a card
//! let card = Card::from_str("As").unwrap();
//!
//! // Create hole cards
//! let hole_cards = HoleCards::from_notation("AKs").unwrap();
//!
//! // Create a board using the fluent builder pattern
//! let board = Board::new()
//!     .with_flop([
//!         Card::from_str("Kd").unwrap(),
//!         Card::from_str("Qh").unwrap(),
//!         Card::from_str("Jc").unwrap(),
//!     ])
//!     .unwrap()
//!     .with_turn(Card::from_str("Ts").unwrap()).unwrap()
//!     .with_river(Card::from_str("9h").unwrap()).unwrap();
//!
//! // Create a complete hand
//! let hand = Hand::from_hole_cards_and_board(&hole_cards, &board).unwrap();
//!
//! // Or create hands from notation strings
//! let hand = Hand::from_notation("As Ks Qs Js Ts").unwrap();
//! ```
//!
//! ## Features
//!
//! - **Performance Optimized**: Zero-based enums for fast bit manipulation and lookup tables
//! - **Memory Efficient**: Compact data structures for mass simulations
//! - **Serialization Ready**: TOML and JSON support for configuration and networking
//! - **Texas Hold'em Support**: Complete representation of hole cards, boards, and hands
//! - **Type Safe**: Strong typing prevents invalid poker states

/// Core poker card representation with zero-based rank/suit enums
pub mod card;

/// Deck of cards representation with shuffle and deal functionality
pub mod deck;

/// Complete poker hand representation for 5-7 card evaluation
pub mod hand;

/// Hole cards representation for player's private cards
pub mod hole_cards;

/// Community cards (board) representation with betting street management
pub mod board;

/// Comprehensive error types for poker operations
pub mod errors;

/// Core hand evaluation functionality with lookup tables
pub mod evaluator;

/// Re-export holdem_core types for convenience
pub use board::Board;
pub use card::Card;
pub use deck::Deck;
pub use hand::Hand;
pub use hole_cards::HoleCards;

/// Re-export Street enum for convenience
pub use board::Street;

/// Re-export error types for convenience
pub use errors::PokerError;

/// Re-export evaluator types for convenience
pub use evaluator::{Evaluator, HandRank, HandValue};

/// Re-export singleton functionality
pub use evaluator::singleton::EvaluatorSingleton;

/// Re-export integration utilities
pub use evaluator::integration::{HandEvaluation, HandEvaluator, HoleCardsEvaluation};

/// Re-export file I/O functionality
pub use evaluator::file_io::{LutFileManager, TableInfo, TableType};

#[cfg(test)]
mod tests {}
