//! Integration tests for the Pell equation solver

use pell991::{pell_min_solution, pell_solution_k, pell_solutions, verify_pell_solution, PellError};
use num_bigint::BigInt;

#[test]
fn test_error_display() {
    assert_eq!(format!("{}", PellError::InvalidD(0)), "D must be > 1, got 0");
    assert_eq!(format!("{}", PellError::PerfectSquare(4)), "D must be non-square, got 4 which is 2²");
    assert_eq!(format!("{}", PellError::InvalidK(0)), "k must be > 0, got 0");
}

#[test]
fn test_large_d_values() {
    // Test with larger D values to ensure the algorithm works correctly
    let large_d_values = [61, 109, 181, 277, 397, 541];
    
    for &d in &large_d_values {
        let result = pell_min_solution(d);
        assert!(result.is_ok(), "Failed to solve for D = {}", d);
        
        let (x, y) = result.unwrap();
        assert!(verify_pell_solution(d, &x, &y), "Solution verification failed for D = {}", d);
        
        // Ensure x and y are positive
        assert!(x > BigInt::from(0), "x should be positive for D = {}", d);
        assert!(y > BigInt::from(0), "y should be positive for D = {}", d);
    }
}

#[test]
fn test_solution_sequence_consistency() {
    let d = 13;
    let (x1, y1) = pell_min_solution(d).unwrap();
    
    // Generate first 5 solutions and verify they're all valid
    for k in 1..=5 {
        let (xk, yk) = pell_solution_k(d, &x1, &y1, k).unwrap();
        assert!(verify_pell_solution(d, &xk, &yk), "Solution {} failed verification for D = {}", k, d);
        
        // Solutions should be increasing
        if k > 1 {
            let (x_prev, _) = pell_solution_k(d, &x1, &y1, k - 1).unwrap();
            assert!(xk > x_prev, "Solutions should be increasing");
        }
    }
}

#[test]
fn test_batch_solution_generation() {
    let d = 7;
    let count = 4;
    
    let solutions = pell_solutions(d, count).unwrap();
    assert_eq!(solutions.len(), count);
    
    // Verify all solutions
    for (i, (x, y)) in solutions.iter().enumerate() {
        assert!(verify_pell_solution(d, x, y), "Batch solution {} failed verification", i + 1);
    }
    
    // Compare with individual generation
    let (x1, y1) = pell_min_solution(d).unwrap();
    for (i, (x, y)) in solutions.iter().enumerate() {
        let (x_individual, y_individual) = pell_solution_k(d, &x1, &y1, (i + 1) as u64).unwrap();
        assert_eq!(*x, x_individual, "Batch and individual solutions differ for k = {}", i + 1);
        assert_eq!(*y, y_individual, "Batch and individual solutions differ for k = {}", i + 1);
    }
}

#[test]
fn test_known_solutions() {
    // Test some well-known Pell equation solutions
    
    // D = 2: (3, 2), (17, 12), (99, 70), ...
    let (x, y) = pell_min_solution(2).unwrap();
    assert_eq!(x, BigInt::from(3));
    assert_eq!(y, BigInt::from(2));
    
    let (x2, y2) = pell_solution_k(2, &x, &y, 2).unwrap();
    assert_eq!(x2, BigInt::from(17));
    assert_eq!(y2, BigInt::from(12));
    
    let (x3, y3) = pell_solution_k(2, &x, &y, 3).unwrap();
    assert_eq!(x3, BigInt::from(99));
    assert_eq!(y3, BigInt::from(70));
    
    // D = 3: (2, 1), (7, 4), (26, 15), ...
    let (x, y) = pell_min_solution(3).unwrap();
    assert_eq!(x, BigInt::from(2));
    assert_eq!(y, BigInt::from(1));
    
    let (x2, y2) = pell_solution_k(3, &x, &y, 2).unwrap();
    assert_eq!(x2, BigInt::from(7));
    assert_eq!(y2, BigInt::from(4));
    
    let (x3, y3) = pell_solution_k(3, &x, &y, 3).unwrap();
    assert_eq!(x3, BigInt::from(26));
    assert_eq!(y3, BigInt::from(15));
}

#[test]
fn test_edge_cases() {
    // Test the smallest valid D
    let (x, y) = pell_min_solution(2).unwrap();
    assert!(verify_pell_solution(2, &x, &y));
    
    // Test D values just above perfect squares
    let test_cases = [5, 8, 10, 12, 13, 15, 17, 18, 19, 20];
    for &d in &test_cases {
        let result = pell_min_solution(d);
        assert!(result.is_ok(), "Failed for D = {}", d);
        let (x, y) = result.unwrap();
        assert!(verify_pell_solution(d, &x, &y), "Verification failed for D = {}", d);
    }
}

#[test]
fn test_performance_with_large_k() {
    let d = 5;
    let (x1, y1) = pell_min_solution(d).unwrap();
    
    // Test that we can compute larger k values efficiently
    let large_k_values = [10, 20, 50];
    
    for &k in &large_k_values {
        let result = pell_solution_k(d, &x1, &y1, k);
        assert!(result.is_ok(), "Failed to compute solution for k = {}", k);
        
        let (x, y) = result.unwrap();
        assert!(verify_pell_solution(d, &x, &y), "Large k solution verification failed for k = {}", k);
    }
}