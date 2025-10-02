pub struct HandEval;

impl HandEval {
    pub fn encode(card: u8) -> u64 {
        1 << (card / 4 * 13 + card % 4)
    }

    pub fn encode_hand(cards: &[u8]) -> u64 {
        cards.iter().fold(0, |acc, &c| acc | Self::encode(c))
    }

    pub fn hand7_eval(hand: u64) -> u32 {
        let c = (hand & 0x1FFF) as u32;
        let d = ((hand >> 13) & 0x1FFF) as u32;
        let h = ((hand >> 26) & 0x1FFF) as u32;
        let s = ((hand >> 39) & 0x1FFF) as u32;
        let ranks = c | d | h | s;

        match ranks.count_ones() {
            5 => Self::eval_5_card_hand(ranks, c, d, h, s),
            6 => Self::eval_6_card_hand(ranks, c, d, h, s),
            7 => Self::eval_7_card_hand(ranks, c, d, h, s),
            _ => 0,
        }
    }

    fn eval_5_card_hand(ranks: u32, c: u32, d: u32, h: u32, s: u32) -> u32 {
        if Self::is_flush(c, d, h, s) {
            return Self::flush_or_straight_flush(ranks);
        }
        Self::rank_hand_category(ranks)
    }

    fn eval_6_card_hand(ranks: u32, c: u32, d: u32, h: u32, s: u32) -> u32 {
        let mut best_rank = u32::MAX;
        for i in 0..6 {
            let modified_ranks = ranks & !(1 << i);
            best_rank = best_rank.min(Self::eval_5_card_hand(modified_ranks, c, d, h, s));
        }
        best_rank
    }

    fn eval_7_card_hand(ranks: u32, c: u32, d: u32, h: u32, s: u32) -> u32 {
        let mut best_rank = u32::MAX;
        for i in 0..7 {
            for j in (i + 1)..7 {
                let modified_ranks = ranks & !(1 << i) & !(1 << j);
                best_rank = best_rank.min(Self::eval_5_card_hand(modified_ranks, c, d, h, s));
            }
        }
        best_rank
    }

    fn is_flush(c: u32, d: u32, h: u32, s: u32) -> bool {
        c.count_ones() >= 5 || d.count_ones() >= 5 || h.count_ones() >= 5 || s.count_ones() >= 5
    }

    fn flush_or_straight_flush(ranks: u32) -> u32 {
        if Self::is_straight(ranks) {
            return 9; // Straight flush category
        }
        6 // Flush category
    }

    fn is_straight(ranks: u32) -> bool {
        let mut sequence = ranks;
        while sequence > 0 {
            if sequence & 0b11111 == 0b11111 {
                return true;
            }
            sequence >>= 1;
        }
        false
    }

    fn rank_hand_category(ranks: u32) -> u32 {
        match ranks.count_ones() {
            5 => 1, // High card
            4 => 2, // One pair
            3 => 4, // Three of a kind
            2 => 7, // Full house
            _ => 0,
        }
    }
}
