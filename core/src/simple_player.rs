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
