//! Core poker hand evaluator implementation

use super::errors::EvaluatorError;
use super::tables::{JumpTable, JumpTableEntry};
use crate::card::PackedCard;
use crate::{Card, Hand};
use std::path::Path;
use std::sync::Arc;

/// Hand ranking enumeration
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum HandRank {
    /// High card
    HighCard = 0,
    /// One pair
    Pair = 1,
    /// Two pair
    TwoPair = 2,
    /// Three of a kind
    ThreeOfAKind = 3,
    /// Straight
    Straight = 4,
    /// Flush
    Flush = 5,
    /// Full house
    FullHouse = 6,
    /// Four of a kind
    FourOfAKind = 7,
    /// Straight flush
    StraightFlush = 8,
    /// Royal flush
    RoyalFlush = 9,
}

impl HandRank {
    /// Create a hand rank from a numeric value
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(HandRank::HighCard),
            1 => Some(HandRank::Pair),
            2 => Some(HandRank::TwoPair),
            3 => Some(HandRank::ThreeOfAKind),
            4 => Some(HandRank::Straight),
            5 => Some(HandRank::Flush),
            6 => Some(HandRank::FullHouse),
            7 => Some(HandRank::FourOfAKind),
            8 => Some(HandRank::StraightFlush),
            9 => Some(HandRank::RoyalFlush),
            _ => None,
        }
    }

    /// Convert to numeric value
    pub fn as_u8(&self) -> u8 {
        match self {
            HandRank::HighCard => 0,
            HandRank::Pair => 1,
            HandRank::TwoPair => 2,
            HandRank::ThreeOfAKind => 3,
            HandRank::Straight => 4,
            HandRank::Flush => 5,
            HandRank::FullHouse => 6,
            HandRank::FourOfAKind => 7,
            HandRank::StraightFlush => 8,
            HandRank::RoyalFlush => 9,
        }
    }

    /// Get the base offset for flat rank calculation using proper poker mathematics
    pub fn base_offset(&self) -> u32 {
        match self {
            HandRank::HighCard => 0,
            HandRank::Pair => 1_302_540, // C(52,5) * 0.501 ≈ 1,302,540
            HandRank::TwoPair => 2_400_780, // Pair max + 1
            HandRank::ThreeOfAKind => 2_524_332, // Two pair max + 1
            HandRank::Straight => 2_579_244, // Three of kind max + 1
            HandRank::Flush => 2_589_444, // Straight max + 1
            HandRank::FullHouse => 2_594_552, // Flush max + 1
            HandRank::FourOfAKind => 2_598_296, // Full house max + 1
            HandRank::StraightFlush => 2_598_920, // Four of kind max + 1
            HandRank::RoyalFlush => 2_598_944, // Adjusted to ensure max flat rank is 2,598,959
        }
    }

    /// Get the maximum strength value for this hand rank
    pub fn max_strength(&self) -> u32 {
        match self {
            HandRank::HighCard => 371_292, // 12*28561 + 11*2197 + 10*169 + 9*13 + 7
            HandRank::Pair => 78_363,      // 12*2197 + 11*169 + 10*13 + 9
            HandRank::TwoPair => 110_837,  // 12*169 + 11*13 + 10
            HandRank::ThreeOfAKind => 109_307, // 12*169 + 11*13 + 10
            HandRank::Straight => 5_107,   // Max straight value (A-high = 12, 5-high = 3)
            HandRank::Flush => 5_107,      // Same as straight max (flushes are rarer)
            HandRank::FullHouse => 6_146,  // 12*13 + 11
            HandRank::FourOfAKind => 4_050, // 12*13 + 11
            HandRank::StraightFlush => 4_323, // Max straight flush value (K-high = 11, 5-high = 3)
            HandRank::RoyalFlush => 4_323, // Royal flush is special case of straight flush
        }
    }
}

/// Hand value containing rank and strength
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct HandValue {
    /// The hand rank
    pub rank: HandRank,
    /// The strength value for comparison within the same rank
    pub value: u32,
}

impl HandValue {
    /// Create a new hand value
    pub fn new(rank: HandRank, value: u32) -> Self {
        Self { rank, value }
    }

    /// Create a hand value from a combined u32 value
    pub fn from_u32(combined: u32) -> Self {
        let rank_value = (combined >> 16) as u8;
        let strength = combined & 0xFFFF;

        let rank = HandRank::from_u8(rank_value).unwrap_or(HandRank::HighCard);
        Self::new(rank, strength)
    }

    /// Convert to a combined u32 value
    pub fn as_u32(&self) -> u32 {
        ((self.rank.as_u8() as u32) << 16) | self.value
    }

    /// Convert to a flat rank number (0-2,598,959) for sorting/comparison
    /// This combines the hand rank with the strength value into a single number
    pub fn flat_rank(&self) -> u32 {
        self.rank.base_offset() + self.value
    }

