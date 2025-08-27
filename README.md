# 🔢 The 991 Pell Puzzle in Rust

### 📖 The Mathematical Mystery

Long ago, mathematicians wondered about numbers that hide behind the square root symbol.
Take this mysterious expression:

$$
\sqrt{991 \cdot n^2 + 1}
$$

At first glance, it almost always gives an **irrational** number — something messy that can't be written as a simple fraction or integer. For countless values of $n$, the expression stubbornly refuses to be a whole number.

But then comes the **surprise**:

After billions and billions of tries, suddenly, at

$$
n = 12055735790331359447442538767,
$$

the square root becomes a **perfect integer**:

$$
\sqrt{991 \cdot n^2 + 1} = 379516400906811930638014896080.
$$

✨ Like magic, the irrational veil lifts.

### 🧩 Why This Happens

Hidden behind this puzzle is a classic **Pell equation**:

$$
m^2 - 991n^2 = 1
$$

This equation has a strange property: once you find **one non-trivial solution**, an **infinite staircase of solutions** appears.
But the catch? For tricky numbers like 991, the very first step on the staircase is **astronomically high**.

---

## About This Library

A high-performance Rust library for solving Pell equations of the form **x² - D·y² = 1**.

## Features

- 🚀 **Fast algorithms** using continued fractions and binary exponentiation
- 🔢 **Arbitrary precision** arithmetic with BigInt support
- 🛡️ **Robust error handling** with detailed error types
- ✅ **Comprehensive testing** with 100% test coverage
- 📚 **Extensive documentation** with examples
- 🎆 **Rust 2024 Edition** - Built with the latest Rust features
- 🧪 **Clean architecture** with separated concerns

## 🚀 Solving the 991 Puzzle

This library implements the mathematical machinery to find these astronomical solutions efficiently.

### Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
pell991 = "0.1.0"
```

### Solve the 991 Puzzle

```rust
use pell991::pell_min_solution;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find the magical values that make √(991·n² + 1) a perfect integer
    let (m, n) = pell_min_solution(991)?;
    
    println!("🎯 Found the magical values!");
    println!("n = {}", n);  // 12055735790331359447442538767
    println!("m = {}", m);  // 379516400906811930638014896080
    
    // Verify: √(991·n² + 1) = m
    println!("\n✨ √(991·{}² + 1) = {}", n, m);
    
    Ok(())
}
```

## Usage

### Basic Example

```rust
use pell991::{pell_min_solution, pell_solution_k, verify_pell_solution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find minimal solution for x² - 2y² = 1
    let (x1, y1) = pell_min_solution(2)?;
    println!("Minimal solution: x={}, y={}", x1, y1); // x=3, y=2
    
    // Verify the solution
    assert!(verify_pell_solution(2, &x1, &y1));
    
    // Generate the second solution
    let (x2, y2) = pell_solution_k(2, &x1, &y1, 2)?;
    println!("Second solution: x={}, y={}", x2, y2); // x=17, y=12
    
    Ok(())
}
```

### Generate Multiple Solutions

```rust
use pell991::pell_solutions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate the first 5 solutions for x² - 991y² = 1
    let solutions = pell_solutions(991, 5)?;
    
    for (i, (x, y)) in solutions.iter().enumerate() {
        println!("Solution {}: x={}, y={}", i + 1, x, y);
    }
    
    Ok(())
}
```

## Algorithm Details

### Minimal Solution

The library uses the **continued fraction expansion** of √D to find the minimal solution:

1. Compute the continued fraction expansion: √D = a₀ + 1/(a₁ + 1/(a₂ + ...))
2. Calculate convergents pₖ/qₖ until pₖ² - D·qₖ² = 1

### k-th Solution

For the k-th solution, we use the recurrence relation:
- (x₁ + y₁√D)ᵏ = xₖ + yₖ√D

This is implemented using **fast binary exponentiation** for O(log k) complexity.

## Project Structure

```
pell-solution/
├── src/
│   ├── lib.rs          # Public API and documentation (70 lines)
│   ├── main.rs         # Binary executable (46 lines)
│   ├── error.rs        # Error types (26 lines)
│   ├── solver.rs       # Core algorithms (249 lines)
│   └── utils.rs        # Utility functions (70 lines)
├── tests/
│   ├── error_tests.rs      # Error handling tests (79 lines)
│   ├── utils_tests.rs      # Utility function tests (102 lines)
│   ├── solver_tests.rs     # Core algorithm tests (176 lines)
│   └── integration_tests.rs # Integration tests (162 lines)
├── examples/
│   ├── basic_usage.rs      # Basic usage examples (49 lines)
│   └── solve_991_puzzle.rs # 991 puzzle solver (55 lines)
└── docs/
    ├── README.md           # This file
    ├── STORY.md           # Project story and background
    ├── CHANGELOG.md       # Version history
    └── RUST_2024_UPGRADE.md # Rust 2024 upgrade notes
