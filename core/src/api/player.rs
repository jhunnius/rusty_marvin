//! # Player Module
//!
//! This module defines the core `Player` trait that all poker bot implementations must
//! implement. The Player trait provides the interface between the poker game engine
//! and individual bot logic, enabling different AI strategies and human player interfaces.
//!
//! ## Player Trait Overview
//!
//! The `Player` trait defines the essential methods that a poker bot must implement:
//! - **Initialization**: Setup with preferences and game parameters
//! - **Hole Cards**: Receive private cards at the start of a hand
//! - **Action Selection**: Choose appropriate actions during betting rounds
//! - **Game Observation**: Receive information about game state changes
//!
//! ## Key Methods
//!
//! - `init()`: Initialize player with preferences and configuration
//! - `hole_cards()`: Receive private hole cards for the current hand
//! - `get_action()`: Return the player's chosen action when it's their turn
//! - GameObserver methods: Receive updates about game state and other players' actions
//!
//! ## Implementation Requirements
//!
//! Any struct implementing `Player` must:
//! - Handle initialization with arbitrary preferences
//! - Process hole cards correctly
//! - Return valid actions in `get_action()`
//! - Implement all `GameObserver` methods appropriately
//!
//! ## Examples
//!
//! ### Basic Player Implementation
//!
//! ```rust
//! use poker_api::api::player::Player;
//! use poker_api::api::action::{Action, ActionType};
//! use poker_api::api::card::Card;
//! use poker_api::api::game_observer::GameObserver;
//! use poker_api::api::preferences::Preferences;
//!
//! struct SimpleBot {
//!     hole_cards: Option<(Card, Card)>,
//! }
//!
//! impl Player for SimpleBot {
//!     fn init(&mut self, _prefs: Box<dyn Preferences>) {
//!         // Initialize bot with preferences
//!     }
//!
//!     fn hole_cards(&mut self, c1: Card, c2: Card, _seat: usize) {
//!         self.hole_cards = Some((c1, c2));
//!     }
//!
//!     fn get_action(&mut self) -> Action {
//!         // Simple logic: always call if cards are good
//!         Action::call_action(0.0)
//!     }
//! }
//!
//! impl GameObserver for SimpleBot {
//!     // Implement game observation methods...
//! #   fn game_start(&mut self, _info: poker_api::api::game_info::GameInfo) {}
//! #   fn game_end(&mut self) {}
//! #   fn round_start(&mut self, _round: u8) {}
//! #   fn round_end(&mut self) {}
//! #   fn player_action(&mut self, _seat: usize, _action: Action) {}
//! #   fn community_cards(&mut self, _cards: Vec<Card>) {}
//! }
//! ```
//!
//! ## Design Decisions
//!
//! - **Trait-Based Design**: Enables polymorphism between different bot types
//! - **Event-Driven Architecture**: Players receive game state through observer pattern
//! - **Minimal Interface**: Core methods required for any poker bot implementation
//! - **Extensible Preferences**: Support for arbitrary bot configuration
//! - **Action-Based Decisions**: Players return complete action objects
//!
//! ## Performance Characteristics
//!
//! - **Memory**: Minimal trait object overhead
//! - **Method Calls**: O(1) trait method dispatch
//! - **State Management**: Player manages internal state as needed
//! - **Action Generation**: Variable time depending on bot complexity

use crate::api::action::Action;
use crate::api::card::Card;
use crate::api::game_observer::GameObserver;
use crate::api::preferences::Preferences;

pub trait Player: GameObserver {
    /// Initialize your player from the given preferences.
    fn init(&mut self, prefs: Box<dyn Preferences>);

    /// Receive your hole cards.
    fn hole_cards(&mut self, c1: Card, c2: Card, seat: usize);

    /// Requests an Action from the player. Called when it is the Player's turn to act.
    fn get_action(&mut self) -> Action;
}
