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

// fn short_weierstrass_form_curve_op<T, F: prime::PrimeField<T>> (
//     u1: F,
//     v1: F,
//     u2: F,
//     v2: F,
//     a1: F,
//     a2: F,
//     a3: F,
//     a4: F,
//     a6: F
// ) -> (F, F) {
//     if u1 == F::zero() && v1 == F::zero() {
//         return (u2, v2);
//     } else if u2 == F::zero() && v2 == F::zero() {
//         return (u1, v1);
//     } else if u1 == u2 {
//         if v1 != v2 {
//             return (F::zero(), F::zero());
//         } else if v1 * F::from(2u16) + u1 * a1 + a3 == F::zero() {
//             return (F::zero(), F::zero())
//         } else {
//             let lam = (
//                 pow(u1.value(), 2) * 3u16
//                     + u1.value() * a2.value() * 2u16
//                     - v1.value() * a1.value() + a4.value()
//             ) / (
//                 v1.value() * 2u16 + u1.value() * a1.value() + a3.value()
//             );
//             let u3 = pow(lam, 2) + pow(lam, a1.value()) - a2.value() - u2.value() * 2u16;
//             let v3 = lam * (u1.value() - u3.value()) - v1.value() - a1.value() * u3.value() - a3;
//             return (u3, v3);
//         }
//     } else {
//         let lam = (v1 - v2) / (u1-u2);
//         let u3 = lam ** 2 + a1 * lam - a2 - u1 - u2;
//         let v3 = lam * (u1-u3) - v1 - a1 * u3 - a3;
//         return (u3, v3);
//     }
// }
