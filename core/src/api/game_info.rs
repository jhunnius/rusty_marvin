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
