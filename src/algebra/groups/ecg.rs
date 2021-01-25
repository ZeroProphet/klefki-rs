use crate::algebra::fields::prime;
use std::ops::{Add, Sub};


#[allow(non_snake_case)]
pub struct CurveParams<T> {
    pub A: prime::PrimeField<T>,
    pub B: prime::PrimeField<T>
}

#[allow(non_snake_case)]
pub struct CurvePoint<T> {
    pub X: prime::PrimeField<T>,
    pub Y: prime::PrimeField<T>
}

pub trait New<F> {
    fn new(point: CurvePoint<F>);
}

pub trait EllipticCurveGroupProperty<T, F> {
    fn params(&self) -> CurveParams<T>;
}

pub type EllipticCurveGroup<T, F> = Box<dyn EllipticCurveGroupProperty<T, F>>;

// impl <T, F> Add for EllipticCurveGroup<T, F>
//     where T: New<F>
// {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//     }
// }
