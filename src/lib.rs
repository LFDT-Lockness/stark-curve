//! # Stark Curve
//!
//! Pure Rust implementation of [Stark Curve][stark-specs]. Provides basic elliptic curve arithmetic backed by
//! [primeorder], [elliptic_curve], and [ff] crates. `#![no_std]` friendly.
//!
//! ## Curve parameters
//! As specified in a stark curve [specs][stark-specs], this crate provides an implementation of a curve defined
//! by equation:
//!
//! $$y^2 = x^3 + \alpha x + \beta \pmod p$$
//!
//! where: \
//! $\begin{aligned}
//! \alpha &= 1\\\\
//! \beta &= 3141592653589793238462643383279502884197169399375105820974944592307816406665\\\\
//! p &= 3618502788666131213697322783095070105623107215331596699973092056135872020481\\\\
//!   &=  2^{251} + 17 \cdot 2^{192} + 1
//! \end{aligned}$
//!
//! Also, curve order $n$, which is not mentioned in the specs but can be found [here][curve-order]: \
//! $n = 3618502788666131213697322783095070105526743751716087489154079457884512865583$
//!
//! Both $p$ and $n$ are prime.
//!
//! ## Security
//! This crate doesn't implement any sensitive cryptography code. Instead, we delegate scalar arithmetic
//! to [ff] crate, and elliptic point arithmetic to [primeorder] crate, which are considered to be heavily
//! used and tested.
//!
//! [stark-specs]: https://docs.starkware.co/starkex/crypto/stark-curve.html
//! [curve-order]: https://github.com/starkware-libs/starkware-crypto-utils/blob/d3a1e655105afd66ebc07f88a179a3042407cc7b/src/js/signature.js#L62

#![no_std]
#![deny(missing_docs)]

pub use primeorder::{
    self,
    elliptic_curve::{
        self,
        bigint::{self, rand_core},
        ff,
        generic_array::{self, typenum},
    },
};

use bigint::U256;
use elliptic_curve::{
    scalar::{FromUintUnchecked, ScalarPrimitive},
    Curve, CurveArithmetic, PrimeCurve,
};
use primeorder::PrimeCurveParams;

use self::core::{field_element::FieldElementCore, scalar::ScalarCore, W};

pub mod constants;
pub mod core;

/// Field element (unsigned integer mod $p$)
pub type FieldElement = W<FieldElementCore>;
/// Scalar (unsigned integer mod $n$)
pub type Scalar = W<ScalarCore>;
/// Affine point on stark curve
pub type AffinePoint = primeorder::AffinePoint<StarkCurve>;
/// Projective point on stark curve
pub type ProjectivePoint = primeorder::ProjectivePoint<StarkCurve>;

/// Stark curve
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct StarkCurve;

impl Curve for StarkCurve {
    type FieldBytesSize = typenum::U32;
    type Uint = U256;

    const ORDER: Self::Uint =
        U256::from_be_hex("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");
}

impl PrimeCurve for StarkCurve {}

impl CurveArithmetic for StarkCurve {
    type Scalar = Scalar;
    type AffinePoint = AffinePoint;
    type ProjectivePoint = ProjectivePoint;
}

impl PrimeCurveParams for StarkCurve {
    type FieldElement = FieldElement;
    type PointArithmetic = primeorder::point_arithmetic::EquationAIsGeneric;

    const EQUATION_A: Self::FieldElement = constants::EQUATION_A;
    const EQUATION_B: Self::FieldElement = constants::EQUATION_B;

    const GENERATOR: (Self::FieldElement, Self::FieldElement) = constants::GENERATOR;
}

impl elliptic_curve::FieldBytesEncoding<StarkCurve> for U256 {}

impl From<Scalar> for ScalarPrimitive<StarkCurve> {
    fn from(s: Scalar) -> Self {
        ScalarPrimitive::from_uint_unchecked(s.to_uint())
    }
}
