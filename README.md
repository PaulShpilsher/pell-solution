# Pell Equation Solver

A high-performance Rust library for solving Pell equations of the form **x² - D·y² = 1**.

## Features

- 🚀 **Fast algorithms** using continued fractions and binary exponentiation
- 🔢 **Arbitrary precision** arithmetic with BigInt support
- 🛡️ **Robust error handling** with detailed error types
- ✅ **Comprehensive testing** with 100% test coverage
- 📊 **Performance benchmarks** included
- 📚 **Extensive documentation** with examples

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
pell991 = "0.1.0"
```

## Usage

### Basic Example

```rust
use pell991::{pell_min_solution, pell_solution_k, verify_pell_solution};
use num_bigint::BigInt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find the minimal solution for x² - 2y² = 1
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

## Performance

The library is optimized for performance:

- **Newton's method** for integer square roots
- **Binary exponentiation** for computing higher-order solutions
- **Efficient BigInt operations** with minimal allocations

Run benchmarks with:
```bash
cargo bench
```

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

The library includes:
- Unit tests for all functions
- Property-based tests
- Error condition tests
- Integration tests

## Examples

See the `examples/` directory for more usage examples:

```bash
cargo run --example basic
cargo run --example large_numbers
cargo run --example performance_demo
```

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