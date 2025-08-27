//! Benchmarks for the Pell equation solver

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use pell991::{pell_min_solution, pell_solution_k, pell_solutions};

fn bench_minimal_solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("minimal_solutions");
    
    // Test different D values to see how performance scales
    let d_values = [2, 3, 5, 7, 13, 61, 109, 181, 277, 397, 541, 991];
    
    for &d in &d_values {
        group.bench_with_input(BenchmarkId::new("pell_min_solution", d), &d, |b, &d| {
            b.iter(|| pell_min_solution(black_box(d)))
        });
    }
    
    group.finish();
}

fn bench_kth_solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("kth_solutions");
    
    // Pre-compute minimal solution for D=2
    let d = 2;
    let (x1, y1) = pell_min_solution(d).unwrap();
    
    // Test different k values
    let k_values = [1, 2, 5, 10, 20, 50, 100];
    
    for &k in &k_values {
        group.bench_with_input(BenchmarkId::new("pell_solution_k", k), &k, |b, &k| {
            b.iter(|| pell_solution_k(black_box(d), black_box(&x1), black_box(&y1), black_box(k)))
        });
    }
    
    group.finish();
}

fn bench_multiple_solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_solutions");
    
    let d = 2;
    let counts = [1, 5, 10, 20, 50];
    
    for &count in &counts {
        group.bench_with_input(BenchmarkId::new("pell_solutions", count), &count, |b, &count| {
            b.iter(|| pell_solutions(black_box(d), black_box(count)))
        });
    }
    
    group.finish();
}

fn bench_solution_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("solution_methods");
    
    let d = 2;
    let count = 10;
    let (x1, y1) = pell_min_solution(d).unwrap();
    
    // Compare batch generation vs individual generation
    group.bench_function("batch_generation", |b| {
        b.iter(|| pell_solutions(black_box(d), black_box(count)))
    });
    
    group.bench_function("individual_generation", |b| {
        b.iter(|| {
            let mut solutions = Vec::new();
            for k in 1..=count {
                let solution = pell_solution_k(black_box(d), black_box(&x1), black_box(&y1), black_box(k as u64)).unwrap();
                solutions.push(solution);
            }
            solutions
        })
    });
    
    group.finish();
}

fn bench_large_d_values(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_d_values");
    
    // Test with larger D values that have longer continued fraction periods
    let large_d_values = [991, 1999, 9999, 19999];
    
    for &d in &large_d_values {
        group.bench_with_input(BenchmarkId::new("large_d", d), &d, |b, &d| {
            b.iter(|| pell_min_solution(black_box(d)))
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_minimal_solutions,
    bench_kth_solutions,
    bench_multiple_solutions,
    bench_solution_comparison,
    bench_large_d_values
);
criterion_main!(benches);