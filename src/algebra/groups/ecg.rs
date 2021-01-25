use crate::algebra::fields::prime;
use crate::algebra::traits::Group;
use num::traits::Zero;
use std::cmp::{Eq, PartialEq};
use std::ops::Neg;
use std::ops::{Add, Sub};

#[allow(non_snake_case)]
pub struct CurveParams<F> {
    pub A: prime::PrimeField<F>,
    pub B: prime::PrimeField<F>,
}

pub trait Curve<F, G> {
    fn new(x: prime::PrimeField<F>, y: prime::PrimeField<F>) -> Box<dyn CurvePoint<F, G>>;

    fn op(a: Box<dyn CurvePoint<F, G>>, b: Box<dyn CurvePoint<F, G>>) -> Box<dyn CurvePoint<F, G>>;
}

pub trait CurvePoint<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
    fn x(&self) -> prime::PrimeField<F>;
    fn y(&self) -> prime::PrimeField<F>;
}

pub type EllipticCurveGroup<F, G> = Box<dyn CurvePoint<F, G>>;

impl<F, G> Add for EllipticCurveGroup<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return G::op(self, rhs);
    }
}

impl<F, G> Zero for EllipticCurveGroup<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
    fn zero() -> Self {
        return G::new(
            prime::PrimeField::<F>::zero(),
            prime::PrimeField::<F>::zero(),
        );
    }
    fn is_zero(&self) -> bool {
        return self.x() == prime::PrimeField::<F>::zero()
            && self.y() == prime::PrimeField::<F>::zero();
    }
}

impl<F, G> Neg for EllipticCurveGroup<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
    type Output = Self;
    fn neg(self) -> Self {
        return G::new(self.x(), -self.y());
    }
}

impl<F, G> Sub for EllipticCurveGroup<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return self + (-rhs);
    }
}

impl<F, G> PartialEq for EllipticCurveGroup<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
    fn eq(&self, rhs: &Self) -> bool {
        return self.x() == rhs.x() && self.y() == rhs.y();
    }

    fn ne(&self, rhs: &Self) -> bool {
        return self.x() != rhs.x() || self.y() != rhs.y();
    }
}

impl<F, G> Eq for EllipticCurveGroup<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
}

impl<F, G> Group for EllipticCurveGroup<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
}
