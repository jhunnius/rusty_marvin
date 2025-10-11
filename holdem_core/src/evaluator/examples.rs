//! # Comprehensive Usage Examples and Integration Guides
//!
//! This module provides practical examples and detailed integration guides
//! for using the LUT hand evaluator system in real-world applications.
//! From basic usage patterns to advanced poker engine implementations,
//! these examples demonstrate best practices and common use cases.
//!
//! ## Basic Usage Examples
//!
//! ### Simple Hand Evaluation
//!
//! ```rust
//! use math::{Evaluator, HandRank};
//! use holdem_core::{Card, Hand};
//! use std::str::FromStr;
//!
//! fn basic_hand_evaluation_example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Get the singleton evaluator instance
//!     let evaluator = Evaluator::instance();
//!
//!     // Create a hand from string notation
//!     let hand = Hand::from_notation("As Ks Qs Js Ts")?;
//!
//!     // Evaluate the hand
//!     let hand_value = evaluator.evaluate_hand(&hand);
//!
//!     println!("Hand: Royal Flush");
//!     println!("Rank: {:?}", hand_value.rank);
//!     println!("Value: {}", hand_value.value);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Direct Card Array Evaluation
//!
//! ```rust
//! use math::Evaluator;
//! use holdem_core::Card;
//! use std::str::FromStr;
//!
//! fn direct_card_evaluation_example() -> Result<(), Box<dyn std::error::Error>> {
//!     let evaluator = Evaluator::instance();
//!
//!     // Evaluate 5 cards directly
//!     let five_cards = [
//!         Card::from_str("As")?,
//!         Card::from_str("Ks")?,
//!         Card::from_str("Qs")?,
//!         Card::from_str("Js")?,
//!         Card::from_str("Ts")?,
//!     ];
//!
//!     let five_card_value = evaluator.evaluate_5_card(&five_cards);
//!     println!("5-card hand: {:?}", five_card_value);
//!
//!     // Evaluate 7 cards (finds best 5-card combination)
//!     let seven_cards = [
//!         Card::from_str("As")?,
//!         Card::from_str("Ks")?,
//!         Card::from_str("Qs")?,
//!         Card::from_str("Js")?,
//!         Card::from_str("Ts")?,
//!         Card::from_str("7h")?,
//!         Card::from_str("3d")?,
//!     ];
//!
//!     let seven_card_value = evaluator.evaluate_7_card(&seven_cards);
//!     println!("7-card hand: {:?}", seven_card_value);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Integration with Poker Applications
//!
//! ### Poker Hand Analyzer
//!
//! ```rust
//! use math::{Evaluator, HandRank};
//! use math::evaluator::integration::HandEvaluator;
//! use holdem_core::{Hand, HoleCards, Board};
//! use std::str::FromStr;
//! use std::collections::HashMap;
//!
//! struct PokerHandAnalyzer {
//!     evaluator: math::Evaluator,
//! }
//!
//! impl PokerHandAnalyzer {
//!     fn new() -> Result<Self, String> {
//!         let evaluator = Evaluator::instance();
//!         Ok(Self { evaluator })
//!     }
//!
//!     fn analyze_hand(&self, hand_notation: &str) -> Result<String, String> {
//!         let hand = Hand::from_notation(hand_notation)
//!             .map_err(|e| format!("Invalid hand notation: {}", e))?;
//!
//!         let hand_value = self.evaluator.evaluate_hand(&hand);
//!         let rank_name = HandEvaluator::hand_rank_name(hand_value.rank);
//!
//!         Ok(format!("{} ({})", rank_name, hand_value.value))
//!     }
//!
//!     fn analyze_hand_strength(&self, hole_cards: &HoleCards, board: &Board) -> Result<String, String> {
//!         let hand = Hand::from_hole_cards_and_board(hole_cards, board)
//!             .map_err(|e| format!("Invalid hand configuration: {}", e))?;
//!
//!         let hand_value = self.evaluator.evaluate_hand(&hand);
//!         let rank_name = HandEvaluator::hand_rank_name(hand_value.rank);
//!
//!         Ok(format!("{} ({})", rank_name, hand_value.value))
//!     }
//!
//!     fn compare_hands(&self, hand1: &str, hand2: &str) -> Result<String, String> {
//!         let h1 = Hand::from_notation(hand1)
//!             .map_err(|e| format!("Invalid hand 1: {}", e))?;
//!         let h2 = Hand::from_notation(hand2)
//!             .map_err(|e| format!("Invalid hand 2: {}", e))?;
//!
//!         let value1 = self.evaluator.evaluate_hand(&h1);
//!         let value2 = self.evaluator.evaluate_hand(&h2);
//!
//!         match value1.cmp(&value2) {
//!             std::cmp::Ordering::Greater => Ok(format!("{} beats {}", hand1, hand2)),
//!             std::cmp::Ordering::Less => Ok(format!("{} beats {}", hand2, hand1)),
//!             std::cmp::Ordering::Equal => Ok(format!("{} ties {}", hand1, hand2)),
//!         }
//!     }
//! }
//!
//! fn poker_analyzer_example() -> Result<(), Box<dyn std::error::Error>> {
//!     let analyzer = PokerHandAnalyzer::new()?;
//!
//!     // Analyze individual hands
//!     println!("{}", analyzer.analyze_hand("As Ks Qs Js Ts")?); // Royal Flush
//!     println!("{}", analyzer.analyze_hand("Ah Ac Ad As Kh")?); // Four of a Kind
//!     println!("{}", analyzer.analyze_hand("Ah Kd Qc Js Th")?); // Straight
//!
//!     // Compare hands
//!     println!("{}", analyzer.compare_hands("As Ks Qs Js Ts", "Ah Kh Qh Jh Th")?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Texas Hold'em Game State Evaluator
//!
//! ```rust
//! use math::Evaluator;
//! use math::evaluator::integration::HandEvaluator;
//! use holdem_core::{HoleCards, Board, Hand};
//! use std::str::FromStr;
//! use std::collections::HashMap;
//!
//! struct TexasHoldemEvaluator {
//!     evaluator: math::Evaluator,
//! }
//!
//! impl TexasHoldemEvaluator {
//!     fn new() -> Result<Self, String> {
//!         let evaluator = Evaluator::instance();
//!         Ok(Self { evaluator })
//!     }
//!
//!     fn evaluate_game_state(
//!         &self,
//!         hole_cards: &HoleCards,
//!         board: &Board,
//!         villain_hole_cards: Option<&HoleCards>
//!     ) -> Result<GameStateAnalysis, String> {
//!         let hand = Hand::from_hole_cards_and_board(hole_cards, board)
//!             .map_err(|e| format!("Invalid game state: {}", e))?;
//!
//!         let hand_value = self.evaluator.evaluate_hand(&hand);
//!         let rank_name = HandEvaluator::hand_rank_name(hand_value.rank);
//!
//!         let mut analysis = GameStateAnalysis {
//!             hand_strength: format!("{} ({})", rank_name, hand_value.value),
//!             hand_rank: hand_value.rank,
//!             hand_value: hand_value.value,
//!             board_texture: self.analyze_board_texture(board),
//!             showdown_value: None,
//!         };
//!
//!         // If villain's cards are known, evaluate showdown
//!         if let Some(villain_cards) = villain_hole_cards {
//!             let villain_hand = Hand::from_hole_cards_and_board(villain_cards, board)
//!                 .map_err(|e| format!("Invalid villain hand: {}", e))?;
//!
//!             let villain_value = self.evaluator.evaluate_hand(&villain_hand);
//!             let villain_rank_name = HandEvaluator::hand_rank_name(villain_value.rank);
//!
//!             analysis.showdown_value = Some(ShowdownAnalysis {
//!                 hero_strength: analysis.hand_strength.clone(),
//!                 villain_strength: format!("{} ({})", villain_rank_name, villain_value.value),
//!                 winner: match hand_value.cmp(&villain_value) {
//!                     std::cmp::Ordering::Greater => Player::Hero,
//!                     std::cmp::Ordering::Less => Player::Villain,
//!                     std::cmp::Ordering::Equal => Player::Tie,
//!                 }
//!             });
//!         }
//!
//!         Ok(analysis)
//!     }
//!
//!     fn analyze_board_texture(&self, board: &Board) -> BoardTexture {
//!         let board_cards = board.cards();
//!
//!         // Simple board texture analysis
//!         if board_cards.len() >= 3 {
//!             let ranks: Vec<u8> = board_cards.iter().map(|c| c.rank()).collect();
//!             let suits: Vec<u8> = board_cards.iter().map(|c| c.suit()).collect();
//!
//!             let is_flush_possible = suits.iter().any(|&suit| {
//!                 suits.iter().filter(|&&s| s == suit).count() >= 3
//!             });
//!
//!             let is_straight_possible = self.is_straight_possible(&ranks);
//!
//!             if is_flush_possible && is_straight_possible {
//!                 BoardTexture::FlushDraw
//!             } else if is_flush_possible {
//!                 BoardTexture::FlushPossible
//!             } else if is_straight_possible {
//!                 BoardTexture::StraightPossible
//!             } else if ranks.iter().all(|&r| r >= 10) {
//!                 BoardTexture::HighCard
//!             } else {
//!                 BoardTexture::Dry
//!             }
//!         } else {
//!             BoardTexture::Dry
//!         }
//!     }
//!
//!     fn is_straight_possible(&self, ranks: &[u8]) -> bool {
//!         // Simplified straight possibility check
//!         let mut sorted_ranks = ranks.to_vec();
//!         sorted_ranks.sort();
//!
//!         // Check for consecutive ranks or wheel possibility
//!         for i in 0..sorted_ranks.len() - 1 {
//!             if sorted_ranks[i + 1] - sorted_ranks[i] == 1 {
//!                 return true;
//!             }
//!         }
//!
//!         // Check for wheel possibility (A,2,3,4,5)
//!         sorted_ranks.contains(&0) &&
//!         sorted_ranks.contains(&1) &&
//!         sorted_ranks.contains(&2) &&
//!         sorted_ranks.contains(&3) &&
//!         sorted_ranks.contains(&12)
//!     }
//! }
//!
//! #[derive(Debug, Clone)]
//! struct GameStateAnalysis {
//!     hand_strength: String,
//!     hand_rank: HandRank,
//!     hand_value: u32,
//!     board_texture: BoardTexture,
//!     showdown_value: Option<ShowdownAnalysis>,
//! }
//!
//! #[derive(Debug, Clone)]
//! struct ShowdownAnalysis {
//!     hero_strength: String,
//!     villain_strength: String,
//!     winner: Player,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Player {
//!     Hero,
//!     Villain,
//!     Tie,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum BoardTexture {
//!     Dry,
//!     HighCard,
//!     FlushPossible,
//!     StraightPossible,
//!     FlushDraw,
//! }
//!
//! fn texas_holdem_example() -> Result<(), Box<dyn std::error::Error>> {
//!     let evaluator = TexasHoldemEvaluator::new()?;
//!
//!     // Example: Hero has AK, board is QJT
//!     let hero_cards = HoleCards::from_strings(&["As", "Ks"])?;
//!     let board = Board::from_strings(&["Qs", "Js", "Ts", "7h"])?;
//!
//!     let analysis = evaluator.evaluate_game_state(&hero_cards, &board, None)?;
//!     println!("Game state analysis: {:?}", analysis);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Performance Optimization Examples
//!
//! ### High-Performance Poker Engine
//!
//! ```rust
//! use math::Evaluator;
//! use holdem_core::{Card, Hand};
//! use std::time::{Instant, Duration};
//! use std::str::FromStr;
//!
//! struct HighPerformancePokerEngine {
//!     evaluator: math::Evaluator,
//!     stats: EngineStats,
//! }
//!
//! #[derive(Default)]
//! struct EngineStats {
//!     total_evaluations: u64,
//!     total_time: Duration,
//!     cache_hits: u64,
//!     cache_misses: u64,
//! }
//!
//! impl HighPerformancePokerEngine {
//!     fn new() -> Result<Self, String> {
//!         let evaluator = Evaluator::instance();
//!
//!         // Validate tables are ready
//!         if !evaluator.validate_table_files().unwrap_or(false) {
//!             println!("Warning: Table files may need initialization");
//!         }
//!
//!         Ok(Self {
//!             evaluator,
//!             stats: EngineStats::default(),
//!         })
//!     }
//!
//!     fn evaluate_hand_fast(&mut self, cards: &[Card; 5]) -> math::HandValue {
//!         let start = Instant::now();
//!         let result = self.evaluator.evaluate_5_card(cards);
//!         let elapsed = start.elapsed();
//!
//!         self.stats.total_evaluations += 1;
//!         self.stats.total_time += elapsed;
//!
//!         result
//!     }
//!
//!     fn batch_evaluate_hands(&mut self, hands: &[Hand]) -> Vec<math::HandValue> {
//!         let start = Instant::now();
//!
//!         let results: Vec<_> = hands.iter()
//!             .map(|hand| self.evaluator.evaluate_hand(hand))
//!             .collect();
//!
//!         let elapsed = start.elapsed();
//!         let evaluations_per_second = hands.len() as f64 / elapsed.as_secs_f64();
//!
//!         println!("Batch evaluation: {:.0} hands/sec", evaluations_per_second);
//!
//!         self.stats.total_evaluations += hands.len() as u64;
//!         self.stats.total_time += elapsed;
//!
//!         results
//!     }
//!
//!     fn get_performance_stats(&self) -> PerformanceStats {
//!         let avg_time_per_evaluation = if self.stats.total_evaluations > 0 {
//!             self.stats.total_time / self.stats.total_evaluations as u32
//!         } else {
//!             Duration::default()
//!         };
//!
//!         let evaluations_per_second = if self.stats.total_time.as_secs_f64() > 0.0 {
//!             self.stats.total_evaluations as f64 / self.stats.total_time.as_secs_f64()
//!         } else {
//!             0.0
//!         };
//!
//!         PerformanceStats {
//!             total_evaluations: self.stats.total_evaluations,
//!             total_time: self.stats.total_time,
//!             avg_time_per_evaluation,
//!             evaluations_per_second,
//!         }
//!     }
//!
//!     fn benchmark_evaluation_speed(&mut self) -> BenchmarkResults {
//!         let test_cases = vec![
//!             // Royal flush
//!             [
//!                 Card::from_str("As").unwrap(),
//!                 Card::from_str("Ks").unwrap(),
//!                 Card::from_str("Qs").unwrap(),
//!                 Card::from_str("Js").unwrap(),
//!                 Card::from_str("Ts").unwrap(),
//!             ],
//!             // Four of a kind
//!             [
//!                 Card::from_str("Ah").unwrap(),
//!                 Card::from_str("Ac").unwrap(),
//!                 Card::from_str("Ad").unwrap(),
//!                 Card::from_str("As").unwrap(),
//!                 Card::from_str("Kh").unwrap(),
//!             ],
//!             // Full house
//!             [
//!                 Card::from_str("Ah").unwrap(),
//!                 Card::from_str("Ac").unwrap(),
//!                 Card::from_str("Ad").unwrap(),
//!                 Card::from_str("Ks").unwrap(),
//!                 Card::from_str("Kh").unwrap(),
//!             ],
//!         ];
//!
//!         let mut results = BenchmarkResults::default();
//!
//!         for (test_name, cards) in vec![
//!             ("5-card", test_cases),
//!         ] {
//!             let iterations = 100_000;
//!             let start = Instant::now();
//!
//!             for _ in 0..iterations {
//!                 for hand in &test_cases {
//!                     let _result = self.evaluator.evaluate_5_card(hand);
//!                 }
//!             }
//!
//!             let duration = start.elapsed();
//!             let total_evaluations = iterations * test_cases.len();
//!             let evaluations_per_second = total_evaluations as f64 / duration.as_secs_f64();
//!
//!             println!("{}: {:.0} evaluations/sec", test_name, evaluations_per_second);
//!
//!             results.benchmarks.push(BenchmarkResult {
//!                 test_name: test_name.to_string(),
//!                 evaluations_per_second,
//!                 total_time: duration,
//!                 iterations,
//!             });
//!         }
//!
//!         results
//!     }
//! }
//!
//! #[derive(Debug, Clone)]
//! struct PerformanceStats {
//!     total_evaluations: u64,
//!     total_time: Duration,
//!     avg_time_per_evaluation: Duration,
//!     evaluations_per_second: f64,
//! }
//!
//! #[derive(Debug, Default)]
//! struct BenchmarkResults {
//!     benchmarks: Vec<BenchmarkResult>,
//! }
//!
//! #[derive(Debug)]
//! struct BenchmarkResult {
//!     test_name: String,
//!     evaluations_per_second: f64,
//!     total_time: Duration,
//!     iterations: usize,
//! }
//!
//! fn performance_engine_example() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut engine = HighPerformancePokerEngine::new()?;
//!
//!     // Run performance benchmarks
//!     let results = engine.benchmark_evaluation_speed();
//!     println!("Benchmark results: {:?}", results);
//!
//!     // Get performance statistics
//!     let stats = engine.get_performance_stats();
//!     println!("Engine stats: {:?}", stats);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling and Recovery Examples
//!
//! ### Robust Application Integration
//!
//! ```rust
//! use math::{Evaluator, evaluator::errors::EvaluatorError};
//! use math::evaluator::file_io::{LutFileManager, TableType};
//! use std::io;
//!
//! struct RobustPokerApplication {
//!     evaluator: Option<math::Evaluator>,
//!     file_manager: LutFileManager,
//! }
//!
//! impl RobustPokerApplication {
//!     fn new() -> Result<Self, String> {
//!         let file_manager = LutFileManager::default();
//!
//!         // Try to initialize evaluator
//!         let evaluator = match Evaluator::new() {
//!             Ok(evaluator) => {
//!                 println!("Evaluator initialized successfully");
//!                 Some(evaluator)
//!             }
//!             Err(EvaluatorError::FileNotFound(_)) => {
//!                 println!("Table files not found - will be generated on first use");
//!                 None
//!             }
//!             Err(EvaluatorError::ChecksumValidationFailed(msg)) => {
//!                 println!("Table files corrupted: {} - regenerating", msg);
//!                 // Try to delete corrupted files
//!                 for table_type in [TableType::FiveCard, TableType::SixCard, TableType::SevenCard] {
//!                     let _ = file_manager.delete_table(table_type);
//!                 }
//!                 None
//!             }
//!             Err(e) => {
//!                 println!("Cannot initialize evaluator: {}", e);
//!                 return Err(format!("Evaluator initialization failed: {}", e));
//!             }
//!         };
//!
//!         Ok(Self { evaluator, file_manager })
//!     }
//!
//!     fn ensure_evaluator(&mut self) -> Result<&math::Evaluator, String> {
//!         if self.evaluator.is_none() {
//!             match Evaluator::new() {
//!                 Ok(evaluator) => {
//!                     self.evaluator = Some(evaluator);
//!                     println!("Evaluator initialized on-demand");
//!                 }
//!                 Err(e) => {
//!                     return Err(format!("Failed to initialize evaluator: {}", e));
//!                 }
//!             }
//!         }
//!
//!         Ok(self.evaluator.as_ref().unwrap())
//!     }
//!
//!     fn evaluate_hand_safe(&mut self, notation: &str) -> Result<String, String> {
//!         use holdem_core::Hand;
//!
//!         let evaluator = self.ensure_evaluator()?;
//!         let hand = Hand::from_notation(notation)
//!             .map_err(|e| format!("Invalid hand notation: {}", e))?;
//!
//!         let hand_value = evaluator.evaluate_hand(&hand);
//!         let rank_name = math::evaluator::integration::HandEvaluator::hand_rank_name(hand_value.rank);
//!
//!         Ok(format!("{} ({})", rank_name, hand_value.value))
//!     }
//!
//!     fn check_system_health(&self) -> SystemHealth {
//!         let mut health = SystemHealth::default();
//!
//!         // Check table file status
//!         for table_type in [TableType::FiveCard, TableType::SixCard, TableType::SevenCard] {
//!             if self.file_manager.table_exists(table_type) {
//!                 health.table_files_available += 1;
//!
//!                 if let Ok(info) = self.file_manager.get_table_info(table_type) {
//!                     health.total_table_size += info.size;
//!                 }
//!             } else {
//!                 health.missing_tables.push(table_type);
//!             }
//!         }
//!
//!         // Check evaluator status
//!         health.evaluator_ready = self.evaluator.is_some();
//!
//!         health
//!     }
//! }
//!
//! #[derive(Debug, Default)]
//! struct SystemHealth {
//!     evaluator_ready: bool,
//!     table_files_available: usize,
//!     total_table_size: u64,
//!     missing_tables: Vec<TableType>,
//! }
//!
//! fn robust_application_example() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut app = RobustPokerApplication::new()?;
//!
//!     // Check system health
//!     let health = app.check_system_health();
//!     println!("System health: {:?}", health);
//!
//!     // Evaluate hands (will initialize evaluator if needed)
//!     println!("{}", app.evaluate_hand_safe("As Ks Qs Js Ts")?);
//!     println!("{}", app.evaluate_hand_safe("Ah Ac Ad As Kh")?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Poker Analysis Examples
//!
//! ### Hand Range Analysis
//!
//! ```rust
//! use math::Evaluator;
//! use math::evaluator::integration::HandEvaluator;
//! use holdem_core::{HoleCards, Board, Hand};
//! use std::str::FromStr;
//! use std::collections::HashMap;
//!
//! struct HandRangeAnalyzer {
//!     evaluator: math::Evaluator,
//! }
//!
//! impl HandRangeAnalyzer {
//!     fn new() -> Result<Self, String> {
//!         let evaluator = Evaluator::instance();
//!         Ok(Self { evaluator })
//!     }
//!
//!     fn analyze_hand_range(
//!         &self,
//!         hole_cards_list: &[&str],
//!         board: &Board
//!     ) -> Result<RangeAnalysis, String> {
//!         let mut analysis = RangeAnalysis::default();
//!
//!         for notation in hole_cards_list {
//!             let hole_cards = HoleCards::from_strings(&[notation])
//!                 .map_err(|e| format!("Invalid hole cards '{}': {}", notation, e))?;
//!
//!             let hand = Hand::from_hole_cards_and_board(&hole_cards, board)
//!                 .map_err(|e| format!("Cannot create hand for '{}': {}", notation, e))?;
//!
//!             let hand_value = self.evaluator.evaluate_hand(&hand);
//!             let rank_name = HandEvaluator::hand_rank_name(hand_value.rank);
//!
//!             analysis.hands_analyzed += 1;
//!
//!             match hand_value.rank {
//!                 HandRank::RoyalFlush => analysis.royal_flushes += 1,
//!                 HandRank::StraightFlush => analysis.straight_flushes += 1,
//!                 HandRank::FourOfAKind => analysis.quads += 1,
//!                 HandRank::FullHouse => analysis.full_houses += 1,
//!                 HandRank::Flush => analysis.flushes += 1,
//!                 HandRank::Straight => analysis.straights += 1,
//!                 HandRank::ThreeOfAKind => analysis.trips += 1,
//!                 HandRank::TwoPair => analysis.two_pairs += 1,
//!                 HandRank::Pair => analysis.pairs += 1,
//!                 HandRank::HighCard => analysis.high_cards += 1,
//!             }
//!
//!             analysis.strengths.push(HandStrength {
//!                 notation: notation.to_string(),
//!                 rank: hand_value.rank,
//!                 value: hand_value.value,
//!                 rank_name: rank_name.to_string(),
//!             });
//!         }
//!
//!         // Sort by hand strength
//!         analysis.strengths.sort_by(|a, b| b.value.cmp(&a.value));
//!
//!         Ok(analysis)
//!     }
//! }
//!
//! #[derive(Debug, Default)]
//! struct RangeAnalysis {
//!     hands_analyzed: usize,
//!     royal_flushes: usize,
//!     straight_flushes: usize,
//!     quads: usize,
//!     full_houses: usize,
//!     flushes: usize,
//!     straights: usize,
//!     trips: usize,
//!     two_pairs: usize,
//!     pairs: usize,
//!     high_cards: usize,
//!     strengths: Vec<HandStrength>,
//! }
//!
//! #[derive(Debug)]
//! struct HandStrength {
//!     notation: String,
//!     rank: HandRank,
//!     value: u32,
//!     rank_name: String,
//! }
//!
//! fn range_analysis_example() -> Result<(), Box<dyn std::error::Error>> {
//!     let analyzer = HandRangeAnalyzer::new()?;
//!     let board = Board::from_strings(&["Qs", "Js", "Ts", "7h", "3d"])?;
//!
//!     let premium_hands = vec![
//!         "As Ks", "Ah Kh", "Ac Kc", "Ad Kd",
//!         "As Qs", "Ah Qh", "Ac Qc", "Ad Qd",
//!         "Js Ts", "Jh Th", "Jc Tc", "Jd Td",
//!     ];
//!
//!     let analysis = analyzer.analyze_hand_range(&premium_hands, &board)?;
//!     println!("Range analysis: {:?}", analysis);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Testing and Validation Examples
//!
//! ### Comprehensive Test Suite
//!
//! ```rust
//! #[cfg(test)]
//! mod comprehensive_tests {
//!     use super::*;
//!     use math::{Evaluator, HandRank};
//!     use math::evaluator::integration::HandEvaluator;
//!     use holdem_core::{Card, Hand};
//!     use std::str::FromStr;
//!
//!     #[test]
//!     fn test_all_hand_types() {
//!         let evaluator = Evaluator::instance();
//!
//!         let test_cases = vec![
//!             ("As Ks Qs Js Ts", HandRank::RoyalFlush),
//!             ("9h 8h 7h 6h 5h", HandRank::StraightFlush),
//!             ("Ah Ac Ad As Kh", HandRank::FourOfAKind),
//!             ("Ah Ac Ad Ks Kh", HandRank::FullHouse),
//!             ("Ah Kh Qh Jh 7h", HandRank::Flush),
//!             ("Ah Kd Qc Js Th", HandRank::Straight),
//!             ("Ah Ac Ad Ks Qh", HandRank::ThreeOfAKind),
//!             ("Ah Ac Kd Ks Qh", HandRank::TwoPair),
//!             ("Ah Ac Kd Qs Jh", HandRank::Pair),
//!             ("Ah Kd Qc Js 9h", HandRank::HighCard),
//!         ];
//!
//!         for (notation, expected_rank) in test_cases {
//!             let hand = Hand::from_notation(notation).unwrap();
//!             let hand_value = evaluator.evaluate_hand(&hand);
//!
//!             assert_eq!(
//!                 hand_value.rank,
//!                 expected_rank,
//!                 "Hand '{}' evaluated as {:?}, expected {:?}",
//!                 notation,
//!                 hand_value.rank,
//!                 expected_rank
//!             );
//!         }
//!     }
//!
//!     #[test]
//!     fn test_hand_comparison() {
//!         let evaluator = Evaluator::instance();
//!
//!         // Test that better hands have higher values
//!         let royal_flush = Hand::from_notation("As Ks Qs Js Ts").unwrap();
//!         let straight_flush = Hand::from_notation("9h 8h 7h 6h 5h").unwrap();
//!         let four_of_kind = Hand::from_notation("Ah Ac Ad As Kh").unwrap();
//!
//!         let royal_value = evaluator.evaluate_hand(&royal_flush);
//!         let straight_flush_value = evaluator.evaluate_hand(&straight_flush);
//!         let four_value = evaluator.evaluate_hand(&four_of_kind);
//!
//!         assert!(royal_value > straight_flush_value);
//!         assert!(straight_flush_value > four_value);
//!     }
//!
//!     #[test]
//!     fn test_deterministic_evaluation() {
//!         let evaluator = Evaluator::instance();
//!         let hand = Hand::from_notation("As Ks Qs Js Ts").unwrap();
//!
//!         // Evaluate same hand multiple times
//!         let value1 = evaluator.evaluate_hand(&hand);
//!         let value2 = evaluator.evaluate_hand(&hand);
//!         let value3 = evaluator.evaluate_hand(&hand);
//!
//!         assert_eq!(value1, value2);
//!         assert_eq!(value2, value3);
//!     }
//!
//!     #[test]
//!     fn test_error_handling() {
//!         // Test that invalid inputs are handled gracefully
//!         let result = HandEvaluator::evaluate_from_notation("invalid hand");
//!
//!         assert!(result.is_err());
//!
//!         if let Err(e) = result {
//!             println!("Appropriate error for invalid input: {}", e);
//!         }
//!     }
//! }
//! ```
//!
//! ## Real-World Application Examples
//!
//! ### Poker Odds Calculator
//!
//! ```rust
//! use math::Evaluator;
//! use math::evaluator::integration::HandEvaluator;
//! use holdem_core::{HoleCards, Board, Hand};
//! use std::str::FromStr;
//!
//! struct PokerOddsCalculator {
//!     evaluator: math::Evaluator,
//! }
//!
//! impl PokerOddsCalculator {
//!     fn new() -> Result<Self, String> {
//!         let evaluator = Evaluator::instance();
//!         Ok(Self { evaluator })
//!     }
//!
//!     fn calculate_hand_odds(
//!         &self,
//!         hole_cards: &HoleCards,
//!         board: &Board,
//!         opponents: usize
//!     ) -> Result<OddsCalculation, String> {
//!         let hero_hand = Hand::from_hole_cards_and_board(hole_cards, board)
//!             .map_err(|e| format!("Invalid hero hand: {}", e))?;
//!
//!         let hero_value = self.evaluator.evaluate_hand(&hero_hand);
//!
//!         // Simulate opponent hands (simplified - in reality would use range analysis)
//!         let mut wins = 0;
//!         let mut ties = 0;
//!         let mut losses = 0;
//!
//!         // This is a simplified simulation - real implementation would
//!         // enumerate possible opponent hands based on ranges
//!         let simulations = 1000;
//!
//!         for _ in 0..simulations {
//!             // Simplified opponent hand generation
//!             let opponent_cards = HoleCards::from_strings(&["7h", "6d"]).unwrap();
//!             let opponent_hand = Hand::from_hole_cards_and_board(&opponent_cards, board)
//!                 .map_err(|e| format!("Invalid opponent hand: {}", e))?;
//!
//!             let opponent_value = self.evaluator.evaluate_hand(&opponent_hand);
//!
//!             match hero_value.cmp(&opponent_value) {
//!                 std::cmp::Ordering::Greater => wins += 1,
//!                 std::cmp::Ordering::Equal => ties += 1,
//!                 std::cmp::Ordering::Less => losses += 1,
//!             }
//!         }
//!
//!         let win_percentage = wins as f64 / simulations as f64 * 100.0;
//!         let tie_percentage = ties as f64 / simulations as f64 * 100.0;
//!         let loss_percentage = losses as f64 / simulations as f64 * 100.0;
//!
//!         Ok(OddsCalculation {
//!             hero_hand: HandEvaluator::format_hand_value(hero_value),
//!             win_percentage,
//!             tie_percentage,
//!             loss_percentage,
//!             simulations,
//!         })
//!     }
//! }
//!
//! #[derive(Debug)]
//! struct OddsCalculation {
//!     hero_hand: String,
//!     win_percentage: f64,
//!     tie_percentage: f64,
//!     loss_percentage: f64,
//!     simulations: usize,
//! }
//!
//! fn odds_calculator_example() -> Result<(), Box<dyn std::error::Error>> {
//!     let calculator = PokerOddsCalculator::new()?;
//!
//!     let hole_cards = HoleCards::from_strings(&["As", "Ks"])?;
//!     let board = Board::from_strings(&["Qs", "Js", "Ts", "7h"])?;
//!
//!     let odds = calculator.calculate_hand_odds(&hole_cards, &board, 1)?;
//!     println!("Odds calculation: {:?}", odds);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Best Practices Summary
//!
//! ### Performance Optimization
//! - **Cache evaluator instance**: Store in application state rather than accessing repeatedly
//! - **Batch evaluations**: Evaluate multiple hands together when possible
//! - **Pre-validate inputs**: Check hand notation before evaluation
//! - **Monitor performance**: Track evaluation speed for optimization opportunities
//!
//! ### Error Handling
//! - **Graceful degradation**: Continue operation even with partial failures
//! - **User-friendly messages**: Provide clear error messages for end users
//! - **Recovery mechanisms**: Implement automatic recovery where possible
//! - **Logging strategy**: Log errors for debugging without exposing to users
//!
//! ### Code Organization
//! - **Separation of concerns**: Use integration layer for type conversion only
//! - **Consistent patterns**: Use similar error handling patterns throughout application
//! - **Documentation**: Document expected input formats and behavior
//! - **Testing**: Test both success and error paths thoroughly
//!
//! ### Production Deployment
//! - **Health monitoring**: Monitor table file integrity and performance
//! - **Backup strategy**: Consider backing up table files for fast recovery
//! - **Update mechanisms**: Plan for table file updates and version management
//! - **Resource allocation**: Ensure sufficient memory for table loading
