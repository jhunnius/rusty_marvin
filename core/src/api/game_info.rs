//! # Game Info Module
//!
//! This module defines the `GameInfo` trait that provides comprehensive access to the
//! current state of a poker game. This trait serves as the primary interface for bots
//! and observers to understand game conditions, player states, and betting information.
//!
//! ## Game Info Trait Overview
//!
//! The `GameInfo` trait provides read-only access to all aspects of a poker game:
//! - **Game State**: Current betting round, game stage, and status
//! - **Betting Information**: Blinds, bets, raises, and pot sizes
//! - **Player Information**: Active players, bankrolls, and positions
//! - **Table State**: Board cards, button position, and seating
//! - **Game Rules**: Limit types, special modes, and configurations
//!
//! ## Key Information Categories
//!
//! ### Game State
//! - Current betting stage (preflop, flop, turn, river)
//! - Game completion status and winner information
//! - Round counts and raise counts
//!
//! ### Betting Information
//! - Blind sizes and current bet amounts
//! - Pot sizes (main pot and side pots)
//! - Minimum raise amounts and call requirements
//!
//! ### Player Information
//! - Player counts and seating positions
//! - Bankroll information and amounts at risk
//! - Player states (active, committed, all-in)
//!
//! ### Table Information
//! - Community cards and board state
//! - Button, blinds, and current player positions
//! - Eligible pot amounts for each player
//!
//! ## Implementation Requirements
//!
//! Any struct implementing `GameInfo` must provide:
//! - Accurate game state information
//! - Up-to-date betting and pot information
//! - Current player states and positions
//! - Proper handling of edge cases (empty pots, all-in players, etc.)
//!
//! ## Examples
//!
//! ### Basic Game State Queries
//!
//! ```rust
//! use poker_api::api::game_info::GameInfo;
//!
//! fn analyze_game_state(info: &dyn GameInfo) {
//!     println!("Game stage: {}", info.get_stage());
//!     println!("Is preflop: {}", info.is_pre_flop());
//!     println!("Is game over: {}", info.is_game_over());
//!
//!     if info.is_post_flop() {
//!         println!("We're past the flop");
//!     }
//! }
//! ```
//!
//! ### Betting Information
//!
//! ```rust
//! use poker_api::api::game_info::GameInfo;
//!
//! fn analyze_betting(info: &dyn GameInfo, my_seat: usize) {
//!     println!("Total pot: ${}", info.get_total_pot_size());
//!     println!("Main pot: ${}", info.get_main_pot_size());
//!     println!("Side pots: {}", info.get_num_side_pots());
//!
//!     if info.can_raise(my_seat) {
//!         println!("I can raise");
//!         println!("Min raise: ${}", info.get_min_raise());
//!     }
//!
//!     println!("Amount to call: ${}", info.get_amount_to_call(my_seat));
//! }
//! ```
//!
//! ### Player Analysis
//!
//! ```rust
//! use poker_api::api::game_info::GameInfo;
//!
//! fn analyze_players(info: &dyn GameInfo) {
//!     println!("Total players: {}", info.get_num_players());
//!     println!("Active players: {}", info.get_num_active_players());
//!     println!("All-in players: {}", info.get_num_all_in_players());
//!
//!     let button_seat = info.get_button_seat();
//!     let small_blind_seat = info.get_small_blind_seat();
//!     let big_blind_seat = info.get_big_blind_seat();
//!
//!     println!("Button at seat: {}", button_seat);
//! }
//! ```
//!
//! ## Design Decisions
//!
//! - **Read-Only Interface**: All methods take `&self` for safe state access
//! - **Comprehensive Coverage**: Includes all information needed for poker decisions
//! - **Trait-Based Design**: Enables different game implementations
//! - **Numeric Types**: Uses appropriate types for money (f64) and counts (usize)
//! - **Error Handling**: Returns Options for fallible lookups
//!
//! ## Performance Characteristics
//!
//! - **Method Calls**: O(1) for most state queries
//! - **Player Lookups**: O(n) where n is number of players
//! - **Memory**: No additional allocations required
//! - **Thread Safety**: Read-only access supports concurrent use

use crate::api::hand::Hand;
use crate::api::player_info::PlayerInfo;

pub trait GameInfo {
    // Game State
    fn get_stage(&self) -> u8;
    fn is_pre_flop(&self) -> bool;
    fn is_post_flop(&self) -> bool;
    fn is_flop(&self) -> bool;
    fn is_turn(&self) -> bool;
    fn is_river(&self) -> bool;
    fn is_game_over(&self) -> bool;

    // Blinds and Antes
    fn get_ante(&self) -> f64;
    fn get_small_blind_size(&self) -> f64;
    fn get_big_blind_size(&self) -> f64;
    fn get_current_bet_size(&self) -> f64;
    fn get_min_raise(&self) -> f64;

    // Pot Information
    fn get_total_pot_size(&self) -> f64;
    fn get_main_pot_size(&self) -> f64;
    fn get_num_side_pots(&self) -> usize;
    fn get_side_pot_size(&self, index: usize) -> f64;
    fn get_rake(&self) -> f64;

    // Player Information
    fn get_num_players(&self) -> usize;
    fn get_num_active_players(&self) -> usize;
    fn get_num_active_players_not_all_in(&self) -> usize;
    fn get_num_all_in_players(&self) -> usize;
    fn get_num_to_act(&self) -> usize;
    fn get_unacted(&self) -> usize;
    fn get_num_seats(&self) -> usize;

    // Player Actions
    fn can_raise(&self, seat: usize) -> bool;
    fn get_amount_to_call(&self, seat: usize) -> f64;
    fn get_bets_to_call(&self, seat: usize) -> f64;
    fn get_bankroll(&self, seat: usize) -> f64;
    fn get_bankroll_at_risk(&self, seat: usize) -> f64;

    // Player State
    fn in_game(&self, seat: usize) -> bool;
    fn is_active(&self, seat: usize) -> bool;
    fn is_committed(&self, seat: usize) -> bool;

    // Table Information
    fn get_button_seat(&self) -> usize;
    fn get_small_blind_seat(&self) -> usize;
    fn get_big_blind_seat(&self) -> usize;
    fn get_current_player_seat(&self) -> usize;

    // Board and Hand Information
    fn get_board(&self) -> Hand;
    fn get_eligible_pot(&self, seat: usize) -> f64;

    // Game Rules
    fn is_no_limit(&self) -> bool;
    fn is_fixed_limit(&self) -> bool;
    fn is_pot_limit(&self) -> bool;
    fn is_simulation(&self) -> bool;
    fn is_zip_mode(&self) -> bool;
    fn is_reverse_blinds(&self) -> bool;

    // Miscellaneous
    fn get_game_id(&self) -> u64;
    fn get_log_directory(&self) -> String;
    fn get_player(&self, seat: usize) -> Option<Box<dyn PlayerInfo>>;
    fn get_player_by_name(&self, name: &str) -> Option<Box<dyn PlayerInfo>>;
    fn get_player_name(&self, seat: usize) -> Option<String>;
    fn get_player_seat(&self, name: &str) -> Option<usize>;
    fn get_players_in_pot(&self, amount_in: f64) -> Vec<usize>;
    fn get_num_winners(&self) -> usize;
    fn get_num_raises(&self) -> usize;
}
