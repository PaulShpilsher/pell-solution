//! Core Pell equation solving algorithms

use num_bigint::BigInt;
use num_traits::{One, Zero};
use crate::error::PellError;
use crate::utils::{isqrt_u64, is_square_u64};

/// Solve the Pell equation x² - D·y² = 1 for non-square D > 1.
///
/// This function uses the continued fraction expansion of √D to find
/// the minimal positive integer solution (x, y) to the Pell equation.
///
/// # Arguments
///
/// * `d_constant` - The coefficient D in the Pell equation (must be > 1 and non-square)
///
/// # Returns
///
/// A `Result` containing a tuple `(x, y)` representing the minimal solution,
/// or a `PellError` if the input is invalid.
///
/// # Errors
///
/// Returns `PellError::InvalidD` if `d_constant` ≤ 1.
/// Returns `PellError::PerfectSquare` if `d_constant` is a perfect square.
///
/// # Algorithm
///
/// Uses the continued fraction expansion of √D:
/// √D = a₀ + 1/(a₁ + 1/(a₂ + ...))
///
/// The convergents pₖ/qₖ of this expansion eventually yield a solution
/// to the Pell equation when pₖ² - D·qₖ² = 1.
///
/// # Examples
///
/// ```
/// use num_bigint::BigInt;
/// # use pell991::pell_min_solution;
///
/// let (x, y) = pell_min_solution(2).unwrap();
/// assert_eq!(x, BigInt::from(3));
/// assert_eq!(y, BigInt::from(2));
/// // Verification: 3² - 2·2² = 9 - 8 = 1 ✓
/// ```
pub fn pell_min_solution(d_constant: u64) -> Result<(BigInt, BigInt), PellError> {
    if d_constant <= 1 {
        return Err(PellError::InvalidD(d_constant));
    }
    if is_square_u64(d_constant) {
        return Err(PellError::PerfectSquare(d_constant));
    }

    let a0 = isqrt_u64(d_constant);
    let mut m: i128 = 0;
    let mut d: i128 = 1;
    let mut a: i128 = a0 as i128;

    // Convergents: p[-2]=0, p[-1]=1; q[-2]=1, q[-1]=0
    let p_prev2 = BigInt::zero();
    let mut p_prev1 = BigInt::one();
    let q_prev2 = BigInt::one();
    let mut q_prev1 = BigInt::zero();

    let mut p = BigInt::from(a) * &p_prev1 + &p_prev2;
    let mut q = BigInt::from(a) * &q_prev1 + &q_prev2;

    let big_d = BigInt::from(d_constant);

    loop {
        let lhs = &p * &p - &big_d * &q * &q;
        if lhs.is_one() {
            return Ok((p, q));
        }

        m = d * a - m;
        d = ((d_constant as i128) - m * m) / d;
        a = ((a0 as i128) + m) / d;

        let a_big = BigInt::from(a);

        let p_next = &a_big * &p + &p_prev1;
        let q_next = &a_big * &q + &q_prev1;

        p_prev1 = p;
        q_prev1 = q;
        p = p_next;
        q = q_next;
    }
}

/// Solve the Pell equation x² - D·y² = 1 for non-square D > 1 (panicking version).
///
/// This is a convenience wrapper around `pell_min_solution` that panics on error.
/// Use `pell_min_solution` for better error handling.
///
/// # Panics
///
/// Panics if `d_constant` ≤ 1 or if `d_constant` is a perfect square.
pub fn pell_min_solution_unchecked(d_constant: u64) -> (BigInt, BigInt) {
    pell_min_solution(d_constant).unwrap()
}

