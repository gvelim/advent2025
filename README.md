# Advent of Code 2025

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) implemented in Rust.

## Project Overview

This repository contains Rust solutions for Advent of Code 2025 puzzles. Each day's solution includes:

- Efficient algorithm implementations
- Comprehensive documentation explaining problem-solving approach
- Educational commentary on design decisions and programming principles
- Unit tests for correctness verification

## Implemented Days

- **Day 1**: [Rotating Dial Problem](src/bin/day1/README.md) - Count zero landings and crossings using modular arithmetic and functional programming patterns

## Running Solutions

To run any day's solution:

```bash
# Run Day 1
cargo run --bin day1

# Run tests for Day 1
cargo test --bin day1
```

## Project Structure

```
advent2025/
├── src/
│   └── bin/
│       └── day1/
│           ├── main.rs          # Main solution implementation
│           ├── README.md        # Detailed documentation
│           ├── dial/           # Dial module
│           │   └── mod.rs
│           └── input.txt       # Problem input
```

## Learning Resources

Each implemented day includes educational documentation that covers:

- Problem-solving intuition and mathematical insights
- Design patterns and architectural decisions
- Common pitfalls and how to avoid them
- Rust-specific idioms and best practices

See individual day README files for detailed explanations.

## License

This project is for educational purposes as part of Advent of Code 2025.
