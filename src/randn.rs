//! RNG from Noraml Distribution

use super::rand;
use std::f64::consts::PI;
use std::mem;

const PI2: f64 = 2_f64 * PI;

/// `n` pseudorandom numbers from N(μ=0, σ=1)
///
/// Using the [Box–Muller transform](https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform)
///
pub fn randn(n: u64) -> Vec<f64> {
    let m = (n / 2 + 1) * 2;
    let u = rand::rand(m);
    let mut r: Vec<f64> = Vec::new();
    for pair in u.chunks(2) {
        let mut u1 = pair[0];
        let mut u2 = pair[1];
        if u1 == 0. {
            mem::swap(&mut u1, &mut u2);
        };
        let rad = (-2_f64 * u1.ln()).sqrt();
        let n1 = rad * (PI2 * u2).cos();
        let n2 = rad * (PI2 * u2).sin();
        r.push(n1);
        r.push(n2);
    }
    return r[..n as usize].to_vec();
}

/// `One` pseudorandom number from N(μ=0, σ=1)
pub fn randn1() -> f64 {
    return randn(1)[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check μ=0 and σ=1 for the pseudorandom numbers from the Standard Noraml Dist.
    #[test]
    fn check_randn() {
        let n: u64 = 1000_000;
        let r = randn(n);
        let mut mu: f64 = 0.; // running mean
        let mut ss: f64 = 0.; // runnin sum of squared
        for x in &r {
            let new_mu = (*x - mu) / n as f64;
            ss = ss + (*x - mu) * (*x - new_mu);
            mu = new_mu;
        }
        let var = ss / (n - 1) as f64;

        // check size
        assert_eq!(n, r.len() as u64);

        // check mean
        assert!((mu - 0.).abs() < 0.01);

        // check variance
        assert!((var - 1.).abs() < 0.01);
    }

    /// Check generation of a signle pseudorandom number from N(μ=0, σ=1)
    /// (Standard Noraml Dist.)
    #[test]
    fn check_randn1() {
        let _: f64 = randn1();
    }
}
