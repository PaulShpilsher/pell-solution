//! Solve the 991 Pell Puzzle - Find the magical values that make √(991·n² + 1) a perfect integer

use pell991::{pell_min_solution, verify_pell_solution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 The 991 Pell Puzzle Solver");
    println!("{}", "=".repeat(40));
    println!();
    
    println!("🔍 Searching for the magical values where √(991·n² + 1) becomes a perfect integer...");
    println!();
    
    // Find the minimal solution to the Pell equation m² - 991n² = 1
    let (m, n) = pell_min_solution(991)?;
    
    println!("🎯 Found the magical values!");
    println!("   n = {}", n);
    println!("   m = {}", m);
    println!();
    
    // Verify the solution
    assert!(verify_pell_solution(991, &m, &n));
    println!("✅ Verification: m² - 991n² = 1");
    
    // Show the magnitude
    println!();
    println!("📊 Magnitude of the discovery:");
    println!("   n has {} digits", n.to_string().len());
    println!("   m has {} digits", m.to_string().len());
    println!();
    
    // Explain what this means
    println!("✨ What this means:");
    println!("   After billions and billions of tries,");
    println!("   at n = {},", n);
    println!("   the expression √(991·n² + 1) finally becomes");
    println!("   the perfect integer m = {}", m);
    println!();
    
    // Show that this is indeed the case by computing 991·n² + 1
    let expression_value = &n * &n * 991 + 1;
    println!("🧮 Mathematical verification:");
    println!("   991·n² + 1 = {}", expression_value);
    
    // Verify that m² equals this value
    let m_squared = &m * &m;
    println!("   m² = {}", m_squared);
    println!("   Equal? {}", expression_value == m_squared);
    println!();
    
    println!("🌟 The irrational veil has lifted!");
    println!("   This demonstrates the beautiful connection between");
    println!("   Pell equations and seemingly impossible integer solutions.");
    
    Ok(())
}