//! Property-based tests for poker data structures
//!
//! These tests verify fundamental properties of poker structures like cards,
//! hands, and decks through comprehensive testing of edge cases and invariants.

use holdem_core::{card::Card, deck::Deck, hand::Hand};
use std::collections::HashSet;
use std::str::FromStr;

#[cfg(test)]
mod card_properties {
    use super::*;

    #[test]
    fn test_card_properties_comprehensive() {
        // Test all possible card combinations for key properties
        for rank in 0..=12 {
            for suit in 0..=3 {
                let card = Card::new(rank, suit).unwrap();

                // Property: Valid cards are always within valid rank/suit bounds
                assert!(card.rank() <= 12);
                assert!(card.suit() <= 3);

                // Property: Card creation is deterministic (same inputs = same card)
                let card2 = Card::new(rank, suit).unwrap();
                assert_eq!(card, card2);
                assert_eq!(card.rank(), rank);
                assert_eq!(card.suit(), suit);

                // Property: String parsing round-trips correctly
                let string_repr = format!("{}", card);
                let parsed = Card::from_str(&string_repr).unwrap();
                assert_eq!(card, parsed);

                // Property: Card serialization round-trips correctly
                let json = serde_json::to_string(&card).unwrap();
                let deserialized: Card = serde_json::from_str(&json).unwrap();
                assert_eq!(card, deserialized);

                let toml = toml::to_string(&card).unwrap();
                let deserialized: Card = toml::from_str(&toml).unwrap();
                assert_eq!(card, deserialized);
            }
        }
    }

    #[test]
    fn test_card_ordering_properties() {
        // Test card ordering with various combinations
        let test_cases = vec![
            ((12, 3), (11, 3)), // Ace > King, same suit
            ((12, 3), (12, 0)), // Ace spades > Ace hearts (Spades=4 > Hearts=3)
            ((12, 0), (12, 1)), // Ace hearts > Ace diamonds (Hearts=3 > Diamonds=2)
            ((12, 1), (12, 2)), // Ace diamonds > Ace clubs (Diamonds=2 > Clubs=1)
            ((5, 3), (5, 0)),   // Five spades > Five hearts
            ((5, 0), (5, 1)),   // Five hearts > Five diamonds
            ((5, 1), (5, 2)),   // Five diamonds > Five clubs
        ];

        for ((rank1, suit1), (rank2, suit2)) in test_cases {
            let card1 = Card::new(rank1, suit1).unwrap();
            let card2 = Card::new(rank2, suit2).unwrap();

            // Property: Cards are properly ordered (higher rank first, then higher suit)
            if rank1 != rank2 {
                if rank1 > rank2 {
                    assert!(
                        card1 > card2,
                        "Card {:?} should be > Card {:?}",
                        card1,
                        card2
                    );
                } else {
                    assert!(
                        card1 < card2,
                        "Card {:?} should be < Card {:?}",
                        card1,
                        card2
                    );
                }
            } else {
                // If ranks are equal, higher suit order should be greater
                // Suit order: Spades(4) > Hearts(3) > Diamonds(2) > Clubs(1)
                let suit_order1 = match suit1 {
                    3 => 4, // Spades
                    0 => 3, // Hearts
                    1 => 2, // Diamonds
                    2 => 1, // Clubs
                    _ => 0,
                };
                let suit_order2 = match suit2 {
                    3 => 4, // Spades
                    0 => 3, // Hearts
                    1 => 2, // Diamonds
                    2 => 1, // Clubs
                    _ => 0,
                };

                if suit_order1 > suit_order2 {
                    assert!(
                        card1 > card2,
                        "Card {:?} should be > Card {:?}",
                        card1,
                        card2
                    );
                } else {
                    assert!(
                        card1 < card2,
                        "Card {:?} should be < Card {:?}",
                        card1,
                        card2
                    );
                }
            }
        }
    }

    #[test]
    fn test_all_cards_unique_property() {
        let mut seen = HashSet::new();

        // Property: All 52 possible cards are unique
        for rank in 0..=12 {
            for suit in 0..=3 {
                let card = Card::new(rank, suit).unwrap();
                assert!(seen.insert(card), "Card {} was not unique", card);
            }
        }

        assert_eq!(seen.len(), 52, "Should have exactly 52 unique cards");
    }
}

#[cfg(test)]
mod hand_properties {
    use super::*;

