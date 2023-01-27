mod wrapper;

pub use wrapper::W;

pub mod field_element {
    use ff::PrimeField;
    use generic_array::{typenum, GenericArray};

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

pub mod scalar {
    use ff::PrimeField;
    use generic_array::{typenum, GenericArray};

    #[derive(PrimeField)]
    #[PrimeFieldModulus = "3618502788666131213697322783095070105526743751716087489154079457884512865583"]
    #[PrimeFieldGenerator = "3"]
    #[PrimeFieldReprEndianness = "big"]
    pub struct ScalarCore([u64; 4]);

    impl ScalarCore {
        pub(crate) const fn from_internal_repr(repr: [u64; 4]) -> Self {
            Self(repr)
        }

        #[cfg(test)]
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

impl W<field_element::FieldElementCore> {
    pub const ZERO: Self = crate::constants::ZERO_FE;
    pub const ONE: Self = crate::constants::ONE_FE;
}

impl W<scalar::ScalarCore> {
    pub const ZERO: Self = crate::constants::ZERO_S;
    pub const ONE: Self = crate::constants::ONE_S;
}
