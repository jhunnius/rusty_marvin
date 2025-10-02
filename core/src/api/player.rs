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
