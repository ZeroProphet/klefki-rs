use num_bigint::BigUint;
use num_bigint::BigInt;
use num::traits::{One, Zero};

pub fn extended_euclidean_algorithm(a: BigUint, b: BigUint) -> (BigInt, BigInt, BigInt) {
    let (mut s, mut t, mut r) = (BigInt::zero(), BigInt::one(), BigInt::from(b));
    let (mut old_s, mut old_t, mut old_r) = (BigInt::one(), BigInt::zero(), BigInt::from(a));

    let mut quoient: BigInt;
    let mut m: BigInt;

    while r != BigInt::zero() {
        quoient = &old_r / &r;
        m = old_r.clone();
        old_r = r.clone();
        r = &m - &quoient * r;
        m = old_s.clone();
        old_s = s.clone();
        s = &m - &quoient * s;
        m = old_t.clone();
        old_t = t.clone();
        t = &m - quoient * t;
    }
    return (
        old_r,
        old_s,
        old_t,
    );
}


#[cfg(test)]
mod test {
    use num_bigint::BigUint;
    use num_bigint::BigInt;
    use num::traits::{One, Zero};
    use crate::algebra::fields::arithmetic::extended_euclidean_algorithm;
    #[test]
    fn test_modinv() {
        let a = BigUint::one();
        let b = BigUint::from(13u16);
        let (r, s, t) = extended_euclidean_algorithm(a.clone(), b.clone());
        assert_eq!(r.clone(), BigInt::one());
        assert_eq!(s.clone(), BigInt::one());
        assert_eq!(t.clone(), BigInt::zero());
    }
}
