use std::ops::{Add, Sub, AddAssign, SubAssign};
use num::traits::Zero;
use std::ops::{Index, IndexMut};


#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct u256([u64;6]);

impl Index<usize> for u256 {
    type Output = u64;

    fn index(&self, i: usize) -> &Self::Output{
        return &self.0[i]
    }
}

impl IndexMut<usize> for u256 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output{
        return &mut self.0[i]
    }
}



impl From<u64> for u256 {
    fn from(a: u64) -> Self {
        return Self([a, 0, 0, 0, 0, 0]);
    }
}

impl Zero for u256 {
    fn zero() -> Self {
        return Self([0u64;6]);
    }

    fn is_zero(&self) -> bool {
        return self.0 == [0u64;6];
    }
}

impl PartialEq for u256 {
    fn eq(&self, rhs: &Self) -> bool {
        return self.0 == rhs.0;
    }
}

impl Add for u256 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut out = Self::zero();
        let mut carry = 0;
        for i in 0 .. 6 {
            (|(sum, overflow)| {
                out[i] = sum + carry;
                carry = overflow as u64;

            })(self[i].overflowing_add(rhs[i]))
        }
        return out;
    }
}

impl AddAssign for u256 {
    fn add_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        let mut overflow: bool;

        for i in 0 .. 6 {
            (self[i], overflow) = self[i].overflowing_add(rhs[i]);
            self[i] += carry;
            carry = overflow as u64;
        }
    }
}

impl Sub for u256 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut out = Self::zero();
        let mut borrowed = 0;
        for i in 0 .. 6 {
            (|(delta, overflow)| {
                out[i] = delta - borrowed;
                borrowed = overflow as u64;

            })(self[i].overflowing_sub(rhs[i]))
        }
        return out
    }
}

impl SubAssign for u256 {
    fn sub_assign(&mut self, rhs: Self) {
        let mut borrowed = 0;
        let mut overflow: bool;
        for i in 0 .. 6 {
            (self[i], overflow) = self[i].overflowing_sub(rhs[i]);
            self[i] -= borrowed;
            borrowed = overflow as u64;
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::u256;
    use rand::Rng;
    use test::Bencher;

    #[test]
    fn test_arith() {
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

        let mut c = a + b;
        c -= a;
        assert_eq!(c, b);
        c += a;
        assert_eq!(c, a + b)
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
