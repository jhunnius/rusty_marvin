# API Documentation

## Core Types

### `LookupHandEvaluator`

The main hand evaluation engine that uses precomputed lookup tables for O(1) hand evaluation.

#### Constructor

```rust
pub fn new() -> io::Result<Self>
```

Creates a new evaluator instance. On first use, generates evaluation tables if they don't exist.

**Returns:**
- `Ok(LookupHandEvaluator)` - Successfully initialized evaluator
- `Err(io::Error)` - Failed to load or generate tables

#### Methods

##### `rank_hand(&self, hand: &Hand) -> u32`

Evaluates any poker hand (5, 6, or 7 cards) and returns its rank value.

```rust
let rank = evaluator.rank_hand(&hand);
```

**Parameters:**
- `hand: &Hand` - Poker hand containing 5-7 cards

**Returns:**
- `u32` - Hand rank (higher values = stronger hands)

##### `rank_hand5(&self, hand: &Hand) -> u32`

Optimized evaluation for exactly 5-card hands.

```rust
let rank = evaluator.rank_hand5(&hand);
```

**Parameters:**
- `hand: &Hand` - Poker hand containing exactly 5 cards

**Returns:**
- `u32` - Hand rank value

##### `rank_hand6(&self, cards: &[u32]) -> u32`

Evaluates a 6-card hand by finding the best 5-card combination.

```rust
let rank = evaluator.rank_hand6(&card_indices);
```

**Parameters:**
- `cards: &[u32]` - Array of 6 card indices

**Returns:**
- `u32` - Best possible 5-card hand rank

##### `rank_hand7(&self, cards: &[u32]) -> u32`

Evaluates a 7-card hand by finding the best 5-card combination.

```rust
let rank = evaluator.rank_hand7(&card_indices);
```

**Parameters:**
- `cards: &[u32]` - Array of 7 card indices

**Returns:**
- `u32` - Best possible 5-card hand rank

### `HandEval` Trait

Standardized interface for poker hand evaluators.

#### Constants

```rust
const HAND_CATEGORY_SHIFT: u32 = 24;
const HAND_CATEGORY_MASK: u32 = 0xF << Self::HAND_CATEGORY_SHIFT;
const VALUE_MASK: u32 = 0x000FFFFF;
```

#### Methods

##### `rank_hand(&self, hand: &Hand) -> u32`

Evaluates a poker hand of any supported size.

##### `rank_hand7(&self, cards: &[Card; 7]) -> u32`

Evaluates exactly 7 cards.

##### `hand_description(&self, hand_value: u32) -> String`

Returns human-readable description of a hand rank.

##### `hand_description_hand(&self, hand: &Hand) -> String`

Describes a hand by evaluating it first.

##### `hand_description7(&self, cards: &[Card; 7]) -> String`

Describes the best 5-card hand from 7 cards.

## Supporting Types

### `Hand`

Collection of cards that can be evaluated.

#### Key Methods

```rust
// Construction
let mut hand = Hand::new();
hand.add_card(card).unwrap();

// Information
let size = hand.size();
let is_valid = hand.size() >= 5;

// String representation
println!("{}", hand.to_string()); // "As Ks Qs Js Ts"
```

### `Card`

Individual playing card representation.

#### Construction

```rust
// From string notation
let card = Card::from_string("As").unwrap();
let card = Card::from_string("Kh").unwrap();

// String representation
assert_eq!(card.to_string(), "As");
```

#### Properties

```rust
let rank = card.rank();  // Card rank (0-12)
let suit = card.suit();  // Card suit (0-3)
let index = card.index(); // Unique card index (0-51)
```

## Hand Ranking Values

### Category Extraction

```rust
// Extract hand category (bits 24-27)
let category = (hand_value & HandEval::HAND_CATEGORY_MASK) >> HandEval::HAND_CATEGORY_SHIFT;

// Extract relative value (bits 0-23)
let value = hand_value & HandEval::VALUE_MASK;
```

### Hand Categories

| Category | Value | Description |
|----------|-------|-------------|
| Straight Flush | 9 | Five cards in sequence, same suit |
| Four of a Kind | 8 | Four cards of same rank |
| Full House | 7 | Three of a kind + pair |
| Flush | 6 | Five cards, same suit |
| Straight | 5 | Five cards in sequence |
| Three of a Kind | 4 | Three cards of same rank |
| Two Pair | 3 | Two different pairs |
| One Pair | 2 | Two cards of same rank |
| High Card | 1 | No matching cards |

### Rank Value Ranges

| Hand Type | Rank Range | Count |
|-----------|------------|-------|
| Straight Flush | 368,793,253 - 371,292,191 | 40 |
| Four of a Kind | 184,396,629 - 368,793,252 | 624 |
| Full House | 92,198,315 - 184,396,628 | 3,744 |
| Flush | 46,099,158 - 92,198,314 | 5,108 |
| Straight | 23,049,579 - 46,099,157 | 10,200 |
| Three of a Kind | 11,524,790 - 23,049,578 | 54,912 |
| Two Pair | 5,762,395 - 11,524,789 | 123,552 |
| One Pair | 2,881,198 - 5,762,394 | 1,098,240 |
| High Card | 1 - 2,881,197 | 1,302,540 |

