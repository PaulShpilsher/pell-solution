//! Utility functions for mathematical operations

/// Compute the integer square root of a u64: ⌊√n⌋
///
/// Returns the largest integer x such that x² ≤ n.
/// Uses Newton's method for better performance on large numbers.
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
    
    // Use Newton's method starting with a good initial guess
    let mut x = (n as f64).sqrt() as u64;
    
    // Newton's method: x_{n+1} = (x_n + n/x_n) / 2
    loop {
        let x_new = (x + n / x) / 2;
        if x_new >= x {
            break;
        }
        x = x_new;
    }
    
    // Ensure we have the floor
    while x * x > n {
        x -= 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_square_u64() {
        assert!(is_square_u64(0));
        assert!(is_square_u64(1));
        assert!(is_square_u64(4));
        assert!(is_square_u64(9));
        assert!(is_square_u64(16));
        assert!(is_square_u64(25));
        assert!(is_square_u64(100));
        assert!(is_square_u64(10000));
        assert!(!is_square_u64(2));
        assert!(!is_square_u64(3));
        assert!(!is_square_u64(5));
        assert!(!is_square_u64(991));
        assert!(!is_square_u64(999));
    }

    #[test]
    fn test_isqrt_u64() {
        assert_eq!(isqrt_u64(0), 0);
        assert_eq!(isqrt_u64(1), 1);
        assert_eq!(isqrt_u64(2), 1);
        assert_eq!(isqrt_u64(3), 1);
        assert_eq!(isqrt_u64(4), 2);
        assert_eq!(isqrt_u64(8), 2);
        assert_eq!(isqrt_u64(9), 3);
        assert_eq!(isqrt_u64(15), 3);
        assert_eq!(isqrt_u64(16), 4);
        assert_eq!(isqrt_u64(991), 31);
        assert_eq!(isqrt_u64(u64::MAX), 4294967295); // ⌊√(2^64-1)⌋
    }
}