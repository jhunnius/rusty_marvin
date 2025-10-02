# Developer Guide

## Architecture Overview

The Poker Hand Evaluator implements the **Meerkat algorithm** for perfect hash-based poker hand evaluation. This section covers the internal architecture, algorithms, and design decisions.

## System Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────┐
│                  LookupHandEvaluator                    │
│  ┌─────────────────────────────────────────────────┐    │
│  │             Hand Ranking Table                  │    │
│  │              (32M+ entries)                     │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────┬───────────────────────────────────┘
                      │ Uses
                      ▼
┌─────────────────────────────────────────────────────────┐
│               StateTableGenerator                       │
│  ┌─────────────────────────────────────────────────┐    │
│  │            Algorithm Modules                    │    │
│  │  • Flushes  • Products  • Unique  • Values      │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

### Data Flow

1. **Table Generation** (one-time):
   ```
   Card Combinations → Perfect Hash → Hand Evaluation → Rank Storage
   ```

2. **Hand Evaluation** (runtime):
   ```
   Input Hand → Perfect Hash → Table Lookup → Rank Value
   ```

## Algorithm Details

### Perfect Hash Function

The core innovation is a perfect hash function that maps every possible card combination to a unique index:

```rust
fn perfect_hash(cards: &[u32]) -> usize {
    let mut hash = 53u32; // Initial value

    for &card in cards {
        let index = card.index();
        hash = TABLE[(hash + index + 1) as usize] as u8;
    }

    hash as usize
}
```

**Key Properties:**
- **Perfect**: No collisions for valid inputs
- **Minimal**: Uses minimal memory for given input size
- **Fast**: O(1) computation time
- **Deterministic**: Same input always produces same output

### Card Encoding

Each card is encoded using multiple techniques:

```rust
struct EncodedCard {
    prime: u32,        // Prime number for rank (2, 3, 5, 7, 11, ...)
    rank_bits: u32,    // Bit position for rank identification
    suit_bits: u32,    // Bit mask for suit detection
    meta_bits: u32,    // Additional metadata bits
}
```

**Prime Encoding:**
```rust
const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
let prime = PRIMES[rank]; // Unique prime for each rank
```

### Hand Evaluation Strategy

#### 5-Card Hands

Direct evaluation using specialized lookup tables:

```rust
fn evaluate_5card(cards: [u32; 5]) -> u32 {
    // 1. Check for flush
    if is_flush(cards) {
        return FLUSH_TABLE[flush_index(cards)];
    }

    // 2. Check for unique hands (straight flush, quads, etc.)
    let unique_index = unique_hash(cards);
    if let Some(rank) = UNIQUE_TABLE.get(unique_index) {
        return rank;
    }

    // 3. Standard hand evaluation
    let product_index = prime_product(cards);
    VALUES_TABLE[product_index]
}
```

#### 6-7 Card Hands

Brute force evaluation of all 5-card combinations:

```rust
fn evaluate_7card(cards: [u32; 7]) -> u32 {
    let mut best_rank = 0;

    // Generate all C(7,5) = 21 combinations
    for combination in combinations(cards, 5) {
        let rank = evaluate_5card(combination);
        best_rank = best_rank.max(rank);
    }

    best_rank
}
```

## Implementation Details

### Table Generation Process

#### Phase 1: Card Combination Enumeration

```rust
const TABLE_SIZE: usize = 612_978; // All possible combinations

for key in 0..TABLE_SIZE {
    let cards = decode_key_to_cards(key);

    // Validate card combination
    if is_valid_combination(cards) {
        let rank = evaluate_hand(cards);
        table[key] = rank;
    }
}
```

#### Phase 2: Perfect Hash Construction

The perfect hash is constructed by:

1. **Initial Mapping**: Map card combinations to initial hash values
2. **Conflict Resolution**: Resolve collisions using secondary tables
3. **Optimization**: Minimize table size while maintaining perfection

#### Phase 3: Table Serialization

