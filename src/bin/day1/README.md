# Day 1: Rotating Dial Problem

## Problem Overview

A rotary dial (similar to a combination lock or safe dial) that can rotate left and right. Starting from position 50, we execute a sequence of movements and need to track:

- **Part 1**: How many times does the needle land exactly on position 0?
- **Part 2**: How many times does the needle cross position 0 during all movements?

The challenge is to count these events efficiently without simulating every single step of movement.

## Solution Intuition

The key insight is that we don't need to simulate step-by-step movement. Instead, we can use mathematical properties to calculate zero crossings directly.

### The Core Mathematical Insight

Any movement can be decomposed into:

- **Full perimeter crossings**: Complete rotations around the dial
- **Residual movement**: The remaining steps after full rounds

For a dial with perimeter 100:

- Moving 871 steps from position 80 → 8 full rounds + 71 residual steps
- Each full round crosses zero exactly once
- We only need to check if the **residual** movement also crosses zero

This transforms a potentially O(n²) simulation problem into an O(n) calculation with O(1) per-action complexity.

## Architecture & Design

### Data Structures

```rust
pub struct RotaryDial {
    perimeter: Steps,    // Dial size (100 in this problem)
    cursor: Steps,      // Raw position (can be negative, handles wrap-around)
    needle: Steps,      // Normalized position (always 0-99)
}
```

**Design Decisions:**

1. **Separate cursor and needle**:
   - `cursor`: Tracks raw mathematical position with wrap-around
   - `needle`: Provides user-friendly 0-99 representation
   - This separation allows different methods to use the appropriate representation

2. **Using modular arithmetic**:
   - Natural handling of wrap-around behavior
   - No conditional branching for boundary conditions
   - Performance: O(1) operations instead of O(n)

## Implementation Details

### 1. The Turn Operation

```rust
pub fn turn(&mut self, act: &Action) -> Steps {
    self.cursor = (self.cursor + (act.turn as Steps) * act.steps) % self.perimeter;
    self.needle = self.cursor + if self.cursor < 0 { 100 } else { 0 };
    self.needle
}
```

**How it works:**

1. **Modular wrap-around**: `cursor % perimeter` handles rotation automatically
   - `150 % 100 = 50` (1 full rotation + 50)
   - `-50 % 100 = -50` (Rust preserves sign, handled in step 2)

2. **Needle normalization**: Convert to 0-99 range
   - If `cursor < 0`, add perimeter to get correct representation
   - Example: cursor = -15 → needle = 85

3. **Returns normalized position**: Enables easy checking for `needle == 0`

**Why this works:**

The combination of modular arithmetic and normalization ensures:

- Correct wrap-around behavior in both directions
- Consistent 0-99 representation for zero-checking
- No special cases needed for boundary conditions

### 2. Zero Crossing Calculation

```rust
pub fn count_zero_crossings(&self, act: &Action) -> Steps {
    let RotaryDial {
        perimeter,
        needle: last,
        ..
    } = *self;

    let residual_steps = act.steps % perimeter;
    let x_zone = match act.turn {
        Turn::Left | Turn::Right if last == 0 => perimeter,
        Turn::Left => last,
        Turn::Right => perimeter - last,
    };

    act.steps / perimeter + if residual_steps >= x_zone { 1 } else { 0 }
}
```

**Breaking down the logic:**

#### Part 1: Full Rounds

```rust
act.steps / perimeter
```

Integer division gives us the number of complete rotations. Each rotation crosses zero exactly once.

**Example:** `851 steps / 100 = 8` full rounds = 8 crossings

#### Part 2: The Crossing Zone

The crossing zone determines if residual movement crosses zero:

- **For left turns**: Need to move at least `last` steps to cross zero
  - Starting at 25, moving left: need 25+ steps to reach 0

- **For right turns**: Need to move at least `perimeter - last` steps
  - Starting at 75, moving right: need 25+ steps (100-75) to reach 0

#### Part 3: Critical Edge Case

```rust
Turn::Left | Turn::Right if last == 0 => perimeter
```

**Why this matters:**

When starting at position 0, you must move a full perimeter distance to complete a circle and return to 0.

**Without this fix:**

- `x_zone = 0` for left turns from position 0
- `residual >= 0` is always true → incorrectly counts extra crossings
- Example: 0 ← 305 steps should be 3 crossings, not 4

