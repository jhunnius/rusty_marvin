use poker_api::deck::Deck;

#[test]
fn test_deck_integration() {
    let mut deck = Deck::new();
    let card = deck.deal().unwrap();
    assert_eq!(deck.cards_left(), 51);
    assert!(!deck.in_deck(card));
}
