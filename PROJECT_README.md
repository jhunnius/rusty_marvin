# Poker Hand Evaluation Project

## Overview

This project is a comprehensive Rust-based poker hand evaluation and bot development framework. Built around a high-performance poker hand evaluation library, it provides tools for poker bot development, game simulation, strategy analysis, and performance monitoring.

## Architecture

The project is organized into several key components:

```
poker-project/
├── core/                 # Core poker API and hand evaluation
│   ├── src/api/         # Core data types and traits
│   ├── src/hand_evaluator.rs    # High-performance hand evaluation
│   ├── src/evaluator_generator/ # Lookup table generation
│   └── src/texas_holdem/        # Texas Hold'em game logic
├── bots/                # Poker bot implementations
├── simulator/           # Game simulation and testing tools
├── tools/               # Analysis and monitoring utilities
└── docs/                # Documentation and guides
```

## Core Components

### 🃏 Core Library (`core/`)

**High-Performance Poker Hand Evaluation**
- **Perfect Hash Algorithm**: O(1) hand evaluation using precomputed tables
- **Multi-Hand Support**: Evaluate 5, 6, or 7-card poker hands
- **Memory Efficient**: ~128MB for complete evaluation tables
- **Standards Compliant**: Based on proven poker evaluation algorithms

**Key Features:**
- Card and Hand management
- Deck operations and shuffling
- Player action representation
- Game state information
- Bot interface definitions
- Configuration management

### 🤖 Poker Bots (`bots/`)

**AI Player Implementations**
- Multiple bot strategies (random, tight, aggressive, adaptive)
- Complete Player and GameObserver trait implementations
- Configurable behavior and decision-making
- Performance tracking and analysis

### 🎲 Simulator (`simulator/`)

**Game Simulation Framework**
- Complete Texas Hold'em game simulation
- Monte Carlo analysis for strategy testing
- Range vs range analysis
- Tournament and cash game simulation
- Statistical analysis and reporting

### 🔧 Tools (`tools/`)

**Development and Analysis Utilities**
- Log analysis and pattern detection
- Real-time performance monitoring
- Automated testing frameworks
- Report generation and visualization

## Performance Characteristics

| Component | Performance | Memory Usage |
|-----------|-------------|--------------|
| Hand Evaluation | ~10-20ns per hand | ~128MB tables |
| Bot Decision | < 10ms per action | ~10KB per bot |
| Game Simulation | ~1ms per hand | ~1MB per simulation |
| Log Analysis | ~100ms per 1000 hands | ~50MB for analysis |

## Quick Start

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
println!("Hand rank: {}", rank); // Higher values = stronger hands
```

### Creating a Poker Bot

```rust
use poker_api::api::player::Player;
use poker_api::api::preferences::Preferences;
use poker_api::api::action::Action;

struct MyBot {
    hole_cards: Option<(Card, Card)>,
    aggression: f64,
}

impl Player for MyBot {
    fn init(&mut self, prefs: Box<dyn Preferences>) {
        self.aggression = prefs.get_double("aggression", 0.5);
    }

    fn hole_cards(&mut self, c1: Card, c2: Card, _seat: usize) {
        self.hole_cards = Some((c1, c2));
    }

    fn get_action(&mut self) -> Action {
        if self.hole_cards.is_some() {
            // Make decision based on hole cards and aggression
            Action::check_action()
        } else {
            Action::fold_action(0.0)
        }
    }
}
```

### Running Simulations

```rust
use poker_api::simulator::texas_holdem::TexasHoldemSimulation;

// Create and configure simulation
let mut simulation = TexasHoldemSimulation::new();
simulation.set_player_range(0, "AA,KK,QQ".to_string());
simulation.set_player_range(1, "22+,A2+".to_string());

// Run simulation
let results = simulation.run(10000);
println!("Simulation results: {:?}", results);
```

## Algorithm Details

### Perfect Hash Hand Evaluation

The core algorithm uses a perfect hash function to map each possible card combination to a unique index in a precomputed ranking table:

1. **Card Encoding**: Each card is encoded into a compact binary representation
2. **Perfect Hash**: Cards are mapped to unique indices using mathematical functions
3. **Table Lookup**: Precomputed ranking tables provide instant hand strength values
4. **Best Hand Selection**: For 6-7 card hands, finds optimal 5-card combination

### Table Generation Process

The first time an evaluator is created, it generates evaluation tables (~1-2 seconds):
- **612,978 combinations** processed for 5-card hands
- **32+ million entries** in final lookup table
- **Mathematical verification** ensures accuracy
- **Binary serialization** for fast loading on subsequent runs

## Development

### Building the Project

```bash
# Clone the repository
git clone <repository-url>
cd poker-project

# Build all components
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

### Project Structure

Each component is designed to work independently while integrating seamlessly:

- **Core Library**: Standalone poker hand evaluation
- **Bot Framework**: Pluggable AI player system
- **Simulation Engine**: Controlled testing environment
- **Analysis Tools**: Performance monitoring and strategy analysis

### Contributing

The project welcomes contributions in several areas:
- **Bot Strategies**: New AI playing strategies
- **Game Variants**: Additional poker game types
- **Performance**: Optimizations and hardware acceleration
- **Analysis Tools**: Enhanced monitoring and reporting
- **Documentation**: Guides and tutorials

## Use Cases

### Poker Bot Development
- Develop and test poker AI strategies
- Analyze bot performance against known ranges
- Optimize decision-making algorithms

### Game Simulation
- Test poker scenarios and edge cases
- Analyze range vs range situations
- Study game theory optimal strategies

### Strategy Research
- Research optimal bet sizing
- Analyze position and stack size effects
- Develop exploitative strategies

### Performance Analysis
- Monitor poker application performance
- Identify bottlenecks and optimization opportunities
- Load testing for production systems

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- **Meerkat API**: Original Java implementation by Ray Wotton
- **Poker Standards**: 2+2 Poker hand ranking standards
- **Rust Community**: Performance optimizations and best practices
- **Contributors**: All contributors to the poker evaluation algorithms

---

**Note**: This project focuses on poker hand evaluation and bot development. It does not include real-money gambling functionality and is intended for educational and research purposes.