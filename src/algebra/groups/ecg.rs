use crate::algebra::fields::prime;
use crate::algebra::groups::arithmetic::double_and_add_algorithm;
use crate::algebra::traits::Group;
use crate::algebra::traits::Scalar;
use num::traits::Num;
use num::traits::Zero;
use num_bigint::BigUint;
use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Neg;
use std::ops::{Add, Sub};

pub trait FromBigUint<F, G> {
    fn from(x: prime::PrimeField<F>, y: prime::PrimeField<F>) -> Box<dyn CurvePoint<F, G>>;
}

pub trait Op<F, G> {
    fn op(a: Box<dyn CurvePoint<F, G>>, b: Box<dyn CurvePoint<F, G>>) -> Box<dyn CurvePoint<F, G>>;
}

pub trait CurvePoint<F, G>: fmt::Debug
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    fn x(&self) -> prime::PrimeField<F>;
    fn y(&self) -> prime::PrimeField<F>;
}

pub type EllipticCurveGroup<F, G> = Box<dyn CurvePoint<F, G>>;

impl<F, G> Add for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return G::op(self, rhs);
    }
}

impl<F, G> Zero for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    fn zero() -> Self {
        return G::from(
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
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    type Output = Self;
    fn neg(self) -> Self {
        return G::from(self.x(), -self.y());
    }
}

impl<F, G> Sub for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return self + (-rhs);
    }
}

impl<F, G> PartialEq for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
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
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
}

impl<F, G> Group for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
}

impl<F, G> From<(u32, u32)> for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    fn from(v: (u32, u32)) -> Self {
        return G::from(F::from(BigUint::from(v.0)), F::from(BigUint::from(v.1)));
    }
}

impl<F, G> From<(u16, u16)> for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    fn from(v: (u16, u16)) -> Self {
        return G::from(F::from(BigUint::from(v.0)), F::from(BigUint::from(v.1)));
    }
}

impl<F, G> From<(&[u32], &[u32])> for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    fn from(v: (&[u32], &[u32])) -> Self {
        return G::from(
            F::from(BigUint::from_slice(v.0)),
            F::from(BigUint::from_slice(v.1)),
        );
    }
}

impl<F, G> From<(BigUint, BigUint)> for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    fn from(v: (BigUint, BigUint)) -> Self {
        return G::from(F::from(v.0), F::from(v.1));
    }
}

impl<F, G> TryFrom<(&str, &str)> for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    type Error = <BigUint as Num>::FromStrRadixErr;
    fn try_from(v: (&str, &str)) -> Result<Self, Self::Error> {
        let x = BigUint::from_str_radix(v.0, 10)?;
        let y = BigUint::from_str_radix(v.1, 10)?;
        return Ok(Self::from((x, y)));
    }
}

impl<F, G> Clone for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    fn clone(&self) -> Self {
        return G::from(self.x().clone(), self.y().clone());
    }
}

impl<F, G> Scalar<BigUint> for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    type Output = Self;
    fn scalar(self, u: BigUint) -> Self {
        return double_and_add_algorithm(u, self.clone(), Self::zero());
    }
}

impl<F, G> Scalar<usize> for EllipticCurveGroup<F, G>
where
    G: FromBigUint<F, G> + Op<F, G>,
    F: prime::FromBigUint,
{
    type Output = Self;
    fn scalar(self, u: usize) -> Self {
        return double_and_add_algorithm(BigUint::from(u), self.clone(), Self::zero());
    }
}