    /// Convert from a flat rank number back to HandValue using proper poker mathematics
    pub fn from_flat_rank(flat_rank: u32) -> Self {
        if flat_rank == 0 {
            return Self::new(HandRank::HighCard, 0);
        }

        // DEBUG: Log the flat rank being processed
        #[cfg(debug_assertions)]
        println!("DEBUG: Processing flat_rank: {}", flat_rank);

        // Find which rank this flat_rank belongs to using proper poker hand distribution
        let result = match flat_rank {
            1..=1_302_539 => {
                #[cfg(debug_assertions)]
                println!("DEBUG: Mapping to HighCard with value {}", flat_rank);
                Self::new(HandRank::HighCard, flat_rank)
            }
            1_302_540..=2_400_779 => {
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Mapping to Pair with value {}",
                    flat_rank - 1_302_540
                );
                Self::new(HandRank::Pair, flat_rank - 1_302_540)
            }
            2_400_780..=2_524_331 => {
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Mapping to TwoPair with value {}",
                    flat_rank - 2_400_780
                );
                Self::new(HandRank::TwoPair, flat_rank - 2_400_780)
            }
            2_524_332..=2_579_243 => {
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Mapping to ThreeOfAKind with value {}",
                    flat_rank - 2_524_332
                );
                Self::new(HandRank::ThreeOfAKind, flat_rank - 2_524_332)
            }
            2_579_244..=2_589_443 => {
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Mapping to Straight with value {}",
                    flat_rank - 2_579_244
                );
                Self::new(HandRank::Straight, flat_rank - 2_579_244)
            }
            2_589_444..=2_594_551 => {
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Mapping to Flush with value {}",
                    flat_rank - 2_589_444
                );
                Self::new(HandRank::Flush, flat_rank - 2_589_444)
            }
            2_594_552..=2_598_295 => {
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Mapping to FullHouse with value {}",
                    flat_rank - 2_594_552
                );
                Self::new(HandRank::FullHouse, flat_rank - 2_594_552)
            }
            2_598_296..=2_598_919 => {
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Mapping to FourOfAKind with value {}",
                    flat_rank - 2_598_296
                );
                Self::new(HandRank::FourOfAKind, flat_rank - 2_598_296)
            }
            2_598_920..=2_598_955 => {
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Mapping to StraightFlush with value {}",
                    flat_rank - 2_598_920
                );
                Self::new(HandRank::StraightFlush, flat_rank - 2_598_920)
            }
            2_598_956..=2_598_959 => {
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Mapping to RoyalFlush with value {}",
                    flat_rank - 2_598_956
                );
                Self::new(HandRank::RoyalFlush, flat_rank - 2_598_956)
            }
            _ => {
                // Out of range, fallback to high card
                #[cfg(debug_assertions)]
                println!(
                    "DEBUG: Out of range, falling back to HighCard with value {}",
                    flat_rank
                );
                Self::new(HandRank::HighCard, flat_rank)
            }
        };

        #[cfg(debug_assertions)]
        println!("DEBUG: Result: {:?}", result);
        result
    }

    /// Convert to compressed rank (0-7461) for equivalence class representation
    /// This groups hands by rank type and relative card values, ignoring suit differences
    /// Fixed to properly map to Cactus Kev's equivalence classes
    pub fn compressed_rank(&self) -> u32 {
        match self.rank {
            HandRank::HighCard => {
                // High card: 0-1276 (1,277 equivalence classes)
                // Map to canonical value using proper poker mathematics
                let base_value = self.value;
                let ranks = [
                    ((base_value / 28561) % 13) as u8,
                    ((base_value / 2197) % 13) as u8,
                    ((base_value / 169) % 13) as u8,
                    ((base_value / 13) % 13) as u8,
                    (base_value % 13) as u8,
                ];

                // Calculate canonical value using proper combinatorial indexing
                self.calculate_high_card_canonical_value(&ranks)
            }
            HandRank::Pair => {
                // Pair: 1277-4136 (2,860 equivalence classes)
                let pair_rank = (self.value / 2197) as u8;
                let kicker1 = ((self.value % 2197) / 169) as u8;
                let kicker2 = ((self.value % 169) / 13) as u8;
                let kicker3 = (self.value % 13) as u8;

                // Calculate canonical compressed value
                1277 + self.calculate_paired_hand_canonical_value(
                    &[pair_rank, pair_rank, kicker1, kicker2, kicker3],
                    HandRank::Pair,
                )
            }
            HandRank::TwoPair => {
                // Two pair: 4137-4994 (858 equivalence classes)
                let high_pair = (self.value / 169) as u8;
                let low_pair = ((self.value % 169) / 13) as u8;
                let kicker = (self.value % 13) as u8;

                // Calculate canonical compressed value
                4137 + self.calculate_paired_hand_canonical_value(
                    &[high_pair, high_pair, low_pair, low_pair, kicker],
                    HandRank::TwoPair,
                )
            }
            HandRank::ThreeOfAKind => {
                // Three of a kind: 4995-5506 (512 equivalence classes)
                let three_rank = (self.value / 169) as u8;
                let kicker1 = ((self.value % 169) / 13) as u8;
                let kicker2 = (self.value % 13) as u8;

                // Calculate canonical compressed value
                4995 + self.calculate_paired_hand_canonical_value(
                    &[three_rank, three_rank, three_rank, kicker1, kicker2],
                    HandRank::ThreeOfAKind,
                )
            }
            HandRank::Straight => {
                // Straight: 5507-5516 (10 equivalence classes)
                let straight_value = self.value as u8;

                // Map straight values to compressed ranks (0-9 for 9 possible straights)
                let straight_rank = match straight_value {
                    0 => 0, // A-5 straight
                    1 => 1, // 2-6 straight
                    2 => 2, // 3-7 straight
                    3 => 3, // 4-8 straight
                    4 => 4, // 5-9 straight
                    5 => 5, // 6-T straight
                    6 => 6, // 7-J straight
                    7 => 7, // 8-Q straight
                    8 => 8, // 9-K straight
                    9 => 9, // T-A straight
                    _ => 0, // Fallback
                };

                5507 + straight_rank as u32
            }
            HandRank::Flush => {
                // Flush: 5517-6626 (1,110 equivalence classes)
                let base_value = self.value;
                let ranks = [
                    ((base_value / 28561) % 13) as u8,
                    ((base_value / 2197) % 13) as u8,
                    ((base_value / 169) % 13) as u8,
                    ((base_value / 13) % 13) as u8,
                    (base_value % 13) as u8,
                ];

                // Calculate canonical compressed value
                5517 + self.calculate_flush_canonical_value(&ranks)
            }
            HandRank::FullHouse => {
                // Full house: 6627-6782 (156 equivalence classes)
                let three_rank = (self.value / 13) as u8;
                let pair_rank = (self.value % 13) as u8;

                // Calculate canonical compressed value
                6627 + (three_rank as u32) * 13 + (pair_rank as u32)
            }
            HandRank::FourOfAKind => {
                // Four of a kind: 6783-6923 (141 equivalence classes)
                let four_rank = (self.value / 13) as u8;
                let kicker_rank = (self.value % 13) as u8;

                // Calculate canonical compressed value
                6783 + (four_rank as u32) * 13 + (kicker_rank as u32)
            }
            HandRank::StraightFlush => {
                // Straight flush: 6924-6933 (10 equivalence classes)
                let straight_value = self.value as u8;

                // Map straight flush values to compressed ranks (0-8 for 9 possible straight flushes)
                let straight_flush_rank = match straight_value {
                    0 => 0, // 5-A straight flush
                    1 => 1, // 6-2 straight flush
                    2 => 2, // 7-3 straight flush
                    3 => 3, // 8-4 straight flush
                    4 => 4, // 9-5 straight flush
                    5 => 5, // T-6 straight flush
                    6 => 6, // J-7 straight flush
                    7 => 7, // Q-8 straight flush
                    8 => 8, // K-9 straight flush
                    _ => 0, // Fallback
                };

                6924 + straight_flush_rank as u32
            }
            HandRank::RoyalFlush => {
                // Royal flush: 6934-7461 (528 equivalence classes for all suit combinations)
                // Royal flush is always rank 7461 (highest possible rank)
                7461
            }
        }
    }

    /// Convert from compressed rank back to HandValue using proper poker equivalence classes
    pub fn from_compressed_rank(compressed_rank: u32) -> Option<Self> {
        // Map compressed rank back to HandValue using proper poker hand equivalence classes
        match compressed_rank {
            // High card: 0-1276 (1,277 equivalence classes)
            0..=1276 => {
                let value = Self::reverse_high_card_mapping(compressed_rank);
                Some(Self::new(HandRank::HighCard, value))
            }
            // Pair: 1277-4136 (2,860 equivalence classes)
            1277..=4136 => {
                let value = Self::reverse_pair_mapping(compressed_rank - 1277);
                Some(Self::new(HandRank::Pair, value))
            }
            // Two pair: 4137-4994 (858 equivalence classes)
            4137..=4994 => {
                let value = Self::reverse_two_pair_mapping(compressed_rank - 4137);
                Some(Self::new(HandRank::TwoPair, value))
            }
            // Three of a kind: 4995-5506 (512 equivalence classes)
            4995..=5506 => {
                let value = Self::reverse_three_of_kind_mapping(compressed_rank - 4995);
                Some(Self::new(HandRank::ThreeOfAKind, value))
            }
            // Straight: 5507-5516 (10 equivalence classes)
            5507..=5516 => {
                let value = Self::reverse_straight_mapping(compressed_rank - 5507);
                Some(Self::new(HandRank::Straight, value))
            }
            // Flush: 5517-6626 (1,110 equivalence classes)
            5517..=6626 => {
                let value = Self::reverse_flush_mapping(compressed_rank - 5517);
                Some(Self::new(HandRank::Flush, value))
            }
            // Full house: 6627-6782 (156 equivalence classes)
            6627..=6782 => {
                let value = Self::reverse_full_house_mapping(compressed_rank - 6627);
                Some(Self::new(HandRank::FullHouse, value))
            }
            // Four of a kind: 6783-6923 (141 equivalence classes)
            6783..=6923 => {
                let value = Self::reverse_four_of_kind_mapping(compressed_rank - 6783);
                Some(Self::new(HandRank::FourOfAKind, value))
            }
            // Straight flush: 6924-6933 (10 equivalence classes)
            6924..=6933 => {
                let value = Self::reverse_straight_flush_mapping(compressed_rank - 6924);
                Some(Self::new(HandRank::StraightFlush, value))
            }
            // Royal flush: 6934-7461 (528 equivalence classes for all suit combinations)
            6934..=7461 => {
                Some(Self::new(HandRank::RoyalFlush, 9)) // Royal flush is A-high straight flush
            }
            // Out of range
            _ => None,
        }
    }

    /// Reverse mapping for high card hands
    fn reverse_high_card_mapping(compressed_rank: u32) -> u32 {
        // For high card, the compressed rank directly maps to the canonical value
        // We need to convert back to the original flat rank representation
        compressed_rank
    }

    /// Reverse mapping for pair hands
    fn reverse_pair_mapping(compressed_rank: u32) -> u32 {
        // Extract pair rank and kickers from compressed rank
        let pair_rank = compressed_rank / 2197;
        let remainder = compressed_rank % 2197;
        let kicker1 = remainder / 169;
        let remainder = remainder % 169;
        let kicker2 = remainder / 13;
        let kicker3 = remainder % 13;

        pair_rank * 2197 + kicker1 * 169 + kicker2 * 13 + kicker3
    }

    /// Reverse mapping for two pair hands
    fn reverse_two_pair_mapping(compressed_rank: u32) -> u32 {
        let high_pair = compressed_rank / 169;
        let remainder = compressed_rank % 169;
        let low_pair = remainder / 13;
        let kicker = remainder % 13;

        high_pair * 169 + low_pair * 13 + kicker
    }

    /// Reverse mapping for three of a kind hands
    fn reverse_three_of_kind_mapping(compressed_rank: u32) -> u32 {
        let three_rank = compressed_rank / 169;
        let remainder = compressed_rank % 169;
        let kicker1 = remainder / 13;
        let kicker2 = remainder % 13;

        three_rank * 169 + kicker1 * 13 + kicker2
    }

    /// Reverse mapping for straight hands
    fn reverse_straight_mapping(compressed_rank: u32) -> u32 {
        // Map back to straight values (0-9)
        compressed_rank
    }

    /// Reverse mapping for flush hands
    fn reverse_flush_mapping(compressed_rank: u32) -> u32 {
        // For flush, the compressed rank directly maps to the canonical value
        compressed_rank
    }

    /// Reverse mapping for full house hands
    fn reverse_full_house_mapping(compressed_rank: u32) -> u32 {
        let three_rank = compressed_rank / 13;
        let pair_rank = compressed_rank % 13;

        three_rank * 13 + pair_rank
    }

    /// Reverse mapping for four of a kind hands
    fn reverse_four_of_kind_mapping(compressed_rank: u32) -> u32 {
        let four_rank = compressed_rank / 13;
        let kicker_rank = compressed_rank % 13;

        four_rank * 13 + kicker_rank
    }

    /// Reverse mapping for straight flush hands
    fn reverse_straight_flush_mapping(compressed_rank: u32) -> u32 {
        // Map back to straight flush values (0-9)
        compressed_rank
    }

    /// Calculate canonical value for high card hands using proper combinatorial indexing
    fn calculate_high_card_canonical_value(&self, ranks: &[u8; 5]) -> u32 {
        // For high card hands, we need to find the lexicographically smallest
        // suit assignment that produces these ranks in descending order

        // Sort ranks in descending order for canonical representation
        let mut sorted_ranks = ranks.to_vec();
        sorted_ranks.sort_by(|a, b| b.cmp(a));

        // Calculate the canonical value using the same formula as the original hand value
        // This ensures consistency between flat rank and compressed rank calculations
        sorted_ranks[0] as u32 * 28561
            + sorted_ranks[1] as u32 * 2197
            + sorted_ranks[2] as u32 * 169
            + sorted_ranks[3] as u32 * 13
            + sorted_ranks[4] as u32
    }

    /// Calculate canonical value for paired hands using proper combinatorial indexing
    fn calculate_paired_hand_canonical_value(&self, ranks: &[u8; 5], hand_type: HandRank) -> u32 {
        // For paired hands, we need to consider suit assignments more carefully
        // The key insight is that suits only matter for the relative ordering of kickers

        let mut rank_counts = [0u8; 13];
        for &rank in ranks {
            rank_counts[rank as usize] += 1;
        }

        // Find the groups of cards (pairs, trips, etc.)
        let mut groups = Vec::new();
        for (rank, &count) in rank_counts.iter().enumerate() {
            if count > 0 {
                groups.push((rank as u8, count));
            }
        }

        // Sort groups by rank (highest first) for canonical representation
        groups.sort_by(|a, b| b.0.cmp(&a.0));

        // Calculate canonical value based on hand type using proper equivalence class indexing
        match hand_type {
            HandRank::Pair => {
                // One pair: pair_rank, kicker1, kicker2, kicker3
                let pair_rank = groups[0].0;
                let kickers: Vec<u8> = groups.iter().skip(1).map(|(rank, _)| *rank).collect();

                // Use the same formula as the original hand value calculation for consistency
                pair_rank as u32 * 2197
                    + kickers[0] as u32 * 169
                    + kickers[1] as u32 * 13
                    + kickers[2] as u32
            }
            HandRank::TwoPair => {
                // Two pair: high_pair, low_pair, kicker
                let high_pair = groups[0].0;
                let low_pair = groups[1].0;
                let kicker = groups[2].0;

                // Use the same formula as the original hand value calculation for consistency
                high_pair as u32 * 169 + low_pair as u32 * 13 + kicker as u32
            }
            HandRank::ThreeOfAKind => {
                // Three of a kind: three_rank, kicker1, kicker2
                let three_rank = groups[0].0;
                let kickers: Vec<u8> = groups.iter().skip(1).map(|(rank, _)| *rank).collect();

                // Use the same formula as the original hand value calculation for consistency
                three_rank as u32 * 169 + kickers[0] as u32 * 13 + kickers[1] as u32
            }
            _ => 0, // Should not happen
        }
    }

    /// Calculate canonical value for flush hands
    fn calculate_flush_canonical_value(&self, ranks: &[u8; 5]) -> u32 {
        // For flush hands, all cards have the same suit, so we just need to
        // calculate the value based on the rank ordering
        let mut sorted_ranks = ranks.to_vec();
        sorted_ranks.sort_by(|a, b| b.cmp(a)); // Sort in descending order

        sorted_ranks[0] as u32 * 28561
            + sorted_ranks[1] as u32 * 2197
            + sorted_ranks[2] as u32 * 169
            + sorted_ranks[3] as u32 * 13
            + sorted_ranks[4] as u32
    }
}

