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

pub struct CurvePoint<F> {
    pub x: prime::PrimeField<F>,
    pub y: prime::PrimeField<F>,
}

pub trait Curve<F, G> {
    fn new(
        x: prime::PrimeField<F>,
        y: prime::PrimeField<F>,
    ) -> Box<dyn EllipticCurveGroupProperty<F, G>>;
    fn add(a: CurvePoint<F>, b: CurvePoint<F>) -> Box<dyn EllipticCurveGroupProperty<F, G>>;
}

pub trait EllipticCurveGroupProperty<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
    fn point(&self) -> CurvePoint<F>;
}

pub type EllipticCurveGroup<F, G> = Box<dyn EllipticCurveGroupProperty<F, G>>;

impl<F, G> Add for EllipticCurveGroup<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return G::add(self.point(), rhs.point());
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
        return self.point().x == prime::PrimeField::<F>::zero()
            && self.point().y == prime::PrimeField::<F>::zero();
    }
}

impl<F, G> Neg for EllipticCurveGroup<F, G>
where
    G: Curve<F, G>,
    F: prime::New,
{
    type Output = Self;
    fn neg(self) -> Self {
        return G::new(self.point().x, -self.point().y);
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
        return self.point().x == rhs.point().x && self.point().y == rhs.point().y;
    }

    fn ne(&self, rhs: &Self) -> bool {
        return self.point().x != rhs.point().x || self.point().y != rhs.point().y;
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
