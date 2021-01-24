use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;
use std::cmp::{Eq, PartialEq};
use std::ops::Neg;
use num_bigint::BigUint;
use crate::algebra::traits::{Group, Ring, Field, MulInv};
use crate::algebra::fields::arithmetic::extended_euclidean_algorithm;

pub trait New {
    fn new(value: BigUint) -> PrimeField<Self>;
}

pub trait PrimeFieldProperty<T>: Debug {
    fn prime(&self) -> BigUint;
    fn value(&self) -> BigUint;
}

pub type PrimeField<T> = Box<dyn PrimeFieldProperty<T>>;

impl <T> Add for PrimeField<T>
    where T: New
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return T::new(
            (self.value() + rhs.value()) % self.prime()
        )
    }
}

impl <T> Mul for PrimeField<T>
    where T: New
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        return T::new(
            (self.value() * rhs.value()) % self.prime()
        )
    }
}

impl <T> Neg for PrimeField<T>
    where T: New
{
    type Output = Self;
    fn neg(self) -> Self {
        return T::new(
            self.prime() - self.value()
        )
    }
}


impl <T> Sub for PrimeField<T>
    where T: New
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return self + (-rhs)
    }
}


impl <T> Div for PrimeField<T>
    where T: New
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        return self * rhs.mul_inv();
    }
}

impl <T> MulInv for PrimeField<T>
    where T: New
{
    type Output = Self;
    fn mul_inv(self) -> Self {
        let (_gcd, x, _y) = extended_euclidean_algorithm(self.value().clone(), self.prime().clone());
        return T::new (
            x
        )
    }
}

impl <T> PartialEq for PrimeField<T> {
    fn eq(&self, rhs: &Self) -> bool {
        return self.value() == rhs.value();
    }

    fn ne(&self, rhs: &Self) -> bool {
        return self.value() != rhs.value();
    }
}

impl <T> Eq for PrimeField<T> {}

impl <T> Group for PrimeField<T> where T: New{}
impl <T> Ring for PrimeField<T> where T: New {}
impl <T> Field  for PrimeField<T> where T: New {}


#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use crate::algebra::fields::prime::PrimeField;
    use crate::algebra::fields::prime::New;
    use crate::algebra::fields::prime::PrimeFieldProperty;


    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Secp256k1FieldIns {
        pub value: BigUint
    }

    impl New for Secp256k1FieldIns {
        fn new(value: BigUint) -> PrimeField<Secp256k1FieldIns> {
            return (box Self {
                value: value
            }) as PrimeField<Secp256k1FieldIns>
        }
    }


    impl PrimeFieldProperty<Secp256k1FieldIns> for Secp256k1FieldIns {
        fn prime(&self) -> BigUint {
            return BigUint::from_slice(&[
                0xfffffc2fu32,
                0xfffffffeu32,
                0xffffffffu32,
                0xffffffffu32,
                0xffffffffu32,
                0xffffffffu32,
                0xffffffffu32,
                0xffffffffu32
            ]);
        }
        fn value(&self) -> BigUint {
            return self.value.clone();
        }
    }
    type Secp256k1FinateField = Box<dyn PrimeFieldProperty<Secp256k1FieldIns>>;

    impl From<u16> for Secp256k1FinateField {
        fn from(v: u16) -> Self {
            return Secp256k1FieldIns::new(
                BigUint::from(v)
            )
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
