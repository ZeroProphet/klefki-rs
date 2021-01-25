use crate::algebra::fields::prime;
use num_bigint::BigUint;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;

pub const BABY_JUBJUB_P: [u32; 8] = [
    0xf0000001u32,
    0x43e1f593u32,
    0x79b97091u32,
    0x2833e848u32,
    0x8181585du32,
    0xb85045b6u32,
    0xe131a029u32,
    0x30644e72u32,
];

pub const BABY_JUBJUB_A: u32 = 168700u32;
pub const BABY_JUBJUB_B: u32 = 168696u32;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BabyJubJubFieldEle {
    pub value: BigUint,
}

impl prime::PrimeFieldProperty<BabyJubJubFieldEle> for BabyJubJubFieldEle {
    fn prime(&self) -> BigUint {
        return BigUint::from_slice(&BABY_JUBJUB_P);
    }
    fn value(&self) -> BigUint {
        return self.value.clone();
    }
}

impl prime::New for BabyJubJubFieldEle {
    fn new(value: BigUint) -> prime::PrimeField<BabyJubJubFieldEle> {
        return (box Self { value: value }) as prime::PrimeField<Self>;
    }
}

pub type BabyJubJubField = Box<dyn prime::PrimeFieldProperty<BabyJubJubFieldEle>>;
