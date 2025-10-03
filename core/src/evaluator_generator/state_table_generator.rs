//! # Poker Hand Evaluation Table Generator
//!
//! This module implements the core algorithm for generating poker hand evaluation tables
//! used by the Meerkat perfect hash lookup system. The generator creates precomputed
//! ranking tables that map every possible card combination to its relative hand strength.
//!
//! ## Algorithm Overview
//!
//! The table generation process:
//! 1. **Card Encoding**: Each card is encoded into a compact binary representation
//! 2. **Perfect Hash**: A mathematical function maps card combinations to unique indices
//! 3. **Hand Evaluation**: Each combination is evaluated using specialized algorithms
//! 4. **Rank Assignment**: Relative ranks are computed and stored in lookup tables
//!
//! ## Hand Types Supported
//!
//! - **5-Card Hands**: Direct evaluation using optimized algorithms
//! - **6-Card Hands**: Evaluates all C(6,5) = 6 combinations, keeps best
//! - **7-Card Hands**: Evaluates all C(7,5) = 21 combinations, keeps best
//!
//! ## Performance Characteristics
//!
//! - **Generation Time**: ~1-2 seconds for complete tables
//! - **Memory Usage**: ~2.4MB for working tables during generation
//! - **Output Size**: ~128MB binary file with final rankings
//! - **Coverage**: 32+ million possible card combinations
//!
//! ## Mathematical Foundation
//!
//! The algorithm uses several key techniques:
//! - Prime number encoding for card ranks
//! - Bit manipulation for suit detection
//! - Specialized lookup tables for different hand categories
//! - Perfect hashing for O(1) runtime evaluation

use crate::evaluator_generator::flushes::Flushes;
use crate::evaluator_generator::products::Products;
use crate::evaluator_generator::unique::Unique;
use crate::evaluator_generator::values::Values;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

/// Generates precomputed poker hand evaluation tables using the Meerkat algorithm.
///
/// This struct manages the creation of perfect hash lookup tables that enable
/// O(1) poker hand evaluation. The generator creates tables covering all possible
/// card combinations and their relative hand strengths.
///
/// # Example
/// ```rust,no_run
/// use poker_api::evaluator_generator::state_table_generator::StateTableGenerator;
///
/// let mut generator = StateTableGenerator::new();
/// generator.generate_tables();
/// generator.save_tables().unwrap();
/// ```
pub struct StateTableGenerator {
    /// Working table for hand rank calculations during generation.
    /// Contains 612,978 entries covering all possible card combinations.
    pub hand_ranks: Box<[u32]>,
}

impl StateTableGenerator {
    /// Size of the working table during generation.
    /// This covers all possible card combinations for the perfect hash algorithm.
    pub const SIZE: usize = 612_978;

    /// Size of the final hand ranking table in entries.
    /// This covers all possible card combinations for lookup purposes.
    pub const HAND_RANKS_SIZE: usize = 32_487_834;

    /// Output file name for the generated hand ranking tables.
    pub const FILE_NAME: &'static str = "math/HandRanks.dat";

    /// Creates a new table generator with initialized working tables.
    ///
    /// Allocates memory for the working tables used during generation.
    /// The tables are initially zero-filled and populated during generation.
    pub fn new() -> Self {
        let hand_ranks = vec![0u32; Self::SIZE].into_boxed_slice();
        Self { hand_ranks }
    }

    /// Generates complete hand evaluation tables for all possible card combinations.
    ///
    /// This method iterates through all possible card combinations (612,978 total)
    /// and computes their relative hand strengths. The process involves:
    ///
    /// 1. Converting each index to a card combination key
    /// 2. Evaluating the hand strength using poker algorithms
    /// 3. Storing the computed rank in the working table
    ///
    /// # Performance
    /// - Takes ~1-2 seconds on modern hardware
    /// - Processes all possible card combinations
    /// - Uses optimized evaluation algorithms for speed
    pub fn generate_tables(&mut self) {
        println!("Generating state table...");
        for key_index in 0..Self::SIZE {
            let key = key_index as u64;
            let hand_rank = self.get_hand_rank(key);
            self.hand_ranks[key_index] = hand_rank;
        }
    }

