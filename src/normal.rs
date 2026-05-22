//! Normal Distribution Utility Functions

/// Z-score = (x - μ) / σ
pub fn z_score(x: f64, mu: f64, sigma: f64) -> f64 {
    return (x - mu) / sigma;
}

/// Probability Density Function (PDF) of Normal Dist.
pub fn pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    const PI: f64 = std::f64::consts::PI;
    let sigma2: f64 = sigma.powi(2);
    let a: f64 = 1_f64 / (2_f64 * PI * sigma2).sqrt();
    let b: f64 = -1_f64 * (x - mu).powi(2) / (2_f64 * sigma2);
    return a * b.exp();
}

/// Cumulative Distribution Function (CDF) Approx. of Normal Dist.
///
/// Φ(z) = 1 / (1 + e^(-y))
/// 
/// where
/// 
/// `y` = 1.5957764 z + 0.0726161 z^3 + 0.00003318 z^6 −0.00021785 z^7 + 0.00006293 z^8 − 0.00000519 z^9
/// 
/// and `z` is the z-score of `x` = (x - mu) / sigma.
/// 
/// Reference: Eidous OM and Ananbeh EA. Approximations for cumulative distribution function of standard normal. J Stat Manage Syst 2022; 25: 541–547
/// 
/// Accessed via [Eidous, Omar & Rawwash, Mohammad. (2025)](https://www.researchgate.net/publication/389838151_Approximations_for_standard_normal_distribution_function_and_its_invertible). 
/// Approximations for standard normal distribution function and its invertible. 19. 10.1177/174830262513221. 
/// [CC BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/)
/// 
pub fn cdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let z: f64 = z_score(x, mu, sigma);
    let y = 1.5957764 * z + 0.0726161 * z.powi(3) + 0.00003318 * z.powi(6) - 0.00021785 * z.powi(7)
        + 0.00006293 * z.powi(8)
        - 0.00000519 * z.powi(9);
    return 1. / (1. + (-y).exp());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_z_score() {
        let z_true: f64 = 2.5;
        let mu: f64 = -1.;
        let sigma: f64 = 2.;
        let x: f64 = z_true * 2. - 1.; // x corresponding to the true z
        let z_test: f64 = z_score(x, mu, sigma);
        assert_eq!(z_test, z_true);
    }

    #[test]
    fn check_pdf() {
        let pdf_tue: f64 = 0.00876415; // Python (scipy)
        let pdf_test: f64 = (pdf(4., -1., 2.) * 1e8).round() / 1e8;
        assert_eq!(pdf_test, pdf_tue);
    }

    #[test]
    fn check_standard_normal_cdf() {
        let z: f64 = -1.5;
        let cdf_true: f64 = 0.066807; // Python (scipy)
        let cdf_test: f64 = (cdf(z, 0., 1.) * 1e6).round() / 1e6;
        assert!((cdf_test - cdf_true).abs() <= 1e-3);
    }

    #[test]
    fn check_cdf() {
        let mu = -1.;
        let sigma = 0.2;
        let x: f64 = mu + 1.96 * sigma;
        let cdf_true: f64 = (0.9750021048517795_f64 * 1e4 as f64).round() / 1e4; // Python (scipy)
        let cdf_test: f64 = (cdf(x, mu, sigma) * 1e4).round() / 1e4;
        assert_eq!(cdf_test, cdf_true);
    }
}
