use std::vec::Vec;
use std::ops::Add;
use std::fmt::Debug;
use num::traits::{Zero, One};
use num_bigint::BigUint;

pub fn double_and_add_algorithm<T>(times: BigUint, x: T, init: T) -> T
    where T: Sized + Add<Output=T> + Clone + Debug
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
    // [0, 1]
    for bit in bits(times) {
        if bit == BigUint::one() {
            // ret = add + 0
            result = added.clone() + result;
        }
        // x = x + x
        added = added.clone() + added.clone();

    }
    return result;
}
