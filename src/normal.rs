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

/// CDF of Standard Normal Dist.
/// See the cdf() doc for details
fn standard_normal_cdf(z: f64) -> f64 {
    let a: f64 = -0.07056 * z.powi(3);
    let b: f64 = -1.5976 * z;
    return 1_f64 / (1_f64 + (a + b).exp());
}

/// Cumulative Distribution Function (CDF) Approx. of Normal Dist.
///
/// F(z) ≈ 1 / (1 + e^(-0.07056 * z^3 - 1.5976 * z))
///
/// where `z` is the z-score of `x` ∈ N(μ, σ)
/// 
/// Source: [SR. Bowling et al., 2009](https://hdl.handle.net/10419/188388)
/// 
pub fn cdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let z: f64 = z_score(x, mu, sigma);
    return standard_normal_cdf(z);
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
        let cdf_true: f64 = 0.066948; // source approx. (vs Python (scipy) 0.066807)
        let cdf_test: f64 = (standard_normal_cdf(z) * 1e6).round() / 1e6;
        assert_eq!(cdf_test, cdf_true);
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
