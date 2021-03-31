use crate::algebra::fields::arithmetic::extended_euclidean_algorithm;
use crate::algebra::traits::{Field, Group, MulInv, Ring};
use num::traits::Num;
use num::traits::{One, Zero};
use num_bigint::BigInt;
use num_bigint::BigUint;
use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::Neg;
use std::ops::{Add, Div, Mul, Sub};

pub trait FromBigUint {
    fn from(value: BigUint) -> BoxedPrimeField<Self>;
}

pub trait PrimeField<T>: Debug {
    fn prime(&self) -> BigUint;
    fn value(&self) -> BigUint;
}

pub type BoxedPrimeField<T> = Box<dyn PrimeField<T>>;

impl<T> Add for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return T::from((self.value() + rhs.value()) % self.prime());
    }
}

impl<T> Zero for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    fn zero() -> Self {
        return T::from(BigUint::from(0u32));
    }
    fn is_zero(&self) -> bool {
        return self.value() == BigUint::from(0u32);
    }
}

impl<T> One for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    fn one() -> Self {
        return T::from(BigUint::from(1u32));
    }
    fn is_one(&self) -> bool {
        return self.value() == BigUint::from(1u32);
    }
}

impl<T> Mul for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        return T::from((self.value() * rhs.value()) % self.prime());
    }
}

impl<T> Neg for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn neg(self) -> Self {
        return T::from(self.prime() - self.value());
    }
}

impl<T> Sub for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return self + (-rhs);
    }
}

impl<T> Div for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        return self * rhs.mul_inv();
    }
}

impl<T> MulInv for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn mul_inv(self) -> Self {
        let m = BigInt::from(self.prime());
        let (_gcd, x, _y) =
            extended_euclidean_algorithm(self.value().clone(), self.prime().clone());
        return T::from(BigUint::try_from((x % m.clone() + m.clone()) % m.clone()).unwrap());
    }
}

impl<T> PartialEq for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    fn eq(&self, rhs: &Self) -> bool {
        return self.value() == rhs.value();
    }

    fn ne(&self, rhs: &Self) -> bool {
        return self.value() != rhs.value();
    }
}

impl<T> Eq for BoxedPrimeField<T> where T: FromBigUint {}
impl<T> Group for BoxedPrimeField<T> where T: FromBigUint {}
impl<T> Ring for BoxedPrimeField<T> where T: FromBigUint {}
impl<T> Field for BoxedPrimeField<T> where T: FromBigUint {}

impl<T> Clone for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    fn clone(&self) -> Self {
        return T::from(self.value().clone());
    }
}

impl<T> From<BigUint> for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    fn from(v: BigUint) -> Self {
        return T::from(v);
    }
}

impl<T> From<u32> for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    fn from(v: u32) -> Self {
        return T::from(BigUint::from(v));
    }
}

impl<T> From<u16> for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    fn from(v: u16) -> Self {
        return T::from(BigUint::from(v));
    }
}

impl<T> From<&[u32]> for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    fn from(v: &[u32]) -> Self {
        return T::from(BigUint::from_slice(v));
    }
}

impl<T> TryFrom<&str> for BoxedPrimeField<T>
where
    T: FromBigUint,
{
    type Error = <BigUint as Num>::FromStrRadixErr;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        return Ok(T::from(BigUint::from_str_radix(s, 10)?));
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::fields::prime::FromBigUint;
    use crate::algebra::fields::prime::PrimeField;
    use crate::algebra::fields::prime::BoxedPrimeField;
    use num_bigint::BigUint;
    use std::convert::TryFrom;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Secp256k1FieldEle {
        pub value: BigUint,
    }

    impl FromBigUint for Secp256k1FieldEle {
        fn from(value: BigUint) -> BoxedPrimeField<Secp256k1FieldEle> {
            return (box Self { value: value }) as BoxedPrimeField<Secp256k1FieldEle>;
        }
    }

    impl PrimeField<Secp256k1FieldEle> for Secp256k1FieldEle {
        fn prime(&self) -> BigUint {
            return BigUint::from_slice(&[
                0xfffffc2fu32,
                0xfffffffeu32,
                0xffffffffu32,
                0xffffffffu32,
                0xffffffffu32,
                0xffffffffu32,
                0xffffffffu32,
                0xffffffffu32,
            ]);
        }
        fn value(&self) -> BigUint {
            return self.value.clone();
        }
    }
    type Secp256k1FinateField = Box<dyn PrimeField<Secp256k1FieldEle>>;

    #[test]
    fn ff_add_and_sub() {
        let a = Secp256k1FinateField::from(1u16);
        let b = Secp256k1FinateField::from(2u16);
        let c = Secp256k1FinateField::from(3u16);
        assert_eq!(a.clone() + b.clone() == c.clone(), true);
        assert_eq!(c.value(), BigUint::from(3u16));
        assert_eq!(c.clone() - a.clone() == b.clone(), true);
    }

    #[test]
    fn ff_mul_and_div() {
        let a = Secp256k1FinateField::from(1u16);
        let b = Secp256k1FinateField::from(2u16);
        let c = Secp256k1FinateField::from(2u16);
        assert_eq!(a.clone() * b.clone() == c.clone(), true);
        assert_eq!(c.clone() / a.clone() == b.clone(), true);
    }

    #[test]
    fn ff_from() {
        let a = "2626589144620713026669568689430873010625803728049924121243784502389097019475";
        let b: [u32; 8] = [
            3575560275, 1167457983, 588660917, 3001516614, 1119721974, 3807046053, 453375103,
            97425606,
        ];
        let fa = Secp256k1FinateField::try_from(a).unwrap();
        let fb = Secp256k1FinateField::from(&b[..]);
        assert_eq!(fa == fb, true);
    }
}