**With this fix:**

- `x_zone = perimeter` (100 steps needed)
- `residual >= 100` only true for actual full rounds → correct counting

#### Putting It Together

```rust
let res = act.steps / perimeter + if residual_steps >= x_zone { 1 } else { 0 };
```

**Total crossings = full rounds + (did residual cross zero?)**

### 3. Functional Accumulation

#### The Problematic Reduce Implementation

```rust
.map(|a| (dial.count_zero_crossings(a), dial.turn(a)))
.reduce(|(acc_cross, acc_zero), (crossings, needle)| {
    (
        acc_cross + crossings,
        acc_zero + if needle == 0 { 1 } else { 0 },
    )
})
```

**The Root Cause:**

`reduce()` uses the **first element as the initial accumulator**. This means:

- If first action lands on zero → `acc_zero` starts at 79 instead of 0
- Every subsequent check builds from this incorrect baseline
- Result: Overcounting by exactly the number of initial zero landings

This caused Part 1 to return 1048 instead of 969 (79 extra counts).

#### The Correct Fold Implementation

```rust
.fold((0, 0), |(crossing_sum, landing_sum), action| {
    let crossings = dial.count_zero_crossings(action);
    let needle = dial.turn(action);
    (
        crossing_sum + crossings,
        landing_sum + if needle == 0 { 1 } else { 0 },
    )
})
```

**Why this works:**

1. **Explicit initial state**: `(0, 0)` ensures both counters start at zero
2. **Clear separation**: Computation (`count_zero_crossings`, `turn`) vs accumulation (`crossing_sum`, `landing_sum`)
3. **No tuple confusion**: No ambiguity about which value represents what
4. **Idiomatic**: `fold()` is designed for this exact use case

## Key Lessons

### 1. Modular Arithmetic for Cyclic Problems

For any problem involving cycles, rotations, or periodic behavior:

- Use modulo (`%`) to handle wrap-around naturally
- Avoid conditional branching for boundary cases
- Performance: O(1) instead of O(n)

**Example:**

```rust
// Bad: Step-by-step simulation
for _ in 0..1000 {
    position = (position + direction) % perimeter;
    if position == 0 { count += 1; }
}

// Good: Direct calculation
let crossings = steps / perimeter + check_residual(steps % perimeter, position);
```

### 2. State Management Patterns

When tracking evolving state:

- Separate "computation" from "accumulation"
- Use pure functions where possible
- Explicit initial values avoid subtle bugs

**Good pattern:**

```rust
.fold((0, 0), |acc, item| {
    let computed = compute(item);     // Pure computation
    accumulate(acc, computed)       // Accumulation
})
```

### 3. Iterator Method Selection

Choose the right iterator method for the job:

| Method     | Use Case                         | Returns          | Initial Value |
| ---------- | -------------------------------- | ---------------- | ------------- |
| `map()`    | Transform each element           | New iterator     | N/A           |
| `filter()` | Select specific elements         | New iterator     | N/A           |
| `reduce()` | Fold to single value             | `Option<Result>` | First element |
| `fold()`   | Fold with explicit initial value | `Result`         | You specify   |

**Key difference:** `reduce()` uses first element as initial value, which can cause bugs when the first element shouldn't influence accumulation.

### 4. Edge Case Handling

Always test boundary conditions:

- Starting at zero (the "trivial" case)
- Exactly perimeter distance
- Multiple full rotations

**Pattern matching approach:**

```rust
match act.turn {
    Turn::Left | Turn::Right if last == 0 => perimeter,  // Edge case
    Turn::Left => last,                                     // Normal left
    Turn::Right => perimeter - last,                            // Normal right
}
```

This provides a clean, declarative way to handle edge cases.

## Results

```
Part 1: 969   (needle landings on zero)
Part 2: 5887  (zero crossings during movement)
```

Both answers were verified and accepted by Advent of Code 2025.

## Running the Solution

```bash
cargo run --bin day1
```

Run tests:

```bash
cargo test --bin day1
```

## Complexity Analysis

- **Time Complexity**: O(n) where n is the number of actions
- **Space Complexity**: O(1) - only state variables, no additional data structures
- **Per-Action Complexity**: O(1) - constant-time arithmetic operations

## References

- Advent of Code 2025 Day 1
- Modular arithmetic principles
- Functional programming patterns in Rust
