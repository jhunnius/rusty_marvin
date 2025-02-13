use poker_api::card::Card;

#[test]
fn test_card_integration() {
    let card = Card::from_string("As").unwrap();
    assert_eq!(card.to_string(), "As");
}

