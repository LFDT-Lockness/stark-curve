#![doc = include_str!("../README.md")]
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

impl From<&Scalar> for ScalarPrimitive<StarkCurve> {
    fn from(s: &Scalar) -> Self {
        ScalarPrimitive::from_uint_unchecked(s.to_uint())
    }
}
