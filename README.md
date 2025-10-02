# Poker Hand Evaluator

[![Crates.io](https://img.shields.io/crates/v/poker-api.svg)](https://crates.io/crates/poker-api)
[![Documentation](https://docs.rs/poker-api/badge.svg)](https://docs.rs/poker-api)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A high-performance poker hand evaluation library for Rust, based on the Java Meerkat API algorithm. This library provides lightning-fast poker hand strength evaluation using precomputed lookup tables and perfect hashing.

## Features

- **Blazing Fast**: O(1) hand evaluation using perfect hash lookup tables
- **Memory Efficient**: ~128MB for complete evaluation tables
- **Standards Compliant**: Based on proven poker evaluation algorithms
- **Multi-Hand Support**: Evaluate 5, 6, or 7-card poker hands
- **Texas Hold'em Ready**: Optimized for Texas Hold'em hand evaluation
- **Comprehensive Testing**: Extensive test suite with millions of verification cases

## Performance

| Hand Type | Evaluation Time | Combinations Evaluated |
|-----------|----------------|----------------------|
| 5-Card    | ~10-20ns       | Direct lookup        |
| 6-Card    | ~50-80ns       | 6 combinations       |
| 7-Card    | ~150-250ns     | 21 combinations      |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
poker-api = "0.1.0"
```

## Quick Start

```rust
use poker_api::hand_evaluator::LookupHandEvaluator;
use poker_api::api::hand::Hand;

// Create evaluator (generates tables on first use)
let evaluator = LookupHandEvaluator::new().unwrap();

// Create a poker hand
let mut hand = Hand::new();
// ... add cards to hand

// Evaluate hand strength
let rank = evaluator.rank_hand(&hand);
println!("Hand rank: {}", rank);
```

## Algorithm Overview

This library implements the **Meerkat algorithm**, a perfect hash-based approach to poker hand evaluation:

1. **Card Encoding**: Each card is encoded into a compact binary representation
2. **Perfect Hash**: Cards are mapped to unique indices using mathematical functions
3. **Table Lookup**: Precomputed ranking tables provide instant hand strength values
4. **Best Hand Selection**: For 6-7 card hands, finds optimal 5-card combination

### Table Generation

The first time an evaluator is created, it generates evaluation tables:

```rust
// Tables are generated automatically on first use
let evaluator = LookupHandEvaluator::new().unwrap();
// Output: "Generating state table..." (first run only)
// Output: "Hand evaluation tables saved to math/HandRanks.dat"
```

Tables are saved to `math/HandRanks.dat` and reused for subsequent runs.

## Hand Types

The evaluator recognizes all standard poker hand types:

| Hand Type | Description | Example |
|-----------|-------------|---------|
| Straight Flush | Five cards in sequence, same suit | A♠ K♠ Q♠ J♠ T♠ |
| Four of a Kind | Four cards of same rank | A♠ A♥ A♦ A♣ K♦ |
| Full House | Three of a kind + one pair | K♠ K♥ K♦ Q♠ Q♥ |
| Flush | Five cards of same suit | A♠ J♠ 8♠ 5♠ 2♠ |
| Straight | Five cards in sequence | A♦ K♣ Q♥ J♠ T♦ |
| Three of a Kind | Three cards of same rank | Q♠ Q♥ Q♦ A♣ 7♦ |
| Two Pair | Two different pairs | J♠ J♥ 7♦ 7♣ A♦ |
| One Pair | Two cards of same rank | A♠ A♥ K♦ Q♣ 7♦ |
| High Card | No matching cards | A♠ K♥ Q♦ J♣ 9♦ |

## Advanced Usage

### 7-Card Hand Evaluation

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

## Mathematical Foundation

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

## References

- **Meerkat API**: Original Java implementation by Ray Wotton
- **C Implementation**: Paul Senzee's optimized C version
- **Perfect Hash**: Kevin Suffecool's hashing algorithm
- **Poker Standards**: 2+2 Poker hand ranking standards

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Areas for improvement:

- Hardware acceleration (SIMD optimizations)
- Additional poker variants (Omaha, Stud, etc.)
- Memory-mapped table loading
- Distributed table generation
- Advanced benchmarking tools

## Support

For questions, issues, or contributions, please:

1. Check existing documentation
2. Review test cases for usage examples
3. Open GitHub issues for bugs or feature requests
4. Submit pull requests with comprehensive tests

---

**Performance Note**: First run includes ~1-2 second table generation. Subsequent runs load precomputed tables in ~100-200ms.