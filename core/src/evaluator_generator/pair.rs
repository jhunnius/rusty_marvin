use crate::api::card::Card;

pub struct Pair {
    pub ordinal: usize,
    pub cards: [Card; 2],
}

static mut PAIR_VALUES: Vec<Pair> = Vec::new();
static mut PAIR_BY_CARD: Vec<Vec<usize>> = Vec::new();
static mut INTERSECTS_PAIR: Vec<Vec<bool>> = Vec::new();
static mut INTERSECTS_CARD: Vec<Vec<bool>> = Vec::new();

impl Pair {
    pub const COUNT: usize = ((Card::NUM_CARDS as u16 * (Card::NUM_CARDS - 1) as u16) / 2) as usize;

    pub fn new(c1: Card, c2: Card, ordinal: usize) -> Self {
        Self {
            cards: [c1, c2],
            ordinal,
        }
    }

    pub fn get(c1: &Card, c2: &Card) -> Option<&'static Pair> {
        unsafe {
            let index = PAIR_BY_CARD
                .get_unchecked(c1.index() as usize)
                .get_unchecked(c2.index() as usize);
            PAIR_VALUES.get(*index)
        }
    }

    pub fn intersects(&self, other: &Pair) -> bool {
        self.cards.iter().any(|c| other.cards.contains(c))
    }

    pub fn intersects_card(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }
}

pub fn init_pairs() {
    unsafe {
        let mut pairs = Vec::with_capacity(Pair::COUNT);
        let mut pair_by_card = vec![vec![0; Card::NUM_CARDS as usize]; Card::NUM_CARDS as usize];
        let mut intersects_pair = vec![vec![false; Pair::COUNT]; Pair::COUNT];
        let mut intersects_card = vec![vec![false; Pair::COUNT]; Card::NUM_CARDS as usize];

        let mut k = 0;
        for i in 0..Card::NUM_CARDS {
            for j in (i + 1)..Card::NUM_CARDS {
                let pair = Pair::new(
                    Card::from_index(i).expect("Invalid Index!"),
                    Card::from_index(j).expect("Invalid Index!"),
                    k,
                );
                pairs.push(pair);
                pair_by_card[i as usize][j as usize] = k;
                pair_by_card[j as usize][i as usize] = k;
                k += 1;
            }
        }

        for i in 0..Pair::COUNT {
            for j in 0..Pair::COUNT {
                intersects_pair[i][j] = pairs[i].intersects(&pairs[j]);
            }
        }

        for i in 0..Pair::COUNT {
            for j in 0..Card::NUM_CARDS {
                intersects_card[i as usize][j as usize] = pairs[i as usize]
                    .intersects_card(&Card::from_index(j).expect("Invalid Index!"));
            }
        }

        PAIR_VALUES = pairs;
        PAIR_BY_CARD = pair_by_card;
        INTERSECTS_PAIR = intersects_pair;
        INTERSECTS_CARD = intersects_card;
    }
}
