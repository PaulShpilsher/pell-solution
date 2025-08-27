use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::cmp::Ordering;

/// Solve the Pell equation x^2 - D*y^2 = 1 for non-square D > 1.
/// Returns the minimal solution (x, y).
pub fn pell_min_solution(d_constant: u64) -> (BigInt, BigInt) {
    assert!(d_constant > 1, "D must be > 1");
    assert!(!is_square_u64(d_constant), "D must be non-square");

    let a0 = isqrt_u64(d_constant);
    let mut m: i128 = 0;
    let mut d: i128 = 1;
    let mut a: i128 = a0 as i128;

    // Convergents: p[-2]=0, p[-1]=1; q[-2]=1, q[-1]=0
    let mut p_prev2 = BigInt::zero();
    let mut p_prev1 = BigInt::one();
    let mut q_prev2 = BigInt::one();
    let mut q_prev1 = BigInt::zero();

    let mut p = BigInt::from(a) * &p_prev1 + &p_prev2;
    let mut q = BigInt::from(a) * &q_prev1 + &q_prev2;

    let big_d = BigInt::from(d_constant);

    loop {
        let lhs = &p * &p - &big_d * &q * &q;
        if lhs.is_one() {
            return (p, q);
        }

        m = d * a - m;
        d = ((d_constant as i128) - m * m) / d;
        a = ((a0 as i128) + m) / d;

        let a_big = BigInt::from(a);

        let p_next = &a_big * &p + &p_prev1;
        let q_next = &a_big * &q + &q_prev1;

        p_prev1 = p;
        q_prev1 = q;
        p = p_next;
        q = q_next;
    }
}

/// Generate the k-th Pell solution (x_k, y_k) given the minimal solution.
/// Uses fast exponentiation of (x1 + y1*√D).
pub fn pell_solution_k(d_constant: u64, x1: &BigInt, y1: &BigInt, k: u64) -> (BigInt, BigInt) {
    if k == 1 {
        return (x1.clone(), y1.clone());
    }

    let mut x = BigInt::one();
    let mut y = BigInt::zero();

    let mut base_x = x1.clone();
    let mut base_y = y1.clone();

    let mut exp = k;
    let big_d = BigInt::from(d_constant);

    while exp > 0 {
        if exp % 2 == 1 {
            let new_x = &x * &base_x + &big_d * &y * &base_y;
            let new_y = &x * &base_y + &y * &base_x;
            x = new_x;
            y = new_y;
        }
        let new_x = &base_x * &base_x + &big_d * &base_y * &base_y;
        let new_y = BigInt::from(2u32) * &base_x * &base_y;
        base_x = new_x;
        base_y = new_y;

        exp /= 2;
    }

    (x, y)
}

/// Integer square root for u64: floor(sqrt(n))
fn isqrt_u64(n: u64) -> u64 {
    let mut x = (n as f64).sqrt() as u64;
    while (x + 1) * (x + 1) <= n { x += 1; }
    while x * x > n { x -= 1; }
    x
}

/// Check if n is a perfect square.
fn is_square_u64(n: u64) -> bool {
    let r = isqrt_u64(n);
    r * r == n
}

fn main() {
    let d = 991_u64;
    let (x1, y1) = pell_min_solution(d);

    println!("Minimal solution for x^2 - {d}y^2 = 1:");
    println!("x1 = {x1}");
    println!("y1 = {y1}");

    // Generate a few more solutions
    for k in 1..=5 {
        let (xk, yk) = pell_solution_k(d, &x1, &y1, k);
        println!("\nSolution {k}:");
        println!("x = {xk}");
        println!("y = {yk}");
    }
}
