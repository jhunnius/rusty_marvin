//! # Game Observer Module
//!
//! This module defines the `GameObserver` trait that enables poker bots and observers
//! to receive real-time notifications about game events and state changes. This trait
//! implements the Observer pattern, allowing decoupled communication between the game
//! engine and interested parties.
//!
//! ## Game Observer Trait Overview
//!
//! The `GameObserver` trait provides event-driven notifications for:
//! - **Game Lifecycle**: Start, end, and state changes
//! - **Player Actions**: Every action taken by any player
//! - **Card Events**: Hole cards dealt and showdown reveals
//! - **Betting Stages**: New betting rounds and stage transitions
//! - **Results**: Winners, amounts won, and final hand information
//!
//! ## Event Types
//!
//! ### Game Events
//! - `game_start_event()`: Hand initialization with game info
//! - `game_over_event()`: Hand completion
//! - `game_state_changed()`: State update after action processing
//!
//! ### Action Events
//! - `action_event()`: Individual player actions (bet, raise, fold, etc.)
//! - `stage_event()`: New betting round begins
//!
//! ### Card Events
//! - `deal_hole_cards_event()`: Hole cards being distributed
//! - `showdown_event()`: Player reveals hole cards at showdown
//!
//! ### Result Events
//! - `win_event()`: Player wins pot with hand information
//!
//! ## Implementation Requirements
//!
//! Any struct implementing `GameObserver` must handle:
//! - All event types appropriately
//! - Proper state management for game information
//! - Thread-safe event processing if needed
//! - Graceful handling of unexpected events
//!
//! ## Examples
//!
//! ### Basic Observer Implementation
//!
//! ```rust
//! use poker_api::api::game_observer::GameObserver;
//! use poker_api::api::action::Action;
//! use poker_api::api::card::Card;
//! use poker_api::api::game_info::GameInfo;
//!
//! struct LoggingObserver {
//!     game_count: usize,
//!     action_count: usize,
//! }
//!
//! impl GameObserver for LoggingObserver {
//!     fn game_start_event(&mut self, _gi: Box<dyn GameInfo>) {
//!         self.game_count += 1;
//!         println!("Game {} started", self.game_count);
//!     }
//!
//!     fn game_over_event(&mut self) {
//!         println!("Game {} ended", self.game_count);
//!     }
//!
//!     fn action_event(&mut self, pos: usize, act: Action) {
//!         self.action_count += 1;
//!         println!("Seat {}: {}", pos, act.to_string());
//!     }
//!
//!     fn stage_event(&mut self, stage: u8) {
//!         let stage_name = match stage {
//!             0 => "Preflop",
//!             1 => "Flop",
//!             2 => "Turn",
//!             3 => "River",
//!             _ => "Unknown",
//!         };
//!         println!("New stage: {}", stage_name);
//!     }
//!
//!     fn win_event(&mut self, pos: usize, amount: f64, hand_name: String) {
//!         println!("Seat {} won ${:.2} with {}", pos, amount, hand_name);
//!     }
//!
//!     fn game_state_changed(&mut self) {
//!         // State updated after action processing
//!     }
//!
//!     fn deal_hole_cards_event(&mut self) {
//!         println!("Dealing hole cards");
//!     }
//!
//!     fn showdown_event(&mut self, pos: usize, c1: Card, c2: Card) {
//!         println!("Seat {} shows: {} {}", pos, c1, c2);
//!     }
//! }
//! ```
//!
//! ### Advanced Observer with State Tracking
//!
//! ```rust
//! use poker_api::api::game_observer::GameObserver;
//! use poker_api::api::action::Action;
//! use poker_api::api::card::Card;
//! use poker_api::api::game_info::GameInfo;
//!
//! struct StatisticsTracker {
//!     total_games: usize,
//!     action_history: Vec<(usize, Action)>,
//!     pot_sizes: Vec<f64>,
//! }
//!
//! impl GameObserver for StatisticsTracker {
//!     fn game_start_event(&mut self, gi: Box<dyn GameInfo>) {
//!         self.total_games += 1;
//!         self.action_history.clear();
//!         self.pot_sizes.push(gi.get_total_pot_size());
//!     }
//!
//!     fn action_event(&mut self, pos: usize, act: Action) {
//!         self.action_history.push((pos, act));
//!     }
//!
//!     fn game_over_event(&mut self) {
//!         println!("Actions this hand: {}", self.action_history.len());
//!         println!("Final pot: ${:.2}", self.pot_sizes.last().unwrap_or(&0.0));
//!     }
//!
//!     fn game_state_changed(&mut self) {
//!         // Update pot size tracking
//!         if let Some(current_pot) = self.pot_sizes.last_mut() {
//!             // Update with new pot size if available
//!         }
//!     }
//!
//!     // Implement other required methods...
//! #   fn deal_hole_cards_event(&mut self) {}
//! #   fn stage_event(&mut self, _stage: u8) {}
//! #   fn showdown_event(&mut self, _pos: usize, _c1: Card, _c2: Card) {}
//! #   fn win_event(&mut self, _pos: usize, _amount: f64, _hand_name: String) {}
//! }
//! ```
//!
//! ## Design Decisions
//!
//! - **Observer Pattern**: Decoupled event notification system
//! - **Comprehensive Events**: Cover all important game state changes
//! - **Minimal Interface**: Only essential events to avoid overhead
//! - **Mutable Receivers**: Allow observers to maintain state
//! - **Boxed GameInfo**: Avoid object slicing with trait objects
//!
//! ## Performance Characteristics
//!
//! - **Event Notification**: O(1) per observer for most events
//! - **Memory**: Minimal per-event overhead
//! - **State Management**: Observer handles its own state complexity
//! - **Threading**: Not inherently thread-safe (implement as needed)

use crate::api::action::Action;
use crate::api::card::Card;
use crate::api::game_info::GameInfo;

pub trait GameObserver {
    /// A player can override this method to receive events for each action made by a player.
    fn action_event(&mut self, pos: usize, act: Action);

    /// The hole cards are being dealt.
    fn deal_hole_cards_event(&mut self);

    /// The hand is now over.
    fn game_over_event(&mut self);

    /// The hand is starting.
    fn game_start_event(&mut self, gi: Box<dyn GameInfo>);

    /// The game info state has been updated. Called after an action event has been fully processed.
    fn game_state_changed(&mut self);

    /// Player `pos` has shown two cards.
    fn showdown_event(&mut self, pos: usize, c1: Card, c2: Card);

    /// A new stage (betting round) has begun.
    fn stage_event(&mut self, stage: u8);

    /// A player at `pos` has won `amount` with the hand `hand_name`.
    fn win_event(&mut self, pos: usize, amount: f64, hand_name: String);
}
