//! # Action Module
//!
//! This module provides types and functionality for representing poker actions taken by players
//! during a hand. The module includes both the `ActionType` enum for categorizing different
//! types of actions and the `Action` struct for representing specific action instances.
//!
//! ## Action Types
//!
//! The module supports all standard poker actions:
//! - **Fold**: Player discards their hand and forfeits the pot
//! - **Check**: Player passes without betting (when no bet is pending)
//! - **Call**: Player matches the current bet or raise
//! - **Bet**: Player makes an initial wager
//! - **Raise**: Player increases the current bet
//! - **Blind Actions**: Small blind, big blind, and other forced bets
//! - **Special Actions**: All-in, muck, sit out, etc.
//!
//! ## Action Structure
//!
//! Each `Action` contains:
//! - `action_type`: The type of action being taken
//! - `to_call`: Amount required to call (if applicable)
//! - `amount`: The amount being bet/raised (if applicable)
//!
//! ## Key Features
//!
//! - **Type Safety**: Strongly typed action representation
//! - **Factory Methods**: Convenient constructors for common actions
//! - **Validation**: Built-in action property checking
//! - **Formatting**: Human-readable string representations
//! - **Memory Efficient**: Compact representation using enums
//!
//! ## Examples
//!
//! ### Creating Basic Actions
//!
//! ```rust
//! use poker_api::api::action::{Action, ActionType};
//!
//! // Create a fold action
//! let fold = Action::fold_action(50.0);
//! assert!(fold.is_fold());
//!
//! // Create a bet action
//! let bet = Action::bet_action(100.0);
//! assert!(bet.is_bet());
//! assert_eq!(bet.amount(), 100.0);
//! ```
//!
//! ### Action Properties
//!
//! ```rust
//! use poker_api::api::action::{Action, ActionType};
//!
//! let raise = Action::raise_action(50.0, 200.0);
//!
//! println!("Action type: {:?}", raise.action_type());
//! println!("Amount to call: {}", raise.to_call());
//! println!("Raise amount: {}", raise.amount());
//! println!("Is raise: {}", raise.is_raise());
//! println!("Is voluntary: {}", raise.is_voluntary());
//! println!("String: {}", raise.to_string());
//! ```
//!
//! ### Blind Actions
//!
//! ```rust
//! use poker_api::api::action::Action;
//!
//! // Post blinds
//! let small_blind = Action::small_blind_action(10.0);
//! let big_blind = Action::big_blind_action(20.0);
//!
//! // Post ante
//! let ante = Action::post_ante_action(5.0);
//! ```
//!
//! ## Design Decisions
//!
//! - **Enum Representation**: Uses `repr(u8)` for memory efficiency and serialization
//! - **Floating Point Amounts**: Uses `f64` for precise monetary calculations
//! - **Immutable Actions**: Actions are immutable once created
//! - **Comprehensive Coverage**: Supports all common poker action types
//! - **Factory Pattern**: Provides convenient constructors for common cases
//!
//! ## Performance Characteristics
//!
//! - **Memory**: 17 bytes per action (1 + 8 + 8)
//! - **Comparisons**: O(1) enum comparisons
//! - **Formatting**: O(1) string formatting
//! - **Validation**: O(1) property checks

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    Invalid = 0,
    Fold = 1,
    Check = 2,
    Call = 3,
    Bet = 4,
    Raise = 5,
    SmallBlind = 6,
    BigBlind = 7,
    PostBlind = 8,
    AllInPass = 9,
    Muck = 10,
    PostAnte = 11,
    SitOut = 12,
    PostDeadBlind = 13,
}

impl ActionType {
    // Convert from u8 to ActionType
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ActionType::Invalid),
            1 => Some(ActionType::Fold),
            2 => Some(ActionType::Check),
            3 => Some(ActionType::Call),
            4 => Some(ActionType::Bet),
            5 => Some(ActionType::Raise),
            6 => Some(ActionType::SmallBlind),
            7 => Some(ActionType::BigBlind),
            8 => Some(ActionType::PostBlind),
            9 => Some(ActionType::AllInPass),
            10 => Some(ActionType::Muck),
            11 => Some(ActionType::PostAnte),
            12 => Some(ActionType::SitOut),
            13 => Some(ActionType::PostDeadBlind),
            _ => None,
        }
    }

    // Convert from ActionType to u8
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Action {
    action_type: ActionType,
    to_call: f64, // Amount required to call
    amount: f64,  // Amount of the action (e.g., bet or raise amount)
}