/// Generate the k-th Pell solution (xₖ, yₖ) given the minimal solution.
///
/// This function computes the k-th solution to the Pell equation x² - D·y² = 1
/// using the recurrence relation based on the fundamental solution.
///
/// # Arguments
///
/// * `d_constant` - The coefficient D in the Pell equation
/// * `x1` - The x-coordinate of the minimal solution
/// * `y1` - The y-coordinate of the minimal solution  
/// * `k` - The index of the desired solution (k ≥ 1)
///
/// # Returns
///
/// A `Result` containing a tuple `(xₖ, yₖ)` representing the k-th solution,
/// or a `PellError` if k is invalid.
///
/// # Errors
///
/// Returns `PellError::InvalidK` if `k` is 0.
///
/// # Algorithm
///
/// Uses the identity: (x₁ + y₁√D)ᵏ = xₖ + yₖ√D
/// 
/// Implemented using fast binary exponentiation for efficiency.
///
/// # Examples
///
/// ```
/// use num_bigint::BigInt;
/// # use pell991::{pell_min_solution, pell_solution_k};
///
/// let d = 2;
/// let (x1, y1) = pell_min_solution(d).unwrap();
/// let (x2, y2) = pell_solution_k(d, &x1, &y1, 2).unwrap();
/// assert_eq!(x2, BigInt::from(17));
/// assert_eq!(y2, BigInt::from(12));
/// ```
pub fn pell_solution_k(d_constant: u64, x1: &BigInt, y1: &BigInt, k: u64) -> Result<(BigInt, BigInt), PellError> {
    if k == 0 {
        return Err(PellError::InvalidK(k));
    }
    if k == 1 {
        return Ok((x1.clone(), y1.clone()));
    }

    let mut x = BigInt::one();
    let mut y = BigInt::zero();

    let mut base_x = x1.clone();
    let mut base_y = y1.clone();

    let mut exp = k;
    let big_d = BigInt::from(d_constant);

    while exp > 0 {
        if exp % 2 == 1 {
            let new_x = &x * &base_x + &big_d * &y * &base_y;
            let new_y = &x * &base_y + &y * &base_x;
            x = new_x;
            y = new_y;
        }
        let new_x = &base_x * &base_x + &big_d * &base_y * &base_y;
        let new_y = BigInt::from(2u32) * &base_x * &base_y;
        base_x = new_x;
        base_y = new_y;

        exp /= 2;
    }

    Ok((x, y))
}

/// Generate the k-th Pell solution (xₖ, yₖ) given the minimal solution (panicking version).
///
/// This is a convenience wrapper around `pell_solution_k` that panics on error.
/// Use `pell_solution_k` for better error handling.
///
/// # Panics
///
/// Panics if `k` is 0.
pub fn pell_solution_k_unchecked(d_constant: u64, x1: &BigInt, y1: &BigInt, k: u64) -> (BigInt, BigInt) {
    pell_solution_k(d_constant, x1, y1, k).unwrap()
}

/// Verify that a given (x, y) pair is a solution to the Pell equation x² - D·y² = 1
///
/// # Arguments
///
/// * `d` - The coefficient D in the Pell equation
/// * `x` - The x-coordinate to verify
/// * `y` - The y-coordinate to verify
///
/// # Returns
///
/// `true` if (x, y) is a valid solution, `false` otherwise
///
/// # Examples
///
/// ```
/// use num_bigint::BigInt;
/// # use pell991::verify_pell_solution;
///
/// assert!(verify_pell_solution(2, &BigInt::from(3), &BigInt::from(2)));
/// assert!(!verify_pell_solution(2, &BigInt::from(2), &BigInt::from(1)));
/// ```
pub fn verify_pell_solution(d: u64, x: &BigInt, y: &BigInt) -> bool {
    let lhs = x * x;
    let rhs = BigInt::from(d) * y * y + BigInt::one();
    lhs == rhs
}

