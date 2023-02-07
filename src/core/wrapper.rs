use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, ShlAssign, ShrAssign, Sub, SubAssign,
};

use primeorder::elliptic_curve::ops::Invert;
use subtle::{ConditionallySelectable, ConstantTimeEq, ConstantTimeGreater, CtOption};

use crate::bigint::{Encoding, Uint, U256};
use crate::elliptic_curve::{
    self,
    scalar::{FromUintUnchecked, IsHigh, ScalarPrimitive},
    Field, PrimeField,
};
use crate::generic_array::GenericArray;
use crate::typenum;

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
                .fold(F::ZERO, |s, b| s * F::from(256) + F::from(u64::from(*b))),
        )
    }

    pub fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        Self(
            bytes
                .iter()
                .rev()
                .fold(F::ZERO, |s, b| s * F::from(256) + F::from(u64::from(*b))),
        )
    }

    pub fn from_uint_mod_order(uint: &U256) -> Self {
        Self::from_be_bytes_mod_order(&uint.to_be_bytes())
    }
}

impl<F: PrimeField> W<F>
where
    [u8; 32]: From<F::Repr>,
{
    pub fn to_uint(&self) -> U256 {
        U256::from_be_bytes(self.to_be_bytes().into())
    }
}

impl<F> AsRef<W<F>> for W<F> {
    fn as_ref(&self) -> &W<F> {
        self
    }
}

impl<F: From<u64>> From<u64> for W<F> {
    fn from(n: u64) -> Self {
        Self(F::from(n))
    }
}

impl<F: PrimeField, const LIMBS: usize> From<W<F>> for Uint<LIMBS>
where
    Uint<LIMBS>: Encoding,
    <Uint<LIMBS> as Encoding>::Repr: From<F::Repr>,
{
    fn from(s: W<F>) -> Self {
        Uint::from_be_bytes(s.to_be_bytes().into())
    }
}

impl<F: PrimeField> FromUintUnchecked for W<F> {
    type Uint = U256;

    fn from_uint_unchecked(uint: Self::Uint) -> Self {
        let bytes_be = uint.to_be_bytes();
        Self::from_be_bytes_mod_order(&bytes_be)
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

impl<F: Sum> Sum for W<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|f| f.0).sum())
    }
}

impl<'f, F: Sum<&'f F>> Sum<&'f W<F>> for W<F> {
    fn sum<I: Iterator<Item = &'f W<F>>>(iter: I) -> Self {
        Self(iter.map(|f| &f.0).sum())
    }
}

impl<F: Product> Product for W<F> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|f| f.0).product())
    }
}

impl<'f, F: Product<&'f F>> Product<&'f W<F>> for W<F> {
    fn product<I: Iterator<Item = &'f W<F>>>(iter: I) -> Self {
        Self(iter.map(|f| &f.0).product())
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

impl<F: ShlAssign<usize>> ShlAssign<usize> for W<F> {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs
    }
}

impl<F> ShrAssign<usize> for W<F>
where
    [u8; 32]: From<F::Repr>,
    F: PrimeField,
{
    fn shr_assign(&mut self, rhs: usize) {
        let n = self.to_uint();
        *self = Self::from_uint_mod_order(&(n >> rhs))
    }
}

impl<F: Field> Invert for W<F> {
    type Output = CtOption<Self>;

    fn invert(&self) -> Self::Output {
        self.0.invert().map(Self)
    }
}

impl<F> elliptic_curve::ops::Reduce<U256> for W<F>
where
    F: PrimeField,
    W<F>: PrimeField,
{
    type Bytes = <W<F> as PrimeField>::Repr;

    fn reduce(n: U256) -> Self {
        Self::from_be_bytes_mod_order(&n.to_be_bytes())
    }

    fn reduce_bytes(bytes: &Self::Bytes) -> Self {
        Self::from_be_bytes_mod_order(bytes.as_ref())
    }
}

impl<F: Field> Field for W<F> {
    const ZERO: Self = W(F::ZERO);
    const ONE: Self = W(F::ONE);

    fn random(rng: impl crate::rand_core::RngCore) -> Self {
        Self(F::random(rng))
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

    fn sqrt_ratio(num: &Self, div: &Self) -> (subtle::Choice, Self) {
        let (choice, res) = F::sqrt_ratio(num, div);
        (choice, Self(res))
    }
}

impl<F> PrimeField for W<F>
where
    F: PrimeField,
    F::Repr: From<GenericArray<u8, typenum::U32>>,
    GenericArray<u8, typenum::U32>: From<F::Repr>,
{
    type Repr = GenericArray<u8, typenum::U32>;

    const MODULUS: &'static str = F::MODULUS;
    const NUM_BITS: u32 = F::NUM_BITS;
    const CAPACITY: u32 = F::CAPACITY;
    const TWO_INV: Self = Self(F::TWO_INV);
    const MULTIPLICATIVE_GENERATOR: Self = Self(F::MULTIPLICATIVE_GENERATOR);
    const S: u32 = F::S;
    const ROOT_OF_UNITY: Self = Self(F::ROOT_OF_UNITY);
    const ROOT_OF_UNITY_INV: Self = Self(F::ROOT_OF_UNITY_INV);
    const DELTA: Self = Self(F::DELTA);

    fn from_repr(repr: Self::Repr) -> CtOption<Self> {
        F::from_repr(repr.into()).map(Self)
    }

    fn to_repr(&self) -> Self::Repr {
        self.0.to_repr().into()
    }

    fn is_odd(&self) -> subtle::Choice {
        self.0.is_odd()
    }
}

impl<F: Default + Copy> zeroize::DefaultIsZeroes for W<F> {}

impl<F: PrimeField, C: elliptic_curve::Curve> From<ScalarPrimitive<C>> for W<F> {
    fn from(s: ScalarPrimitive<C>) -> Self {
        let bytes_be = s.as_uint().to_be_bytes();
        Self::from_be_bytes_mod_order(bytes_be.as_ref())
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
        let n = Self::ZERO - Self::ONE;
        let n_2 = U256::from(n) >> 1;

        let s = U256::from(*self);

        s.ct_gt(&n_2)
    }
}
