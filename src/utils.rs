//! Utility functions for mathematical operations

/// Compute the integer square root of a u64: ⌊√n⌋
///
/// Returns the largest integer x such that x² ≤ n.
/// Uses Newton's method with proper bounds checking.
///
/// # Arguments
///
/// * `n` - The number to compute the square root of
///
/// # Returns
///
/// The floor of the square root of n
///
/// # Examples
///
/// ```
/// # use pell991::isqrt_u64;
/// assert_eq!(isqrt_u64(15), 3); // √15 ≈ 3.87, so ⌊√15⌋ = 3
/// assert_eq!(isqrt_u64(16), 4); // √16 = 4
/// ```
pub fn isqrt_u64(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n <= 3 {
        return 1;
    }
    if n <= 8 {
        return 2;
    }
    if n <= 15 {
        return 3;
    }
    
    // Use Newton's method starting with a good initial guess
    let mut x = (n as f64).sqrt() as u64;
    
    // Ensure initial guess is reasonable
    if x == 0 {
        x = 1;
    }
    
    // Newton's method: x_{n+1} = (x_n + n/x_n) / 2
    for _ in 0..64 { // At most 64 iterations should be enough for u64
        let x_new = (x + n / x) / 2;
        if x_new >= x {
            break;
        }
        x = x_new;
    }
    
    // Ensure we have the correct floor value
    while x * x > n {
        x -= 1;
    }
    
    // Check if we can go one higher
    if let Some(next) = x.checked_add(1) {
        if let Some(next_sq) = next.checked_mul(next) {
            if next_sq <= n {
                x = next;
            }
        }
    }
    
    x
}

/// Check if a number is a perfect square.
///
/// # Arguments
///
/// * `n` - The number to test
///
/// # Returns
///
/// `true` if n is a perfect square, `false` otherwise
///
/// # Examples
///
/// ```
/// # use pell991::is_square_u64;
/// assert!(is_square_u64(16));  // 16 = 4²
/// assert!(!is_square_u64(15)); // 15 is not a perfect square
/// ```
pub fn is_square_u64(n: u64) -> bool {
    let r = isqrt_u64(n);
    r * r == n
}

/// Check if a given D value is valid for Pell equation solving
///
/// A valid D must be > 1 and not a perfect square.
///
/// # Arguments
///
/// * `d` - The D value to validate
///
/// # Returns
///
/// `true` if D is valid for Pell equation solving, `false` otherwise
///
/// # Examples
///
/// ```
/// # use pell991::is_valid_pell_d;
/// assert!(is_valid_pell_d(2));   // Valid: 2 > 1 and not a perfect square
/// assert!(is_valid_pell_d(991)); // Valid: 991 > 1 and not a perfect square
/// assert!(!is_valid_pell_d(1));  // Invalid: 1 is not > 1
/// assert!(!is_valid_pell_d(4));  // Invalid: 4 is a perfect square
/// ```
pub fn is_valid_pell_d(d: u64) -> bool {
    d > 1 && !is_square_u64(d)
}