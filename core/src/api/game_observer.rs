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