/// Generate multiple Pell solutions efficiently using iterative approach
///
/// This is more efficient than calling `pell_solution_k` repeatedly as it
/// uses the recurrence relation directly without binary exponentiation.
///
/// # Arguments
///
/// * `d` - The coefficient D in the Pell equation
/// * `count` - Number of solutions to generate (starting from k=1)
///
/// # Returns
///
/// A `Result` containing a vector of solution tuples, or a `PellError` if the input is invalid.
///
/// # Examples
///
/// ```
/// # use pell991::pell_solutions;
///
/// let solutions = pell_solutions(2, 3).unwrap();
/// assert_eq!(solutions.len(), 3);
/// ```
pub fn pell_solutions(d: u64, count: usize) -> Result<Vec<(BigInt, BigInt)>, PellError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    
    let (x1, y1) = pell_min_solution(d)?;
    let mut solutions = Vec::with_capacity(count);
    
    // Add the first solution
    solutions.push((x1.clone(), y1.clone()));
    
    if count == 1 {
        return Ok(solutions);
    }
    
    // Use iterative approach for better performance
    // (x_k, y_k) = (x1 * x_{k-1} + d * y1 * y_{k-1}, x1 * y_{k-1} + y1 * x_{k-1})
    let mut x_prev = x1.clone();
    let mut y_prev = y1.clone();
    let big_d = BigInt::from(d);
    
    for _ in 2..=count {
        let x_next = &x1 * &x_prev + &big_d * &y1 * &y_prev;
        let y_next = &x1 * &y_prev + &y1 * &x_prev;
        
        solutions.push((x_next.clone(), y_next.clone()));
        x_prev = x_next;
        y_prev = y_next;
    }
    
    Ok(solutions)
}

/// Iterator for generating Pell equation solutions on-demand
///
/// This iterator generates solutions lazily, which is memory-efficient
/// for large sequences and allows for infinite iteration.
///
/// # Examples
///
/// ```
/// # use pell991::PellSolutionIterator;
/// let mut iter = PellSolutionIterator::new(2).unwrap();
/// let first_three: Vec<_> = iter.take(3).collect();
/// assert_eq!(first_three.len(), 3);
/// ```
pub struct PellSolutionIterator {
    d: u64,
    x1: BigInt,
    y1: BigInt,
    current_x: BigInt,
    current_y: BigInt,
    big_d: BigInt,
    k: u64,
}

impl PellSolutionIterator {
    /// Create a new iterator for Pell equation solutions
    ///
    /// # Arguments
    ///
    /// * `d` - The coefficient D in the Pell equation
    ///
    /// # Returns
    ///
    /// A `Result` containing the iterator, or a `PellError` if D is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pell991::PellSolutionIterator;
    /// let iter = PellSolutionIterator::new(2).unwrap();
    /// ```
    pub fn new(d: u64) -> Result<Self, PellError> {
        let (x1, y1) = pell_min_solution(d)?;
        let big_d = BigInt::from(d);
        
        Ok(PellSolutionIterator {
            d,
            current_x: x1.clone(),
            current_y: y1.clone(),
            x1,
            y1,
            big_d,
            k: 1,
        })
    }
    
    /// Get the current k value (1-indexed)
    pub fn current_k(&self) -> u64 {
        self.k
    }
    
    /// Get the D value for this iterator
    pub fn d_value(&self) -> u64 {
        self.d
    }
    
    /// Reset the iterator to the beginning
    pub fn reset(&mut self) {
        self.current_x = self.x1.clone();
        self.current_y = self.y1.clone();
        self.k = 1;
    }
}

impl Iterator for PellSolutionIterator {
    type Item = (BigInt, BigInt);
    
    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.current_x.clone(), self.current_y.clone());
        
        // Compute next solution using recurrence relation
        // (x_{k+1}, y_{k+1}) = (x1 * x_k + d * y1 * y_k, x1 * y_k + y1 * x_k)
        let next_x = &self.x1 * &self.current_x + &self.big_d * &self.y1 * &self.current_y;
        let next_y = &self.x1 * &self.current_y + &self.y1 * &self.current_x;
        
        self.current_x = next_x;
        self.current_y = next_y;
        self.k += 1;
        
        Some(result)
    }
}