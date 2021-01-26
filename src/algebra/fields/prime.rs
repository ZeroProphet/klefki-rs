use crate::algebra::fields::arithmetic::extended_euclidean_algorithm;
use crate::algebra::traits::{Field, Group, MulInv, Ring};
use num::traits::{One, Zero};
use num_bigint::BigUint;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::ops::Neg;
use std::ops::{Add, Div, Mul, Sub};

pub trait FromBigUint {
    fn from(value: BigUint) -> PrimeField<Self>;
}

pub trait PrimeFieldEle<T>: Debug {
    fn prime(&self) -> BigUint;
    fn value(&self) -> BigUint;
}

pub type PrimeField<T> = Box<dyn PrimeFieldEle<T>>;

impl<T> Add for PrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return T::from((self.value() + rhs.value()) % self.prime());
    }
}

impl<T> Zero for PrimeField<T>
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

impl<T> One for PrimeField<T>
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

impl<T> Mul for PrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        return T::from((self.value() * rhs.value()) % self.prime());
    }
}

impl<T> Neg for PrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn neg(self) -> Self {
        return T::from(self.prime() - self.value());
    }
}

impl<T> Sub for PrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return self + (-rhs);
    }
}

impl<T> Div for PrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        return self * rhs.mul_inv();
    }
}

impl<T> MulInv for PrimeField<T>
where
    T: FromBigUint,
{
    type Output = Self;
    fn mul_inv(self) -> Self {
        let (_gcd, x, _y) =
            extended_euclidean_algorithm(self.value().clone(), self.prime().clone());
        return T::from(x);
    }
}

impl<T> PartialEq for PrimeField<T>
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

impl<T> Eq for PrimeField<T> where T: FromBigUint {}
impl<T> Group for PrimeField<T> where T: FromBigUint {}
impl<T> Ring for PrimeField<T> where T: FromBigUint {}
impl<T> Field for PrimeField<T> where T: FromBigUint {}

impl<T> Clone for PrimeField<T>
where
    T: FromBigUint,
{
    fn clone(&self) -> Self {
        return T::from(self.value().clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::fields::prime::FromBigUint;
    use crate::algebra::fields::prime::PrimeField;
    use crate::algebra::fields::prime::PrimeFieldEle;
    use num_bigint::BigUint;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Secp256k1FieldEle {
        pub value: BigUint,
    }

    impl FromBigUint for Secp256k1FieldEle {
        fn from(value: BigUint) -> PrimeField<Secp256k1FieldEle> {
            return (box Self { value: value }) as PrimeField<Secp256k1FieldEle>;
        }
    }

    impl PrimeFieldEle<Secp256k1FieldEle> for Secp256k1FieldEle {
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
    type Secp256k1FinateField = Box<dyn PrimeFieldEle<Secp256k1FieldEle>>;

    impl From<u16> for Secp256k1FinateField {
        fn from(v: u16) -> Self {
            return FromBigUint::from(BigUint::from(v));
        }
    }

    #[test]
    fn ff_add() {
        let a = Secp256k1FinateField::from(1u16);
        let b = Secp256k1FinateField::from(2u16);
        let c = Secp256k1FinateField::from(3u16);
        assert_eq!(a + b == c, true);
    }
}
