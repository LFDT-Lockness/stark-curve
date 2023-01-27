use crypto_bigint::U256;
use elliptic_curve::{AffineArithmetic, Curve, PrimeCurve, ProjectiveArithmetic, ScalarArithmetic};
use primeorder::PrimeCurveParams;

use self::core::{field_element::FieldElementCore, scalar::ScalarCore, W};

pub mod constants;
pub mod core;

pub type FieldElement = W<FieldElementCore>;
pub type Scalar = W<ScalarCore>;
pub type AffinePoint = primeorder::AffinePoint<StarkCurve>;
pub type ProjectivePoint = primeorder::ProjectivePoint<StarkCurve>;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct StarkCurve;

impl Curve for StarkCurve {
    type UInt = U256;

    const ORDER: Self::UInt =
        U256::from_be_hex("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");
}

impl PrimeCurve for StarkCurve {}

impl ScalarArithmetic for StarkCurve {
    type Scalar = Scalar;
}

impl AffineArithmetic for StarkCurve {
    type AffinePoint = AffinePoint;
}

impl ProjectiveArithmetic for StarkCurve {
    type ProjectivePoint = ProjectivePoint;
}

impl PrimeCurveParams for StarkCurve {
    type FieldElement = FieldElement;

    const ZERO: Self::FieldElement = FieldElement::ZERO;
    const ONE: Self::FieldElement = FieldElement::ONE;

    const EQUATION_A: Self::FieldElement = constants::EQUATION_A;
    const EQUATION_B: Self::FieldElement = constants::EQUATION_B;

    const GENERATOR: (Self::FieldElement, Self::FieldElement) = constants::GENERATOR;

    const EQUATION_A_EQUALS_MINUS_3: bool = false;
}