impl Action {
    // Constants for action types
    pub const INVALID: ActionType = ActionType::Invalid;
    pub const FOLD: ActionType = ActionType::Fold;
    pub const CHECK: ActionType = ActionType::Check;
    pub const CALL: ActionType = ActionType::Call;
    pub const BET: ActionType = ActionType::Bet;
    pub const RAISE: ActionType = ActionType::Raise;
    pub const SMALL_BLIND: ActionType = ActionType::SmallBlind;
    pub const BIG_BLIND: ActionType = ActionType::BigBlind;
    pub const POST_BLIND: ActionType = ActionType::PostBlind;
    pub const ALLIN_PASS: ActionType = ActionType::AllInPass;
    pub const MUCK: ActionType = ActionType::Muck;
    pub const POST_ANTE: ActionType = ActionType::PostAnte;
    pub const SIT_OUT: ActionType = ActionType::SitOut;
    pub const POST_DEAD_BLIND: ActionType = ActionType::PostDeadBlind;

    // Constructor
    pub fn new(action_type: ActionType, to_call: f64, amount: f64) -> Self {
        Self {
            action_type,
            to_call,
            amount,
        }
    }

    // Factory methods for creating actions
    pub fn fold_action(to_call: f64) -> Self {
        Self::new(ActionType::Fold, to_call, 0.0)
    }

    pub fn check_action() -> Self {
        Self::new(ActionType::Check, 0.0, 0.0)
    }

    pub fn call_action(to_call: f64) -> Self {
        Self::new(ActionType::Call, to_call, to_call)
    }

    pub fn bet_action(amount: f64) -> Self {
        Self::new(ActionType::Bet, 0.0, amount)
    }

    pub fn raise_action(to_call: f64, amount: f64) -> Self {
        Self::new(ActionType::Raise, to_call, amount)
    }

    pub fn small_blind_action(to_post: f64) -> Self {
        Self::new(ActionType::SmallBlind, 0.0, to_post)
    }

    pub fn big_blind_action(to_post: f64) -> Self {
        Self::new(ActionType::BigBlind, 0.0, to_post)
    }

    pub fn post_blind_action(to_post: f64) -> Self {
        Self::new(ActionType::PostBlind, 0.0, to_post)
    }

    pub fn all_in_pass_action() -> Self {
        Self::new(ActionType::AllInPass, 0.0, 0.0)
    }

    pub fn muck_action() -> Self {
        Self::new(ActionType::Muck, 0.0, 0.0)
    }

    pub fn post_ante_action(to_post: f64) -> Self {
        Self::new(ActionType::PostAnte, 0.0, to_post)
    }

    pub fn sit_out_action() -> Self {
        Self::new(ActionType::SitOut, 0.0, 0.0)
    }

    pub fn post_dead_blind_action(to_post: f64) -> Self {
        Self::new(ActionType::PostDeadBlind, 0.0, to_post)
    }

    // Getters
    pub fn action_type(&self) -> ActionType {
        self.action_type
    }

    pub fn to_call(&self) -> f64 {
        self.to_call
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    // Check action properties
    pub fn is_fold(&self) -> bool {
        self.action_type == ActionType::Fold
    }

    pub fn is_check(&self) -> bool {
        self.action_type == ActionType::Check
    }

    pub fn is_call(&self) -> bool {
        self.action_type == ActionType::Call
    }

    pub fn is_bet(&self) -> bool {
        self.action_type == ActionType::Bet
    }

    pub fn is_raise(&self) -> bool {
        self.action_type == ActionType::Raise
    }

    pub fn is_voluntary(&self) -> bool {
        match self.action_type {
            ActionType::Fold
            | ActionType::Check
            | ActionType::Call
            | ActionType::Bet
            | ActionType::Raise => true,
            _ => false,
        }
    }

    // Formatting
    pub fn to_string(&self) -> String {
        match self.action_type {
            ActionType::Fold => "Fold".to_string(),
            ActionType::Check => "Check".to_string(),
            ActionType::Call => format!("Call {}", self.to_call),
            ActionType::Bet => format!("Bet {}", self.amount),
            ActionType::Raise => format!("Raise {}", self.amount),
            _ => "Invalid Action".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_action() {
        let action = Action::fold_action(10.0);
        assert!(action.is_fold());
        assert_eq!(action.to_call(), 10.0);
        assert_eq!(action.amount(), 0.0);
    }

    #[test]
    fn test_check_action() {
        let action = Action::check_action();
        assert!(action.is_check());
        assert_eq!(action.to_call(), 0.0);
        assert_eq!(action.amount(), 0.0);
    }

    #[test]
    fn test_call_action() {
        let action = Action::call_action(20.0);
        assert!(action.is_call());
        assert_eq!(action.to_call(), 20.0);
        assert_eq!(action.amount(), 20.0);
    }

    #[test]
    fn test_bet_action() {
        let action = Action::bet_action(50.0);
        assert!(action.is_bet());
        assert_eq!(action.to_call(), 0.0);
        assert_eq!(action.amount(), 50.0);
    }

    #[test]
    fn test_raise_action() {
        let action = Action::raise_action(30.0, 100.0);
        assert!(action.is_raise());
        assert_eq!(action.to_call(), 30.0);
        assert_eq!(action.amount(), 100.0);
    }
}
