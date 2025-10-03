//! # Poker API Module
//!
//! This module provides the core application programming interface (API) for the poker
//! hand evaluation library. It contains all the essential types, traits, and functionality
//! needed to work with poker cards, hands, players, and game state.
//!
//! ## Module Overview
//!
//! The API module is organized into several key areas:
//! - **Card Management**: [`card`] - Individual playing card representation
//! - **Hand Management**: [`hand`] - Collections of cards for poker hands
//! - **Deck Operations**: [`deck`] - Standard 52-card deck with shuffling
//! - **Player Actions**: [`action`] - Betting actions and action types
//! - **Player Interface**: [`player`] - Bot interface and player trait
//! - **Game Information**: [`game_info`] - Game state and information access
//! - **Game Observation**: [`game_observer`] - Event-driven game state updates
//! - **Player Information**: [`player_info`] - Individual player state queries
//! - **Preferences**: [`preferences`] - Configuration and settings management
//! - **Hand Evaluation**: [`hand_eval`] - Core hand evaluation trait
//!
//! ## Core Types
//!
//! ### Card and Hand Management
//! - [`Card`](card::Card) - Individual playing card with rank and suit
//! - [`Hand`](hand::Hand) - Collection of cards (up to 7 for Texas Hold'em)
//! - [`Deck`](deck::Deck) - Standard 52-card deck with shuffle/deal operations
//!
//! ### Player and Actions
//! - [`Action`](action::Action) - Player betting actions (bet, raise, fold, etc.)
//! - [`Player`](player::Player) - Trait for poker bot implementations
//! - [`PlayerInfo`](player_info::PlayerInfo) - Individual player state information
//!
//! ### Game State
//! - [`GameInfo`](game_info::GameInfo) - Comprehensive game state access
//! - [`GameObserver`](game_observer::GameObserver) - Event notification system
//! - [`Preferences`](preferences::Preferences) - Configuration management
//!
//! ## Key Features
//!
//! - **Type Safety**: Strongly typed card and hand representations
//! - **Memory Efficiency**: Compact binary representations for performance
//! - **Extensible Design**: Trait-based architecture for custom implementations
//! - **Event-Driven**: Observer pattern for game state notifications
//! - **Configuration**: Flexible preference system for bot customization
//!
//! ## Examples
//!
//! ### Basic Card and Hand Operations
//!
//! ```rust
//! use poker_api::api::{card::Card, hand::Hand, deck::Deck};
//!
//! // Create cards and hands
//! let ace_spades = Card::from_string("As").unwrap();
//! let king_spades = Card::from_string("Ks").unwrap();
//!
//! let mut hand = Hand::new();
//! hand.add_card(ace_spades).unwrap();
//! hand.add_card(king_spades).unwrap();
//!
//! // Work with decks
//! let mut deck = Deck::new();
//! deck.shuffle();
//! let dealt_card = deck.deal().unwrap();
//! ```
//!
//! ### Player Actions
//!
//! ```rust
//! use poker_api::api::action::{Action, ActionType};
//!
//! let fold_action = Action::fold_action(50.0);
//! let bet_action = Action::bet_action(100.0);
//! let raise_action = Action::raise_action(50.0, 200.0);
//!
//! println!("Action: {}", fold_action.to_string());
//! ```
//!
//! ### Game State Access
//!
//! ```rust
//! use poker_api::api::game_info::GameInfo;
//!
//! fn analyze_game(game_info: &dyn GameInfo) {
//!     println!("Total pot: ${}", game_info.get_total_pot_size());
//!     println!("Players: {}", game_info.get_num_players());
//!     println!("Stage: {}", game_info.get_stage());
//! }
//! ```
//!
//! ## Architecture
//!
//! The API module follows a layered architecture:
//! - **Data Types**: Concrete types for cards, hands, and actions
//! - **Traits**: Interfaces for players, observers, and game info
//! - **Implementations**: Default implementations and utilities
//! - **Integration**: Seamless integration between all components
//!
//! ## Performance Considerations
//!
//! - **Memory Layout**: Types optimized for cache efficiency
//! - **Copy Semantics**: Small types implement Copy for performance
//! - **Trait Objects**: Used sparingly to minimize vtable overhead
//! - **Inline Operations**: Common operations inlined for speed

pub mod action;
pub mod card;
pub mod deck;
pub mod game_info;
pub mod game_observer;
pub mod hand;
mod hand_eval;
pub mod player;
pub mod player_info;
pub mod preferences;
