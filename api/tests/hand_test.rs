use poker_api::card::Card;
use poker_api::hand::Hand;

#[test]
fn test_hand_integration() {
    let mut hand = Hand::new();
    hand.add_card(Card::from_string("As").unwrap()).unwrap();
    hand.add_card(Card::from_string("Ks").unwrap()).unwrap();
    assert_eq!(hand.size(), 2);
    assert_eq!(hand.to_string(), "As Ks");
}
