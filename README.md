# Stark Curve

Pure Rust implementation of [Stark Curve][stark-specs]. Provides basic elliptic curve arithmetic backed by
[primeorder], [elliptic-curve], and [ff] crates. `#![no_std]` friendly.

## Curve parameters
As specified in a stark curve [specs][stark-specs], this crate provides an implementation of a curve defined
by equation:

$$y^2 = x^3 + \alpha x + \beta \pmod p$$

where: 

$$
\begin{aligned}
\alpha &= 1\\\\
\beta &= 3141592653589793238462643383279502884197169399375105820974944592307816406665\\\\
p &= 3618502788666131213697322783095070105623107215331596699973092056135872020481\\\\
  &=  2^{251} + 17 \cdot 2^{192} + 1
\end{aligned}
$$

Also, curve order $n$, which is not mentioned in the specs but can be found [here][curve-order]: \
$n = 3618502788666131213697322783095070105526743751716087489154079457884512865583$

Both $p$ and $n$ are prime.

## Security
This crate doesn't implement any sensitive cryptography code. Instead, we delegate scalar arithmetic
to [ff] crate, and elliptic point arithmetic to [primeorder] crate, which are considered to be heavily
used and tested.

[stark-specs]: https://docs.starkware.co/starkex/crypto/stark-curve.html
[curve-order]: https://github.com/starkware-libs/starkware-crypto-utils/blob/d3a1e655105afd66ebc07f88a179a3042407cc7b/src/js/signature.js#L62
[primeorder]: https://crates.io/crates/primeorder
[ff]: https://crates.io/crates/ff
[elliptic-curve]: https://crates.io/crates/elliptic-curve

## License

Licensed under either of

 * Apache License, Version 2.0
 * MIT license

at your option.
