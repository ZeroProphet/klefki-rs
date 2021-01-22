use num_bigint::BigUint;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::Eq;
use std::ops::Neg;
use std::cmp::PartialEq;
use crate::traits::{Group, Ring, Field, MulInv};
use crate::fields::arithmetic::extended_euclidean_algorithm;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PrimeFieldIns {
    pub prime: BigUint,
    pub value: BigUint
}

trait PrimeField {
    fn prime(&self) -> &BigUint;
    fn value(&self) -> &BigUint;
    fn set_value(&mut self, value: BigUint);
}

impl PrimeField for PrimeFieldIns {
    fn prime(&self) -> &BigUint {
        return &self.prime;
    }
    fn value(&self) -> &BigUint {
        return &self.value;
    }
    fn set_value(&mut self, value: BigUint) {
        self.value = value;
    }
}

impl PartialEq for Box<dyn PrimeField> {
    fn eq(&self, rhs: &Self) -> bool {
        return self.value() == rhs.value();
    }

    fn ne(&self, rhs: &Self) -> bool {
        return self.value() != rhs.value();
    }
}

impl Eq for Box<dyn PrimeField> {}


impl Add for Box<dyn PrimeField> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return Box::new(PrimeFieldIns {
            prime: self.prime().clone(),
            value: (self.value() + rhs.value()) % self.prime()
        });
    }
}

impl Mul for Box<dyn PrimeField> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        return box PrimeFieldIns {
            prime: self.prime().clone(),
            value: (self.value() * rhs.value()) % rhs.prime()
        }
    }
}

impl Neg for Box<dyn PrimeField> {
    type Output = Self;
    fn neg(self) -> Self {
        return box PrimeFieldIns {
            prime: self.prime().clone(),
            value: self.prime() - self.value()
        }
    }
}


impl Sub for Box<dyn PrimeField> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return self + (-rhs)
    }
}


impl Div for Box<dyn PrimeField> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        return self * rhs.mul_inv();
    }
}

impl MulInv for Box<dyn PrimeField> {
    type Output = Self;
    fn mul_inv(self) -> Self {
        let (_gcd, x, _y) = extended_euclidean_algorithm(self.value().clone(), self.prime().clone());
        return box PrimeFieldIns {
            prime: self.prime().clone(),
            value: x
        }
    }
}

impl Group for Box<dyn PrimeField> {}
impl Ring for Box<dyn PrimeField> {}
impl Field  for Box<dyn PrimeField> {}
