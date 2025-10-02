use crate::api::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    pub(crate) cards: [Option<Card>; 7], // Up to 7 cards
    size: usize,                         // Number of cards in the hand
}

impl Hand {
    pub const MAX_CARDS: usize = 7;

    // Create a new empty hand
    pub fn new() -> Self {
        Self {
            cards: [None; Self::MAX_CARDS],
            size: 0,
        }
    }

    // Create a hand from a string representation (e.g., "As Ks Qs")
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        let mut hand = Self::new();
        for card_str in s.split_whitespace() {
            let card = Card::from_string(card_str)?;
            hand.add_card(card)?;
        }
        Ok(hand)
    }

    // Add a card to the hand
    pub fn add_card(&mut self, card: Card) -> Result<(), &'static str> {
        if self.size >= Self::MAX_CARDS {
            return Err("Hand is full");
        }
        self.cards[self.size] = Some(card);
        self.size += 1;
        Ok(())
    }

    // Remove the last card in the hand
    pub fn remove_card(&mut self) {
        if self.size > 0 {
            self.size -= 1;
            self.cards[self.size] = None;
        }
    }

    // Get the card at a specific position (1-based indexing)
    pub fn get_card(&self, pos: usize) -> Option<Card> {
        if pos == 0 || pos > self.size {
            return None;
        }
        self.cards[pos - 1]
    }

    // Get the card index at a specific position (1-based indexing)
    pub fn get_card_index(&self, pos: usize) -> Option<u8> {
        self.get_card(pos).map(|card| card.index())
    }

    // Get the last card in the hand
    pub fn get_last_card(&self) -> Option<Card> {
        if self.size == 0 {
            None
        } else {
            self.cards[self.size - 1]
        }
    }

    // Get the last card index in the hand
    pub fn get_last_card_index(&self) -> Option<u8> {
        self.get_last_card().map(|card| card.index())
    }

    // Clear the hand (remove all cards)
    pub fn clear(&mut self) {
        self.cards = [None; Self::MAX_CARDS];
        self.size = 0;
    }

    // Sort the hand in descending order by card index
    pub fn sort(&mut self) {
        self.cards[..self.size].sort_by(|a, b| {
            let a_index = a.map(|card| card.index()).unwrap_or(0);
            let b_index = b.map(|card| card.index()).unwrap_or(0);
            b_index.cmp(&a_index) // Sort in descending order
        });
    }

    // Check if the hand contains a specific card
    pub fn contains(&self, card: Card) -> bool {
        self.cards[..self.size].contains(&Some(card))
    }

    // Get a string representation of the hand (e.g., "As Ks Qs")
    pub fn to_string(&self) -> String {
        self.cards[..self.size]
            .iter()
            .filter_map(|&card| card.map(|c| c.to_string()))
            .collect::<Vec<String>>()
            .join(" ")
    }

    // Get a string representation for flashing purposes
    pub fn flashing_string(&self) -> String {
        self.to_string()
    }

    // Get the size of the hand
    pub fn size(&self) -> usize {
        self.size
    }

    // Get the array of card indexes (for LUT-based evaluators)
    pub fn get_card_array(&self) -> Vec<u8> {
        self.cards[..self.size]
            .iter()
            .filter_map(|&card| card.map(|c| c.index()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::card::Card;

    #[test]
    fn test_hand_creation() {
        let hand = Hand::new();
        assert_eq!(hand.size(), 0);
    }

    #[test]
    fn test_add_card() {
        let mut hand = Hand::new();
        let card = Card::from_string("As").unwrap();
        hand.add_card(card).unwrap();
        assert_eq!(hand.size(), 1);
        assert_eq!(hand.get_card(1), Some(card));
    }

    #[test]
    fn test_remove_card() {
        let mut hand = Hand::new();
        let card = Card::from_string("As").unwrap();
        hand.add_card(card).unwrap();
        hand.remove_card();
        assert_eq!(hand.size(), 0);
    }

    #[test]
    fn test_hand_from_string() {
        let hand = Hand::from_string("As Ks Qs").unwrap();
        assert_eq!(hand.size(), 3);
        assert_eq!(hand.get_card(1), Some(Card::from_string("As").unwrap()));
        assert_eq!(hand.get_card(2), Some(Card::from_string("Ks").unwrap()));
        assert_eq!(hand.get_card(3), Some(Card::from_string("Qs").unwrap()));
    }

    #[test]
    fn test_hand_sort() {
        let mut hand = Hand::from_string("2s As Ks").unwrap();
        hand.sort();
        assert_eq!(hand.get_card(1), Some(Card::from_string("As").unwrap()));
        assert_eq!(hand.get_card(2), Some(Card::from_string("Ks").unwrap()));
        assert_eq!(hand.get_card(3), Some(Card::from_string("2s").unwrap()));
    }

    #[test]
    fn test_hand_sort_with_empty_slots() {
        let mut hand = Hand::new();
        hand.add_card(Card::from_string("2s").unwrap()).unwrap();
        hand.add_card(Card::from_string("As").unwrap()).unwrap();
        hand.add_card(Card::from_string("Ks").unwrap()).unwrap();
        hand.sort();
        assert_eq!(hand.get_card(1), Some(Card::from_string("As").unwrap()));
        assert_eq!(hand.get_card(2), Some(Card::from_string("Ks").unwrap()));
        assert_eq!(hand.get_card(3), Some(Card::from_string("2s").unwrap()));
    }
}