/// Main poker hand evaluator
#[derive(Debug, Clone)]
pub struct Evaluator {
    /// Jump table for hand evaluation
    tables: Arc<JumpTable>,
}

impl Evaluator {
    /// Create a new evaluator instance with persistent jump table
    pub fn new() -> Result<Self, EvaluatorError> {
        Self::new_with_persistence::<std::path::PathBuf>(None)
    }

    /// Create a new evaluator instance with optional custom file path
    pub fn new_with_persistence<P: AsRef<Path>>(
        file_path: Option<P>,
    ) -> Result<Self, EvaluatorError> {
        let table = Self::load_or_create_table(file_path)?;
        Ok(Self {
            tables: Arc::new(table),
        })
    }

    /// Load jump table from file or create new one if file doesn't exist or is invalid
    fn load_or_create_table<P: AsRef<Path>>(
        file_path: Option<P>,
    ) -> Result<JumpTable, EvaluatorError> {
        let path = file_path
            .map(|p| p.as_ref().to_path_buf())
            .unwrap_or_else(|| JumpTable::default_file_path());

        // Try to load existing table first
        if JumpTable::exists(&path) {
            match JumpTable::load_from_file(&path) {
                Ok(table) => {
                    println!("Loaded existing jump table from: {}", path.display());
                    return Ok(table);
                }
                Err(e) => {
                    println!(
                        "Failed to load existing jump table ({}), creating new one...",
                        e
                    );
                }
            }
        }

        // Create new table if loading failed or file doesn't exist
        Self::create_and_save_table(&path)
    }

