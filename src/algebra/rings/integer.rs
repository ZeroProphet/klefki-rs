use std::ops::{Add, Div, Mul, Sub};
use num::traits::{One, Zero};
use std::convert::TryFrom;
use std::cmp::Eq;


#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct u256([u64;6]);

impl Zero for u256 {
    fn zero() -> Self {
        return Self([0u64;6]);
    }

    fn is_zero(&self) -> bool {
        return self.0 == [0u64;6];
    }
}

// impl One for u256 {
//     fn one() -> Self {
//         Self([[1u64], [0u64;6]].concat());
//     }
//     fn is_one(&self) {
//         return self.0 == [[1u64], [0u64;6]].concat();
//     }
// }


impl PartialEq for u256 {
    fn eq(&self, rhs: &Self) -> bool {
        return self.0 == rhs.0;
    }
}

impl Add for u256 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut out = [0u64;6];
        let mut borrowed = 0;
        for i in 0 .. 6 {
            (|(sum, overflow)| {
                out[i] = sum + borrowed;
                borrowed = overflow as u64;

            })(self.0[i].overflowing_add(rhs.0[i]))
        }
        return Self(out);
    }
}

impl Sub for u256 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut out = [0u64;6];
        let mut borrowed = 0;
        for i in 0 .. 6 {
            (|(delta, overflow)| {
                out[i] = delta - borrowed;
                borrowed = overflow as u64;

            })(self.0[i].overflowing_sub(rhs.0[i]))
        }
        return Self(out)
    }
}


#[cfg(test)]
mod tests {
    extern crate test;

    use super::u256;
    use rand::Rng;
    use test::Bencher;

    #[test]
    fn test_add_sub() {
        let mut rng = rand::thread_rng();
        let a = u256(
            [
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen()
            ]
        );
        let b = u256(
            [
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen()
            ]
        );
        assert_eq!(a + b, b + a);
        assert_eq!(a + b - a, b);
        assert_eq!(a + b - b, a);
    }


    #[bench]
    fn bench_add(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let a = u256(
            [
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen()
            ]
        );
        b.iter(|| a + a);
    }
}
