//! # Poker API Module - Bot Testing Framework
//!
//! This module provides the core application programming interface (API) specifically
//! designed for poker bot testing and development. It offers Java-compatible types,
//! traits, and functionality optimized for automated poker bot testing frameworks,
//! ensuring consistent behavior across different poker tools and testing environments.
//!
//! ## Poker Testbed Context
//!
//! The API module serves as the foundation for comprehensive poker bot testing:
//! - **Bot Development**: Standardized interfaces for poker bot implementations
//! - **Testing Frameworks**: Consistent APIs for automated bot testing scenarios
//! - **Tool Integration**: Java-compatible interfaces for existing poker tool ecosystems
//! - **Performance Testing**: Efficient data structures for high-performance bot evaluation
//! - **Cross-Platform Testing**: Consistent behavior across different poker environments
//!
//! ## Java Compatibility Design
//!
//! All API components maintain strict Java compatibility:
//! - **Card Encoding**: Uses Java Meerkat API card representation standards
//! - **Hand Evaluation**: Compatible with Java poker hand evaluation results
//! - **Action Types**: Matches Java poker action semantics and behavior
//! - **Game State**: Provides Java-compatible game state information
//! - **Observer Pattern**: Event system compatible with Java poker frameworks
//!
//! ## Module Overview
//!
//! The API module is organized into bot-focused areas:
//! - **Card Management**: [`card`] - Java-compatible playing card representation
//! - **Hand Management**: [`hand`] - Poker hand collections optimized for bot testing
//! - **Deck Operations**: [`deck`] - Standard 52-card deck with bot-friendly operations
//! - **Player Actions**: [`action`] - Betting actions for bot decision making
//! - **Player Interface**: [`player`] - Trait-based bot implementation interface
//! - **Game Information**: [`game_info`] - Game state access for bot analysis
//! - **Game Observation**: [`game_observer`] - Event-driven updates for bot testing
//! - **Player Information**: [`player_info`] - Individual player state for bot queries
//! - **Preferences**: [`preferences`] - Configuration management for bot customization
//! - **Hand Evaluation**: [`hand_eval`] - Trait for pluggable evaluation algorithms
//!
//! ## Core Types
//!
//! ### Card and Hand Management
//! - [`Card`](card::Card) - Individual playing card with rank and suit
//! - [`Hand`](hand::Hand) - Collection of cards (up to 7 for Texas Hold'em)
//! - [`Deck`](deck::Deck) - Standard 52-card deck with shuffle/deal operations
//!
//! ### Player and Actions
//! - [`Action`](action::Action) - Player betting actions (bet, raise, fold, etc.)
//! - [`Player`](player::Player) - Trait for poker bot implementations
//! - [`PlayerInfo`](player_info::PlayerInfo) - Individual player state information
//!
//! ### Game State
//! - [`GameInfo`](game_info::GameInfo) - Comprehensive game state access
//! - [`GameObserver`](game_observer::GameObserver) - Event notification system
//! - [`Preferences`](preferences::Preferences) - Configuration management
//!
//! ## Key Features for Bot Testing
//!
//! - **Java Compatibility**: Full compatibility with Java Meerkat API for tool integration
//! - **Bot-Friendly APIs**: Optimized interfaces for poker bot development and testing
//! - **Type Safety**: Strongly typed card and hand representations for reliable bot code
//! - **Memory Efficiency**: Compact binary representations for high-performance testing
//! - **Extensible Design**: Trait-based architecture for custom bot implementations
//! - **Event-Driven**: Observer pattern for reactive bot testing scenarios
//! - **Configuration**: Flexible preference system for bot customization and testing
//! - **Performance Optimized**: Fast operations for large-scale bot testing scenarios
//!
//! ## Examples - Bot Testing Scenarios
//!
//! ### Basic Bot Card and Hand Operations
//!
//! ```rust
//! use poker_api::api::{card::Card, hand::Hand, deck::Deck};
//!
//! // Create hole cards for bot analysis (Java-compatible encoding)
//! let ace_spades = Card::from_string("As").unwrap();
//! let king_spades = Card::from_string("Ks").unwrap();
//!
//! let mut hole_cards = Hand::new();
//! hole_cards.add_card(ace_spades).unwrap();
//! hole_cards.add_card(king_spades).unwrap();
//!
//! // Bot can quickly assess hole card strength
//! println!("Hole cards: {}", hole_cards.to_string());
//! println!("Card count: {}", hole_cards.size());
//! ```
//!
//! ### Bot Action Decision Making
//!
//! ```rust
//! use poker_api::api::action::{Action, ActionType};
//!
//! // Bot creates actions for different scenarios
//! let fold_action = Action::fold_action(50.0);      // Weak hand
//! let bet_action = Action::bet_action(100.0);       // Strong hand
//! let raise_action = Action::raise_action(50.0, 200.0); // Premium hand
//!
//! // Bot can analyze action properties for decision making
//! println!("Action type: {:?}", fold_action.get_action_type());
//! println!("Amount: ${}", fold_action.get_amount());
//! ```
//!
//! ### Bot Game State Analysis
//!
//! ```rust
//! use poker_api::api::game_info::GameInfo;
//!
//! // Bot analyzes game state for decision making
//! fn bot_analyze_game(game_info: &dyn GameInfo) {
//!     // Assess pot odds for bot strategy
//!     let pot_size = game_info.get_total_pot_size();
//!     let num_players = game_info.get_num_players();
//!     let current_stage = game_info.get_stage();
//!
//!     println!("Pot size: ${} - Players: {} - Stage: {:?}",
//!              pot_size, num_players, current_stage);
//!
//!     // Bot uses this information for strategy decisions
//!     let pot_odds = pot_size as f64 / 100.0; // Simplified calculation
//!     println!("Pot odds: {:.2}", pot_odds);
//! }
//! ```
//!
//! ### Bot Hand Evaluation Integration
//!
//! ```rust
//! use poker_api::api::{card::Card, hand::Hand};
//! use poker_api::hand_evaluator::LookupHandEvaluator;
//!
//! // Bot evaluates hand strength for decision making
//! fn bot_evaluate_hand_strength() {
//!     let evaluator = LookupHandEvaluator::new().unwrap();
//!
//!     let mut hand = Hand::new();
//!     hand.add_card(Card::from_string("As").unwrap()).unwrap();
//!     hand.add_card(Card::from_string("Ks").unwrap()).unwrap();
//!     hand.add_card(Card::from_string("Qs").unwrap()).unwrap();
//!     hand.add_card(Card::from_string("Js").unwrap()).unwrap();
//!     hand.add_card(Card::from_string("Ts").unwrap()).unwrap();
//!
//!     // Bot gets Java-compatible hand rank for strategy
//!     let rank = evaluator.rank_hand(&hand);
//!     println!("Hand rank (Java-compatible): {}", rank);
//!
//!     // Lower rank = stronger hand (matches Java convention)
//!     if rank < 1000 { // Strong hand threshold
//!         println!("Bot: This is a strong hand!");
//!     }
//! }
//! ```
//!
//! ## Bot Testing Architecture
//!
//! The API module follows a bot-focused layered architecture:
//! - **Data Types**: Java-compatible concrete types for cards, hands, and actions
//! - **Bot Interfaces**: Trait-based architecture for poker bot implementations
//! - **Testing Utilities**: Specialized utilities for automated bot testing scenarios
//! - **Event System**: Observer pattern for reactive bot testing and analysis
//! - **Configuration**: Flexible preference system for bot customization and testing
//! - **Integration**: Seamless integration with existing Java poker tools and frameworks
//!
//! ## Bot Testing Performance Considerations
//!
//! - **Memory Layout**: Types optimized for cache efficiency in bot testing scenarios
//! - **Copy Semantics**: Small types implement Copy for zero-cost bot operations
//! - **Java Compatibility**: Maintains Java performance characteristics for tool integration
//! - **Trait Objects**: Used sparingly to minimize vtable overhead in bot implementations
//! - **Inline Operations**: Common bot operations inlined for maximum testing speed
//! - **Batch Processing**: Optimized for batch hand evaluation in bot testing frameworks
//! - **Memory Efficiency**: Compact representations for large-scale bot testing scenarios
//! - **Fast Cloning**: Efficient hand and deck cloning for bot simulation scenarios

pub mod action;
pub mod card;
pub mod deck;
pub mod game_info;
pub mod game_observer;
pub mod hand;
mod hand_eval;
pub mod player;
pub mod player_info;
pub mod preferences;
