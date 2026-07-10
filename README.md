# Rust Doom

My attempt to build Doom classic in Rust.

## About

This project is a reimplementation of the classic Doom game, written entirely in Rust. The goal is to recreate the core gameplay and mechanics of the original 1993 game while leveraging modern Rust idioms and safety features.

## Features

- Retro Doom gameplay experience
- Implemented in 100% Rust
- Performance-focused design
- Cross-platform compatibility

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/susbakun/rust-doom.git
cd rust-doom
```

2. Build the project:
```bash
cargo build --release
```

3. Run the game:
```bash
cargo run --release
```

## Development

To build in development mode with debug symbols:
```bash
cargo build
```

To run tests:
```bash
cargo test
```

To run with verbose output:
```bash
cargo run -- --verbose
```

## Roadmap

- [ ] Core rendering engine
- [ ] Game loop and input handling
- [ ] Level loading and map rendering
- [ ] Enemy AI
- [ ] Weapon mechanics
- [ ] HUD and UI
- [ ] Sound and music
- [ ] Save/load functionality

## Resources

- [Raycasting Tutorial](https://lodev.org/cgtutor/raycasting3.html)
