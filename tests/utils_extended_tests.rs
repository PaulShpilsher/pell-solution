//! Extended tests for the new utility functions

use pell991::{estimate_period_length, fundamental_discriminant, is_prime, is_valid_pell_d};

#[test]
fn test_estimate_period_length() {
    // Valid D values should return Some
    assert!(estimate_period_length(2).is_some());
    assert!(estimate_period_length(3).is_some());
    assert!(estimate_period_length(991).is_some());
    
    // Invalid D values should return None
    assert!(estimate_period_length(0).is_none());
    assert!(estimate_period_length(1).is_none());
    assert!(estimate_period_length(4).is_none()); // Perfect square
    assert!(estimate_period_length(9).is_none()); // Perfect square
    
    // Test that estimates are reasonable (non-zero)
    let estimate = estimate_period_length(2).unwrap();
    assert!(estimate > 0);
    
    let estimate = estimate_period_length(991).unwrap();
    assert!(estimate > 0);
}

#[test]
fn test_fundamental_discriminant() {
    assert_eq!(fundamental_discriminant(2), 8);
    assert_eq!(fundamental_discriminant(3), 12);
    assert_eq!(fundamental_discriminant(5), 20);
    assert_eq!(fundamental_discriminant(991), 3964);
    
    // Test the formula: fundamental_discriminant(d) = 4 * d
    for d in 1..100 {
        assert_eq!(fundamental_discriminant(d), 4 * d);
    }
}

#[test]
fn test_is_prime() {
    // Test small primes
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(11));
    assert!(is_prime(13));
    assert!(is_prime(17));
    assert!(is_prime(19));
    
    // Test small composites
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
    assert!(!is_prime(10));
    assert!(!is_prime(12));
    assert!(!is_prime(14));
    assert!(!is_prime(15));
    assert!(!is_prime(16));
    assert!(!is_prime(18));
    assert!(!is_prime(20));
    
    // Test edge cases
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    
    // Test larger primes
    assert!(is_prime(991));
    assert!(is_prime(997));
    
    // Test larger composites
    assert!(!is_prime(1000));
    assert!(!is_prime(1001)); // 7 * 11 * 13
}

#[test]
fn test_is_valid_pell_d() {
    // Valid D values
    assert!(is_valid_pell_d(2));
    assert!(is_valid_pell_d(3));
    assert!(is_valid_pell_d(5));
    assert!(is_valid_pell_d(6));
    assert!(is_valid_pell_d(7));
    assert!(is_valid_pell_d(8));
    assert!(is_valid_pell_d(991));
    
    // Invalid D values (too small)
    assert!(!is_valid_pell_d(0));
    assert!(!is_valid_pell_d(1));
    
    // Invalid D values (perfect squares)
    assert!(!is_valid_pell_d(4));
    assert!(!is_valid_pell_d(9));
    assert!(!is_valid_pell_d(16));
    assert!(!is_valid_pell_d(25));
    assert!(!is_valid_pell_d(100));
    assert!(!is_valid_pell_d(10000));
}

#[test]
fn test_prime_vs_composite_d_values() {
    let test_range = 2..50;
    let mut prime_d_count = 0;
    let mut composite_d_count = 0;
    
    for d in test_range {
        if is_valid_pell_d(d) {
            if is_prime(d) {
                prime_d_count += 1;
            } else {
                composite_d_count += 1;
            }
        }
    }
    
    // Should have both prime and composite valid D values
    assert!(prime_d_count > 0);
    assert!(composite_d_count > 0);
    
    // Some specific checks
    assert!(is_valid_pell_d(2) && is_prime(2));
    assert!(is_valid_pell_d(3) && is_prime(3));
    assert!(is_valid_pell_d(6) && !is_prime(6));
    assert!(is_valid_pell_d(8) && !is_prime(8));
}

#[test]
fn test_utility_function_consistency() {
    // Test that our utility functions are consistent with each other
    for d in 2..100 {
        if is_valid_pell_d(d) {
            // If D is valid, it should not be a perfect square and should be > 1
            assert!(d > 1);
            
            // Fundamental discriminant should always be 4*d
            assert_eq!(fundamental_discriminant(d), 4 * d);
            
            // Period estimate should exist for valid D
            assert!(estimate_period_length(d).is_some());
        } else {
            // If D is invalid, period estimate should be None
            assert!(estimate_period_length(d).is_none());
        }
    }
}

#[test]
fn test_mathematical_properties() {
    // Test some mathematical properties of our utility functions
    
    // All even numbers > 2 should not be prime
    for n in (4..100).step_by(2) {
        assert!(!is_prime(n), "{} should not be prime", n);
    }
    
    // Perfect squares should not be valid Pell D values
    for i in 2..10 {
        let square = i * i;
        assert!(!is_valid_pell_d(square), "{}² = {} should not be valid for Pell", i, square);
    }
    
    // Numbers of the form n² + 1 should be valid (except when they're perfect squares themselves)
    for i in 1..10 {
        let d = i * i + 1;
        if d > 1 {
            // These should generally be valid unless they happen to be perfect squares
            let should_be_valid = !pell991::is_square_u64(d);
            assert_eq!(is_valid_pell_d(d), should_be_valid, 
                      "{}² + 1 = {} validity check failed", i, d);
        }
    }
}