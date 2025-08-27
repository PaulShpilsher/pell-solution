//! # The 991 Pell Puzzle
//!
//! ## The Mathematical Mystery
//!
//! Long ago, mathematicians wondered about numbers that hide behind the square root symbol.
//! Take this mysterious expression:
//!
//! ```text
//! √(991 · n² + 1)
//! ```
//!
//! At first glance, it almost always gives an **irrational** number — something messy that 
//! can't be written as a simple fraction or integer. For countless values of n, the expression 
//! stubbornly refuses to be a whole number.
//!
//! But then comes the **surprise**: After billions and billions of tries, suddenly, at
//!
//! ```text
//! n = 12055735790331359447442538767
//! ```
//!
//! the square root becomes a **perfect integer**:
//!
//! ```text
//! √(991 · n² + 1) = 379516400906811930638014896080
//! ```
//!
//! Like magic, the irrational veil lifts.
//!
//! ## The Hidden Mathematics
//!
//! Behind this puzzle lies a classic **Pell equation**: `m² - 991n² = 1`
//!
//! This crate provides efficient algorithms for solving such equations using continued fractions
//! and fast exponentiation, revealing the infinite staircase of solutions.
//!
//! ## Solving the Puzzle
//!
//! ```rust
//! use pell991::pell_min_solution;
//!
//! // Find the magical values that make √(991·n² + 1) a perfect integer
//! let (m, n) = pell_min_solution(991).unwrap();
//! println!("n = {}", n);  // 12055735790331359447442538767
//! println!("m = {}", m);  // 379516400906811930638014896080
//! ```
//!

pub mod error;
pub mod solver;
pub mod utils;

pub use error::PellError;
pub use solver::{
    pell_min_solution, 
    pell_min_solution_unchecked,
    pell_solution_k, 
    pell_solution_k_unchecked,
    pell_solutions,
    verify_pell_solution,
    PellSolutionIterator,
};
pub use utils::{
    isqrt_u64, 
    is_square_u64, 
    is_valid_pell_d, 
    estimate_period_length, 
    fundamental_discriminant, 
    is_prime
};

/// Re-export BigInt for convenience
pub use num_bigint::BigInt;