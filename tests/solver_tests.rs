//! Unit tests for the solver module

use pell991::{
    pell_min_solution, pell_solution_k, pell_solutions, verify_pell_solution, PellError
};
use num_bigint::BigInt;
use num_traits::One;

#[test]
fn test_pell_min_solution_small_cases() {
    // Test D = 2: minimal solution is (3, 2)
    let (x, y) = pell_min_solution(2).unwrap();
    assert_eq!(x, BigInt::from(3));
    assert_eq!(y, BigInt::from(2));
    
    // Verify: 3^2 - 2*2^2 = 9 - 8 = 1 ✓
    let verification = &x * &x - BigInt::from(2) * &y * &y;
    assert_eq!(verification, BigInt::one());

    // Test D = 3: minimal solution is (2, 1)
    let (x, y) = pell_min_solution(3).unwrap();
    assert_eq!(x, BigInt::from(2));
    assert_eq!(y, BigInt::from(1));
    
    // Verify: 2^2 - 3*1^2 = 4 - 3 = 1 ✓
    let verification = &x * &x - BigInt::from(3) * &y * &y;
    assert_eq!(verification, BigInt::one());
}

#[test]
fn test_pell_solution_k() {
    let d = 2u64;
    let (x1, y1) = pell_min_solution(d).unwrap();
    
    // Test k=1 (should return the minimal solution)
    let (x, y) = pell_solution_k(d, &x1, &y1, 1).unwrap();
    assert_eq!(x, x1);
    assert_eq!(y, y1);
    
    // Test k=2: For D=2, second solution should be (17, 12)
    let (x2, y2) = pell_solution_k(d, &x1, &y1, 2).unwrap();
    assert_eq!(x2, BigInt::from(17));
    assert_eq!(y2, BigInt::from(12));
    
    // Verify: 17^2 - 2*12^2 = 289 - 288 = 1 ✓
    let verification = &x2 * &x2 - BigInt::from(d) * &y2 * &y2;
    assert_eq!(verification, BigInt::one());
}

#[test]
fn test_pell_solution_verification() {
    let test_cases = [2, 3, 5, 6, 7, 8, 10, 11, 12, 13];
    
    for &d in &test_cases {
        let (x, y) = pell_min_solution(d).unwrap();
        let verification = &x * &x - BigInt::from(d) * &y * &y;
        assert_eq!(verification, BigInt::one(), 
            "Failed for D={}: {}^2 - {}*{}^2 != 1", d, x, d, y);
    }
}

#[test]
fn test_pell_error_handling() {
    // Test invalid D values
    assert_eq!(pell_min_solution(0), Err(PellError::InvalidD(0)));
    assert_eq!(pell_min_solution(1), Err(PellError::InvalidD(1)));
    
    // Test perfect square D values
    assert_eq!(pell_min_solution(4), Err(PellError::PerfectSquare(4)));
    assert_eq!(pell_min_solution(9), Err(PellError::PerfectSquare(9)));
    assert_eq!(pell_min_solution(16), Err(PellError::PerfectSquare(16)));
    
    // Test invalid k values
    let (x1, y1) = pell_min_solution(2).unwrap();
    assert_eq!(pell_solution_k(2, &x1, &y1, 0), Err(PellError::InvalidK(0)));
}

#[test]
fn test_verify_pell_solution() {
    // Test valid solutions
    assert!(verify_pell_solution(2, &BigInt::from(3), &BigInt::from(2)));
    assert!(verify_pell_solution(3, &BigInt::from(2), &BigInt::from(1)));
    assert!(verify_pell_solution(2, &BigInt::from(17), &BigInt::from(12)));
    
    // Test invalid solutions
    assert!(!verify_pell_solution(2, &BigInt::from(2), &BigInt::from(1)));
    assert!(!verify_pell_solution(3, &BigInt::from(3), &BigInt::from(2)));
}

#[test]
fn test_pell_solutions() {
    let solutions = pell_solutions(2, 3).unwrap();
    assert_eq!(solutions.len(), 3);
    
    // Verify each solution
    for (i, (x, y)) in solutions.iter().enumerate() {
        assert!(verify_pell_solution(2, x, y), "Solution {} failed verification", i + 1);
    }
    
    // Test empty case
    let empty = pell_solutions(2, 0).unwrap();
    assert!(empty.is_empty());
    
    // Test error propagation
    assert!(pell_solutions(4, 1).is_err()); // Perfect square
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