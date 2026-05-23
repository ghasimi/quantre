//! Black-Scholes Option Pricing Models
//!

use crate::instrument::Option;
use crate::normal;

/// N(d): Standard Normal Dist. CDF
fn _n(d: f64) -> f64 {
    return normal::cdf(d, 0., 1.);
}

/// Black-Scholes-Merton (BSM) Option Pricing Model
///
/// The formula for pricing of European options,
/// named after economists Fischer Black and Myron Scholes,
/// who made it public in 1973, and
/// Robert C. Merton who extended the model to instruments that pay continues dividend yield.
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Black%E2%80%93Scholes_model)
///
pub fn bsm(option: Option) -> (f64, f64) {
    let x = option.underlying;
    let k = option.strike;
    let t = option.ttm;
    let vol = option.vol;
    let r1 = option.quote_rate;
    let r2 = option.base_rate;

    let d1 = ((x / k).ln() + (r1 - r2 + vol * vol / 2.) * t) / (vol * t.sqrt());
    let d2 = d1 - vol * t.sqrt();
    let call = x * _n(d1) * (-r2 * t).exp() - k * _n(d2) * (-r1 * t).exp();
    let put = k * _n(-d2) * (-r1 * t).exp() - x * _n(-d1) * (-r2 * t).exp();
    return (call, put);
}

/// Garman-Kohlhagen - FX Options
///
/// Prices European options on FX rate. The `spot` and `strike` follow
/// the "quote" currency per the "base" currency for consistency with the risk free arguments.
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Foreign_exchange_option#Garman%E2%80%93Kohlhagen_model)
///
pub fn garman_kohlhagen(option: Option) -> (f64, f64) {
    return bsm(option);
}

/// Black 76 - Options on Futures
///
/// A variation of Black-Scholes model to price options on forward or future contracts,
/// bond options, interest rate cap and floors, and swaptions.
/// Named after a paper written by Fischer Black in 1976.
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Black_model)
///
pub fn black(option: Option) -> (f64, f64) {
    let f = option.underlying;
    let k = option.strike;
    let t = option.ttm;
    let vol = option.vol;
    let r = option.quote_rate;

    let d1 = ((f / k).ln() + (vol * vol / 2.) * t) / (vol * t.sqrt());
    let d2 = d1 - vol * t.sqrt();
    let call = (-r * t).exp() * (f * _n(d1) - k * _n(d2));
    let put = (-r * t).exp() * (k * _n(-d2) - f * _n(-d1));
    return (call, put);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bsm() {
        let option = Option {
            underlying: 100.,
            strike: 96.,
            ttm: 3. / 12.,
            vol: 0.2,
            quote_rate: 0.02,
            base_rate: 0.01,
        };
        let (call, put) = bsm(option);
        assert!((call - 6.3674).abs() < 1e-4);
        assert!((put - 2.1383).abs() < 1e-4);
    }

    #[test]
    fn check_garman_kohlhagen() {
        let option = Option {
            underlying: 1.43,
            strike: 1.45,
            ttm: 1. / 12.,
            vol: 0.3,
            quote_rate: 0.05,
            base_rate: 0.03,
        };
        let (call, put) = garman_kohlhagen(option);
        assert!((call - 0.041292).abs() < 1e-4);
        assert!((put - 0.058833).abs() < 1e-4);
    }

    #[test]
    fn check_black() {
        let option = Option {
            underlying: 101.,
            strike: 100.,
            ttm: 1. / 12.,
            vol: 0.3,
            quote_rate: 0.05,
            base_rate: 0.0,
        };
        let (call, put) = black(option);
        assert!((call - 3.977396).abs() < 1e-4);
        assert!((put - 2.981554).abs() < 1e-4);
    }
}
