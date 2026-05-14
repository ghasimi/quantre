//! RNG from Noraml Distribution

use super::rng_uniform::rand;
use std::f64::consts::PI;
use std::mem;

const PI2: f64 = 2_f64 * PI;

/// `n` pseudorandom numbers from N(μ=0, σ=1)
fn standard_randn(n: u64) -> Vec<f64> {
    let m = (n / 2 + 1) * 2;
    let u = rand(m);
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

/// `n` pseudorandom numbers from N(μ, σ)
///
/// Using the [Box–Muller transform](https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform)
///
pub fn randn(n: u64, mu: f64, sigma: f64) -> Vec<f64> {
    let r = standard_randn(n).iter().map(|z| z * sigma + mu).collect();
    return r;
}

/// `One` pseudorandom number from N(μ, σ)
pub fn randn1(mu: f64, sigma: f64) -> f64 {
    return randn(1, mu, sigma)[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check μ=0 and σ=1 for the pseudorandom numbers from the Standard Noraml Dist.
    #[test]
    fn check_randn() {
        let mu_true = -1.;
        let sigma_true = 0.2;
        let n: u64 = 1000_000;
        let r = randn(n, mu_true, sigma_true);

        // mu (test)
        let mut tot: f64 = 0.; // running total
        for x in &r {
            tot = tot + *x;
        }
        let mu: f64 = tot / n as f64;

        // sigma (test)
        let mut ss: f64 = 0.; // running sum of squared
        for x in &r {
            ss = ss + (*x - mu) * (*x - mu);
        }
        let var = ss / (n - 1) as f64;
        let sigma = var.sqrt();

        // check size
        assert_eq!(n, r.len() as u64);

        // check mean
        assert!((mu - mu_true).abs() < 0.01);

        // check sigma
        assert!((sigma - sigma_true).abs() < 0.01);
    }

    /// Check generation of a signle pseudorandom number from N(μ=0, σ=1)
    /// (Standard Noraml Dist.)
    #[test]
    fn check_randn1() {
        let _: f64 = randn1(0., 1.);
    }
}
