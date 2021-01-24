use crate::algebra::fields::prime;

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

pub trait EllipticCurveGroup<T> {
    fn params(&self) -> CurveParams<T>;
    fn op(&self, rhs: &Self) -> Self;
}
