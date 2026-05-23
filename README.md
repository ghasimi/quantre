# quantre

[crates.io](https://crates.io/crates/quantre) | 
[docs.rs](https://docs.rs/quantre) |
[github](https://github.com/ghasimi/quantre) |
[home](https://ghasimi.github.io/quantre)

Quant wheels reinvented in [Rust](https://rust-lang.org/).

## Installation

```bash
cargo add quantre
```

## Examples

### Distributions

```rust
// main.rs
use quantre::random;
use quantre::normal;

fn main() {
    // A random number from [0, 1)
    println!("{}", random::rand1());

    // Five random integers from -2 to 7
    println!("{:?}", random::randi(5, -2, 7));

    // Three random numbers from N(0, 1)
    println!("{:?}", random::randn(3, 0., 1.));

    // P(X <= 1.96), X ∈ N(0, 1)
    println!("{}", normal::cdf(1.96, 0., 1.));
}
```

### Option Pricing

```rust
// main.rs
use quantre::instrument::Option;
use quantre::price;

fn main() {

	let option = Option {
		underlying: 100.,
		strike: 95.,
		ttm: 1. / 12., 		// 1-month to maturity
		vol: 0.35,			// volatility
		quote_rate: 0.02, 	// risk free rate
		base_rate: 0.03 	// dividend yield
	};	

    // Black-Scholes-Merton (BSM)
    let (call, put) = price::bsm(option);

	// Black 76 - Options on Futures
	// let (call, put) = price::black(option);

	// Garman-Kohlhagen - FX options
	// let (call, put) = price::fx(option);

    println!("{:.4} {:.4} ", call, put);
}
```

## Documentation

[docs.rs/quantre](https://docs.rs/quantre)