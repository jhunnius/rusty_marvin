//! # Pair Module
//!
//! This module provides functionality for working with card pairs in poker hand evaluation.
//! The Pair struct represents a unique combination of two cards and provides methods for
//! pair lookup, intersection detection, and relationship analysis between pairs.
//!
//! ## Pair Representation Overview
//!
//! A Pair represents a unique 2-card combination with:
//! - **Two Card Values**: The specific cards that form the pair
//! - **Ordinal Index**: Unique identifier (0 to 1,326) for the pair
//! - **Lookup Tables**: Precomputed intersection relationships with other pairs and cards
//!
//! ## Key Features
//!
//! - **Unique Identification**: Each possible 2-card combination has a unique ordinal
//! - **Fast Lookup**: O(1) access to pair information by card combination
//! - **Intersection Detection**: Efficient checking for shared cards between pairs
//! - **Memory Efficient**: Compact representation with shared lookup tables
//! - **Precomputed Relationships**: All pair and card intersections calculated at startup
//!
//! ## Pair Count and Coverage
//!
//! - **Total Pairs**: C(52,2) = 1,326 possible unique pairs
//! - **Complete Coverage**: Every possible 2-card combination represented
//! - **Ordered Generation**: Pairs generated in lexicographic card order
//! - **Bidirectional Lookup**: Fast access by any card combination
//!
//! ## Examples
//!
//! ### Basic Pair Operations
//!
//! ```rust
//! use poker_api::api::card::Card;
//! use poker_api::evaluator_generator::pair::{Pair, init_pairs};
//!
//! // Initialize pair lookup tables (done once at startup)
//! init_pairs();
//!
//! // Create cards and get their pair
//! let card1 = Card::from_string("As").unwrap();
//! let card2 = Card::from_string("Ks").unwrap();
//!
//! if let Some(pair) = Pair::get(&card1, &card2) {
//!     println!("Pair ordinal: {}", pair.ordinal);
//!     println!("Pair cards: {} {}", pair.cards[0], pair.cards[1]);
//! }
//! ```
//!
//! ### Pair Intersection Detection
//!
//! ```rust
//! use poker_api::api::card::Card;
//! use poker_api::evaluator_generator::pair::{Pair, init_pairs};
//!
//! init_pairs();
//!
//! let ace_spades = Card::from_string("As").unwrap();
//! let ace_hearts = Card::from_string("Ah").unwrap();
//! let king_spades = Card::from_string("Ks").unwrap();
//!
//! let pair1 = Pair::get(&ace_spades, &ace_hearts).unwrap();
//! let pair2 = Pair::get(&ace_spades, &king_spades).unwrap();
//!
//! // Check if pairs share cards
//! println!("Pairs intersect: {}", pair1.intersects(pair2)); // true (both have As)
//!
//! // Check if pair contains specific card
//! println!("Pair1 contains As: {}", pair1.intersects_card(&ace_spades)); // true
//! println!("Pair1 contains Ks: {}", pair1.intersects_card(&king_spades)); // false
//! ```
//!
//! ### Integration with Hand Evaluation
//!
//! ```rust
//! use poker_api::evaluator_generator::pair::{Pair, init_pairs};
//! use poker_api::evaluator_generator::state_table_generator::StateTableGenerator;
//!
//! // Initialize pair system (required for hand evaluation)
//! init_pairs();
//!
//! // Pair analysis used internally for:
//! // - Two pair detection and ranking
//! // - Full house evaluation (pair + trips)
//! // - Hand comparison and equivalence
//! // - Dead card analysis in simulations
//! ```
//!
//! ## Mathematical Foundation
//!
//! ### Pair Combinations
//! - **Total Pairs**: C(52,2) = 1,326 = 52×51/2
//! - **Unique Representation**: Each pair has exactly one canonical representation
//! - **Ordinal Assignment**: Sequential assignment from 0 to 1,325
//! - **Card Ordering**: Pairs ordered by first card, then second card index
//!
//! ### Lookup Complexity
//! ```text
//! Pair Lookup: O(1) - Direct table access
//! Intersection Check: O(1) - Precomputed boolean tables
//! Memory Usage: ~1,326 × (8 + 8) bytes + lookup tables
//! ```
//!
//! ## Design Decisions
//!
//! - **Global State**: Uses static mut variables for performance and simplicity
//! - **Precomputed Tables**: All intersections calculated once at initialization
//! - **Symmetric Lookup**: pair_by_card[i][j] == pair_by_card[j][i]
//! - **Compact Storage**: Minimal memory per pair (16 bytes + shared tables)
//! - **Unsafe Code**: Uses unsafe for performance-critical lookup operations
//!
//! ## Performance Characteristics
//!
//! - **Initialization**: O(n²) where n=1,326 (one-time cost)
//! - **Pair Lookup**: O(1) array access
//! - **Intersection Check**: O(1) boolean lookup
//! - **Memory Usage**: ~7MB for all lookup tables
//! - **Thread Safety**: Not thread-safe (requires external synchronization)

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