```rust
fn save_tables(table: &[u32]) -> Result<(), io::Error> {
    let file = File::create("math/HandRanks.dat")?;
    let mut writer = BufWriter::new(file);

    for &rank in table {
        writer.write_all(&rank.to_ne_bytes())?;
    }

    Ok(())
}
```

### Memory Layout

#### Runtime Table Structure

```
┌─────────────────────────────────────────────────────────┐
│                    HandRanks.dat                        │
├─────────────────────────────────────────────────────────┤
│ Entry 0:    [u32] Rank for combination 0                │
│ Entry 1:    [u32] Rank for combination 1                │
│ ...                                                     │
│ Entry N:    [u32] Rank for combination N                │
└─────────────────────────────────────────────────────────┘
         ↑
         │
    32,487,834 entries × 4 bytes = 129,951,336 bytes
```

#### Generation Working Tables

```rust
struct WorkingTables {
    // Main ranking table during generation
    hand_ranks: Box<[u32; 612_978]>,

    // Specialized evaluation tables
    flushes: FlushesTable,
    products: ProductsTable,
    unique: UniqueTable,
    values: ValuesTable,
}
```

## Performance Optimizations

### SIMD Opportunities

Current implementation is scalar, but offers SIMD optimization potential:

```rust
// Potential SIMD optimization (future enhancement)
#[cfg(target_feature = "avx2")]
fn evaluate_hands_simd(hands: &[Hand]) -> Vec<u32> {
    // Process multiple hands in parallel using AVX instructions
    // Each hand evaluation becomes a vector operation
}
```

### Memory Access Patterns

The current design optimizes for:

- **Sequential Generation**: Tables generated in linear order
- **Random Access Evaluation**: O(1) lookup for any hand
- **Cache Efficiency**: Compact data structures for better cache utilization

### Algorithmic Optimizations

#### Hand-Specific Algorithms

Different hand types use specialized evaluation methods:

```rust
impl HandEvaluator {
    fn evaluate_straight_flush(cards: [u32; 5]) -> u32 {
        // Optimized straight flush detection
        let ranks = extract_ranks(cards);
        if is_consecutive(ranks) && is_same_suit(cards) {
            calculate_straight_flush_rank(ranks[0])
        } else {
            0 // Not a straight flush
        }
    }

    fn evaluate_four_of_a_kind(cards: [u32; 5]) -> u32 {
        // Optimized quad detection using prime products
        let product = calculate_prime_product(cards);
        QUAD_TABLE[product]
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_royal_flush() {
        let hand = create_royal_flush();
        let rank = evaluator.rank_hand(&hand);
        assert!(is_royal_flush_rank(rank));
    }

    #[test]
    fn test_perfect_hash() {
        // Verify no hash collisions
        let mut seen = HashSet::new();
        for combination in all_possible_combinations() {
            let hash = perfect_hash(combination);
            assert!(!seen.contains(&hash));
            seen.insert(hash);
        }
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_vs_reference_implementation() {
        let evaluator = LookupHandEvaluator::new().unwrap();

        // Test against known reference implementation
        for _ in 0..1_000_000 {
            let hand = generate_random_hand();
            let our_rank = evaluator.rank_hand(&hand);
            let ref_rank = reference_evaluate(&hand);

            assert_eq!(our_rank, ref_rank);
        }
    }
}
```

### Performance Benchmarks

```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_hand_evaluation() {
        let evaluator = LookupHandEvaluator::new().unwrap();
        let hands = generate_test_hands(1_000_000);

        let start = Instant::now();
        for hand in hands {
            let _rank = evaluator.rank_hand(&hand);
        }
        let duration = start.elapsed();

        println!("1M evaluations in {:?}", duration);
        println!("Average: {:?} per hand", duration / 1_000_000);
    }
}
```

## Development Workflow

### Code Organization

