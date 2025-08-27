//! Demonstration of the streaming solution iterator for memory-efficient processing

use pell991::{PellSolutionIterator, verify_pell_solution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Pell Equation Solver - Streaming Solutions");
    println!("{}", "=".repeat(45));
    println!();

    // Example 1: Basic iterator usage
    println!("📋 Example 1: Basic Iterator Usage (D=2)");
    println!("{}", "-".repeat(35));
    
    let iter = PellSolutionIterator::new(2)?;
    
    println!("First 5 solutions:");
    for (i, (x, y)) in iter.take(5).enumerate() {
        println!("  Solution {}: x={}, y={}", i + 1, x, y);
        assert!(verify_pell_solution(2, &x, &y));
    }
    println!("✓ All solutions verified");
    println!();

    // Example 2: Memory-efficient processing of large sequences
    println!("💾 Example 2: Memory-Efficient Large Sequence Processing");
    println!("{}", "-".repeat(50));
    
    let mut iter = PellSolutionIterator::new(3)?;
    let mut max_digits = 0;
    let mut solution_count = 0;
    
    // Process solutions until we find one with more than 20 digits
    for (x, y) in iter.by_ref() {
        solution_count += 1;
        let digits = x.to_string().len().max(y.to_string().len());
        max_digits = max_digits.max(digits);
        
        if digits > 20 {
            println!("Found solution with {} digits at k={}", digits, solution_count);
            println!("  x = {}", x);
            println!("  y = {}", y);
            break;
        }
        
        if solution_count <= 10 {
            println!("  k={}: {} digits", solution_count, digits);
        } else if solution_count % 5 == 0 {
            println!("  k={}: {} digits", solution_count, digits);
        }
    }
    
    println!("Processed {} solutions, max digits: {}", solution_count, max_digits);
    println!();

    // Example 3: Iterator reset and reuse
    println!("🔄 Example 3: Iterator Reset and Reuse");
    println!("{}", "-".repeat(35));
    
    let mut iter = PellSolutionIterator::new(5)?;
    
    // Take first 3 solutions
    let first_batch: Vec<_> = iter.by_ref().take(3).collect();
    println!("First batch (3 solutions):");
    for (i, (x, y)) in first_batch.iter().enumerate() {
        println!("  Solution {}: x={}, y={}", i + 1, x, y);
    }
    
    println!("Current k after first batch: {}", iter.current_k());
    
    // Reset and take first 2 solutions again
    iter.reset();
    let second_batch: Vec<_> = iter.take(2).collect();
    println!("After reset, first 2 solutions:");
    for (i, (x, y)) in second_batch.iter().enumerate() {
        println!("  Solution {}: x={}, y={}", i + 1, x, y);
    }
    
    // Verify reset worked correctly
    assert_eq!(first_batch[0], second_batch[0]);
    assert_eq!(first_batch[1], second_batch[1]);
    println!("✓ Reset functionality verified");
    println!();

    // Example 4: Comparing iterator vs batch generation
    println!("⚡ Example 4: Iterator vs Batch Generation Comparison");
    println!("{}", "-".repeat(45));
    
    use pell991::pell_solutions;
    use std::time::Instant;
    
    let d = 7;
    let count = 15;
    
    // Batch generation
    let start = Instant::now();
    let batch_solutions = pell_solutions(d, count)?;
    let batch_time = start.elapsed();
    
    // Iterator generation
    let start = Instant::now();
    let iter_solutions: Vec<_> = PellSolutionIterator::new(d)?.take(count).collect();
    let iter_time = start.elapsed();
    
    println!("Batch generation:    {:8.2}μs", batch_time.as_micros());
    println!("Iterator generation: {:8.2}μs", iter_time.as_micros());
    
    // Verify they produce the same results
    assert_eq!(batch_solutions.len(), iter_solutions.len());
    for (i, ((bx, by), (ix, iy))) in batch_solutions.iter().zip(iter_solutions.iter()).enumerate() {
        assert_eq!(bx, ix, "X mismatch at solution {}", i + 1);
        assert_eq!(by, iy, "Y mismatch at solution {}", i + 1);
    }
    println!("✓ Results verified identical");
    
    // Memory usage comparison
    let batch_memory = std::mem::size_of_val(&batch_solutions);
    let iter_memory = std::mem::size_of::<PellSolutionIterator>();
    
    println!("Batch memory usage:    ~{} bytes (for {} solutions)", batch_memory, count);
    println!("Iterator memory usage: ~{} bytes (constant)", iter_memory);
    println!();

    // Example 5: Infinite sequence processing with conditions
    println!("♾️  Example 5: Conditional Processing of Infinite Sequence");
    println!("{}", "-".repeat(50));
    
    let iter = PellSolutionIterator::new(13)?;
    let mut palindrome_count = 0;
    let mut total_checked = 0;
    
    // Look for solutions where x is a palindrome (just for fun!)
    for (x, _y) in iter.take(100) { // Limit to first 100 to avoid infinite loop
        total_checked += 1;
        let x_str = x.to_string();
        let is_palindrome = x_str == x_str.chars().rev().collect::<String>();
        
        if is_palindrome {
            palindrome_count += 1;
            println!("  Found palindromic x at k={}: {}", total_checked, x_str);
        }
        
        if palindrome_count >= 3 {
            break;
        }
    }
    
    println!("Found {} palindromic solutions in first {} solutions", palindrome_count, total_checked);
    println!();

    println!("🎯 Streaming API Benefits:");
    println!("• Memory-efficient: O(1) memory usage regardless of sequence length");
    println!("• Lazy evaluation: Solutions computed only when needed");
    println!("• Infinite sequences: Can process arbitrarily long sequences");
    println!("• Flexible: Easy to combine with other iterator methods");
    println!("• Reusable: Can reset and reuse the same iterator");

    Ok(())
}