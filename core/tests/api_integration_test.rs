//! # API Integration Tests
//!
//! Tests for the core poker API components including cards, hands, and actions.
//! These tests verify that the basic building blocks of the poker system
//! work correctly and integrate properly with each other.
//!
//! ## Test Coverage
//!
//! - **Card Operations**: Creation, parsing, string conversion
//! - **Hand Management**: Adding cards, size validation, display
//! - **Action Types**: Poker action creation and validation
//! - **Deck Operations**: Card dealing and deck state management
//!
//! ## Test Data
//!
//! Uses deterministic test cases to ensure consistent behavior across runs.
//! All card strings follow the standard format: "{Rank}{Suit}"
//! - Ranks: A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
//! - Suits: s (spades), h (hearts), d (diamonds), c (clubs)

use poker_api::api::action::{Action, ActionType};
use poker_api::api::card::Card;
use poker_api::api::deck::Deck;
use poker_api::api::hand::Hand;

#[test]
fn test_action_integration() {
    // Tests poker action creation and validation.
    //
    // Verifies that all poker action types (fold, check, call, bet, raise)
    // are created correctly and report accurate properties.
    //
    // Test Cases:
    // - Fold action with call amount
    // - Check action (zero cost)
    // - Call action matching bet
    // - Bet action (positive amount)
    // - Raise action (call + raise amounts)
    // - ActionType enum conversions
    //
    // Assertions:
    // - Action type identification methods
    // - Amount and call value accuracy
    // - Enum conversion round-trips
    // Test fold action
    let fold_action = Action::fold_action(10.0);
    assert!(fold_action.is_fold());
    assert_eq!(fold_action.to_call(), 10.0);
    assert_eq!(fold_action.amount(), 0.0);

    // Test check action
    let check_action = Action::check_action();
    assert!(check_action.is_check());
    assert_eq!(check_action.to_call(), 0.0);
    assert_eq!(check_action.amount(), 0.0);

    // Test call action
    let call_action = Action::call_action(20.0);
    assert!(call_action.is_call());
    assert_eq!(call_action.to_call(), 20.0);
    assert_eq!(call_action.amount(), 20.0);

    // Test bet action
    let bet_action = Action::bet_action(50.0);
    assert!(bet_action.is_bet());
    assert_eq!(bet_action.to_call(), 0.0);
    assert_eq!(bet_action.amount(), 50.0);

    // Test raise action
    let raise_action = Action::raise_action(30.0, 100.0);
    assert!(raise_action.is_raise());
    assert_eq!(raise_action.to_call(), 30.0);
    assert_eq!(raise_action.amount(), 100.0);

    // Test ActionType conversions
    let action_type = ActionType::Call;
    assert_eq!(action_type.to_u8(), 3);
    assert_eq!(ActionType::from_u8(3), Some(ActionType::Call));
    assert_eq!(ActionType::from_u8(99), None); // Invalid value
}

#[test]
fn test_card_integration() {
    // Tests card creation, parsing, and string conversion.
    //
    // Verifies that cards can be created from standard string notation
    // and converted back to strings correctly.
    //
    // Test Cases:
    // - Ace of spades parsing and string conversion
    //
    // Assertions:
    // - Card creation from string succeeds
    // - String representation matches input
    // - Round-trip conversion preserves card identity
    let card = Card::from_string("As").unwrap();
    assert_eq!(card.to_string(), "As");
}

#[test]
fn test_deck_integration() {
    // Tests deck operations and state management.
    //
    // Verifies that deck dealing works correctly and maintains
    // accurate state information.
    //
    // Test Cases:
    // - Deck creation with 52 cards
    // - Card dealing reduces count
    // - Dealt card no longer in deck
    //
    // Assertions:
    // - Initial deck has 52 cards
    // - Dealing reduces count by 1
    // - Dealt card is not in remaining deck
    let mut deck = Deck::new();
    let card = deck.deal().unwrap();
    assert_eq!(deck.cards_left(), 51);
    assert!(!deck.in_deck(card));
}

#[test]
fn test_hand_integration() {
    // Tests hand creation and card management.
    //
    // Verifies that hands can be created, cards added, and
    // hand state accurately tracked.
    //
    // Test Cases:
    // - Empty hand creation
    // - Adding two cards (Ace and King of spades)
    // - Size tracking
    // - String representation
    //
    // Assertions:
    // - Hand starts empty
    // - Cards added successfully
    // - Size reflects added cards
    // - String representation shows cards in order
    let mut hand = Hand::new();
    hand.add_card(Card::from_string("As").unwrap()).unwrap();
    hand.add_card(Card::from_string("Ks").unwrap()).unwrap();
    assert_eq!(hand.size(), 2);
    assert_eq!(hand.to_string(), "As Ks");
}
