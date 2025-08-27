//! # Pell Equation Solver
//!
//! This crate provides efficient algorithms for solving Pell equations of the form:
//! x² - D·y² = 1
//!
//! where D is a positive non-square integer.
//!
//! ## Features
//!
//! - Find the minimal solution using continued fractions
//! - Generate the k-th solution using fast exponentiation
//! - Support for arbitrarily large integers using BigInt
//!
//! ## Example
//!
//! ```rust
//! use pell991::{pell_min_solution, pell_solution_k};
//!
//! let d = 2;
//! let (x1, y1) = pell_min_solution(d).unwrap();
//! println!("Minimal solution: x={}, y={}", x1, y1); // x=3, y=2
//!
//! let (x2, y2) = pell_solution_k(d, &x1, &y1, 2).unwrap();
//! println!("Second solution: x={}, y={}", x2, y2); // x=17, y=12
//! ```

pub mod error;
pub mod solver;
pub mod utils;

// Re-export the main API
pub use error::PellError;
pub use solver::{
    pell_min_solution, 
    pell_min_solution_unchecked,
    pell_solution_k, 
    pell_solution_k_unchecked,
    pell_solutions,
    verify_pell_solution,
};
pub use utils::{isqrt_u64, is_square_u64};