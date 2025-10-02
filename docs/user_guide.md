# User Guide

## Introduction

This guide explains how to use the Poker Hand Evaluator library for evaluating poker hands in your applications. Whether you're building a poker game, analysis tool, or AI bot, this library provides the fast and accurate hand evaluation you need.

## Quick Start

### Basic Setup

1. **Add dependency** to your `Cargo.toml`:
```toml
[dependencies]
poker-api = "0.1.0"
```

2. **Import and use** in your code:
```rust
use poker_api::hand_evaluator::LookupHandEvaluator;
use poker_api::api::hand::Hand;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the evaluator (generates tables on first run)
    let evaluator = LookupHandEvaluator::new()?;

    // Create a poker hand
    let mut hand = Hand::new();
    // ... add cards

    // Evaluate the hand
    let rank = evaluator.rank_hand(&hand);
    println!("Hand strength: {}", rank);

    Ok(())
}
```

### First Run Experience

On the first run, you'll see:
```
Evaluation tables do not exist, generating them...
Generating state table...
Hand evaluation tables saved to math/HandRanks.dat
Evaluation tables loaded.
```

This generates the lookup tables (~1-2 seconds) and saves them for future use.

## Working with Cards

### Card Creation

Cards can be created from string notation:

```rust
use poker_api::api::card::Card;

// Face cards
let ace_spades = Card::from_string("As")?;
let king_hearts = Card::from_string("Kh")?;

// Number cards
let five_clubs = Card::from_string("5c")?;
let ten_diamonds = Card::from_string("Td")?;

// All suits: s (spades), h (hearts), d (diamonds), c (clubs)
// All ranks: A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
```

### Card Properties

```rust
let card = Card::from_string("As")?;

println!("Rank: {}", card.rank());      // 12 (Ace)
println!("Suit: {}", card.suit());      // 0 (Spades)
println!("Index: {}", card.index());    // 51 (unique card ID)
println!("String: {}", card.to_string()); // "As"
```

## Working with Hands

### Creating Hands

```rust
use poker_api::api::hand::Hand;
use poker_api::api::card::Card;

// Create empty hand
let mut hand = Hand::new();

// Add cards one by one
hand.add_card(Card::from_string("As")?).unwrap();
hand.add_card(Card::from_string("Ks")?).unwrap();
hand.add_card(Card::from_string("Qs")?).unwrap();
hand.add_card(Card::from_string("Js")?).unwrap();
hand.add_card(Card::from_string("Ts")?).unwrap();

println!("Hand size: {}", hand.size());    // 5
println!("Hand: {}", hand.to_string());    // "As Ks Qs Js Ts"
```

### Hand Evaluation

```rust
// 5-card hand evaluation
let rank = evaluator.rank_hand(&hand);
println!("Royal flush rank: {}", rank);

// 6-card hand (finds best 5-card combination)
let mut six_card_hand = Hand::new();
six_card_hand.add_card(Card::from_string("As")?).unwrap();
six_card_hand.add_card(Card::from_string("Ks")?).unwrap();
// ... add 4 more cards

let rank = evaluator.rank_hand(&six_card_hand);

// 7-card hand (Texas Hold'em)
let mut seven_card_hand = Hand::new();
// ... add 7 cards
let rank = evaluator.rank_hand(&seven_card_hand);
```

## Understanding Hand Ranks

### Rank Values

Higher rank values indicate stronger hands:

```rust
// Compare hand strengths
let hand1_rank = evaluator.rank_hand(&hand1);
let hand2_rank = evaluator.rank_hand(&hand2);

if hand1_rank > hand2_rank {
    println!("Hand 1 is stronger");
} else if hand1_rank < hand2_rank {
    println!("Hand 2 is stronger");
} else {
    println!("Hands are equal");
}
```

### Hand Categories

Extract hand category from rank:

```rust
use poker_api::api::hand_eval::HandEval;

fn get_hand_category(rank: u32) -> u32 {
    (rank & HandEval::HAND_CATEGORY_MASK) >> HandEval::HAND_CATEGORY_SHIFT
}

let category = get_hand_category(rank);
match category {
    9 => println!("Straight Flush"),
    8 => println!("Four of a Kind"),
    7 => println!("Full House"),
    6 => println!("Flush"),
    5 => println!("Straight"),
    4 => println!("Three of a Kind"),
    3 => println!("Two Pair"),
    2 => println!("One Pair"),
    1 => println!("High Card"),
    _ => println!("Unknown"),
}
```

## Texas Hold'em Example

Here's a complete example of evaluating Texas Hold'em hands:

```rust
use poker_api::hand_evaluator::LookupHandEvaluator;
use poker_api::api::hand::Hand;
use poker_api::api::card::Card;

struct PokerGame {
    evaluator: LookupHandEvaluator,
}

impl PokerGame {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            evaluator: LookupHandEvaluator::new()?,
        })
    }

    fn evaluate_texas_holdem(
        &self,
        hole_cards: &[Card; 2],
        board: &[Card; 5]
    ) -> u32 {
        let mut hand = Hand::new();

        // Add hole cards
        hand.add_card(hole_cards[0]).unwrap();
        hand.add_card(hole_cards[1]).unwrap();

        // Add community cards
        for &card in board {
            hand.add_card(card).unwrap();
        }

        self.evaluator.rank_hand(&hand)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game = PokerGame::new()?;

    // Example: Royal Flush
    let hole_cards = [
        Card::from_string("As").unwrap(),
        Card::from_string("Ks").unwrap(),
    ];

    let board = [
        Card::from_string("Qs").unwrap(),
        Card::from_string("Js").unwrap(),
        Card::from_string("Ts").unwrap(),
        Card::from_string("2d").unwrap(),
        Card::from_string("3c").unwrap(),
    ];

    let rank = game.evaluate_texas_holdem(&hole_cards, &board);
    println!("Royal flush rank: {}", rank);

    Ok(())
}
```