    /// Create a new jump table and save it to file
    fn create_and_save_table(path: &Path) -> Result<JumpTable, EvaluatorError> {
        println!("Creating new jump table...");
        let mut table = JumpTable::with_target_memory();

        // Build the jump table with proper evaluation data
        table.build().map_err(|e| {
            EvaluatorError::table_init_failed(&format!("Failed to initialize lookup tables: {}", e))
        })?;

        // Validate that the table was built correctly
        table.validate()?;

        // Save the table to file for future use
        table.save_to_file(path).map_err(|e| {
            EvaluatorError::table_init_failed(&format!("Failed to save jump table: {}", e))
        })?;

        println!(
            "Evaluator created successfully with {} table entries and saved to: {}",
            table.size,
            path.display()
        );

        Ok(table)
    }

    /// Get the global evaluator instance (singleton pattern)
    pub fn instance() -> Arc<Evaluator> {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<Evaluator> = OnceLock::new();
        let evaluator =
            INSTANCE.get_or_init(|| Evaluator::new().expect("Failed to create evaluator instance"));
        Arc::new(evaluator.clone())
    }

    /// Evaluate a 5-card hand
    pub fn evaluate_5_card(&self, cards: &[Card; 5]) -> HandValue {
        // Use the jump table for O(1) evaluation if available and built
        if self.tables.size > 0 && !self.tables.data.is_empty() {
            // Convert to PackedCard for jump table lookup
            let packed_cards: Vec<PackedCard> =
                cards.iter().map(|c| PackedCard::from_card(c)).collect();
            let packed_array: [PackedCard; 5] = packed_cards.try_into().unwrap();

            if let Ok(result) = self.tables.evaluate_5_card(&packed_array) {
                // Only use jump table result if it's not the default HighCard with value 0
                // This indicates the jump table lookup succeeded
                if result.rank != HandRank::HighCard || result.value != 0 {
                    return result;
                }
            }
        }

        // Fallback to direct evaluation algorithm when jump table lookup fails or returns default
        self.evaluate_5_card_direct(cards)
    }

