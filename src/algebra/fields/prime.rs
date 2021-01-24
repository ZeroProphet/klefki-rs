use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;
use std::cmp::{Eq, PartialEq};
use std::ops::Neg;
use num_bigint::BigUint;
use crate::algebra::traits::{Group, Ring, Field, MulInv};
use crate::algebra::fields::arithmetic::extended_euclidean_algorithm;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PrimeFieldIns {
    pub prime: BigUint,
    pub value: BigUint
}

pub trait PrimeFieldProperty: Debug {
    fn prime(&self) -> BigUint;
    fn value(&self) -> BigUint;
}

pub type PrimeField = Box<dyn PrimeFieldProperty>;

impl PrimeFieldProperty for PrimeFieldIns {
    fn prime(&self) -> BigUint {
        return self.prime.clone();
    }
    fn value(&self) -> BigUint {
        return self.value.clone();
    }
}

impl PartialEq for PrimeField {
    fn eq(&self, rhs: &Self) -> bool {
        return self.value() == rhs.value();
    }

    fn ne(&self, rhs: &Self) -> bool {
        return self.value() != rhs.value();
    }
}

impl Eq for PrimeField {}


impl Add for PrimeField {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return Box::new(PrimeFieldIns {
            prime: self.prime().clone(),
            value: (self.value() + rhs.value()) % self.prime()
        });
    }
}

impl Mul for PrimeField {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        return box PrimeFieldIns {
            prime: self.prime().clone(),
            value: (self.value() * rhs.value()) % rhs.prime()
        }
    }
}

impl Neg for PrimeField {
    type Output = Self;
    fn neg(self) -> Self {
        return box PrimeFieldIns {
            prime: self.prime().clone(),
            value: self.prime() - self.value()
        }
    }
}


impl Sub for PrimeField {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return self + (-rhs)
    }
}


impl Div for PrimeField {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        return self * rhs.mul_inv();
    }
}

impl MulInv for PrimeField {
    type Output = Self;
    fn mul_inv(self) -> Self {
        let (_gcd, x, _y) = extended_euclidean_algorithm(self.value().clone(), self.prime().clone());
        return box PrimeFieldIns {
            prime: self.prime().clone(),
            value: x
        }
    }
}

impl Group for PrimeField {}
impl Ring for PrimeField {}
impl Field  for PrimeField {}


#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use crate::algebra::fields::prime::PrimeField;
    use crate::algebra::fields::prime::PrimeFieldProperty;


    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Secp256k1Field {
        pub value: BigUint
    }

    impl PrimeFieldProperty for Secp256k1Field {
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

    impl Secp256k1Field {
        fn new(v: BigUint) -> PrimeField {
            return (box Self {
                value: v
            }) as PrimeField
        }
    }

    #[test]
    fn ff_add() {
        let a = Secp256k1Field::new(BigUint::from(1u16));
        let b = Secp256k1Field::new(BigUint::from(2u16));
        let c = Secp256k1Field::new(BigUint::from(3u16));
        assert_eq!(a + b == c, true);
    }
}