## Performance Optimization

### Table Preloading

For applications that need immediate response:

```rust
// Warm up the evaluator before use
fn preload_evaluator() -> LookupHandEvaluator {
    println!("Loading poker evaluator...");
    let start = std::time::Instant::now();

    let evaluator = LookupHandEvaluator::new().unwrap();

    let duration = start.elapsed();
    println!("Evaluator ready in {:?}", duration);

    evaluator
}
```

### Batch Evaluation

For evaluating many hands:

```rust
fn evaluate_multiple_hands(evaluator: &LookupHandEvaluator, hands: &[Hand]) -> Vec<u32> {
    hands.iter()
         .map(|hand| evaluator.rank_hand(hand))
         .collect()
}
```

## Error Handling

### Common Errors

```rust
use poker_api::api::card::Card;

// Invalid card strings
let invalid = Card::from_string("XX"); // Err
let invalid = Card::from_string("53"); // Err

// Hand capacity exceeded
let mut hand = Hand::new();
// Adding more than 7 cards will fail

// Table loading failures
let evaluator = LookupHandEvaluator::new();
// May fail if disk is full or permissions are insufficient
```

### Proper Error Handling

```rust
use std::io;

fn create_evaluator() -> Result<LookupHandEvaluator, io::Error> {
    match LookupHandEvaluator::new() {
        Ok(evaluator) => {
            println!("Evaluator created successfully");
            Ok(evaluator)
        }
        Err(e) => {
            eprintln!("Failed to create evaluator: {}", e);
            Err(e)
        }
    }
}
```

## Best Practices

### 1. Reuse Evaluators

```rust
// Good: Reuse the same evaluator
struct GameEngine {
    evaluator: LookupHandEvaluator,
}

impl GameEngine {
    fn new() -> Result<Self, io::Error> {
        Ok(Self {
            evaluator: LookupHandEvaluator::new()?,
        })
    }

    fn evaluate_hand(&self, hand: &Hand) -> u32 {
        self.evaluator.rank_hand(hand)
    }
}
```

### 2. Validate Input Data

```rust
fn safe_evaluate_hand(evaluator: &LookupHandEvaluator, hand: &Hand) -> Option<u32> {
    if hand.size() >= 5 && hand.size() <= 7 {
        Some(evaluator.rank_hand(hand))
    } else {
        None
    }
}
```

### 3. Handle Edge Cases

```rust
// Check for invalid hands
let rank = evaluator.rank_hand(&hand);
if rank == 0 {
    println!("Invalid hand: must have 5-7 cards");
}
```

## Integration Examples

### Poker Bot

```rust
use poker_api::hand_evaluator::LookupHandEvaluator;

struct PokerBot {
    evaluator: LookupHandEvaluator,
}

impl PokerBot {
    fn should_raise(&self, my_hand: &Hand, pot_odds: f32) -> bool {
        let hand_strength = self.evaluator.rank_hand(my_hand);

        // Simple strategy: raise with strong hands
        match hand_strength {
            rank if rank > 300_000_000 => true,  // Very strong
            rank if rank > 100_000_000 => pot_odds > 0.1,  // Medium strength
            _ => false,  // Weak hands
        }
    }
}
```

### Hand Range Analysis

```rust
use poker_api::hand_evaluator::LookupHandEvaluator;
use poker_api::api::hand::Hand;

struct RangeAnalyzer {
    evaluator: LookupHandEvaluator,
}

impl RangeAnalyzer {
    fn analyze_range(&self, hands: &[Hand]) -> HandStrengthStats {
        let ranks: Vec<u32> = hands.iter()
            .map(|hand| self.evaluator.rank_hand(hand))
            .collect();

        HandStrengthStats {
            average_rank: ranks.iter().sum::<u32>() / ranks.len() as u32,
            median_rank: self.calculate_median(&ranks),
            strongest_hand: ranks.iter().max().copied().unwrap_or(0),
            weakest_hand: ranks.iter().min().copied().unwrap_or(0),
        }
    }
}
```

## Troubleshooting

### Common Issues

**Problem**: "Evaluation tables do not exist, generating them..." every time

**Solution**: Ensure write permissions in the current directory, or specify a custom path

**Problem**: Slow first startup

**Solution**: Expected behavior - table generation takes 1-2 seconds

**Problem**: High memory usage

**Solution**: The 128MB lookup table is required for performance. Consider if this fits your deployment constraints

**Problem**: Card parsing fails

**Solution**: Verify card strings use correct format: rank + suit (e.g., "As", "Kh", "Td")

### Performance Issues

If evaluation seems slow:

1. **Check build mode**: Use `cargo build --release`
2. **Verify table loading**: Ensure tables are generated and cached
3. **Monitor memory**: Ensure sufficient RAM for table storage
4. **Consider batching**: Evaluate multiple hands together when possible

## Support

For additional help:

1. Check the [API Documentation](api.md) for detailed reference
2. Review test files for usage examples
3. Open issues on GitHub for bugs or feature requests
4. Study the source code comments for implementation details

This user guide covers the most common use cases. For advanced usage or customization, see the developer guide and source code documentation.