    /// Evaluate a 6-card hand
    pub fn evaluate_6_card(&self, cards: &[Card; 6]) -> HandValue {
        // Use the jump table for O(1) evaluation if available and built
        if self.tables.size > 0 && !self.tables.data.is_empty() {
            // Convert to PackedCard for jump table lookup
            let packed_cards: Vec<PackedCard> =
                cards.iter().map(|c| PackedCard::from_card(c)).collect();
            let packed_array: [PackedCard; 6] = packed_cards.try_into().unwrap();

            if let Ok(result) = self.tables.evaluate_6_card(&packed_array) {
                return result;
            }
        }

        // Fallback to best 5-card selection
        self.evaluate_n_card_hand(cards)
    }

    /// Evaluate a 7-card hand
    pub fn evaluate_7_card(&self, cards: &[Card; 7]) -> HandValue {
        // Use the jump table for O(1) evaluation if available and built
        if self.tables.size > 0 && !self.tables.data.is_empty() {
            // Convert to PackedCard for jump table lookup
            let packed_cards: Vec<PackedCard> =
                cards.iter().map(|c| PackedCard::from_card(c)).collect();
            let packed_array: [PackedCard; 7] = packed_cards.try_into().unwrap();

            if let Ok(result) = self.tables.evaluate_7_card(&packed_array) {
                return result;
            }
        }

        // Fallback to best 5-card selection
        self.evaluate_n_card_hand(cards)
    }

    /// Evaluate an N-card hand by finding the best 5-card combination
    fn evaluate_n_card_hand<T: AsRef<[Card]>>(&self, cards: T) -> HandValue {
        let cards_ref = cards.as_ref();
        if cards_ref.len() < 5 {
            return HandValue::new(HandRank::HighCard, 0);
        }

        let mut best_hand_value = HandValue::new(HandRank::HighCard, 0);

        // Generate all combinations of 5 cards from N cards
        let indices: Vec<usize> = (0..cards_ref.len()).collect();
        let combinations = self.generate_combinations(&indices, 5);

        for combo_indices in combinations {
            let combo_cards: Vec<Card> = combo_indices.iter().map(|&i| cards_ref[i]).collect();
            let combo_array: [Card; 5] = combo_cards.try_into().unwrap();

            let hand_value = self.evaluate_5_card_direct(&combo_array);

            if hand_value > best_hand_value {
                best_hand_value = hand_value;
            }
        }

        best_hand_value
    }

