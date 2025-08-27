// src/main.rs
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Zero};
use std::cmp::Ordering;

/// Solve the Pell equation x^2 - D*y^2 = 1 for non-square D > 1.
/// Returns the minimal positive solution (x, y) as BigInts.
///
/// Algorithm: continued fraction expansion of √D; iterate convergents (p_k/q_k)
/// until p_k^2 - D*q_k^2 == 1.
pub fn pell_min_solution(d_contant: u64) -> (BigInt, BigInt) {
    assert!(d_contant > 1, "D must be > 1");
    assert!(!is_square_u64(d_contant), "D must be non-square");

    // Continued fraction parameters for sqrt(D)
    // Start with: m0 = 0, d0 = 1, a0 = floor(sqrt(D))
    let a0 = isqrt_u64(d_contant);
    let mut m: i128 = 0;
    let mut d: i128 = 1;
    let mut a: i128 = a0 as i128;

    // Convergents recurrence:
    // p[-2]=0, p[-1]=1; q[-2]=1, q[-1]=0
    let p_prev2 = BigInt::zero();
    let mut p_prev1 = BigInt::one();
    let q_prev2 = BigInt::one();
    let mut q_prev1 = BigInt::zero();

    // Current convergent p, q
    let mut p = BigInt::from_i128(a).unwrap() * &p_prev1 + &p_prev2;
    let mut q = BigInt::from_i128(a).unwrap() * &q_prev1 + &q_prev2;

    let big_d = BigInt::from(d_contant);

    loop {
        // Check Pell condition
        let lhs = &p * &p - &big_d * &q * &q;
        match lhs.cmp(&BigInt::one()) {
            Ordering::Equal => return (p, q),
            Ordering::Less | Ordering::Greater => {
                // continue
            }
        }

        // Advance the continued fraction terms:
        // m_{k+1} = d_k * a_k - m_k
        // d_{k+1} = (D - m_{k+1}^2) / d_k
        // a_{k+1} = floor((a0 + m_{k+1}) / d_{k+1})
        m = d * a - m;
        d = ((d_contant as i128) - m * m) / d;
        a = ((a0 as i128) + m) / d;

        // Next convergent
        let a_big = BigInt::from_i128(a).unwrap();

        let p_next = &a_big * &p + &p_prev1;
        let q_next = &a_big * &q + &q_prev1;

        // shift window
        p_prev1 = p;
        q_prev1 = q;
        p = p_next;
        q = q_next;
    }
}

/// Integer square root for u64: floor(sqrt(n))
fn isqrt_u64(n: u64) -> u64 {
    // fast integer sqrt (binary method)
    let mut x = (n as f64).sqrt() as u64;
    // polish to ensure floor correctness
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
    let (m, n) = pell_min_solution(d);

    println!("Minimal solution for x^2 - {d} y^2 = 1:");
    println!("x = {m}");
    println!("y = {n}");

    // Quick sanity check: x^2 - D*y^2 == 1
    let big_d = BigInt::from(d);
    let check = &m * &m - big_d * &n * &n;
    assert!(check.is_one());

    // For the puzzle, y is the 'n' such that sqrt(991*n^2 + 1) is integer.
    // You can compare with the known value:
    let known_n = BigInt::parse_bytes(
        b"12055735790331359447442538767",
        10
    ).unwrap();
    println!("Matches known n: {}", if n == known_n { "yes" } else { "no" });
}
