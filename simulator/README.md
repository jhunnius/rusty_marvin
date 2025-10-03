# Poker Simulator

## Overview

The `simulator/` directory contains tools and frameworks for simulating poker games, testing strategies, and analyzing poker scenarios. These tools enable comprehensive testing and development of poker bots and strategies in controlled environments.

## Components

### Texas Hold'em Simulator (`src/texas_holdem/`)

Core simulation engine for Texas Hold'em poker games.

#### Features
- **Complete Game Simulation**: Full Texas Hold'em game lifecycle
- **Multiple Players**: Support for 2-10 players per simulation
- **Configurable Rules**: Customizable game rules and parameters
- **Hand Range Support**: Define specific hand ranges for testing
- **Equity Calculation**: Accurate pot odds and equity computation
- **Statistical Analysis**: Comprehensive results and analytics

#### Simulation Capabilities
- **Hand Range Analysis**: Test bots against specific hand ranges
- **Monte Carlo Simulation**: Run thousands of scenarios for statistical validity
- **Tournament Simulation**: Multi-table tournament scenarios
- **Cash Game Simulation**: Ring game format with varying stack sizes
- **Heads-Up Simulation**: One-on-one bot testing and analysis

## Usage

### Basic Simulation Setup

```rust
use poker_api::api::card::Card;
use poker_api::api::hand::Hand;

// Create a simulation scenario
let mut simulation = TexasHoldemSimulation::new();

// Configure players
simulation.add_player("Bot1", 1000.0);
simulation.add_player("Bot2", 1000.0);

// Set up hand range for testing
simulation.set_player_range(0, "AA,KK,QQ,AK".to_string());

// Run simulation
let results = simulation.run(1000); // Run 1000 hands

println!("Results: {:?}", results);
```

### Advanced Simulation

```rust
// Configure detailed simulation parameters
let config = SimulationConfig {
    num_players: 6,
    starting_stack: 1000.0,
    blind_levels: vec![
        BlindLevel { small_blind: 10.0, big_blind: 20.0, duration_minutes: 10 },
        BlindLevel { small_blind: 15.0, big_blind: 30.0, duration_minutes: 10 },
    ],
    hand_range: "22+,A2+,K2+,Q2+,J2+,T2+,92+,82+,72+,62+,52+,42+,32+".to_string(),
};

let mut tournament = TournamentSimulation::new(config);
tournament.run();
```

## Analysis Tools

### Hand Analysis

```rust
// Analyze specific hand scenarios
let analyzer = HandAnalyzer::new();

let hole_cards = vec!["As", "Ks"];
let board = vec!["Td", "Jc", "Qh"];

let analysis = analyzer.analyze_hand(hole_cards, board);
println!("Hand strength: {}", analysis.strength);
println!("Equity vs random: {}", analysis.equity_vs_random);
```

### Range Analysis

```rust
// Analyze range vs range scenarios
let range_analyzer = RangeAnalyzer::new();

let range1 = "AA,AK, AQ".to_string(); // Premium hands
let range2 = "22+,A2+,K2+".to_string(); // Wide range

let results = range_analyzer.analyze_ranges(range1, range2, board);
println!("Range1 equity: {:.2}%", results.range1_equity);
println!("Range2 equity: {:.2}%", results.range2_equity);
```

## Testing Framework

### Bot Testing

```rust
use poker_api::simulator::testing::BotTester;

// Create bot tester
let mut tester = BotTester::new();

// Add bots to test
tester.add_bot("AggressiveBot", Box::new(AggressiveBot::new()));
tester.add_bot("TightBot", Box::new(TightBot::new()));

// Define test scenarios
let scenarios = vec![
    TestScenario::heads_up(1000),
    TestScenario::six_max(500),
    TestScenario::full_ring(1000),
];

// Run comprehensive tests
let results = tester.run_scenarios(scenarios);
tester.generate_report("bot_test_results.html");
```

