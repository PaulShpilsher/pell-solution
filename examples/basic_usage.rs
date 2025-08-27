//! Basic usage example for the Pell equation solver library

use pell991::{pell_min_solution, pell_solution_k, pell_solutions, verify_pell_solution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Pell Equation Solver - Basic Usage ===\n");
    
    // Example 1: Find minimal solution for D = 2
    println!("Example 1: Minimal solution for x² - 2y² = 1");
    let d = 2;
    let (x1, y1) = pell_min_solution(d)?;
    println!("Minimal solution: x = {}, y = {}", x1, y1);
    
    // Verify the solution
    assert!(verify_pell_solution(d, &x1, &y1));
    println!("✓ Solution verified: {}² - 2·{}² = {}", 
             x1, y1, &x1 * &x1 - 2 * &y1 * &y1);
    
    // Example 2: Generate the second solution
    println!("\nExample 2: Second solution using k-th solution function");
    let (x2, y2) = pell_solution_k(d, &x1, &y1, 2)?;
    println!("Second solution: x = {}, y = {}", x2, y2);
    assert!(verify_pell_solution(d, &x2, &y2));
    println!("✓ Solution verified");
    
    // Example 3: Generate multiple solutions at once
    println!("\nExample 3: Generate first 4 solutions for D = 3");
    let d = 3;
    let solutions = pell_solutions(d, 4)?;
    
    for (i, (x, y)) in solutions.iter().enumerate() {
        println!("Solution {}: x = {}, y = {}", i + 1, x, y);
        assert!(verify_pell_solution(d, x, y));
    }
    println!("✓ All solutions verified");
    
    // Example 4: Demonstrate solution growth
    println!("\nExample 4: Solution growth for D = 5");
    let d = 5;
    let (x1, y1) = pell_min_solution(d)?;
    
    for k in 1..=3 {
        let (xk, yk) = pell_solution_k(d, &x1, &y1, k)?;
        println!("Solution {}: x = {} ({} digits), y = {} ({} digits)", 
                 k, xk, xk.to_string().len(), yk, yk.to_string().len());
    }
    
    println!("\n=== All examples completed successfully! ===");
    Ok(())
}