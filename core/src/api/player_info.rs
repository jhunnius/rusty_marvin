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