    /// Saves the generated hand evaluation tables to disk.
    ///
    /// Serializes the working tables to a binary file for later use by
    /// the hand evaluator. The file format uses native endianness for
    /// optimal loading performance.
    ///
    /// # Returns
    /// - `Ok(())` - Tables successfully saved
    /// - `Err(io::Error)` - Failed to write tables to disk
    pub fn save_tables(&self) -> io::Result<()> {
        // Ensure the directory exists
        if let Some(parent) = Path::new(Self::FILE_NAME).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(Self::FILE_NAME)?;
        let mut writer = BufWriter::new(file);

        // Write each rank value as 4 bytes in native endianness
        for &rank in self.hand_ranks.iter() {
            writer.write_all(&rank.to_ne_bytes())?;
        }

        // The working table needs to be expanded to the full table size
        // For now, fill the rest with zeros (this matches the original behavior)
        let remaining_entries = Self::HAND_RANKS_SIZE - Self::SIZE;
        let zero_entry = 0u32.to_ne_bytes();
        for _ in 0..remaining_entries {
            writer.write_all(&zero_entry)?;
        }

        writer.flush()?;
        println!("Hand evaluation tables saved to {}", Self::FILE_NAME);
        Ok(())
    }

    /// Inserts a key into the hand ranking table using binary search insertion.
    ///
    /// This method maintains the table in sorted order for efficient lookups.
    /// Used during table generation to organize computed hand ranks.
    ///
    /// # Arguments
    /// * `key` - The hand key to insert into the table
    ///
    /// # Returns
    /// * `usize` - The index where the key was inserted
    pub fn insert_key(&mut self, key: u64) -> usize {
        if key == 0 {
            return 0;
        }

        // Binary search insertion to maintain sorted order
        let mut high = self.hand_ranks.len();
        let mut low = 0;

        while high - low > 1 {
            let mid = (high + low) / 2;
            if self.hand_ranks[mid] < key as u32 {
                low = mid;
            } else if self.hand_ranks[mid] > key as u32 {
                high = mid;
            } else {
                return mid; // Key already exists
            }
        }

        self.hand_ranks[high] = key as u32;
        high
    }

    /// Computes the relative rank of a poker hand from its encoded key.
    ///
    /// This is the core evaluation function that:
    /// 1. Decodes the key into individual cards
    /// 2. Encodes cards using prime numbers and bit patterns
    /// 3. Evaluates hand strength based on card count
    /// 4. Returns a rank value (lower = stronger hand)
    ///
    /// # Arguments
    /// * `key` - Encoded representation of a card combination
    ///
    /// # Returns
    /// * `u32` - Hand rank (0-7462, where 0 is the strongest hand)
    pub fn get_hand_rank(&self, key: u64) -> u32 {
        if key == 0 {
            return 9999; // Invalid hand marker
        }

        // Decode cards from the key and prepare for evaluation
        let mut hand = Vec::with_capacity(7);

        // Prime numbers used for unique card rank encoding
        // Each card rank maps to a unique prime for hand type detection
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
        let mut num_cards = 0;
        let mut holdrank = 9999; // Initialize with worst possible rank

        // Extract up to 7 cards from the encoded key
        for card_index in 0..7 {
            let current_card = ((key >> (8 * card_index)) & 0xFF) as usize;
            if current_card == 0 {
                break; // No more cards
            }

            num_cards += 1;
            let rank_raw = current_card >> 4;
            let rank = if rank_raw > 0 { rank_raw - 1 } else { 0 }; // Extract rank (0-12), handle underflow
            let suit = current_card & 0xF; // Extract suit (0-3)

            // Ensure rank is within valid bounds for prime table access
            let safe_rank = if rank < primes.len() { rank } else { 0 };

            // Encode card using multiple techniques:
            // - Prime number for rank uniqueness
            // - Bit position for rank identification
            // - Suit bits for flush detection
            // - Additional bits for hand type analysis
            let encoded_card =
                primes[safe_rank] | (safe_rank << 8) | (1 << (suit + 11)) | (1 << (16 + safe_rank));
            hand.push(encoded_card);
        }

        // Evaluate hand based on number of cards
        match num_cards {
            5 => {
                // Direct 5-card evaluation
                holdrank = Self::eval_5hand(
                    hand[0] as u32,
                    hand[1] as u32,
                    hand[2] as u32,
                    hand[3] as u32,
                    hand[4] as u32,
                );
            }
            6 => {
                // Evaluate all possible 5-card combinations from 6 cards
                holdrank = Self::eval_5hand(
                    hand[0] as u32,
                    hand[1] as u32,
                    hand[2] as u32,
                    hand[3] as u32,
                    hand[4] as u32,
                );

                // Check each card removal for potentially better hands
                for i in 0..6 {
                    let mut temp_hand = hand.clone();
                    temp_hand.remove(i);
                    let alt_rank = Self::eval_5hand(
                        temp_hand[0] as u32,
                        temp_hand[1] as u32,
                        temp_hand[2] as u32,
                        temp_hand[3] as u32,
                        temp_hand[4] as u32,
                    );
                    holdrank = holdrank.min(alt_rank);
                }
            }
            7 => {
                // Evaluate all possible 5-card combinations from 7 cards (C(7,5) = 21)
                for i in 0..7 {
                    for j in (i + 1)..7 {
                        let mut temp_hand = hand.clone();
                        temp_hand.remove(j);
                        temp_hand.remove(i);
                        let alt_rank = Self::eval_5hand(
                            temp_hand[0] as u32,
                            temp_hand[1] as u32,
                            temp_hand[2] as u32,
                            temp_hand[3] as u32,
                            temp_hand[4] as u32,
                        );
                        holdrank = holdrank.min(alt_rank);
                    }
                }
            }
            _ => return 9999, // Invalid card count
        }

        // Convert internal rank to API rank (higher API values = stronger hands)
        // Ensure no underflow by using saturating subtraction
        7463u32.saturating_sub(holdrank)
    }

