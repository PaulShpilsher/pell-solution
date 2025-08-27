//! Performance analysis example for different D values and solution generation methods

use pell991::{pell_min_solution, pell_solution_k, pell_solutions};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Pell Equation Solver - Performance Analysis");
    println!("{}", "=".repeat(50));
    println!();

    // Test 1: Minimal solution performance for different D values
    println!("📊 Test 1: Minimal Solution Performance");
    println!("{}", "-".repeat(40));
    
    let test_d_values = [2, 3, 5, 7, 13, 61, 109, 181, 277, 397, 541, 991];
    
    for &d in &test_d_values {
        let start = Instant::now();
        let (x, y) = pell_min_solution(d)?;
        let duration = start.elapsed();
        
        println!("D = {:4}: {:8.2}μs | Solution digits: x={}, y={}", 
                 d, duration.as_micros(), x.to_string().len(), y.to_string().len());
    }
    
    println!();
    
    // Test 2: K-th solution performance
    println!("📈 Test 2: K-th Solution Performance (D=2)");
    println!("{}", "-".repeat(40));
    
    let d = 2;
    let (x1, y1) = pell_min_solution(d)?;
    let k_values = [1, 2, 5, 10, 20, 50, 100];
    
    for &k in &k_values {
        let start = Instant::now();
        let (xk, yk) = pell_solution_k(d, &x1, &y1, k)?;
        let duration = start.elapsed();
        
        println!("k = {:3}: {:8.2}μs | Solution digits: x={}, y={}", 
                 k, duration.as_micros(), xk.to_string().len(), yk.to_string().len());
    }
    
    println!();
    
    // Test 3: Batch vs Individual solution generation
    println!("⚡ Test 3: Batch vs Individual Generation (D=2, count=20)");
    println!("{}", "-".repeat(40));
    
    let count = 20;
    
    // Batch generation
    let start = Instant::now();
    let batch_solutions = pell_solutions(d, count)?;
    let batch_duration = start.elapsed();
    
    // Individual generation
    let start = Instant::now();
    let mut individual_solutions = Vec::new();
    for k in 1..=count {
        let (xk, yk) = pell_solution_k(d, &x1, &y1, k as u64)?;
        individual_solutions.push((xk, yk));
    }
    let individual_duration = start.elapsed();
    
    println!("Batch generation:      {:8.2}μs", batch_duration.as_micros());
    println!("Individual generation: {:8.2}μs", individual_duration.as_micros());
    println!("Speedup: {:.2}x", individual_duration.as_secs_f64() / batch_duration.as_secs_f64());
    
    // Verify they produce the same results
    assert_eq!(batch_solutions.len(), individual_solutions.len());
    for (i, ((bx, by), (ix, iy))) in batch_solutions.iter().zip(individual_solutions.iter()).enumerate() {
        assert_eq!(bx, ix, "X mismatch at solution {}", i + 1);
        assert_eq!(by, iy, "Y mismatch at solution {}", i + 1);
    }
    println!("✓ Results verified identical");
    
    println!();
    
    // Test 4: Memory usage analysis
    println!("💾 Test 4: Memory Usage Analysis");
    println!("{}", "-".repeat(40));
    
    let memory_test_d_values = [2, 13, 61, 991];
    
    for &d in &memory_test_d_values {
        let (x, y) = pell_min_solution(d)?;
        let x_bytes = x.to_string().len();
        let y_bytes = y.to_string().len();
        let estimated_memory = (x_bytes + y_bytes) * 8; // Rough estimate
        
        println!("D = {:3}: ~{:6} bytes | x={} digits, y={} digits", 
                 d, estimated_memory, x_bytes, y_bytes);
    }
    
    println!();
    
    // Test 5: Solution growth analysis
    println!("📏 Test 5: Solution Growth Analysis (D=2)");
    println!("{}", "-".repeat(40));
    
    let growth_k_values = [1, 2, 3, 4, 5, 10, 15, 20];
    let mut prev_x_len = 0;
    
    for &k in &growth_k_values {
        let (xk, yk) = pell_solution_k(d, &x1, &y1, k)?;
        let x_len = xk.to_string().len();
        let y_len = yk.to_string().len();
        
        let growth_factor = if prev_x_len > 0 { 
            x_len as f64 / prev_x_len as f64 
        } else { 
            1.0 
        };
        
        println!("k = {:2}: x={:2} digits, y={:2} digits | Growth: {:.2}x", 
                 k, x_len, y_len, growth_factor);
        
        prev_x_len = x_len;
    }
    
    println!();
    println!("🎯 Analysis Complete!");
    println!("Key Insights:");
    println!("• Larger D values generally take longer to solve");
    println!("• Batch generation is significantly faster than individual generation");
    println!("• Solution size grows exponentially with k");
    println!("• Memory usage scales with solution magnitude");
    
    Ok(())
}