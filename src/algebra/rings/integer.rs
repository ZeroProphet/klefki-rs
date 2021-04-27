use std::ops::{Add, Div, Mul, Sub};
use std::convert::TryFrom;
use std::cmp::Eq;


#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct u256([u64;6]);

impl PartialEq for u256 {
    fn eq(&self, rhs: &Self) -> bool {
        return self.0 == rhs.0
    }
}

impl Add for u256 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut out = [0u64;6];
        let mut borrowed = 0;

        for i in 0 .. 6 {
            (|(sum, overflow)| {
                out[i] = sum + borrowed as u64;
                borrowed = overflow as u64;

            })(self.0[i].overflowing_add(rhs.0[i]))
        }
        return Self(out);
    }
}

impl Sub for u256 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut borrow = 0;
        let mut out = [0u64;6];
        let mut delta: i128;

        for i in 0 .. 6 {
            delta = i128::from(self.0[i]) - i128::from(rhs.0[i]) - borrow;
            borrow = (delta < 0) as i128;
            out[i] = u64::try_from(delta & i128::from(u64::MAX)).unwrap();
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