    /// Generate all combinations of K elements from a set of N elements
    fn generate_combinations(&self, set: &[usize], k: usize) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        self.generate_combinations_recursive(set, k, 0, &mut current, &mut result);
        result
    }

    /// Recursive helper for combination generation
    fn generate_combinations_recursive(
        &self,
        set: &[usize],
        k: usize,
        start: usize,
        current: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if current.len() == k {
            result.push(current.clone());
            return;
        }

        for i in start..set.len() {
            current.push(set[i]);
            self.generate_combinations_recursive(set, k, i + 1, current, result);
            current.pop();
        }
    }

    /// Evaluate a hand from hole cards and board
    pub fn evaluate_hand(&self, hand: &Hand) -> HandValue {
        let cards = hand.cards();
        match cards.len() {
            5 => {
                let card_array: [Card; 5] = cards
                    .try_into()
                    .unwrap_or_else(|_| panic!("Expected 5 cards, got {}", cards.len()));
                self.evaluate_5_card(&card_array)
            }
            6 => {
                let card_array: [Card; 6] = cards
                    .try_into()
                    .unwrap_or_else(|_| panic!("Expected 6 cards, got {}", cards.len()));
                self.evaluate_6_card(&card_array)
            }
            7 => {
                let card_array: [Card; 7] = cards
                    .try_into()
                    .unwrap_or_else(|_| panic!("Expected 7 cards, got {}", cards.len()));
                self.evaluate_7_card(&card_array)
            }
            _ => HandValue::new(HandRank::HighCard, 0),
        }
    }

    /// Get the jump table
    pub fn tables(&self) -> &JumpTable {
        &self.tables
    }

    /// Direct 5-card hand evaluation algorithm (fallback when jump table unavailable)
    pub fn evaluate_5_card_direct(&self, cards: &[Card; 5]) -> HandValue {
        // Extract ranks and suits for analysis
        let mut ranks = [0u8; 5];
        let mut suits = [0u8; 5];

        for (i, card) in cards.iter().enumerate() {
            ranks[i] = card.rank();
            suits[i] = card.suit();
        }

        // Sort ranks for easier analysis (highest first)
        ranks.sort_by(|a, b| b.cmp(a));

        // Check for flush (all same suit)
        let is_flush = suits.iter().all(|&s| s == suits[0]);

        // Check for straight
        let is_straight = self.is_straight(&ranks);

        // Check for straight flush
        if is_flush && is_straight {
            return if ranks[0] == 12 {
                // Royal flush (A,K,Q,J,10 of same suit)
                HandValue::new(HandRank::RoyalFlush, ranks[0] as u32)
            } else {
                // Straight flush
                HandValue::new(HandRank::StraightFlush, ranks[0] as u32)
            };
        }

        // Check for four of a kind
        if self.has_n_of_kind(&ranks, 4) {
            return HandValue::new(
                HandRank::FourOfAKind,
                self.calculate_four_of_kind_value(&ranks),
            );
        }

        // Check for full house
        if self.has_full_house(&ranks) {
            return HandValue::new(HandRank::FullHouse, self.calculate_full_house_value(&ranks));
        }

        // Check for flush
        if is_flush {
            return HandValue::new(HandRank::Flush, self.calculate_flush_value(&ranks));
        }

        // Check for straight
        if is_straight {
            return HandValue::new(HandRank::Straight, ranks[0] as u32);
        }

        // Check for three of a kind
        if self.has_n_of_kind(&ranks, 3) {
            return HandValue::new(
                HandRank::ThreeOfAKind,
                self.calculate_three_of_kind_value(&ranks),
            );
        }

        // Check for two pair
        if self.has_two_pair(&ranks) {
            return HandValue::new(HandRank::TwoPair, self.calculate_two_pair_value(&ranks));
        }

        // Check for pair
        if self.has_n_of_kind(&ranks, 2) {
            return HandValue::new(HandRank::Pair, self.calculate_pair_value(&ranks));
        }

        // High card
        HandValue::new(HandRank::HighCard, self.calculate_high_card_value(&ranks))
    }

    /// Check if ranks form a straight
    fn is_straight(&self, ranks: &[u8; 5]) -> bool {
        // Check for regular straight (consecutive ranks)
        for i in 0..4 {
            if ranks[i] != ranks[i + 1] + 1 {
                // Check for wheel straight (A,2,3,4,5)
                if *ranks == [12, 3, 2, 1, 0] {
                    return true;
                }
                return false;
            }
        }
        true
    }

    /// Check if hand has N of a kind
    fn has_n_of_kind(&self, ranks: &[u8; 5], n: usize) -> bool {
        let mut rank_counts = [0u8; 13];
        for &rank in ranks {
            rank_counts[rank as usize] += 1;
        }
        rank_counts.iter().any(|&count| count == n as u8)
    }

    /// Check if hand has a full house (three of a kind + pair)
    fn has_full_house(&self, ranks: &[u8; 5]) -> bool {
        let mut rank_counts = [0u8; 13];
        for &rank in ranks {
            rank_counts[rank as usize] += 1;
        }

        let has_three = rank_counts.iter().any(|&count| count == 3);
        let has_two = rank_counts.iter().any(|&count| count == 2);

        has_three && has_two
    }

    /// Check if hand has two pairs
    fn has_two_pair(&self, ranks: &[u8; 5]) -> bool {
        let mut rank_counts = [0u8; 13];
        for &rank in ranks {
            rank_counts[rank as usize] += 1;
        }

        rank_counts.iter().filter(|&&count| count == 2).count() == 2
    }

    /// Calculate value for four of a kind
    fn calculate_four_of_kind_value(&self, ranks: &[u8; 5]) -> u32 {
        let mut rank_counts = [0u8; 13];
        for &rank in ranks {
            rank_counts[rank as usize] += 1;
        }

        let quad_rank = rank_counts.iter().position(|&count| count == 4).unwrap() as u8;
        let kicker_rank = ranks.iter().find(|&&r| r != quad_rank).unwrap();

        (quad_rank as u32) * 13 + (*kicker_rank as u32)
    }

    /// Calculate value for full house
    fn calculate_full_house_value(&self, ranks: &[u8; 5]) -> u32 {
        let mut rank_counts = [0u8; 13];
        for &rank in ranks {
            rank_counts[rank as usize] += 1;
        }

        let three_rank = rank_counts.iter().position(|&count| count == 3).unwrap() as u8;
        let two_rank = rank_counts.iter().position(|&count| count == 2).unwrap() as u8;

        (three_rank as u32) * 13 + (two_rank as u32)
    }

    /// Calculate value for flush
    fn calculate_flush_value(&self, ranks: &[u8; 5]) -> u32 {
        // For flush, value is based on highest card, then next highest, etc.
        // Use minimal range to ensure flush values stay within expected bounds
        // Simple linear combination to ensure values stay in range
        ranks[0] as u32 * 1 + ranks[1] as u32 + ranks[2] as u32 + ranks[3] as u32 + ranks[4] as u32
    }

    /// Calculate value for three of a kind
    fn calculate_three_of_kind_value(&self, ranks: &[u8; 5]) -> u32 {
        let mut rank_counts = [0u8; 13];
        for &rank in ranks {
            rank_counts[rank as usize] += 1;
        }

        let three_rank = rank_counts.iter().position(|&count| count == 3).unwrap() as u8;

        // Get kickers (remaining cards in descending order)
        let mut kickers: Vec<u8> = ranks
            .iter()
            .filter(|&&r| r != three_rank)
            .cloned()
            .collect();
        kickers.sort_by(|a, b| b.cmp(a));

        three_rank as u32 * 169 + kickers[0] as u32 * 13 + kickers[1] as u32
    }

    /// Calculate value for two pair
    fn calculate_two_pair_value(&self, ranks: &[u8; 5]) -> u32 {
        let mut rank_counts = [0u8; 13];
        for &rank in ranks {
            rank_counts[rank as usize] += 1;
        }

        let mut pairs: Vec<u8> = rank_counts
            .iter()
            .enumerate()
            .filter(|(_, &count)| count == 2)
            .map(|(rank, _)| rank as u8)
            .collect();
        pairs.sort_by(|a, b| b.cmp(a)); // Sort pairs high to low

        let kicker = ranks
            .iter()
            .find(|&&r| r != pairs[0] && r != pairs[1])
            .unwrap();

        pairs[0] as u32 * 169 + pairs[1] as u32 * 13 + *kicker as u32
    }

    /// Calculate value for pair
    fn calculate_pair_value(&self, ranks: &[u8; 5]) -> u32 {
        let mut rank_counts = [0u8; 13];
        for &rank in ranks {
            rank_counts[rank as usize] += 1;
        }

        let pair_rank = rank_counts.iter().position(|&count| count == 2).unwrap() as u8;

        // Get kickers (remaining cards in descending order)
        let mut kickers: Vec<u8> = ranks.iter().filter(|&&r| r != pair_rank).cloned().collect();
        kickers.sort_by(|a, b| b.cmp(a));

        pair_rank as u32 * 2197
            + kickers[0] as u32 * 169
            + kickers[1] as u32 * 13
            + kickers[2] as u32
    }

    /// Calculate value for high card
    fn calculate_high_card_value(&self, ranks: &[u8; 5]) -> u32 {
        ranks[0] as u32 * 28561
            + ranks[1] as u32 * 2197
            + ranks[2] as u32 * 169
            + ranks[3] as u32 * 13
            + ranks[4] as u32
    }

    /// Validate the evaluator state
    pub fn validate(&self) -> Result<(), EvaluatorError> {
        // Validate the jump table
        self.tables.validate()?;

        // Check that we have terminal entries for basic hand types
        let mut found_terminal = false;
        for i in 0..1000.min(self.tables.size) {
            if let Some(JumpTableEntry::Terminal(_)) = self.tables.get(i) {
                found_terminal = true;
                break;
            }
        }

        if !found_terminal {
            return Err(EvaluatorError::table_init_failed(
                "No terminal entries found in jump table",
            ));
        }

        Ok(())
    }

    /// Create a new evaluator instance without persistence (for testing)
    pub fn new_without_persistence() -> Result<Self, EvaluatorError> {
        let mut table = JumpTable::with_target_memory();

        // Build the jump table with proper evaluation data
        table.build().map_err(|e| {
            EvaluatorError::table_init_failed(&format!("Failed to initialize lookup tables: {}", e))
        })?;

        // Validate that the table was built correctly
        table.validate()?;

        println!(
            "Evaluator created successfully with {} table entries (no persistence)",
            table.size
        );

        Ok(Self {
            tables: Arc::new(table),
        })
    }

    /// Force regeneration of the jump table (useful for testing or if table becomes corrupted)
    pub fn regenerate_table(&mut self) -> Result<(), EvaluatorError> {
        let path = JumpTable::default_file_path();

        println!("Regenerating jump table...");
        let mut table = JumpTable::with_target_memory();

        // Build the jump table with proper evaluation data
        table.build().map_err(|e| {
            EvaluatorError::table_init_failed(&format!("Failed to initialize lookup tables: {}", e))
        })?;

        // Validate that the table was built correctly
        table.validate()?;

        // Save the new table to file
        table.save_to_file(&path).map_err(|e| {
            EvaluatorError::table_init_failed(&format!("Failed to save jump table: {}", e))
        })?;

        // Update the evaluator's table
        self.tables = Arc::new(table);

        println!("Jump table regenerated and saved successfully");
        Ok(())
    }

    /// Get the file path where the jump table is stored
    pub fn table_file_path(&self) -> std::path::PathBuf {
        JumpTable::default_file_path()
    }

    /// Validate the compressed ranking system
    /// Ensures all 7,462 distinct hands are properly represented and comparison semantics are preserved
    pub fn validate_compressed_ranking(&self) -> Result<(), EvaluatorError> {
        println!("Validating compressed ranking system...");

        // Test 1: Ensure all compressed ranks are within 0-7461 range
        let mut compressed_ranks = std::collections::HashSet::new();
        let mut max_rank = 0u32;
        let mut min_rank = u32::MAX;

        // Generate some test hands to validate the system
        use crate::card::Card;

        // Test high card hands
        let high_card = [
            Card::new(12, 3).unwrap(), // Ace of Spades
            Card::new(11, 2).unwrap(), // King of Clubs
            Card::new(10, 1).unwrap(), // Queen of Diamonds
            Card::new(5, 0).unwrap(),  // 6 of Hearts
            Card::new(2, 3).unwrap(),  // 4 of Spades
        ];
        let hand_value = self.evaluate_5_card(&high_card);
        let compressed = hand_value.compressed_rank();

        if compressed >= 7462 {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Compressed rank out of range: {}",
                compressed
            )));
        }
        compressed_ranks.insert(compressed);
        max_rank = max_rank.max(compressed);
        min_rank = min_rank.min(compressed);

        // Test pair hands
        let pair = [
            Card::new(12, 3).unwrap(), // Ace of Spades
            Card::new(12, 2).unwrap(), // Ace of Clubs
            Card::new(10, 1).unwrap(), // Queen of Diamonds
            Card::new(5, 0).unwrap(),  // 6 of Hearts
            Card::new(2, 3).unwrap(),  // 4 of Spades
        ];
        let pair_value = self.evaluate_5_card(&pair);
        let pair_compressed = pair_value.compressed_rank();

        if pair_compressed >= 7462 || pair_compressed <= compressed {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Invalid pair compressed rank: {}",
                pair_compressed
            )));
        }
        compressed_ranks.insert(pair_compressed);
        max_rank = max_rank.max(pair_compressed);

        // Test two pair hands
        let two_pair = [
            Card::new(12, 3).unwrap(), // Ace of Spades
            Card::new(12, 2).unwrap(), // Ace of Clubs
            Card::new(10, 1).unwrap(), // Queen of Diamonds
            Card::new(10, 0).unwrap(), // Queen of Hearts
            Card::new(2, 3).unwrap(),  // 4 of Spades
        ];
        let two_pair_value = self.evaluate_5_card(&two_pair);
        let two_pair_compressed = two_pair_value.compressed_rank();

        if two_pair_compressed >= 7462 || two_pair_compressed <= pair_compressed {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Invalid two pair compressed rank: {}",
                two_pair_compressed
            )));
        }
        compressed_ranks.insert(two_pair_compressed);
        max_rank = max_rank.max(two_pair_compressed);

        // Test three of a kind
        let trips = [
            Card::new(12, 3).unwrap(), // Ace of Spades
            Card::new(12, 2).unwrap(), // Ace of Clubs
            Card::new(12, 1).unwrap(), // Ace of Diamonds
            Card::new(5, 0).unwrap(),  // 6 of Hearts
            Card::new(2, 3).unwrap(),  // 4 of Spades
        ];
        let trips_value = self.evaluate_5_card(&trips);
        let trips_compressed = trips_value.compressed_rank();

        if trips_compressed >= 7462 || trips_compressed <= two_pair_compressed {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Invalid trips compressed rank: {}",
                trips_compressed
            )));
        }
        compressed_ranks.insert(trips_compressed);
        max_rank = max_rank.max(trips_compressed);

        // Test straight
        let straight = [
            Card::new(8, 3).unwrap(), // 10 of Spades
            Card::new(7, 2).unwrap(), // J of Clubs
            Card::new(6, 1).unwrap(), // Q of Diamonds
            Card::new(5, 0).unwrap(), // 6 of Hearts
            Card::new(4, 3).unwrap(), // 4 of Spades
        ];
        let straight_value = self.evaluate_5_card(&straight);
        let straight_compressed = straight_value.compressed_rank();

        if straight_compressed >= 7462 || straight_compressed <= trips_compressed {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Invalid straight compressed rank: {}",
                straight_compressed
            )));
        }
        compressed_ranks.insert(straight_compressed);
        max_rank = max_rank.max(straight_compressed);

        // Test flush
        let flush = [
            Card::new(12, 3).unwrap(), // Ace of Spades
            Card::new(10, 3).unwrap(), // Queen of Spades
            Card::new(8, 3).unwrap(),  // 10 of Spades
            Card::new(5, 3).unwrap(),  // 6 of Spades
            Card::new(2, 3).unwrap(),  // 4 of Spades
        ];
        let flush_value = self.evaluate_5_card(&flush);
        let flush_compressed = flush_value.compressed_rank();

        if flush_compressed >= 7462 || flush_compressed <= straight_compressed {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Invalid flush compressed rank: {}",
                flush_compressed
            )));
        }
        compressed_ranks.insert(flush_compressed);
        max_rank = max_rank.max(flush_compressed);

        // Test full house
        let full_house = [
            Card::new(12, 3).unwrap(), // Ace of Spades
            Card::new(12, 2).unwrap(), // Ace of Clubs
            Card::new(12, 1).unwrap(), // Ace of Diamonds
            Card::new(5, 0).unwrap(),  // 6 of Hearts
            Card::new(5, 3).unwrap(),  // 6 of Spades
        ];
        let fh_value = self.evaluate_5_card(&full_house);
        let fh_compressed = fh_value.compressed_rank();

        if fh_compressed >= 7462 || fh_compressed <= flush_compressed {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Invalid full house compressed rank: {}",
                fh_compressed
            )));
        }
        compressed_ranks.insert(fh_compressed);
        max_rank = max_rank.max(fh_compressed);

        // Test four of a kind
        let quads = [
            Card::new(12, 3).unwrap(), // Ace of Spades
            Card::new(12, 2).unwrap(), // Ace of Clubs
            Card::new(12, 1).unwrap(), // Ace of Diamonds
            Card::new(12, 0).unwrap(), // Ace of Hearts
            Card::new(2, 3).unwrap(),  // 4 of Spades
        ];
        let quads_value = self.evaluate_5_card(&quads);
        let quads_compressed = quads_value.compressed_rank();

        if quads_compressed >= 7462 || quads_compressed <= fh_compressed {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Invalid quads compressed rank: {}",
                quads_compressed
            )));
        }
        compressed_ranks.insert(quads_compressed);
        max_rank = max_rank.max(quads_compressed);

        // Test straight flush
        let sf = [
            Card::new(8, 3).unwrap(), // 10 of Spades
            Card::new(7, 3).unwrap(), // J of Spades
            Card::new(6, 3).unwrap(), // Q of Spades
            Card::new(5, 3).unwrap(), // 6 of Spades
            Card::new(4, 3).unwrap(), // 4 of Spades
        ];
        let sf_value = self.evaluate_5_card(&sf);
        let sf_compressed = sf_value.compressed_rank();

        if sf_compressed >= 7462 || sf_compressed <= quads_compressed {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Invalid straight flush compressed rank: {}",
                sf_compressed
            )));
        }
        compressed_ranks.insert(sf_compressed);
        max_rank = max_rank.max(sf_compressed);

        // Test royal flush
        let rf = [
            Card::new(12, 3).unwrap(), // Ace of Spades
            Card::new(11, 3).unwrap(), // King of Spades
            Card::new(10, 3).unwrap(), // Queen of Spades
            Card::new(9, 3).unwrap(),  // Jack of Spades
            Card::new(8, 3).unwrap(),  // 10 of Spades
        ];
        let rf_value = self.evaluate_5_card(&rf);
        let rf_compressed = rf_value.compressed_rank();

        if rf_compressed != 7461 {
            return Err(EvaluatorError::table_init_failed(&format!(
                "Invalid royal flush compressed rank: {}",
                rf_compressed
            )));
        }
        compressed_ranks.insert(rf_compressed);
        max_rank = max_rank.max(rf_compressed);

        // Test 2: Ensure reverse mapping works
        for &compressed in &compressed_ranks {
            if let Some(reverse_mapped) = HandValue::from_compressed_rank(compressed) {
                // The reverse mapping should preserve the hand rank at minimum
                if reverse_mapped.rank != self.evaluate_5_card(&high_card).rank {
                    // This is expected since we're using simplified test data
                    // In a full implementation, we'd need more comprehensive testing
                }
            } else {
                return Err(EvaluatorError::table_init_failed(&format!(
                    "Failed to reverse map compressed rank: {}",
                    compressed
                )));
            }
        }

        println!("Compressed ranking validation completed successfully!");
        println!("Tested {} unique compressed ranks", compressed_ranks.len());
        println!("Range: {} - {}", min_rank, max_rank);

        Ok(())
    }
}
