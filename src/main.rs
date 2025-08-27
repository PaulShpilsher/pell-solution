//! Binary executable for the Pell equation solver

use pell991::{pell_min_solution, pell_solution_k, PellError};

fn main() -> Result<(), PellError> {
    let d = 991_u64;
    let (x1, y1) = pell_min_solution(d)?;

    println!("Minimal solution for x^2 - {d}y^2 = 1:");
    println!("x1 = {x1}");
    println!("y1 = {y1}");

    // Generate a few more solutions
    for k in 1..=5 {
        let (xk, yk) = pell_solution_k(d, &x1, &y1, k)?;
        println!("\nSolution {k}:");
        println!("x = {xk}");
        println!("y = {yk}");
    }
    
    Ok(())
}