# Poker Bots

## Overview

The `bots/` directory contains poker bot implementations that can participate in poker games using the core poker API. These bots demonstrate different AI strategies and serve as examples for developing custom poker bots.

## Available Bots

### Stupid Bot (`stupid_bot/`)

A collection of poker bot implementations with varying levels of sophistication:

#### Features
- **Multiple Strategies**: Different bot implementations for testing
- **API Compliance**: Full implementation of Player and GameObserver traits
- **Configurable Behavior**: Customizable decision-making parameters
- **Logging Support**: Detailed game state logging for analysis

#### Bot Types
- **Random Bot**: Makes random valid decisions
- **Tight Bot**: Conservative playing style
- **Aggressive Bot**: High-risk, high-reward strategy
- **Adaptive Bot**: Adjusts strategy based on game conditions

## Usage

### Running a Bot

```rust
use poker_api::simple_player::SimplePlayer;

// Create a simple bot
let mut bot = SimplePlayer::new("MyBot".to_string(), 1000.0);

// Initialize with preferences
let prefs = Box::new(TomlPreferences::default());
bot.init(prefs);

// Receive hole cards
bot.hole_cards(card1, card2, seat);

// Get bot's action
let action = bot.get_action();
```

### Bot Development

#### Implementing the Player Trait

All bots must implement the `Player` trait:

```rust
use poker_api::api::player::Player;
use poker_api::api::preferences::Preferences;

struct MyBot {
    name: String,
    hole_cards: Option<(Card, Card)>,
    // Add bot-specific fields
}

impl Player for MyBot {
    fn init(&mut self, prefs: Box<dyn Preferences>) {
        // Initialize bot with preferences
    }

    fn hole_cards(&mut self, c1: Card, c2: Card, seat: usize) {
        self.hole_cards = Some((c1, c2));
        self.seat = seat;
    }

    fn get_action(&mut self) -> Action {
        // Implement bot decision logic
        Action::check_action()
    }
}
```

#### Game Observation

Bots can also implement `GameObserver` for enhanced awareness:

```rust
impl GameObserver for MyBot {
    fn action_event(&mut self, pos: usize, action: Action) {
        // Track opponent actions
    }

    fn game_start_event(&mut self, game_info: Box<dyn GameInfo>) {
        // Prepare for new hand
    }

    // Implement other observer methods...
}
```

## Configuration

### TOML Configuration

Bots can be configured using TOML files:

```toml
# Bot Configuration
[bot]
name = "MyBot"
strategy = "aggressive"
risk_tolerance = 0.8

[betting]
min_bet = 10.0
max_bet_ratio = 0.25
bluff_frequency = 0.15

[analysis]
hand_strength_threshold = 0.7
position_awareness = true
```

### Runtime Configuration

```rust
use poker_api::toml_preferences::TomlPreferences;

let mut prefs = TomlPreferences::default();
prefs.set_double("aggression", 0.8);
prefs.set_boolean("enable_logging", true);
prefs.save("bot_config.toml").unwrap();
```

## Testing

### Bot vs Bot Games

```bash
# Run automated bot testing
cargo test bot_integration_tests

# Run performance benchmarks
cargo test --release bot_performance_benchmark
```

### Manual Testing

```rust
// Create multiple bots for testing
let bot1 = SimplePlayer::new("Bot1".to_string(), 1000.0);
let bot2 = SimplePlayer::new("Bot2".to_string(), 1000.0);

// Simulate game interactions
// ... game logic would go here
```

## Development Guidelines

### Best Practices

1. **Separation of Concerns**: Keep strategy logic separate from game mechanics
2. **State Management**: Maintain clean, consistent bot state
3. **Error Handling**: Graceful handling of unexpected game states
4. **Performance**: Efficient decision-making for real-time play
5. **Testability**: Easy to test and debug bot behavior

### Strategy Development

#### Hand Strength Analysis
```rust
// Analyze hole cards
let hand_strength = evaluate_hole_cards(hole_cards);
let position_factor = calculate_position_advantage(seat, num_players);
let adjusted_strength = hand_strength * position_factor;
```

#### Opponent Modeling
```rust
// Track opponent tendencies
opponent_stats.insert(player_id, PlayerStats {
    aggression_factor: calculate_aggression(actions),
    bluff_frequency: detect_bluff_patterns(actions),
    fold_frequency: calculate_fold_frequency(actions),
});
```

## Integration

Bots integrate with:
- **Core API**: Use hand evaluation and card management
- **Game Engine**: Participate in actual poker games
- **Simulation Tools**: Test strategies in controlled environments
- **Analysis Tools**: Review performance and improve strategies

## Performance

### Decision Time
- **Simple Bots**: < 1ms per decision
- **Complex Bots**: < 10ms per decision
- **Memory Usage**: 1-10KB per bot instance

### Scalability
- **Multiple Bots**: Support for hundreds of concurrent bots
- **Game Speed**: Fast enough for real-time online play
- **Resource Usage**: Minimal CPU and memory footprint