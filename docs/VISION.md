# Poker AI Testbed & Live Server Vision

## Overall Goal

Create a **comprehensive poker AI ecosystem** with two main components:

- **Massive Simulation Testbed** – For developing and testing bot strategies across all game types (NL/FL/PL, Cash/SNG/MTT) with configurable parameters  
- **Live Action Server** – For deploying bots in real poker environments with automatic table selection  

---

## Core Architecture Components

### 1. Advanced Evaluation Engine

- **Perfect Hash Jump Tables**: Revolutionary memory-efficient evaluation (~130MB vs 625MB)
- **Suit Canonicalization**: Advanced isomorphic hand reduction for optimal performance
- **Bottom-up Trie Construction**: Cache-optimized memory layout for CPU efficiency
- **Zero Breaking Changes**: Full backward compatibility with existing evaluation APIs

### 2. Simulation Testbed

- **Configurable bots** (Rule-based, Nash EQ, Statistical, Hand-reading, Reinforcement Learning)
- **Mass parallel simulations** with GPU acceleration (e.g., for turn and river rollouts)
- **Memory-efficient evaluation**: 79% memory reduction enables larger-scale simulations
- **TOML-driven configuration** for all parameters of bots and simulations
- **Comprehensive result analysis** and bot evolution

### 2. Live Action Server

- **Generic protocol** for client communication  
- **Early folding capability** (bots can fold without being polled)  
- **Multiple client adapters** for different poker sites  
- **Automated table selection** based on game configuration  
- **Real-time strategy adaptation**  

### 3. Bot Concepts

- **Modular Strategy** per stage and game situation – e.g., low stack in a tournament, preflop vs. postflop.  
  Plugable sub-strategies and decision methods (first applicable, random, most aggressive, least aggressive, etc.)  
- **Generic helper functions** for strategies like ICM, ppot, not, hand strength, against one and multiple opponents  
- **Incremental, asynchronous rollout** of turn and river on flop and river on turn  
- **Flop Playability score** for preflop based on precalculated rollouts for all possible flops and starting hands  
- **Opponent hand prediction and statistics**, for instance in Poker Tracker format  
- **Real-time strategy adaptation**, e.g., reinforcement learning  

---

## Expected Outcomes

- **Research Platform:** Systematic testing of poker AI strategies  
- **Live Deployment:** Bots that can play on real poker sites  
- **Strategy Insights:** Understanding what works in different game conditions  
- **Bot Evolution:** Self-improving systems through simulation and adaptation