## Performance Characteristics

### Time Complexity

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| 5-Card evaluation | O(1) | Direct table lookup |
| 6-Card evaluation | O(1) | 6 table lookups |
| 7-Card evaluation | O(1) | 21 table lookups |
| Table generation | O(N) | N = 32M combinations |

### Memory Usage

| Component | Size | Description |
|-----------|------|-------------|
| Hand ranks table | ~128 MB | Complete evaluation table |
| Working tables | ~2.4 MB | Generation workspace |
| Code size | ~50 KB | Compiled binary |

### Cache Performance

The lookup table design provides excellent cache performance:

- **Sequential Access**: Table generation processes data sequentially
- **Compact Encoding**: Cards encoded in 8 bits each
- **Prefetch Friendly**: Linear table layout enables hardware prefetching
- **Low Branch Misprediction**: Perfect hash eliminates conditional logic

## Error Handling

### Common Error Cases

```rust
// Invalid hand size
let rank = evaluator.rank_hand(&hand); // Returns 0 for < 5 cards

// Card parsing errors
let card = Card::from_string("XX"); // Returns Err

// Table loading errors
let evaluator = LookupHandEvaluator::new(); // May fail if disk full
```

### Error Types

- `io::Error` - File system errors during table loading/generation
- `CardParseError` - Invalid card string format
- `HandFullError` - Attempting to add cards to full hand

## Thread Safety

The `LookupHandEvaluator` is designed for read-only usage after construction:

```rust
// Safe: Read-only evaluator
let evaluator = LookupHandEvaluator::new().unwrap();

// Multiple threads can use the same evaluator
let rank1 = evaluator.rank_hand(&hand1);
let rank2 = evaluator.rank_hand(&hand2);

// NOT safe: Modifying evaluator during use
// (evaluator is not Sync + Send for mutation)
```

## Integration Examples

### Texas Hold'em Engine

```rust
use poker_api::hand_evaluator::LookupHandEvaluator;
use poker_api::api::{hand::Hand, card::Card};

struct HoldemEngine {
    evaluator: LookupHandEvaluator,
}

impl HoldemEngine {
    fn evaluate_board(&self, hole_cards: &[Card; 2], board: &[Card; 5]) -> u32 {
        let mut all_cards = Vec::new();
        all_cards.extend_from_slice(hole_cards);
        all_cards.extend_from_slice(board);

        let mut hand = Hand::new();
        for card in all_cards {
            hand.add_card(card).unwrap();
        }

        self.evaluator.rank_hand(&hand)
    }
}
```

### Hand Range Analysis

```rust
use poker_api::hand_evaluator::LookupHandEvaluator;

struct RangeAnalyzer {
    evaluator: LookupHandEvaluator,
}

impl RangeAnalyzer {
    fn compare_hands(&self, hand1: &Hand, hand2: &Hand) -> std::cmp::Ordering {
        let rank1 = self.evaluator.rank_hand(hand1);
        let rank2 = self.evaluator.rank_hand(hand2);
        rank1.cmp(&rank2)
    }
}
```

## Testing Integration

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use poker_api::hand_evaluator::LookupHandEvaluator;

    #[test]
    fn test_royal_flush() {
        let evaluator = LookupHandEvaluator::new().unwrap();
        let mut hand = Hand::new();
        // Add A,K,Q,J,10 of same suit
        let rank = evaluator.rank_hand(&hand);
        assert!(rank > 368_793_253); // Royal flush range
    }
}
```

### Benchmark Tests

```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_evaluation() {
        let evaluator = LookupHandEvaluator::new().unwrap();
        let hand = create_test_hand();

        let start = Instant::now();
        for _ in 0..1_000_000 {
            let _rank = evaluator.rank_hand(&hand);
        }
        let duration = start.elapsed();

        println!("1M evaluations in {:?}", duration);
    }
}
```

## Advanced Configuration

### Custom Table Paths

The evaluator uses default paths, but this can be customized by modifying the source:

```rust
// In hand_evaluator.rs
const FILE_NAME: &'static str = "custom/path/HandRanks.dat";
```

### Memory Management

For systems with limited memory, consider:

```rust
// Use Box<[u32]> for heap allocation control
let hand_ranks: Box<[u32]> = vec![0; SIZE].into_boxed_slice();

// Monitor memory usage
println!("Table memory usage: {} MB",
         std::mem::size_of_val(&*hand_ranks) / 1_048_576);
```

### Performance Tuning

For maximum performance:

```rust
// Use release mode
cargo build --release

// Consider SIMD optimizations for batch evaluation
// (requires custom implementation)
```

This API documentation covers the complete public interface for poker hand evaluation. For implementation details, see the source code comments and the developer guide.