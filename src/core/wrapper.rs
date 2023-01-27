use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign};

use crypto_bigint::{Encoding, UInt, U256};
use elliptic_curve::{Field, IsHigh, PrimeField, ScalarCore};
use generic_array::{typenum, GenericArray};
use subtle::{ConditionallySelectable, ConstantTimeEq, ConstantTimeGreater, CtOption};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct W<F>(F);

impl<F> W<F> {
    pub const fn new(n: F) -> Self {
        Self(n)
    }
}

impl<F> Deref for W<F> {
    type Target = F;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> DerefMut for W<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<F: PrimeField> W<F> {
    pub fn to_be_bytes(&self) -> F::Repr {
        self.0.to_repr()
    }

    pub fn to_le_bytes(&self) -> F::Repr {
        let mut bytes = self.to_be_bytes();
        bytes.as_mut().reverse();
        bytes
    }

    pub fn from_be_bytes(bytes: F::Repr) -> CtOption<Self> {
        F::from_repr(bytes).map(Self)
    }

    pub fn from_le_bytes(mut bytes: F::Repr) -> CtOption<Self> {
        bytes.as_mut().reverse();
        Self::from_be_bytes(bytes)
    }

    pub fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        Self(
            bytes
                .iter()
                .fold(F::zero(), |s, b| s * F::from(256) + F::from(u64::from(*b))),
        )
    }

    pub fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        Self(
            bytes
                .iter()
                .rev()
                .fold(F::zero(), |s, b| s * F::from(256) + F::from(u64::from(*b))),
        )
    }
}

impl<F: From<u64>> From<u64> for W<F> {
    fn from(n: u64) -> Self {
        Self(F::from(n))
    }
}

impl<F: PrimeField, const LIMBS: usize> From<W<F>> for UInt<LIMBS>
where
    UInt<LIMBS>: Encoding,
    <UInt<LIMBS> as Encoding>::Repr: From<F::Repr>,
{
    fn from(s: W<F>) -> Self {
        UInt::from_be_bytes(s.to_be_bytes().into())
    }
}

impl<F: ConditionallySelectable> ConditionallySelectable for W<F> {
    fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
        Self(F::conditional_select(&a.0, &b.0, choice))
    }
}

impl<F: ConstantTimeEq> ConstantTimeEq for W<F> {
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        self.0.ct_eq(&other.0)
    }
}

impl<F: Add<Output = F>> Add for W<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<F: Sub<Output = F>> Sub for W<F> {
    type Output = W<F>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<F: Mul<Output = F>> Mul for W<F> {
    type Output = W<F>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<F: Neg<Output = F>> Neg for W<F> {
    type Output = W<F>;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<'r, F: Add<&'r F, Output = F>> Add<&'r W<F>> for W<F> {
    type Output = W<F>;

    fn add(self, rhs: &'r W<F>) -> Self::Output {
        Self(self.0 + &rhs.0)
    }
}

impl<'r, F: Mul<&'r F, Output = F>> Mul<&'r W<F>> for W<F> {
    type Output = W<F>;

    fn mul(self, rhs: &'r W<F>) -> Self::Output {
        Self(self.0 * &rhs.0)
    }
}

impl<'r, F: Sub<&'r F, Output = F>> Sub<&'r W<F>> for W<F> {
    type Output = W<F>;

    fn sub(self, rhs: &'r W<F>) -> Self::Output {
        Self(self.0 - &rhs.0)
    }
}

impl<F: MulAssign> MulAssign for W<F> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl<F: AddAssign> AddAssign for W<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<F: SubAssign> SubAssign for W<F> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<'r, F: MulAssign<&'r F>> MulAssign<&'r W<F>> for W<F> {
    fn mul_assign(&mut self, rhs: &'r W<F>) {
        self.0 *= &rhs.0
    }
}

impl<'r, F: AddAssign<&'r F>> AddAssign<&'r W<F>> for W<F> {
    fn add_assign(&mut self, rhs: &'r W<F>) {
        self.0 += &rhs.0
    }
}

impl<'r, F: SubAssign<&'r F>> SubAssign<&'r W<F>> for W<F> {
    fn sub_assign(&mut self, rhs: &'r W<F>) {
        self.0 -= &rhs.0
    }
}

impl<F: Field> Field for W<F> {
    fn random(rng: impl crypto_bigint::rand_core::RngCore) -> Self {
        Self(F::random(rng))
    }

    fn zero() -> Self {
        Self(F::zero())
    }

    fn one() -> Self {
        Self(F::one())
    }

    fn square(&self) -> Self {
        Self(self.0.square())
    }

    fn double(&self) -> Self {
        Self(self.0.double())
    }

    fn invert(&self) -> CtOption<Self> {
        self.0.invert().map(Self)
    }

    fn sqrt(&self) -> CtOption<Self> {
        self.0.sqrt().map(Self)
    }
}

impl<F> PrimeField for W<F>
where
    F: PrimeField,
    F::Repr: From<GenericArray<u8, typenum::U32>>,
    GenericArray<u8, typenum::U32>: From<F::Repr>,
{
    type Repr = GenericArray<u8, typenum::U32>;

    const NUM_BITS: u32 = F::NUM_BITS;
    const CAPACITY: u32 = F::CAPACITY;
    const S: u32 = F::S;

    fn from_repr(repr: Self::Repr) -> CtOption<Self> {
        F::from_repr(repr.into()).map(Self)
    }

    fn to_repr(&self) -> Self::Repr {
        self.0.to_repr().into()
    }

    fn is_odd(&self) -> subtle::Choice {
        self.0.is_odd()
    }

    fn multiplicative_generator() -> Self {
        Self(F::multiplicative_generator())
    }

    fn root_of_unity() -> Self {
        Self(F::root_of_unity())
    }
}

impl<F: Default + Copy> zeroize::DefaultIsZeroes for W<F> {}

impl<F: PrimeField, C: elliptic_curve::Curve> From<ScalarCore<C>> for W<F> {
    fn from(s: ScalarCore<C>) -> Self {
        let bytes_be = s.to_be_bytes();
        Self::from_be_bytes_mod_order(&bytes_be)
    }
}

impl<F> From<W<F>> for GenericArray<u8, typenum::U32>
where
    W<F>: PrimeField<Repr = GenericArray<u8, typenum::U32>>,
{
    fn from(s: W<F>) -> Self {
        s.to_repr()
    }
}

impl<F> IsHigh for W<F>
where
    F: PrimeField,
    W<F>: Sub<Output = W<F>>,
    U256: From<W<F>>,
{
    fn is_high(&self) -> subtle::Choice {
        let n = Self::zero() - Self::one();
        let n_2 = U256::from(n) >> 1;

        let s = U256::from(*self);

        s.ct_gt(&n_2)
    }
}
