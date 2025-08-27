//! Integration tests for the Pell equation solver

use pell991::{pell_min_solution, pell_solution_k, pell_solutions, verify_pell_solution};
use num_bigint::BigInt;

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

#[test]
fn test_solution_growth_patterns() {
    // Test that solutions grow exponentially as expected
    let d = 2;
    let (x1, y1) = pell_min_solution(d).unwrap();
    
    let mut prev_x = BigInt::from(0);
    for k in 1..=5 {
        let (xk, _) = pell_solution_k(d, &x1, &y1, k).unwrap();
        
        // Each solution should be significantly larger than the previous
        if k > 1 {
            assert!(xk > prev_x * 2, "Solution growth too slow for k = {}", k);
        }
        prev_x = xk;
    }
}

#[test]
fn test_cross_module_integration() {
    // Test that all modules work together correctly
    use pell991::{isqrt_u64, is_square_u64};
    
    // Test that utility functions are used correctly in solver
    for d in 2..50 {
        if !is_square_u64(d) {
            let sqrt_d = isqrt_u64(d);
            assert!(sqrt_d * sqrt_d < d, "isqrt should be floor for non-squares");
            
            let result = pell_min_solution(d);
            assert!(result.is_ok(), "Should solve for non-square D = {}", d);
        } else {
            let result = pell_min_solution(d);
            assert!(result.is_err(), "Should fail for perfect square D = {}", d);
        }
    }
}

#[test]
fn test_very_large_d() {
    // Test with a very large D to ensure robustness
    let large_d = 99991; // Large prime
    
    let result = pell_min_solution(large_d);
    assert!(result.is_ok(), "Failed to solve for very large D = {}", large_d);
    
    let (x, y) = result.unwrap();
    assert!(verify_pell_solution(large_d, &x, &y), "Verification failed for very large D");
    
    // The solution should be quite large
    assert!(x.to_string().len() > 10, "Solution should have many digits for large D");
}

#[test]
fn test_comprehensive_workflow() {
    // Test a complete workflow: solve, verify, generate more solutions
    let d = 13;
    
    // Step 1: Find minimal solution
    let (x1, y1) = pell_min_solution(d).unwrap();
    assert!(verify_pell_solution(d, &x1, &y1));
    
    // Step 2: Generate individual solutions
    let mut individual_solutions = Vec::new();
    for k in 1..=5 {
        let (xk, yk) = pell_solution_k(d, &x1, &y1, k).unwrap();
        assert!(verify_pell_solution(d, &xk, &yk));
        individual_solutions.push((xk, yk));
    }
    
    // Step 3: Generate batch solutions
    let batch_solutions = pell_solutions(d, 5).unwrap();
    
    // Step 4: Verify they match
    assert_eq!(individual_solutions.len(), batch_solutions.len());
    for (i, ((x_ind, y_ind), (x_batch, y_batch))) in 
        individual_solutions.iter().zip(batch_solutions.iter()).enumerate() {
        assert_eq!(x_ind, x_batch, "X mismatch at solution {}", i + 1);
        assert_eq!(y_ind, y_batch, "Y mismatch at solution {}", i + 1);
    }
}

#[test]
fn test_mathematical_properties() {
    // Test mathematical properties of Pell equation solutions
    let d = 7;
    let (x1, y1) = pell_min_solution(d).unwrap();
    
    // Property: If (x, y) is a solution, then x² - D*y² = 1
    for k in 1..=3 {
        let (xk, yk) = pell_solution_k(d, &x1, &y1, k).unwrap();
        let lhs = &xk * &xk;
        let rhs = BigInt::from(d) * &yk * &yk + BigInt::from(1);
        assert_eq!(lhs, rhs, "Pell equation not satisfied for k = {}", k);
    }
    
    // Property: Solutions form a multiplicative group
    // (x₁ + y₁√D)² = x₂ + y₂√D
    let (x2, y2) = pell_solution_k(d, &x1, &y1, 2).unwrap();
    
    // Calculate (x₁ + y₁√D)² manually
    let x_calc = &x1 * &x1 + BigInt::from(d) * &y1 * &y1;
    let y_calc = BigInt::from(2) * &x1 * &y1;
    
    assert_eq!(x2, x_calc, "Second solution x doesn't match group property");
    assert_eq!(y2, y_calc, "Second solution y doesn't match group property");
}