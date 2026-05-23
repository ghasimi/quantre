//! Option Instruments
//!

/// A generic Option instrument
///
/// Same arguments can be used for different options,
/// like on stocks (w/ or w/o dividend), futures, FX rates, etc.
///
/// * `underlying`: Underlying price, e.g. Spot or Futures price,
/// or the exchange rate (quote-per-base) for FX options
///
/// * `strike`: Strike price
/// * `ttm`: Time-to-maturity in years, e.g., 0.5 for 6-months
/// * `vol`: Volatility, annualized, e.g. 0.35 for 35%
/// * `quote_rate`: Continues risk-free rate, e.g. 0.05 for 5%
/// * `base_rate`: Continues dividend yield, or the risk-free rate of the base currency for FX options
///
pub struct Option {
    pub underlying: f64,
    pub strike: f64,
    pub ttm: f64,
    pub vol: f64,
    pub quote_rate: f64,
    pub base_rate: f64,
}
