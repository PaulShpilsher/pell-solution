//! Binary executable for the 991 Pell Puzzle solver

use pell991::{pell_min_solution, pell_solution_k, verify_pell_solution, PellError};

fn main() -> Result<(), PellError> {
    println!("The 991 Pell Puzzle");
    println!("{}", "=".repeat(25));
    println!();
    
    println!("Solving the mystery: when does √(991·n² + 1) become a perfect integer?");
    println!();
    
    let d = 991_u64;
    let (m, n) = pell_min_solution(d)?;
    
    println!("The magical solution found!");
    println!("   n = {}", n);
    println!("   m = {}", m);
    println!();
    
    // Verify the solution
    assert!(verify_pell_solution(d, &m, &n));
    println!("Verified: m² - 991n² = 1");
    println!();
    
    println!("This means √(991·{}² + 1) = {}", n, m);
    println!("   After billions of tries, the irrational veil finally lifts!");
    println!();
    
    println!("The infinite staircase of solutions:");
    
    // Generate a few more solutions
    for k in 1..=5 {
        let (xk, yk) = pell_solution_k(d, &m, &n, k)?;
        println!("\nSolution {}:", k);
        println!("   x = {}", xk);
        println!("   y = {}", yk);
        if k == 1 {
            println!("   This is our magical pair!");
        }
    }
    
    println!();
    println!("The mathematical beauty of Pell equations revealed!");
    
    Ok(())
}