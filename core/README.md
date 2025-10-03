# Poker API Core Library

## Overview

The Poker API Core Library (`poker-api`) is a high-performance poker hand evaluation library based on the Meerkat API algorithm. This library provides lightning-fast poker hand strength evaluation using precomputed lookup tables and perfect hashing.

## Features

- **Blazing Fast**: O(1) hand evaluation using perfect hash lookup tables
- **Memory Efficient**: ~128MB for complete evaluation tables
- **Standards Compliant**: Based on proven poker evaluation algorithms
- **Multi-Hand Support**: Evaluate 5, 6, or 7-card poker hands
- **Texas Hold'em Ready**: Optimized for Texas Hold'em hand evaluation
- **Comprehensive Testing**: Extensive test suite with millions of verification cases

## Architecture

### Core Components

#### API Module (`src/api/`)
The main application programming interface providing:
- **Card Management**: `Card` struct for individual playing cards
- **Hand Management**: `Hand` struct for collections of cards
- **Deck Operations**: `Deck` struct for standard 52-card deck management
- **Player Actions**: `Action` types for betting operations
- **Game State**: `GameInfo` trait for accessing game information
- **Bot Interface**: `Player` trait for poker bot implementations

#### Hand Evaluator (`src/hand_evaluator.rs`)
High-performance poker hand evaluation using precomputed lookup tables:
- **Perfect Hash Algorithm**: Maps card combinations to unique indices
- **Table-Based Evaluation**: O(1) lookup time after table generation
- **Multi-Hand Support**: Handles 5, 6, and 7-card evaluations
- **Memory Management**: Efficient table loading and caching

#### Evaluator Generator (`src/evaluator_generator/`)
Table generation system for poker hand evaluation:
- **Flushes** (`flushes.rs`): Lookup table for flush hand evaluation
- **Products** (`products.rs`): Prime product-based hand type detection
- **Unique** (`unique.rs`): Special hand types (straight flushes, quads, etc.)
- **Values** (`values.rs`): Standard hand ranking and comparison
- **Pairs** (`pair.rs`): Two-card combination analysis and lookup

#### Supporting Modules
- **Simple Player** (`src/simple_player.rs`): Basic bot implementation example
- **TOML Preferences** (`src/toml_preferences.rs`): Configuration management
- **Texas Hold'em** (`src/texas_holdem/`): Game-specific logic (planned)

## Performance Characteristics

| Hand Type | Evaluation Time | Combinations Evaluated |
|-----------|----------------|----------------------|
| 5-Card    | ~10-20ns       | Direct lookup        |
| 6-Card    | ~50-80ns       | 6 combinations       |
| 7-Card    | ~150-250ns     | 21 combinations      |

### Memory Usage
- **Table Generation**: ~2.4MB working memory during generation
- **Runtime Tables**: ~128MB for complete hand ranking tables
- **Loading Time**: ~100-200ms from disk (after generation)

## Algorithm Overview

### Perfect Hash Algorithm

The evaluator uses a perfect hash function that maps each possible card combination to a unique index:

```
Hash = f(card₁, card₂, ..., cardₙ)
Index = Hash % TABLE_SIZE
Rank = TABLE[Index]
```

### Card Encoding

Each card is encoded using multiple techniques:
- **Prime Numbers**: Unique primes for rank identification (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41)
- **Bit Patterns**: Suit detection using bit manipulation
- **Binary Representation**: Compact 8-bit card encoding

### Hand Strength Calculation

Hand strength is calculated using:
1. **Flush Detection**: Bitwise suit comparison
2. **Rank Analysis**: Prime product uniqueness
3. **Pattern Matching**: Specialized algorithms per hand type
4. **Relative Ranking**: Comparison against all possible hands

## Usage Examples

### Basic Hand Evaluation

```rust
use poker_api::hand_evaluator::LookupHandEvaluator;
use poker_api::api::hand::Hand;
use poker_api::api::card::Card;

// Create evaluator (generates tables on first use)
let evaluator = LookupHandEvaluator::new().unwrap();

// Create a poker hand
let mut hand = Hand::new();
hand.add_card(Card::from_string("As").unwrap()).unwrap();
hand.add_card(Card::from_string("Ks").unwrap()).unwrap();
hand.add_card(Card::from_string("Qs").unwrap()).unwrap();
hand.add_card(Card::from_string("Js").unwrap()).unwrap();
hand.add_card(Card::from_string("Ts").unwrap()).unwrap();

// Evaluate hand strength
let rank = evaluator.rank_hand(&hand);
println!("Hand rank: {}", rank);
```

### 7-Card Hand Evaluation (Texas Hold'em)

```rust
use poker_api::hand_evaluator::LookupHandEvaluator;
use poker_api::api::card::Card;

let evaluator = LookupHandEvaluator::new().unwrap();

// Texas Hold'em: 2 hole cards + 5 community cards
let cards = [
    Card::from_string("As").unwrap(),
    Card::from_string("Ks").unwrap(),
    Card::from_string("Td").unwrap(),
    Card::from_string("Jc").unwrap(),
    Card::from_string("Qh").unwrap(),
    Card::from_string("9s").unwrap(),
    Card::from_string("2d").unwrap(),
];

let rank = evaluator.rank_hand7(&cards);
println!("Best 5-card hand rank: {}", rank);
```

### Custom Table Generation

```rust
use poker_api::evaluator_generator::state_table_generator::StateTableGenerator;

let mut generator = StateTableGenerator::new();
generator.generate_tables();        // ~1-2 seconds
generator.save_tables().unwrap();   // Save to disk
```

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

Run performance benchmarks:

```bash
cargo test --release hand_eval_integration_test -- --nocapture
```

## Development

### Building

```bash
# Build library
cargo build

# Build with optimizations
cargo build --release

# Generate documentation
cargo doc --open
```

### Code Organization

- **API Types**: `src/api/` - Core data types and traits
- **Evaluation**: `src/hand_evaluator.rs` - Main evaluation logic
- **Table Generation**: `src/evaluator_generator/` - Lookup table creation
- **Examples**: `src/simple_player.rs` - Basic bot implementation
- **Configuration**: `src/toml_preferences.rs` - Settings management

### Contributing

Areas for improvement:
- Hardware acceleration (SIMD optimizations)
- Additional poker variants (Omaha, Stud, etc.)
- Memory-mapped table loading
- Distributed table generation
- Advanced benchmarking tools

## Integration

This core library serves as the foundation for:
- **Poker Bots**: AI players in `bots/` directory
- **Simulations**: Game simulation tools in `simulator/` directory
- **Analysis Tools**: Log analysis and monitoring in `tools/` directory
- **Online Play**: Networked poker applications in `online/` directory

## References

- **Meerkat API**: Original Java implementation by Ray Wotton
- **C Implementation**: Paul Senzee's optimized C version
- **Perfect Hash**: Kevin Suffecool's hashing algorithm
- **Poker Standards**: 2+2 Poker hand ranking standards