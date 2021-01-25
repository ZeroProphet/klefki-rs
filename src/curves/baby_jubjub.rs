use crate::algebra::fields::prime;
use crate::algebra::fields::prime::New;
use crate::algebra::groups::ecg;
use num_bigint::BigUint;
use num::One;
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

impl From<u32> for BabyJubJubField {
    fn from(v: u32) -> Self {
        return BabyJubJubFieldEle::new(BigUint::from(v))
    }
}

pub type BabyJubJubField = Box<dyn prime::PrimeFieldProperty<BabyJubJubFieldEle>>;

pub struct BabyJubJubCurve {
    x: BabyJubJubField,
    y: BabyJubJubField
}

pub type BabyJubJubCurveGroup = dyn ecg::CurvePoint<BabyJubJubFieldEle, BabyJubJubCurve>;

impl ecg::Curve<BabyJubJubFieldEle, BabyJubJubCurve> for BabyJubJubCurve {
    fn new(x: BabyJubJubField, y: BabyJubJubField) -> Box<BabyJubJubCurveGroup> {
        return box BabyJubJubCurve {
            x: x,
            y: y
        } as Box<dyn ecg::CurvePoint<BabyJubJubFieldEle, BabyJubJubCurve>>;
    }
    fn op(a: Box<BabyJubJubCurveGroup>, b: Box<BabyJubJubCurveGroup>) -> Box<BabyJubJubCurveGroup> {
        let par_a = BabyJubJubField::from(BABY_JUBJUB_A);
        let m = par_a.clone() * a.x() * b.x() * a.y() * b.y();
        let x3 = (a.x() * b.y() + a.y() * b.x()) / (BabyJubJubField::one() + m.clone());
        let y3 = (a.y() * b.y() - par_a * a.x() * b.x()) / (BabyJubJubField::one() - m);
        return box BabyJubJubCurve {
            x: x3,
            y: y3
        };
    }
}

impl ecg::CurvePoint<BabyJubJubFieldEle, BabyJubJubCurve> for BabyJubJubCurve {
    fn x(&self) -> BabyJubJubField {
        return self.x.clone()
    }
    fn y(&self) -> BabyJubJubField {
        return self.y.clone()
    }
}