    #[test]
    fn test_hand_properties_comprehensive() {
        // Test hands of various sizes for key properties
        for size in 0..=7 {
            if size < 2 {
                continue; // Skip very small hands for some tests
            }

            // Generate distinct cards for this hand size
            let mut cards = Vec::new();
            let mut card_set = HashSet::new();

            for i in 0..size {
                let rank = (i % 13) as u8;
                let suit = (i % 4) as u8;
                let card = Card::new(rank, suit).unwrap();

                // Ensure uniqueness
                if card_set.insert(card) {
                    cards.push(card);
                }

                if cards.len() == size {
                    break;
                }
            }

            if cards.len() == size {
                let hand = Hand::new(cards.clone()).unwrap();

                // Property: Hands never contain duplicate cards
                let hand_cards = hand.cards();
                for i in 0..hand_cards.len() {
                    for j in (i + 1)..hand_cards.len() {
                        assert_ne!(hand_cards[i], hand_cards[j]);
                    }
                }

                // Property: Hands are always sorted correctly (rank descending)
                for i in 1..hand_cards.len() {
                    assert!(hand_cards[i].rank() <= hand_cards[i - 1].rank());
                }

                // Property: All cards in hand are valid cards
                for card in hand.iter() {
                    assert!(card.rank() <= 12);
                    assert!(card.suit() <= 3);
                }

                // Property: Hand serialization round-trips correctly
                let json = serde_json::to_string(&hand).unwrap();
                let deserialized: Hand = serde_json::from_str(&json).unwrap();
                assert_eq!(hand, deserialized);

                let toml = toml::to_string(&hand).unwrap();
                let deserialized: Hand = toml::from_str(&toml).unwrap();
                assert_eq!(hand, deserialized);

                // Property: Hand length matches actual card count
                assert_eq!(hand.len, cards.len());
                assert_eq!(hand_cards.len(), cards.len());

                // Property: Best five cards returns exactly 5 cards (if hand large enough)
                if hand.len >= 5 {
                    let best_five = hand.best_five_cards();
                    assert_eq!(best_five.len(), 5);

                    // Property: Best five cards are subset of hand cards
                    for &card in &best_five {
                        assert!(hand_cards.contains(&card));
                    }
                }
            }
        }
    }

    #[test]
    fn test_hand_length_properties() {
        // Property: Hand length is always <= 7
        for size in 0..=10 {
            let mut cards = Vec::new();
            for i in 0..size {
                let rank = (i % 13) as u8;
                let suit = (i % 4) as u8;
                if let Ok(card) = Card::new(rank, suit) {
                    cards.push(card);
                }
            }

            if size <= 7 {
                let hand = Hand::new(cards);
                assert!(hand.is_ok(), "Hand of size {} should be valid", size);
            } else {
                let hand = Hand::new(cards);
                assert!(hand.is_err(), "Hand of size {} should be invalid", size);
            }
        }
    }

    #[test]
    fn test_hand_display_properties() {
        // Test various hand sizes for display format consistency
        for size in 0..=7 {
            let mut cards = Vec::new();
            for i in 0..size {
                let rank = (i % 13) as u8;
                let suit = (i % 4) as u8;
                if let Ok(card) = Card::new(rank, suit) {
                    cards.push(card);
                }
            }

            if let Ok(hand) = Hand::new(cards) {
                let display = format!("{}", hand);

                // Property: Hand display format is consistent
                assert!(display.starts_with("Hand("));
                assert!(display.ends_with(")"));
                assert!(display.contains("cards:"));
            }
        }
    }
}

#[cfg(test)]
mod deck_properties {
    use super::*;

    #[test]
    fn test_deck_properties_comprehensive() {
        let deck = Deck::new();

        // Property: Fresh decks always contain exactly 52 cards
        assert_eq!(deck.remaining(), 52);
        assert!(!deck.is_empty());

        // Property: All cards in deck are unique
        let cards = deck.cards();
        let mut seen = HashSet::new();
        for &card in cards {
            assert!(seen.insert(card), "Duplicate card in deck: {}", card);
        }
        assert_eq!(seen.len(), 52);

        // Property: All cards in deck are valid cards
        for &card in cards {
            assert!(card.rank() <= 12);
            assert!(card.suit() <= 3);
        }

        // Property: Deck serialization round-trips correctly
        let json = serde_json::to_string(&deck).unwrap();
        let deserialized: Deck = serde_json::from_str(&json).unwrap();
        assert_eq!(deck.cards(), deserialized.cards());

        let toml = toml::to_string(&deck).unwrap();
        let deserialized: Deck = toml::from_str(&toml).unwrap();
        assert_eq!(deck.cards(), deserialized.cards());
    }

    #[test]
    fn test_deck_dealing_properties() {
        let mut deck = Deck::new();
        let initial_remaining = deck.remaining();

        // Property: Dealing cards reduces remaining count
        if initial_remaining > 0 {
            let dealt = deck.deal(1);
            assert_eq!(dealt.len(), 1);
            assert_eq!(deck.remaining(), initial_remaining - 1);
        }

        // Property: Dealing more cards than available returns empty
        let mut empty_deck = Deck::new();
        while empty_deck.remaining() > 0 {
            empty_deck.deal(1);
        }

        assert_eq!(empty_deck.remaining(), 0);
        let dealt_none = empty_deck.deal(1);
        assert!(dealt_none.is_empty());
    }
}

