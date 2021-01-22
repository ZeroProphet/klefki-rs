use num_bigint::BigUint;

pub fn extended_euclidean_algorithm(a: BigUint, b: BigUint) -> (BigUint, BigUint, BigUint) {
    let (mut s, mut t, mut r) = (BigUint::from(0u16), BigUint::from(1u16), b);
    let (mut old_s, mut old_t, mut old_r) = (BigUint::from(1u16), BigUint::from(0u16), a);
    let mut quoient: BigUint;
    while r != BigUint::from(0u16) {
        quoient = &old_r / &r;
        old_r = r.clone();
        r = &old_r - &quoient * r;
        old_s = s.clone();
        s = &old_s - &quoient * s;
        old_t = t.clone();
        t = quoient * t;
    }
    return (old_r, old_s, old_t);
}
