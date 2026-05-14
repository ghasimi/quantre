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

## Documentation

[docs.rs/quantre](https://docs.rs/quantre)