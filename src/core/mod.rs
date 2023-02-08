//! Core functionality
//!
//! Contains [field_element] and [scalar] derived by [ff] crate, and a wrapper struct [`W`] that makes field element
//! and scalar compatible with [elliptic-curve] crate.

mod wrapper;

pub use wrapper::W;

/// Field element, derived by [ff] crate
#[allow(missing_docs)]
pub mod field_element {
    use ff::PrimeField;

    use crate::generic_array::{typenum, GenericArray};

    #[derive(PrimeField)]
    #[PrimeFieldModulus = "3618502788666131213697322783095070105623107215331596699973092056135872020481"]
    #[PrimeFieldGenerator = "3"]
    #[PrimeFieldReprEndianness = "big"]
    pub struct FieldElementCore([u64; 4]);

    impl FieldElementCore {
        pub(crate) const fn from_internal_repr(repr: [u64; 4]) -> Self {
            Self(repr)
        }

        #[cfg(test)]
        pub(crate) const fn internal_repr(&self) -> &[u64; 4] {
            &self.0
        }
    }

    impl From<[u8; 32]> for FieldElementCoreRepr {
        fn from(bytes: [u8; 32]) -> Self {
            Self(bytes)
        }
    }

    impl From<FieldElementCoreRepr> for [u8; 32] {
        fn from(s: FieldElementCoreRepr) -> Self {
            s.0.into()
        }
    }

    impl From<GenericArray<u8, typenum::U32>> for FieldElementCoreRepr {
        fn from(bytes: GenericArray<u8, typenum::U32>) -> Self {
            Self(bytes.into())
        }
    }

    impl From<FieldElementCoreRepr> for GenericArray<u8, typenum::U32> {
        fn from(s: FieldElementCoreRepr) -> Self {
            s.0.into()
        }
    }
}

/// Scalar, derived by [ff] crate
#[allow(missing_docs)]
pub mod scalar {
    use ff::PrimeField;

    use crate::generic_array::{typenum, GenericArray};

    #[derive(PrimeField)]
    #[PrimeFieldModulus = "3618502788666131213697322783095070105526743751716087489154079457884512865583"]
    #[PrimeFieldGenerator = "3"]
    #[PrimeFieldReprEndianness = "big"]
    pub struct ScalarCore([u64; 4]);

    impl ScalarCore {
        #[allow(dead_code)]
        pub(crate) const fn from_internal_repr(repr: [u64; 4]) -> Self {
            Self(repr)
        }

        #[allow(dead_code)]
        pub(crate) const fn internal_repr(&self) -> &[u64; 4] {
            &self.0
        }
    }

    impl From<[u8; 32]> for ScalarCoreRepr {
        fn from(bytes: [u8; 32]) -> Self {
            Self(bytes)
        }
    }

    impl From<ScalarCoreRepr> for [u8; 32] {
        fn from(s: ScalarCoreRepr) -> Self {
            s.0.into()
        }
    }

    impl From<GenericArray<u8, typenum::U32>> for ScalarCoreRepr {
        fn from(bytes: GenericArray<u8, typenum::U32>) -> Self {
            Self(bytes.into())
        }
    }

    impl From<ScalarCoreRepr> for GenericArray<u8, typenum::U32> {
        fn from(s: ScalarCoreRepr) -> Self {
            s.0.into()
        }
    }
}

use crate::ff::Field;

impl W<field_element::FieldElementCore> {
    /// Field element $x = 0$
    pub const ZERO: Self = Self::new(field_element::FieldElementCore::ZERO);
    /// Field element $x = 1$
    pub const ONE: Self = Self::new(field_element::FieldElementCore::ONE);
}

impl W<scalar::ScalarCore> {
    /// Scalar $x = 0$
    pub const ZERO: Self = Self::new(scalar::ScalarCore::ZERO);
    /// Scalar $x = 1$
    pub const ONE: Self = Self::new(scalar::ScalarCore::ONE);
}