#[cfg(test)]
mod integration_properties {
    use super::*;

    #[test]
    fn test_hand_from_hole_cards_and_board_properties() {
        // Test various combinations of hole cards and board cards
        for num_hole in 2..=2 {
            for num_board in 0..=5 {
                let mut hole_cards = Vec::new();
                let mut board_cards = Vec::new();
                let mut all_cards_set = HashSet::new();

                // Generate unique hole cards
                for i in 0..num_hole {
                    let mut attempts = 0;
                    loop {
                        let rank = (i % 13) as u8;
                        let suit = (i % 4) as u8;
                        let card = Card::new(rank, suit).unwrap();
                        if all_cards_set.insert(card) {
                            hole_cards.push(card);
                            break;
                        }
                        attempts += 1;
                        if attempts > 50 {
                            break;
                        }
                    }
                }

                // Generate unique board cards
                for i in 0..num_board {
                    let mut attempts = 0;
                    loop {
                        let rank = ((i + 2) % 13) as u8;
                        let suit = ((i + 1) % 4) as u8;
                        let card = Card::new(rank, suit).unwrap();
                        if all_cards_set.insert(card) {
                            board_cards.push(card);
                            break;
                        }
                        attempts += 1;
                        if attempts > 50 {
                            break;
                        }
                    }
                }

                if hole_cards.len() == num_hole && board_cards.len() == num_board {
                    use holdem_core::{board::Board, hole_cards::HoleCards};

                    let hole_cards_struct = HoleCards::new(hole_cards[0], hole_cards[1]).unwrap();
                    let mut board = Board::new();

                    if board_cards.len() >= 3 {
                        // For testing purposes, just add the first 3 cards as flop
                        board
                            .deal_flop(board_cards.iter().take(3).cloned().collect())
                            .unwrap();
                    }

                    // Property: Hand creation from hole cards and board never exceeds 7 cards
                    let hand = Hand::from_hole_cards_and_board(&hole_cards_struct, &board);
                    assert!(hand.is_ok());

                    let hand = hand.unwrap();
                    assert!(hand.len <= 7);

                    // The hand should contain hole cards + any successfully added board cards
                    let expected_min_len = num_hole; // At least hole cards
                    let expected_max_len = num_hole + num_board; // At most hole cards + all board cards
                    assert!(hand.len >= expected_min_len);
                    assert!(hand.len <= expected_max_len);
                }
            }
        }
    }

    #[test]
    fn test_hand_notation_parsing_properties() {
        // Test hand notation parsing with various valid combinations
        for size in 2..=7 {
            let mut cards = Vec::new();
            let mut card_set = HashSet::new();

            for i in 0..size {
                let mut attempts = 0;
                loop {
                    let rank = (i % 13) as u8;
                    let suit = (i % 4) as u8;
                    let card = Card::new(rank, suit).unwrap();
                    if card_set.insert(card) {
                        cards.push(card);
                        break;
                    }
                    attempts += 1;
                    if attempts > 50 {
                        break;
                    }
                }
            }

            if cards.len() == size {
                let mut notation_parts = Vec::new();
                for card in &cards {
                    notation_parts.push(format!("{}", card));
                }
                let notation = notation_parts.join(" ");

                // Property: Hand notation parsing works for valid hands
                let parsed = Hand::from_notation(&notation);
                if size <= 7 {
                    assert!(parsed.is_ok(), "Failed to parse notation: {}", notation);
                    let parsed_hand = parsed.unwrap();
                    assert_eq!(parsed_hand.len, cards.len());
                } else {
                    assert!(
                        parsed.is_err(),
                        "Should fail to parse hand with {} cards",
                        size
                    );
                }
            }
        }
    }

    #[test]
    fn test_hand_strength_properties() {
        // Test that hand strength is deterministic
        for size in 5..=7 {
            let mut cards = Vec::new();
            let mut card_set = HashSet::new();

            for i in 0..size {
                let mut attempts = 0;
                loop {
                    let rank = (i % 13) as u8;
                    let suit = (i % 4) as u8;
                    let card = Card::new(rank, suit).unwrap();
                    if card_set.insert(card) {
                        cards.push(card);
                        break;
                    }
                    attempts += 1;
                    if attempts > 50 {
                        break;
                    }
                }
            }

            if cards.len() == size {
                // Property: Hand strength is deterministic (same hand = same strength)
                let hand1 = Hand::new(cards.clone()).unwrap();
                let hand2 = Hand::new(cards).unwrap();

                // For now, strength is always 0 (placeholder), but should be consistent
                assert_eq!(hand1.strength(), hand2.strength());
            }
        }
    }
}
