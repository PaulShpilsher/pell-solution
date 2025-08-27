//! Tests for the PellSolutionIterator

use pell991::{PellSolutionIterator, pell_solutions, verify_pell_solution};

#[test]
fn test_iterator_basic_functionality() {
    let mut iter = PellSolutionIterator::new(2).unwrap();
    
    // Test first few solutions
    let first = iter.next().unwrap();
    assert_eq!(first.0.to_string(), "3");
    assert_eq!(first.1.to_string(), "2");
    
    let second = iter.next().unwrap();
    assert_eq!(second.0.to_string(), "17");
    assert_eq!(second.1.to_string(), "12");
    
    let third = iter.next().unwrap();
    assert_eq!(third.0.to_string(), "99");
    assert_eq!(third.1.to_string(), "70");
}

#[test]
fn test_iterator_vs_batch_consistency() {
    let d = 3;
    let count = 10;
    
    // Generate solutions using iterator
    let iter_solutions: Vec<_> = PellSolutionIterator::new(d).unwrap().take(count).collect();
    
    // Generate solutions using batch function
    let batch_solutions = pell_solutions(d, count).unwrap();
    
    // They should be identical
    assert_eq!(iter_solutions.len(), batch_solutions.len());
    for (i, ((ix, iy), (bx, by))) in iter_solutions.iter().zip(batch_solutions.iter()).enumerate() {
        assert_eq!(ix, bx, "X mismatch at solution {}", i + 1);
        assert_eq!(iy, by, "Y mismatch at solution {}", i + 1);
    }
}

#[test]
fn test_iterator_reset() {
    let mut iter = PellSolutionIterator::new(5).unwrap();
    
    // Take first solution
    let first_run_first = iter.next().unwrap();
    let _first_run_second = iter.next().unwrap();
    
    // Reset and take first solution again
    iter.reset();
    let second_run_first = iter.next().unwrap();
    
    // Should be the same
    assert_eq!(first_run_first, second_run_first);
    
    // Current k should be reset
    assert_eq!(iter.current_k(), 2); // After taking one solution post-reset
}

#[test]
fn test_iterator_current_k() {
    let mut iter = PellSolutionIterator::new(7).unwrap();
    
    assert_eq!(iter.current_k(), 1);
    
    iter.next();
    assert_eq!(iter.current_k(), 2);
    
    iter.next();
    assert_eq!(iter.current_k(), 3);
    
    iter.reset();
    assert_eq!(iter.current_k(), 1);
}

#[test]
fn test_iterator_solution_verification() {
    let d = 13;
    let mut iter = PellSolutionIterator::new(d).unwrap();
    
    // Verify first 5 solutions
    for _ in 0..5 {
        let (x, y) = iter.next().unwrap();
        assert!(verify_pell_solution(d, &x, &y), "Solution verification failed");
    }
}

#[test]
fn test_iterator_error_handling() {
    // Test with invalid D values
    assert!(PellSolutionIterator::new(0).is_err());
    assert!(PellSolutionIterator::new(1).is_err());
    assert!(PellSolutionIterator::new(4).is_err()); // Perfect square
    assert!(PellSolutionIterator::new(9).is_err()); // Perfect square
    
    // Test with valid D values
    assert!(PellSolutionIterator::new(2).is_ok());
    assert!(PellSolutionIterator::new(3).is_ok());
    assert!(PellSolutionIterator::new(991).is_ok());
}

#[test]
fn test_iterator_large_sequences() {
    let mut iter = PellSolutionIterator::new(2).unwrap();
    
    // Test that we can generate a reasonably large sequence
    let solutions: Vec<_> = iter.take(50).collect();
    assert_eq!(solutions.len(), 50);
    
    // Verify all solutions
    for (x, y) in &solutions {
        assert!(verify_pell_solution(2, x, y));
    }
    
    // Verify solutions are increasing
    for i in 1..solutions.len() {
        assert!(solutions[i].0 > solutions[i-1].0, "Solutions should be increasing");
    }
}

#[test]
fn test_iterator_with_collect() {
    let solutions: Vec<_> = PellSolutionIterator::new(5).unwrap().take(3).collect();
    
    assert_eq!(solutions.len(), 3);
    
    // Verify specific values for D=5
    assert_eq!(solutions[0].0.to_string(), "9");
    assert_eq!(solutions[0].1.to_string(), "4");
    
    // Verify all solutions
    for (x, y) in &solutions {
        assert!(verify_pell_solution(5, x, y));
    }
}

#[test]
fn test_iterator_chaining() {
    let mut iter = PellSolutionIterator::new(7).unwrap();
    
    // Test iterator chaining operations
    let first_batch: Vec<_> = iter.by_ref().take(3).collect();
    let second_batch: Vec<_> = iter.take(2).collect();
    
    assert_eq!(first_batch.len(), 3);
    assert_eq!(second_batch.len(), 2);
    
    // Verify all solutions
    for (x, y) in first_batch.iter().chain(second_batch.iter()) {
        assert!(verify_pell_solution(7, x, y));
    }
}