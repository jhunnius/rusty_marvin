//! # Simple Player Module
//!
//! This module provides a basic implementation of a poker bot for testing and demonstration
//! purposes. The `SimplePlayer` struct implements both the `Player` and `GameObserver`
//! traits, providing a complete example of how to create a poker bot.
//!
//! ## Simple Player Overview
//!
//! The `SimplePlayer` is a minimal poker bot implementation that:
//! - **Receives Hole Cards**: Stores and displays received hole cards
//! - **Makes Simple Decisions**: Always checks if possible, otherwise folds
//! - **Observes Game Events**: Prints information about game state changes
//! - **Tracks Basic State**: Maintains name, stack size, and seat position
//!
//! ## Key Features
//!
//! - **Complete Implementation**: Implements both Player and GameObserver traits
//! - **Educational Value**: Serves as a starting point for custom bot development
//! - **Debugging Aid**: Provides detailed logging of game events
//! - **Simple Strategy**: Easy-to-understand decision-making logic
//! - **Extensible Design**: Can be modified to implement more sophisticated strategies
//!
//! ## Player State
//!
//! The SimplePlayer tracks:
//! - **Name**: Player identifier for logging and display
//! - **Stack**: Current chip count (though not used in decision-making)
//! - **Hole Cards**: Private cards received at the start of each hand
//! - **Seat**: Table position assigned by the game
//!
//! ## Examples
//!
//! ### Creating a Simple Player
//!
//! ```rust
//! use poker_api::simple_player::SimplePlayer;
//!
//! let player = SimplePlayer::new("Alice".to_string(), 1000.0);
//! println!("Created player: {}", player.name);
//! ```
//!
//! ### Player Decision Making
//!
//! ```rust
//! use poker_api::simple_player::SimplePlayer;
//! use poker_api::api::card::Card;
//! use poker_api::api::preferences::Preferences;
//!
//! let mut player = SimplePlayer::new("Bob".to_string(), 1000.0);
//!
//! // Initialize with preferences (required by Player trait)
//! let prefs = Box::new(player); // SimplePlayer doesn't use preferences
//! player.init(prefs);
//!
//! // Receive hole cards
//! let card1 = Card::from_string("As").unwrap();
//! let card2 = Card::from_string("Ks").unwrap();
//! player.hole_cards(card1, card2, 3);
//!
//! // Get action (will check since player has cards)
//! let action = player.get_action();
//! println!("Player action: {}", action.to_string());
//! ```
//!
//! ### Game Event Observation
//!
//! ```rust
//! use poker_api::simple_player::SimplePlayer;
//! use poker_api::api::action::Action;
//! use poker_api::api::card::Card;
//! use poker_api::api::game_info::GameInfo;
//!
//! let mut player = SimplePlayer::new("Charlie".to_string(), 1000.0);
//!
//! // Game events are logged to console
//! player.game_start_event(Box::new(player)); // Would need proper GameInfo
//! player.action_event(2, Action::fold_action(50.0));
//! player.win_event(1, 100.0, "Two Pair".to_string());
//! ```
//!
//! ## Strategy Logic
//!
//! The SimplePlayer uses extremely basic decision logic:
//! ```text
//! if has_hole_cards:
//!     return Action::check_action()
//! else:
//!     return Action::fold_action(0.0)
//! ```
//!
//! This strategy:
//! - **Never Bets**: Only checks or folds
//! - **No Hand Analysis**: Doesn't evaluate hole card strength
//! - **No Position Awareness**: Doesn't consider table position
//! - **No Opponent Modeling**: Doesn't track other players' tendencies
//!
//! ## Design Decisions
//!
//! - **Minimal Implementation**: Focuses on trait compliance over sophisticated play
//! - **Console Output**: Uses println! for easy debugging and demonstration
//! - **Immutable Actions**: Always returns the same action type for given state
//! - **No State Complexity**: Maintains only essential player information
//! - **Educational Focus**: Prioritizes clarity and understandability
//!
//! ## Performance Characteristics
//!
//! - **Memory**: ~100 bytes per player instance
//! - **Decision Time**: O(1) - simple conditional logic
//! - **Event Processing**: O(1) - direct println calls
//! - **State Updates**: O(1) - direct field assignment
//! - **Scalability**: Suitable for small-scale testing and development

use crate::api::action::Action;
use crate::api::card::Card;
use crate::api::game_info::GameInfo;
use crate::api::game_observer::GameObserver;
use crate::api::player::Player;
use crate::api::preferences::Preferences;

pub struct SimplePlayer {
    name: String,
    stack: f64,
    hole_cards: Option<(Card, Card)>,
    seat: usize,
}

impl SimplePlayer {
    pub fn new(name: String, stack: f64) -> Self {
        Self {
            name,
            stack,
            hole_cards: None,
            seat: 0,
        }
    }
}

impl GameObserver for SimplePlayer {
    fn action_event(&mut self, pos: usize, act: Action) {
        println!("Player {} made action: {:?}", pos, act);
    }

    fn deal_hole_cards_event(&mut self) {
        println!("Hole cards are being dealt.");
    }

    fn game_over_event(&mut self) {
        println!("The hand is over.");
    }

    fn game_start_event(&mut self, gi: Box<dyn GameInfo>) {
        println!(
            "The hand (ID: {:?}) is starting. Game Info: ",
            gi.get_game_id()
        );
    }

    fn game_state_changed(&mut self) {
        println!("Game state has been updated.");
    }

    fn showdown_event(&mut self, pos: usize, c1: Card, c2: Card) {
        println!("Player {} showed cards: {} and {}", pos, c1, c2);
    }

    fn stage_event(&mut self, stage: u8) {
        println!("New stage: {}", stage);
    }

    fn win_event(&mut self, pos: usize, amount: f64, hand_name: String) {
        println!("Player {} won {} with {}", pos, amount, hand_name);
    }
}

impl Player for SimplePlayer {
    fn init(&mut self, prefs: Box<dyn Preferences>) {
        println!("Initializing player with preferences.");
    }

    fn hole_cards(&mut self, c1: Card, c2: Card, seat: usize) {
        self.hole_cards = Some((c1, c2));
        self.seat = seat;
        println!(
            "Player {} received hole cards: {} and {}",
            self.name, c1, c2
        );
    }

    fn get_action(&mut self) -> Action {
        // Simple decision-making logic: always check if possible, otherwise fold
        if self.hole_cards.is_some() {
            Action::check_action()
        } else {
            Action::fold_action(0.0)
        }
    }
}
