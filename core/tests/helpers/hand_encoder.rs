pub struct HandEncoder;

impl HandEncoder {
    pub fn encode_to_meerkat(hand: &[u8]) -> Vec<String> {
        hand.iter().map(|&card| Self::print_card(card)).collect()
    }

    pub fn encode_to_steve_brecher(hand: &[u8]) -> u64 {
        let mut result = 0;
        for &card in hand.iter() {
            result |= 0x1 << ((card % 4) * 13 + (card / 4));
        }
        result
    }

    pub fn print_card(card: u8) -> String {
        if card == 255 {
            return "-".to_string();
        }
        let rank = card / 4;
        let suit = card % 4;
        let rank_str = match rank {
            0..=7 => (rank + 2).to_string(),
            8 => "T".to_string(),
            9 => "J".to_string(),
            10 => "Q".to_string(),
            11 => "K".to_string(),
            12 => "A".to_string(),
            _ => "?".to_string(),
        };
        let suit_str = match suit {
            0 => "c",
            1 => "d",
            2 => "s",
            3 => "h",
            _ => "?",
        };
        format!("{}{}", rank_str, suit_str)
    }
}
