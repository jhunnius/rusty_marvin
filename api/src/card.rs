#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card(u8);

impl Card {
    // Constants for suits
    pub const SPADES: u8 = 0;
    pub const HEARTS: u8 = 1;
    pub const DIAMONDS: u8 = 2;
    pub const CLUBS: u8 = 3;

    // Constants for ranks
    pub const TWO: u8 = 2;
    pub const THREE: u8 = 3;
    pub const FOUR: u8 = 4;
    pub const FIVE: u8 = 5;
    pub const SIX: u8 = 6;
    pub const SEVEN: u8 = 7;
    pub const EIGHT: u8 = 8;
    pub const NINE: u8 = 9;
    pub const TEN: u8 = 10;
    pub const JACK: u8 = 11;
    pub const QUEEN: u8 = 12;
    pub const KING: u8 = 13;
    pub const ACE: u8 = 14;

    // Special constants
    pub const BAD_CARD: u8 = 255;
    pub const NUM_SUITS: u8 = 4;
    pub const NUM_RANKS: u8 = 13;
    pub const NUM_CARDS: u8 = 52;

    // Constructor for an empty/invalid card
    pub fn new() -> Self {
        Self(Self::BAD_CARD)
    }

    // Constructor from rank and suit
    pub fn from_rank_suit(rank: u8, suit: u8) -> Result<Self, &'static str> {
        if rank < Self::TWO || rank > Self::ACE {
            return Err("Invalid rank");
        }
        if suit >= Self::NUM_SUITS {
            return Err("Invalid suit");
        }
        Ok(Self((suit << 4) | (rank & 0xF)))
    }

    // Constructor from index (0..51)
    pub fn from_index(index: u8) -> Result<Self, &'static str> {
        if index >= Self::NUM_CARDS {
            return Err("Invalid index");
        }
        let rank = (index % Self::NUM_RANKS) + Self::TWO;
        let suit = index / Self::NUM_RANKS;
        Self::from_rank_suit(rank, suit)
    }

    // Constructor from string (e.g., "As" for Ace of Spades)
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        if s.len() != 2 {
            return Err("Invalid card string");
        }
        let rank_char = s.chars().nth(0).unwrap();
        let suit_char = s.chars().nth(1).unwrap();
        let rank = Self::rank_from_char(rank_char)?;
        let suit = Self::suit_from_char(suit_char)?;
        Self::from_rank_suit(rank, suit)
    }

    // Get the rank of the card
    pub fn rank(&self) -> u8 {
        self.0 & 0xF
    }

    // Get the suit of the card
    pub fn suit(&self) -> u8 {
        (self.0 >> 4) & 0x3
    }

    // Get the index of the card (0..51)
    pub fn index(&self) -> u8 {
        (self.suit() * Self::NUM_RANKS) + (self.rank() - Self::TWO)
    }

    // Check if the card is valid
    pub fn is_valid(&self) -> bool {
        self.0 != Self::BAD_CARD
    }

    // Convert rank to character (e.g., 14 -> 'A')
    pub fn rank_to_char(rank: u8) -> Result<char, &'static str> {
        match rank {
            Self::TWO => Ok('2'),
            Self::THREE => Ok('3'),
            Self::FOUR => Ok('4'),
            Self::FIVE => Ok('5'),
            Self::SIX => Ok('6'),
            Self::SEVEN => Ok('7'),
            Self::EIGHT => Ok('8'),
            Self::NINE => Ok('9'),
            Self::TEN => Ok('T'),
            Self::JACK => Ok('J'),
            Self::QUEEN => Ok('Q'),
            Self::KING => Ok('K'),
            Self::ACE => Ok('A'),
            _ => Err("Invalid rank"),
        }
    }

    // Convert suit to character (e.g., 0 -> 's')
    pub fn suit_to_char(suit: u8) -> Result<char, &'static str> {
        match suit {
            Self::SPADES => Ok('s'),
            Self::HEARTS => Ok('h'),
            Self::DIAMONDS => Ok('d'),
            Self::CLUBS => Ok('c'),
            _ => Err("Invalid suit"),
        }
    }

    // Convert rank character to rank (e.g., 'A' -> 14)
    pub fn rank_from_char(rank_char: char) -> Result<u8, &'static str> {
        match rank_char {
            '2' => Ok(Self::TWO),
            '3' => Ok(Self::THREE),
            '4' => Ok(Self::FOUR),
            '5' => Ok(Self::FIVE),
            '6' => Ok(Self::SIX),
            '7' => Ok(Self::SEVEN),
            '8' => Ok(Self::EIGHT),
            '9' => Ok(Self::NINE),
            'T' => Ok(Self::TEN),
            'J' => Ok(Self::JACK),
            'Q' => Ok(Self::QUEEN),
            'K' => Ok(Self::KING),
            'A' => Ok(Self::ACE),
            _ => Err("Invalid rank character"),
        }
    }

    // Convert suit character to suit (e.g., 's' -> 0)
    pub fn suit_from_char(suit_char: char) -> Result<u8, &'static str> {
        match suit_char {
            's' => Ok(Self::SPADES),
            'h' => Ok(Self::HEARTS),
            'd' => Ok(Self::DIAMONDS),
            'c' => Ok(Self::CLUBS),
            _ => Err("Invalid suit character"),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.is_valid() {
            write!(f, "Invalid Card")
        } else {
            let rank_char = Self::rank_to_char(self.rank()).unwrap();
            let suit_char = Self::suit_to_char(self.suit()).unwrap();
            write!(f, "{}{}", rank_char, suit_char)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::from_rank_suit(Card::ACE, Card::SPADES).unwrap();
        assert_eq!(card.rank(), Card::ACE);
        assert_eq!(card.suit(), Card::SPADES);
        assert_eq!(card.index(), 12); // Ace of Spades is index 12
        assert!(card.is_valid());
    }

    #[test]
    fn test_card_from_string() {
        let card = Card::from_string("As").unwrap();
        assert_eq!(card.rank(), Card::ACE);
        assert_eq!(card.suit(), Card::SPADES);
        assert_eq!(card.to_string(), "As");
    }

    #[test]
    fn test_card_from_index() {
        let card = Card::from_index(0).unwrap();
        assert_eq!(card.rank(), Card::TWO);
        assert_eq!(card.suit(), Card::SPADES);
        assert_eq!(card.to_string(), "2s");
    }

    #[test]
    fn test_invalid_card() {
        let card = Card::new();
        assert!(!card.is_valid());
        assert_eq!(card.to_string(), "Invalid Card");
    }
}