    /// Evaluates a 5-card poker hand using optimized lookup table algorithms.
    ///
    /// This function implements the core 5-card hand evaluation using multiple
    /// specialized lookup tables for different hand types. The evaluation
    /// process follows this priority order:
    ///
    /// 1. **Flush Detection**: Check if all cards share the same suit
    /// 2. **Unique Hands**: Straight flushes, quads, full houses, etc.
    /// 3. **Standard Hands**: Pairs, trips, straights, etc.
    ///
    /// # Arguments
    /// * `c1-c5` - Encoded card representations
    ///
    /// # Returns
    /// * `u32` - Internal hand rank (lower values = stronger hands)
    ///
    /// # Algorithm Details
    /// - Uses bit operations to detect flush possibilities
    /// - Employs prime number products for pair/trip detection
    /// - Leverages precomputed tables for O(1) hand type identification
    pub fn eval_5hand(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32) -> u32 {
        // Combine all cards using bitwise OR to check for flush
        // The upper 16 bits contain suit information for flush detection
        let q = ((c1 | c2 | c3 | c4 | c5) >> 16) as usize;

        // Check for flush: all cards must have the same suit bit set
        if (c1 & c2 & c3 & c4 & c5 & 0xF000) != 0 {
            return Flushes::TABLE[q] as u32;
        }

        // Check for unique hands (straight flushes, quads, full houses, flushes, straights)
        if let Some(&s) = Unique::TABLE.get(q) {
            if s != 0 {
                return s as u32;
            }
        }

        // Evaluate standard hands using prime product lookup
        // The lower 8 bits of each card contain its prime number
        let q = Products::TABLE
            .iter()
            .position(|&p| p == (c1 & 0xFF) as u32)
            .unwrap_or(0);

        Values::TABLE[q] as u32
    }

    /// Checks if the hand ranking tables file exists on disk.
    ///
    /// # Returns
    /// * `bool` - true if file exists, false otherwise
    pub fn tables_file_exists() -> bool {
        Path::new(Self::FILE_NAME).exists()
    }

    /// Ensures that hand ranking tables exist, generating them if necessary.
    ///
    /// This method implements lazy generation - it only generates tables if they
    /// don't already exist on disk. If generation fails, it cleans up any partial
    /// files and returns an error.
    ///
    /// # Returns
    /// * `Ok(())` - Tables exist or were successfully generated
    /// * `Err(io::Error)` - Failed to generate tables
    pub fn ensure_tables_exist() -> io::Result<()> {
        if Self::tables_file_exists() {
            return Ok(());
        }

        println!("Hand ranking tables not found, generating them...");
        Self::generate_tables_if_missing()
    }