```
src/
├── api/                    # Public API definitions
│   ├── hand_eval.rs       # HandEval trait
│   ├── card.rs           # Card struct
│   └── hand.rs           # Hand struct
├── hand_evaluator.rs     # Main evaluator implementation
├── evaluator_generator/  # Table generation algorithms
│   ├── mod.rs
│   ├── state_table_generator.rs
│   ├── flushes.rs
│   ├── products.rs
│   ├── unique.rs
│   └── values.rs
└── lib.rs               # Library root
```

### Adding New Features

#### New Hand Type Support

1. **Define evaluation algorithm** in `evaluator_generator/`
2. **Add lookup tables** for the new hand type
3. **Update `eval_5hand`** to use new tables
4. **Add tests** for the new hand type
5. **Update documentation**

#### Performance Improvements

1. **Profile** current performance
2. **Identify bottlenecks** (CPU, memory, cache)
3. **Implement optimization** with benchmarks
4. **Verify correctness** with existing tests
5. **Update performance documentation**

### Debugging and Development Tools

#### Debug Table Generation

```rust
fn debug_table_generation() {
    let mut generator = StateTableGenerator::new();

    // Enable debug output
    generator.set_debug_mode(true);

    // Generate with detailed logging
    generator.generate_tables();

    // Inspect specific entries
    println!("Entry 12345: {}", generator.hand_ranks[12345]);
}
```

#### Hand Evaluation Tracing

```rust
fn trace_hand_evaluation(hand: &Hand) -> EvaluationTrace {
    let mut trace = EvaluationTrace::new();

    // Step through evaluation with logging
    trace.log("Starting evaluation");
    let rank = evaluator.rank_hand_traced(&hand, &mut trace);
    trace.log(&format!("Final rank: {}", rank));

    trace
}
```

## Mathematical Background

### Combinatorics

#### Card Combination Space

| Cards | Combinations | Calculation |
|-------|--------------|-------------|
| 5-Card | 2,598,960 | C(52,5) |
| 6-Card | 20,358,520 | C(52,6) |
| 7-Card | 133,784,560 | C(52,7) |

#### Table Size Calculation

```
Working Table: C(52,5) + C(52,6) + C(52,7) = 156,741,540 entries
Final Table: 32,487,834 unique combinations (after deduplication)
```

### Hash Function Design

The perfect hash function uses:

1. **Initial Value**: 53 (empirically chosen)
2. **Recursive Application**: Each card updates the hash
3. **Table Lookup**: Final mapping through ranking table

```rust
fn perfect_hash(cards: &[u32]) -> u32 {
    let mut hash = 53;

    for &card in cards {
        let card_index = card.index();
        hash = TABLE[(hash + card_index + 1) as usize] as u8;
    }

    hash as u32
}
```

## Future Enhancements

### Potential Improvements

1. **SIMD Optimization**: Vectorize hand evaluation for batch processing
2. **Memory Mapping**: Use memory-mapped files for table loading
3. **Compression**: Reduce table size using compression techniques
4. **Hardware Acceleration**: GPU-accelerated evaluation for massive scale
5. **Distributed Generation**: Parallel table generation across multiple machines

### Research Directions

1. **Alternative Hash Functions**: Explore more efficient perfect hash constructions
2. **Machine Learning**: Use ML to optimize evaluation algorithms
3. **Hardware Design**: Custom ASIC for poker hand evaluation
4. **Algorithmic Improvements**: Reduce computational complexity further

## Contributing

### Development Setup

1. **Clone repository**
2. **Install Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. **Run tests**: `cargo test`
4. **Generate documentation**: `cargo doc --open`

### Code Style

- Use `rustfmt` for consistent formatting
- Add comprehensive documentation comments
- Include unit tests for new functionality
- Follow existing error handling patterns

### Performance Guidelines

- Profile before optimizing
- Maintain algorithm correctness
- Add benchmarks for performance regressions
- Document performance characteristics

This developer guide provides the technical foundation for understanding and extending the poker hand evaluation system. For API usage, see the user guide. For detailed reference, see the API documentation.