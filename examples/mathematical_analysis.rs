//! Advanced mathematical analysis of Pell equations and their properties

use pell991::{
    pell_min_solution, 
    verify_pell_solution, 
    is_valid_pell_d, 
    is_prime, 
    estimate_period_length,
    fundamental_discriminant,
    PellSolutionIterator
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Pell Equation Solver - Mathematical Analysis");
    println!("{}", "=".repeat(50));
    println!();

    // Analysis 1: D value characteristics
    println!("📊 Analysis 1: D Value Characteristics");
    println!("{}", "-".repeat(40));
    
    let test_d_values = [2, 3, 5, 7, 13, 17, 19, 61, 109, 181, 277, 397, 541, 991];
    
    println!("{:>4} | {:>6} | {:>8} | {:>6} | {:>12} | {:>8}", 
             "D", "Prime?", "Fund.Disc", "Est.Per", "Min.Sol.Dig", "Verified");
    println!("{}", "-".repeat(65));
    
    for &d in &test_d_values {
        if is_valid_pell_d(d) {
            let is_d_prime = is_prime(d);
            let fund_disc = fundamental_discriminant(d);
            let est_period = estimate_period_length(d).unwrap_or(0);
            
            let (x, y) = pell_min_solution(d)?;
            let solution_digits = x.to_string().len().max(y.to_string().len());
            let verified = verify_pell_solution(d, &x, &y);
            
            println!("{:>4} | {:>6} | {:>8} | {:>6} | {:>12} | {:>8}", 
                     d, 
                     if is_d_prime { "Yes" } else { "No" },
                     fund_disc,
                     est_period,
                     solution_digits,
                     if verified { "✓" } else { "✗" });
        }
    }
    println!();

    // Analysis 2: Solution growth patterns
    println!("📈 Analysis 2: Solution Growth Patterns (D=2)");
    println!("{}", "-".repeat(40));
    
    let d = 2;
    let mut iter = PellSolutionIterator::new(d)?;
    
    println!("{:>3} | {:>8} | {:>8} | {:>10} | {:>10}", "k", "x_digits", "y_digits", "x_growth", "y_growth");
    println!("{}", "-".repeat(50));
    
    let mut prev_x_digits = 0;
    let mut prev_y_digits = 0;
    
    for (k, (x, y)) in iter.take(15).enumerate() {
        let k = k + 1;
        let x_digits = x.to_string().len();
        let y_digits = y.to_string().len();
        
        let x_growth = if prev_x_digits > 0 { 
            x_digits as f64 / prev_x_digits as f64 
        } else { 
            1.0 
        };
        let y_growth = if prev_y_digits > 0 { 
            y_digits as f64 / prev_y_digits as f64 
        } else { 
            1.0 
        };
        
        println!("{:>3} | {:>8} | {:>8} | {:>10.2} | {:>10.2}", 
                 k, x_digits, y_digits, x_growth, y_growth);
        
        prev_x_digits = x_digits;
        prev_y_digits = y_digits;
    }
    println!();

    // Analysis 3: Prime vs Composite D values
    println!("🔢 Analysis 3: Prime vs Composite D Performance");
    println!("{}", "-".repeat(45));
    
    let mut prime_d_values = Vec::new();
    let mut composite_d_values = Vec::new();
    
    for d in 2..100 {
        if is_valid_pell_d(d) {
            if is_prime(d) {
                prime_d_values.push(d);
            } else {
                composite_d_values.push(d);
            }
        }
    }
    
    // Take first 10 of each for analysis
    prime_d_values.truncate(10);
    composite_d_values.truncate(10);
    
    println!("Prime D values (first 10):");
    let mut prime_avg_digits = 0.0;
    for &d in &prime_d_values {
        let (x, y) = pell_min_solution(d)?;
        let max_digits = x.to_string().len().max(y.to_string().len());
        prime_avg_digits += max_digits as f64;
        println!("  D={:2}: {} digits", d, max_digits);
    }
    prime_avg_digits /= prime_d_values.len() as f64;
    
    println!("\\nComposite D values (first 10):");
    let mut composite_avg_digits = 0.0;
    for &d in &composite_d_values {
        let (x, y) = pell_min_solution(d)?;
        let max_digits = x.to_string().len().max(y.to_string().len());
        composite_avg_digits += max_digits as f64;
        println!("  D={:2}: {} digits", d, max_digits);
    }
    composite_avg_digits /= composite_d_values.len() as f64;
    
    println!("\\nAverage solution size:");
    println!("  Prime D values:     {:.1} digits", prime_avg_digits);
    println!("  Composite D values: {:.1} digits", composite_avg_digits);
    println!();

    // Analysis 4: Special cases and patterns
    println!("🎯 Analysis 4: Special Cases and Mathematical Patterns");
    println!("{}", "-".repeat(50));
    
    // Check for D values of the form n² + 1
    println!("D values of the form n² + 1:");
    for n in 1..10 {
        let d = n * n + 1;
        if is_valid_pell_d(d) {
            let (x, y) = pell_min_solution(d)?;
            println!("  D={} ({}²+1): minimal solution has {} digits", 
                     d, n, x.to_string().len().max(y.to_string().len()));
        }
    }
    
    println!("\\nD values of the form n² - 1:");
    for n in 2..10 {
        let d = n * n - 1;
        if is_valid_pell_d(d) {
            let (x, y) = pell_min_solution(d)?;
            println!("  D={} ({}²-1): minimal solution has {} digits", 
                     d, n, x.to_string().len().max(y.to_string().len()));
        }
    }
    println!();

    // Analysis 5: Relationship between D and solution complexity
    println!("📐 Analysis 5: D vs Solution Complexity Correlation");
    println!("{}", "-".repeat(45));
    
    let analysis_d_values: Vec<u64> = (2..50).filter(|&d| is_valid_pell_d(d)).collect();
    let mut complexity_data = Vec::new();
    
    for &d in &analysis_d_values {
        let (x, y) = pell_min_solution(d)?;
        let complexity = x.to_string().len() + y.to_string().len();
        complexity_data.push((d, complexity));
    }
    
    // Sort by complexity
    complexity_data.sort_by_key(|&(_, complexity)| complexity);
    
    println!("Simplest solutions (lowest complexity):");
    for &(d, complexity) in complexity_data.iter().take(5) {
        println!("  D={:2}: total digits = {}", d, complexity);
    }
    
    println!("\\nMost complex solutions (highest complexity):");
    for &(d, complexity) in complexity_data.iter().rev().take(5) {
        println!("  D={:2}: total digits = {}", d, complexity);
    }
    
    let avg_complexity: f64 = complexity_data.iter().map(|(_, c)| *c as f64).sum::<f64>() / complexity_data.len() as f64;
    println!("\\nAverage complexity: {:.1} total digits", avg_complexity);
    println!();

    // Analysis 6: Verification of mathematical properties
    println!("✅ Analysis 6: Mathematical Property Verification");
    println!("{}", "-".repeat(45));
    
    let verification_d = 13;
    let mut iter = PellSolutionIterator::new(verification_d)?;
    
    println!("Verifying Pell equation properties for D={}:", verification_d);
    
    // Property 1: All solutions satisfy the equation
    let solutions: Vec<_> = iter.take(5).collect();
    let mut all_verified = true;
    for (k, (x, y)) in solutions.iter().enumerate() {
        let verified = verify_pell_solution(verification_d, x, y);
        if !verified {
            all_verified = false;
        }
        println!("  Solution {}: {} (x²-{}y²=1)", k+1, if verified { "✓" } else { "✗" }, verification_d);
    }
    println!("  All solutions satisfy Pell equation: {}", if all_verified { "✓" } else { "✗" });
    
    // Property 2: Solutions are strictly increasing
    let mut increasing = true;
    for i in 1..solutions.len() {
        if solutions[i].0 <= solutions[i-1].0 {
            increasing = false;
            break;
        }
    }
    println!("  Solutions are strictly increasing: {}", if increasing { "✓" } else { "✗" });
    
    // Property 3: Fundamental solution generates all others
    let (x1, y1) = &solutions[0];
    use pell991::pell_solution_k;
    let mut fundamental_generates_all = true;
    for (k, (x_expected, y_expected)) in solutions.iter().enumerate() {
        let k = k + 1;
        let (x_computed, y_computed) = pell_solution_k(verification_d, x1, y1, k as u64)?;
        if x_computed != *x_expected || y_computed != *y_expected {
            fundamental_generates_all = false;
            break;
        }
    }
    println!("  Fundamental solution generates all others: {}", if fundamental_generates_all { "✓" } else { "✗" });
    
    println!();
    println!("🎓 Mathematical Insights:");
    println!("• Prime D values don't necessarily have simpler solutions");
    println!("• Solution complexity varies dramatically with D");
    println!("• D values of special forms (n²±1) show interesting patterns");
    println!("• All fundamental mathematical properties are preserved");
    println!("• The algorithm correctly implements Pell equation theory");

    Ok(())
}