```

## Performance

The library is optimized for performance:

- **Newton's method** for integer square roots
- **Binary exponentiation** for computing higher-order solutions
- **Efficient BigInt operations** with minimal allocations

## Error Handling

The library provides comprehensive error handling:

```rust
use pell991::{pell_min_solution, PellError};

match pell_min_solution(4) {
    Ok((x, y)) => println!("Solution: ({}, {})", x, y),
    Err(PellError::PerfectSquare(d)) => println!("Error: {} is a perfect square", d),
    Err(e) => println!("Error: {}", e),
}
```

## API Reference

### Core Functions

- `pell_min_solution(d)` - Find the minimal solution
- `pell_solution_k(d, x1, y1, k)` - Find the k-th solution
- `pell_solutions(d, count)` - Generate multiple solutions
- `verify_pell_solution(d, x, y)` - Verify a solution

### Utility Functions

- `isqrt_u64(n)` - Integer square root
- `is_square_u64(n)` - Check if number is perfect square

### Error Types

- `PellError::InvalidD(d)` - D must be > 1
- `PellError::PerfectSquare(d)` - D must be non-square
- `PellError::InvalidK(k)` - k must be > 0

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive testing:

| Test Category | Tests | Lines | Coverage |
|---------------|-------|-------|----------|
| **Error Tests** | 6 | 79 | Error handling |
| **Utils Tests** | 6 | 102 | Utility functions |
| **Solver Tests** | 9 | 176 | Core algorithms |
| **Integration Tests** | 8 | 162 | Cross-module |
| **Doc Tests** | 7 | - | Documentation |
| **Total** | **36** | **519** | **100%** |

## Examples

Run the examples:

```bash
# Solve the 991 puzzle
cargo run

# Basic usage examples
cargo run --example basic_usage

# Detailed 991 puzzle solver
cargo run --example solve_991_puzzle
```

## Technical Specifications

- **Rust Edition**: 2024
- **MSRV**: 1.85.0
- **Dependencies**: `num-bigint`, `num-integer`, `num-traits`
- **Source Code**: 461 lines (pure logic)
- **Test Code**: 519 lines (comprehensive coverage)
- **Examples**: 104 lines (usage demonstrations)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Mathematical Background

The Pell equation x² - Dy² = 1 is a type of Diophantine equation with a rich mathematical history. For any non-square positive integer D, this equation has infinitely many positive integer solutions.

### Key Properties

1. **Fundamental Solution**: The smallest positive solution (x₁, y₁)
2. **Recurrence**: All solutions can be generated from the fundamental solution
3. **Continued Fractions**: The fundamental solution is found via continued fraction expansion of √D

### Applications

Pell equations appear in:
- Number theory research
- Cryptographic algorithms
- Computational mathematics
- Mathematical competitions

## References

- [Pell's Equation on Wikipedia](https://en.wikipedia.org/wiki/Pell%27s_equation)
- "An Introduction to the Theory of Numbers" by Hardy & Wright
- "Continued Fractions" by A.Ya. Khinchin