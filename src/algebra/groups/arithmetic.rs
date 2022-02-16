use num::traits::{One, Zero};
use num_bigint::BigUint;
use std::fmt::Debug;
use std::ops::Add;
use std::vec::Vec;
use num::pow;
use crate::algebra::fields::prime;


pub fn double_and_add_algorithm<T>(times: BigUint, x: T, init: T) -> T
where
    T: Sized + Add<Output = T> + Clone + Debug,
{
    fn bits(mut n: BigUint) -> Vec<BigUint> {
        let mut ret = Vec::<BigUint>::new();
        while n != BigUint::zero() {
            ret.push(n.clone() & BigUint::one());
            n >>= 1;
        }
        return ret;
    }

    let mut result = init;
    let mut added = x;
    for bit in bits(times) {
        if bit == BigUint::one() {
            result = added.clone() + result;
        }
        added = added.clone() + added;
    }
    return result;
}

pub fn short_weierstrass_form_curve_op<T, F: prime::PrimeField<T>> (
    u1: F,
    v1: F,
    u2: F,
    v2: F,
    a1: F,
    a2: F,
    a3: F,
    a4: F,
) -> (F, F) {
    if u1 == F::zero() && v1 == F::zero() {
        return (u2, v2);
    } else if u2 == F::zero() && v2 == F::zero() {
        return (u1, v1);
    } else if u1 == u2 {
        if v1 != v2 {
            return (F::zero(), F::zero());
        } else if v1.clone() * F::from(2u16) + u1.clone() * a1.clone() + a3.clone() == F::zero() {
            return (F::zero(), F::zero())
        } else {
            let lam = (
                pow(u1.clone().value(), 2) * 3u16
                    + u1.value() * a2.value() * 2u16
                    - v1.clone().value() * a1.clone().value() + a4.value()
            ) / (
                v1.value() * 2u16 + u1.value() * a1.value() + a3.value()
            );
            let u3 = pow(lam.clone(), 2) + lam.clone() * a1.value() - a2.value() - u2.value() * 2u16;
            let v3 = lam * (u1.value() - u3.clone()) - v1.value() - a1.value() * u3.clone() - a3.value();
            return (F::from(u3), F::from(v3));
        }
    } else {
        let lam = (v1.value() - v2.value()) / (u1.value() - u2.value());
        let u3 = pow(lam.clone(), 2) + a1.value() * lam.clone() - a2.value() - u1.value() - u2.value();
        let v3 = lam * (u1.value() - u3.clone()) - v1.value() - a1.value() * u3.clone() - a3.value();
        return (F::from(u3), F::from(v3));
    }
}
