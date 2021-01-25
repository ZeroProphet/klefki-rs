use crate::algebra::fields::prime;
use std::ops::Add;


#[allow(non_snake_case)]
pub struct CurveParams<F> {
    pub A: prime::PrimeField<F>,
    pub B: prime::PrimeField<F>
}

pub struct CurvePoint<F> {
    pub x: prime::PrimeField<F>,
    pub y: prime::PrimeField<F>
}

pub trait Curve<F, T> {
    fn new(x: prime::PrimeField<F>, y: prime::PrimeField<F>) -> Box<dyn EllipticCurveGroupProperty<F, T>>;
    fn add(a: CurvePoint<F>, b: CurvePoint<F>) -> Box<dyn EllipticCurveGroupProperty<F, T>>;
}

pub trait EllipticCurveGroupProperty<F, T>
where T: Curve<F, T>, F: prime::New
{
    fn point(&self) -> CurvePoint<F>;
}

pub type EllipticCurveGroup<F, T> = Box<dyn EllipticCurveGroupProperty<F, T>>;

impl <F, T> Add for EllipticCurveGroup<F, T>
    where T: Curve<F, T>, F: prime::New
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return T::add(self.point(), rhs.point())
    }
}
