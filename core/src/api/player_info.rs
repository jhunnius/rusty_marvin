//! # Player Info Module
//!
//! This module defines the `PlayerInfo` trait that provides detailed information about
//! individual players in a poker game. This trait offers comprehensive access to player
//! state, betting information, and hand status for use by observers and game logic.
//!
//! ## Player Info Trait Overview
//!
//! The `PlayerInfo` trait provides read-only access to:
//! - **Financial State**: Bankroll, gains, and betting amounts
//! - **Game Status**: Active state, folded status, and position
//! - **Hand Information**: Revealed cards and hand state
//! - **Betting State**: Amounts to call, committed status, and raise capabilities
//! - **Action History**: Last action taken and round participation
//!
//! ## Key Information Categories
//!
//! ### Financial Information
//! - Current bankroll and net gain/loss
//! - Bankroll at start of hand and in small bets
//! - Amount at risk and committed to pot
//!
//! ### Game Status
//! - Seat position, name, and active state
//! - Folded, all-in, and sitting out status
//! - Button position indicator
//!
//! ### Hand Information
//! - Revealed hand (if applicable)
//! - Action status and round participation
//! - Amount contributed to current pot
//!
//! ### Betting Information
//! - Amount required to call
//! - Raise capabilities and amounts
//! - Last action taken
//!
//! ## Implementation Requirements
//!
//! Any struct implementing `PlayerInfo` must provide:
//! - Accurate financial calculations
//! - Current game state information
//! - Proper handling of edge cases (all-in, folded players)
//! - Integration with the broader game state
//!
//! ## Examples
//!
//! ### Basic Player Analysis
//!
//! ```rust
//! use poker_api::api::player_info::PlayerInfo;
//!
//! fn analyze_player(player: &dyn PlayerInfo) {
//!     println!("Player: {}", player.get_name());
//!     println!("Seat: {}", player.get_seat());
//!     println!("Bankroll: ${:.2}", player.get_bankroll());
//!     println!("Net gain: ${:.2}", player.get_net_gain());
//!
//!     if player.is_active() {
//!         println!("Player is active");
//!     }
//!
//!     if player.is_all_in() {
//!         println!("Player is all-in");
//!     }
//!
//!     if player.is_folded() {
//!         println!("Player has folded");
//!     }
//! }
//! ```
//!
//! ### Betting Analysis
//!
//! ```rust
//! use poker_api::api::player_info::PlayerInfo;
//!
//! fn analyze_betting(player: &dyn PlayerInfo) {
//!     println!("Amount to call: ${:.2}", player.get_amount_to_call());
//!     println!("Amount in pot: ${:.2}", player.get_amount_in_pot());
//!     println!("Committed: {}", player.is_committed());
//!
//!     if player.has_enough_to_raise() {
//!         println!("Can raise: ${:.2}", player.get_amount_raiseable());
//!     }
//!
//!     println!("Last action: {}", player.get_last_action());
//! }
//! ```
//!
//! ### Hand Analysis
//!
//! ```rust
//! use poker_api::api::player_info::PlayerInfo;
//!
//! fn analyze_hand(player: &dyn PlayerInfo) {
//!     if let Some(hand) = player.get_revealed_hand() {
//!         println!("Revealed hand: {}", hand.to_string());
//!     }
//!
//!     if player.has_acted_this_round() {
//!         println!("Has acted this round");
//!     }
//!
//!     if player.is_button() {
//!         println!("Is the button");
//!     }
//! }
//! ```
//!
//! ## Design Decisions
//!
//! - **Read-Only Interface**: All methods take `&self` for safe state access
//! - **Comprehensive Coverage**: All player information needed for poker decisions
//! - **Financial Precision**: Uses f64 for accurate monetary calculations
//! - **State Consistency**: Integrates with GameInfo for complete picture
//! - **Action Integration**: Links to action system for complete player tracking
//!
//! ## Performance Characteristics
//!
//! - **Method Calls**: O(1) for state queries
//! - **Financial Calculations**: O(1) arithmetic operations
//! - **Memory**: Minimal additional overhead
//! - **Integration**: Requires coordination with game state

use crate::api::game_info::GameInfo;
use crate::api::hand::Hand;

pub trait PlayerInfo {
    // Player State
    fn get_net_gain(&self) -> f64;
    fn get_bankroll(&self) -> f64;
    fn get_bankroll_at_start_of_hand(&self) -> f64;
    fn get_bankroll_in_small_bets(&self) -> f64;
    fn in_game(&self) -> bool;
    fn get_seat(&self) -> usize;
    fn get_name(&self) -> String;

    // Hand State
    fn is_all_in(&self) -> bool;
    fn is_sitting_out(&self) -> bool;
    fn get_revealed_hand(&self) -> Option<Hand>;
    fn is_active(&self) -> bool;
    fn is_folded(&self) -> bool;
    fn is_button(&self) -> bool;

    // Pot and Betting
    fn get_amount_to_call(&self) -> f64;
    fn is_committed(&self) -> bool;
    fn has_acted_this_round(&self) -> bool;
    fn get_amount_in_pot(&self) -> f64;
    fn get_amount_in_pot_this_round(&self) -> f64;

    // Actions
    fn get_last_action(&self) -> u8; // Use constants like Holdem::FOLD, Holdem::CALL, etc.
    fn get_game_info(&self) -> &dyn GameInfo;

    // Raising and Calling
    fn has_enough_to_raise(&self) -> bool;
    fn get_amount_raiseable(&self) -> f64;
    fn get_amount_callable(&self) -> f64;
    fn get_bankroll_at_risk(&self) -> f64;
    fn get_raise_amount(&self, amount_to_raise: f64) -> f64;

    // Miscellaneous
    fn to_string(&self) -> String;
}
