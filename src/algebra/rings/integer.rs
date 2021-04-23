use std::ops::{Add, Div, Mul, Sub};
use std::convert::TryFrom;

/// U256 on littleEdian
pub struct U256([u64;6]);

impl Add for U256 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut carry = 0;
        let mut sum: u128;
        let mut out = [0u64;6];
        for i in 0..6 {
            sum = u128::from(self.0[i]) + u128::from(rhs.0[i]) + u128::from(carry);
            if sum > u64::MAX.into() {
                carry = u64::try_from(sum / u128::from(u64::MAX)).unwrap();
                sum = sum - u128::from(u64::MAX) - 1;
            }
            out[i] = u64::try_from(sum).unwrap();
        }
        return Self(out);
    }
}


#[cfg(test)]
mod tests {
    use super::U256;
    #[test]
    fn test_add() {
        let a = U256(
            [1229802880658540697,
             18446743716600914193,
             18442540718065975295,
             11029315487430344703,
             18446744018751940488,
             15]
        );
        let b = U256(
            [2459603411396185600,
             2305842294996419106,
             15752379989347008497,
             11029315487430344620,
             18446744018751940488,
             271]
        );
        let sum = a + b;
        assert_eq!(sum.0,
                   [3689406292054726297,
                    2305841937887781683,
                    15748176633703432177,
                    3611886901151137708,
                    18446743963794329361,
                    287]
        );
    }
}