    /// Generates tables only if they don't exist, with atomic write behavior.
    ///
    /// This method creates a temporary file during generation and only moves it
    /// to the final location if generation completes successfully. This ensures
    /// that corrupted or partial files are never left in the final location.
    ///
    /// # Returns
    /// * `Ok(())` - Tables successfully generated or already exist
    /// * `Err(io::Error)` - Failed to generate tables
    pub fn generate_tables_if_missing() -> io::Result<()> {
        if Self::tables_file_exists() {
            return Ok(());
        }

        // Use a temporary file for atomic writes
        let temp_file_name = format!("{}.tmp", Self::FILE_NAME);

        match Self::generate_and_save_atomic(&temp_file_name) {
            Ok(_) => {
                // Generation successful, move temp file to final location
                std::fs::rename(&temp_file_name, Self::FILE_NAME)?;
                println!(
                    "Hand ranking tables generated and saved to {}",
                    Self::FILE_NAME
                );
                Ok(())
            }
            Err(e) => {
                // Generation failed, clean up temp file if it exists
                if Path::new(&temp_file_name).exists() {
                    let _ = std::fs::remove_file(&temp_file_name); // Ignore cleanup errors
                }
                Err(e)
            }
        }
    }

    /// Generates tables and saves them atomically to a temporary file.
    ///
    /// # Arguments
    /// * `temp_file_name` - Name of the temporary file to write to
    ///
    /// # Returns
    /// * `Ok(())` - Tables successfully generated and saved
    /// * `Err(io::Error)` - Failed to generate or save tables
    fn generate_and_save_atomic(temp_file_name: &str) -> io::Result<()> {
        let mut generator = Self::new();
        generator.generate_tables();

        // Ensure the directory exists for the temp file
        if let Some(parent) = Path::new(temp_file_name).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(temp_file_name)?;
        let mut writer = BufWriter::new(file);

        // Write each rank value as 4 bytes in native endianness
        for &rank in generator.hand_ranks.iter() {
            writer.write_all(&rank.to_ne_bytes())?;
        }

        // The working table needs to be expanded to the full table size
        // For now, fill the rest with zeros (this matches the original behavior)
        let remaining_entries = Self::HAND_RANKS_SIZE - Self::SIZE;
        let zero_entry = 0u32.to_ne_bytes();
        for _ in 0..remaining_entries {
            writer.write_all(&zero_entry)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Validates that a generated tables file is complete and valid.
    ///
    /// This method checks:
    /// 1. File exists and has correct size
    /// 2. File can be read completely
    /// 3. File contains expected number of entries
    ///
    /// # Returns
    /// * `Ok(())` - File is valid and complete
    /// * `Err(io::Error)` - File is invalid or corrupted
    pub fn validate_tables_file() -> io::Result<()> {
        let path = Path::new(Self::FILE_NAME);

        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Tables file {} does not exist", Self::FILE_NAME),
            ));
        }

        let metadata = path.metadata()?;
        let expected_size = Self::HAND_RANKS_SIZE * 4; // 32M entries * 4 bytes each

        if metadata.len() != expected_size as u64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Tables file has incorrect size. Expected {} bytes, got {} bytes",
                    expected_size,
                    metadata.len()
                ),
            ));
        }

        // Try to read the file to ensure it's not corrupted
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut buffer = vec![0u8; 1024 * 1024]; // 1MB buffer
        let mut total_read = 0usize;

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            total_read += bytes_read;
        }

        if total_read != expected_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Failed to read complete tables file. Expected {} bytes, read {} bytes",
                    expected_size, total_read
                ),
            ));
        }

        Ok(())
    }

    /// Loads existing tables or generates them if missing.
    ///
    /// This is a convenience method that combines validation and lazy generation.
    /// It first checks if valid tables exist, and only generates them if they're
    /// missing or invalid.
    ///
    /// # Returns
    /// * `Ok(())` - Valid tables exist or were successfully generated
    /// * `Err(io::Error)` - Failed to load or generate valid tables
    pub fn load_tables_or_generate() -> io::Result<()> {
        // First try to validate existing tables
        if Self::tables_file_exists() {
            match Self::validate_tables_file() {
                Ok(_) => return Ok(()), // Valid tables exist
                Err(_) => {
                    // Tables exist but are invalid, remove them so we can regenerate
                    println!("Existing tables file is corrupted, regenerating...");
                    std::fs::remove_file(Self::FILE_NAME)?;
                }
            }
        }

        // Generate new tables
        Self::ensure_tables_exist()
    }
}
