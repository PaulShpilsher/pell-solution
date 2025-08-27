//! Unit tests for the utils module

use pell991::{isqrt_u64, is_square_u64};

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

#[test]
fn test_isqrt_edge_cases() {
    // Test perfect squares
    for i in 0..100 {
        let square = i * i;
        assert_eq!(isqrt_u64(square), i, "Failed for perfect square {}", square);
    }
    
    // Test numbers just below perfect squares
    for i in 1..100 {
        let square = i * i;
        if square > 0 {
            assert_eq!(isqrt_u64(square - 1), i - 1, "Failed for {} (just below {}²)", square - 1, i);
        }
    }
}

#[test]
fn test_is_square_consistency() {
    // Test that is_square_u64 is consistent with isqrt_u64
    for n in 0..1000 {
        let sqrt_n = isqrt_u64(n);
        let is_perfect = sqrt_n * sqrt_n == n;
        assert_eq!(is_square_u64(n), is_perfect, "Inconsistency for n = {}", n);
    }
}

#[test]
fn test_large_numbers() {
    // Test with some larger numbers
    let large_numbers = [
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
        10_000_000_000,
    ];
    
    for &n in &large_numbers {
        let sqrt_n = isqrt_u64(n);
        
        // Verify that sqrt_n² ≤ n < (sqrt_n + 1)²
        assert!(sqrt_n * sqrt_n <= n, "sqrt({})² = {} > {}", n, sqrt_n * sqrt_n, n);
        assert!((sqrt_n + 1) * (sqrt_n + 1) > n, "sqrt({}) + 1)² = {} ≤ {}", n, (sqrt_n + 1) * (sqrt_n + 1), n);
        
        // Test is_square_u64 consistency
        let is_perfect = sqrt_n * sqrt_n == n;
        assert_eq!(is_square_u64(n), is_perfect, "is_square_u64 inconsistency for {}", n);
    }
}

#[test]
fn test_performance_edge_cases() {
    // Test maximum value
    let max_val = u64::MAX;
    let sqrt_max = isqrt_u64(max_val);
    assert!(sqrt_max * sqrt_max <= max_val);
    assert!(!is_square_u64(max_val)); // u64::MAX is not a perfect square
    
    // Test large perfect squares
    let large_roots = [1000, 10000, 100000, 1000000];
    for &root in &large_roots {
        let square = (root as u64) * (root as u64);
        assert_eq!(isqrt_u64(square), root as u64);
        assert!(is_square_u64(square));
    }
}