### Strategy Testing

```rust
// Test specific strategies
let strategy_tester = StrategyTester::new();

let strategies = vec![
    Strategy::new("Tight Aggressive", config_tight_aggressive),
    Strategy::new("Loose Passive", config_loose_passive),
    Strategy::new("Balanced", config_balanced),
];

let results = strategy_tester.test_strategies(strategies, 10000);
println!("Strategy comparison: {:?}", results);
```

## Performance

### Simulation Speed
- **Simple Hand**: ~1ms per hand simulation
- **Complex Analysis**: ~10ms per hand with detailed analysis
- **Monte Carlo**: ~100ms per 1000 scenarios
- **Tournament**: ~1s per 100-hand tournament

### Memory Usage
- **Base Simulation**: ~1MB per active simulation
- **Hand History**: ~10KB per 1000 hands stored
- **Analysis Data**: ~50MB per comprehensive analysis

## Configuration

### Simulation Parameters

```toml
[simulation]
num_players = 6
starting_stack = 1000
num_hands = 10000
save_hand_history = true

[blinds]
small_blind = 10.0
big_blind = 20.0
ante = 0.0

[ranges]
player1 = "AA,KK,QQ,AK"
player2 = "22+,A2+,K2+,Q2+,J2+"
player3 = "random"

[analysis]
detailed_equity = true
save_distributions = true
confidence_interval = 0.95
```

### Custom Scenarios

```rust
// Create custom test scenarios
let scenario = SimulationScenario {
    name: "Premium vs Fish".to_string(),
    players: vec![
        PlayerConfig { name: "Pro", range: "AA,KK,QQ,AK,AQ".to_string(), stack: 2000 },
        PlayerConfig { name: "Fish", range: "22+,A2+,K2+".to_string(), stack: 500 },
    ],
    board_texture: BoardTexture::Dry, // Flop texture for analysis
    num_simulations: 10000,
};
```

## Output and Reporting

### Results Format

```rust
pub struct SimulationResults {
    pub total_hands: usize,
    pub player_results: Vec<PlayerResult>,
    pub hand_distribution: HashMap<String, usize>,
    pub equity_distribution: Vec<f64>,
    pub confidence_intervals: Vec<(f64, f64)>,
}
```

### Report Generation

```rust
// Generate comprehensive reports
let reporter = SimulationReporter::new();
reporter.add_results(results);
reporter.generate_html_report("simulation_report.html");
reporter.generate_csv_summary("summary.csv");
```

## Integration

The simulator integrates with:
- **Core API**: Uses hand evaluation and card management
- **Bot Framework**: Tests bot implementations in controlled environments
- **Analysis Tools**: Provides data for strategy analysis and improvement
- **Log System**: Integrates with logging and monitoring tools

## Development

### Extending the Simulator

#### Adding New Game Types
```rust
// Implement new poker variant
pub struct OmahaSimulation {
    // Omaha-specific logic
}

impl PokerSimulation for OmahaSimulation {
    fn simulate_hand(&mut self) -> HandResult {
        // Omaha simulation logic
    }
}
```

#### Custom Analysis Modules
```rust
// Add custom analysis
pub struct BluffAnalyzer {
    // Bluff detection logic
}

impl HandAnalyzer for BluffAnalyzer {
    fn analyze(&self, hand: &Hand) -> AnalysisResult {
        // Custom analysis logic
    }
}
```

## Use Cases

### Bot Development
- Test bot strategies against known ranges
- Identify leaks in bot decision-making
- Optimize bot parameters and configurations

### Strategy Research
- Analyze range vs range scenarios
- Study board texture effects
- Research optimal bet sizing

### Game Theory
- Equilibrium analysis for different stack sizes
- Nash equilibrium calculations
- Exploitability analysis

### Performance Testing
- Load testing for poker platforms
- Scalability analysis
- Performance benchmarking