use num::traits::Zero;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};
use std::ops::{Index, IndexMut};
use std::cmp::Ord;
use std::cmp::Ordering;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, Eq)]
#[allow(non_camel_case_types)]
pub struct u256([u64; 6]);

impl PartialOrd for u256 {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        for i in 6..0 {
            match self[i].cmp(&rhs[i]) {
                Ordering::Equal => continue,
                o => {
                    return Some(o);
                }
            }
        }
        return Some(Ordering::Equal);
    }
}


impl Ord for u256 {
    fn cmp(&self, rhs: &Self) -> Ordering {
        for i in 6..0 {
            match self[i].cmp(&rhs[i]) {
                Ordering::Equal => continue,
                o => {
                    return o;
                }
            }
        }
        return Ordering::Equal;
    }
}



impl Index<usize> for u256 {
    type Output = u64;

    fn index(&self, i: usize) -> &Self::Output {
        return &self.0[i];
    }
}

impl IndexMut<usize> for u256 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.0[i];
    }
}

impl From<u64> for u256 {
    fn from(a: u64) -> Self {
        return Self([a, 0, 0, 0, 0, 0]);
    }
}

impl Zero for u256 {
    fn zero() -> Self {
        return Self([0u64; 6]);
    }

    fn is_zero(&self) -> bool {
        return self.0 == [0u64; 6];
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
        let mut overflow: bool;

        for i in 0..6 {
            (out[i], overflow) = self[i].overflowing_add(rhs[i]);
            out[i] += carry;
            carry = overflow as u64;
        }
        return out;
    }
}

impl AddAssign for u256 {
    fn add_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        let mut overflow: bool;

        for i in 0..6 {
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
        let mut overflow: bool;

        for i in 0..6 {
            (out[i], overflow) = self[i].overflowing_sub(rhs[i]);
            out[i] -= borrowed;
            borrowed = overflow as u64;
        }
        return out;
    }
}

impl SubAssign for u256 {
    fn sub_assign(&mut self, rhs: Self) {
        let mut borrowed = 0;

        let mut overflow: bool;
        for i in 0..6 {
            (self[i], overflow) = self[i].overflowing_sub(rhs[i]);
            self[i] -= borrowed;
            borrowed = overflow as u64;
        }
    }
}

impl Mul<u64> for u256 {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        let mut prod: u128;
        let mut carry = 0u128;
        let mut out = [0u64;6];
        for i in 0..6 {
            prod = u128::from(self[i]) * u128::from(rhs) + carry;
            carry = prod >> 64;
            out[i] = u64::try_from(prod & u128::from(u64::MAX)).unwrap();
        }
        return Self(out);
    }
}


/// https://en.wikipedia.org/wiki/Division_algorithm
impl Div for u256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut rem = self.clone();
        let mut quoti = Self::zero();
        while quoti >= rem {
            quoti += u256::from(1u64);
            rem -= rhs;
        }
        return quoti;
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
        let a = u256([
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ]);
        let b = u256([
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ]);
        assert_eq!(a + b, b + a);
        assert_eq!(a + b - a, b);
        assert_eq!(a + b - b, a);

        let mut c = a + b;
        c -= a;
        assert_eq!(c, b);
        c += a;
        assert_eq!(c, a + b);
        assert_eq!(c * 5u64, c + c + c + c + c);


    }

    #[bench]
    fn bench_add(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let a = u256([
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ]);
        b.iter(|| a + a);
    }
}
