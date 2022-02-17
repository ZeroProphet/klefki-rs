use crate::algebra::fields::prime::BoxedPrimeField;
use crate::algebra::fields::prime::FromBigUint;
use crate::algebra::fields::prime::Property;
use num_bigint::BigUint;
use std::convert::TryFrom;

const SECP256K1_P : [u32; 8] = [
    0xfffffc2fu32,
    0xfffffffeu32,
    0xffffffffu32,
    0xffffffffu32,
    0xffffffffu32,
    0xffffffffu32,
    0xffffffffu32,
    0xffffffffu32,
];

#[derive(Debug, Eq, PartialEq, Clone)]
struct Secp256k1FieldEle {
    pub value: BigUint,
}

impl Property<Secp256k1FieldEle> for Secp256k1FieldEle {
    fn prime(&self) -> BigUint {
        BigUint::from_slice(&SECP256K1_P)
    }
    fn value(&self) -> BigUint {
        return self.value.clone();
    }
}

impl FromBigUint for Secp256k1FieldEle {
    fn from(value: BigUint) -> BoxedPrimeField<Secp256k1FieldEle> {
        return (box Self { value: value }) as BoxedPrimeField<Secp256k1FieldEle>;
    }
}


type Secp256k1FinateField = Box<dyn Property<Secp256k1FieldEle>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ff_add_and_sub() {
        let a = Secp256k1FinateField::from(1u16);
        let b = Secp256k1FinateField::from(2u16);
        let c = Secp256k1FinateField::from(3u16);
        assert_eq!(a.clone() + b.clone() == c.clone(), true);
        assert_eq!(c.value(), BigUint::from(3u16));
        assert_eq!(c.clone() - a.clone() == b.clone(), true);
    }

    #[test]
    fn ff_mul_and_div() {
        let a = Secp256k1FinateField::from(1u16);
        let b = Secp256k1FinateField::from(2u16);
        let c = Secp256k1FinateField::from(2u16);
        assert_eq!(a.clone() * b.clone() == c.clone(), true);
        assert_eq!(c.clone() / a.clone() == b.clone(), true);
    }

    #[test]
    fn ff_from() {
        let a = "2626589144620713026669568689430873010625803728049924121243784502389097019475";
        let b: [u32; 8] = [
            3575560275, 1167457983, 588660917, 3001516614, 1119721974, 3807046053, 453375103,
            97425606,
        ];
        let fa = Secp256k1FinateField::try_from(a).unwrap();
        let fb = Secp256k1FinateField::from(&b[..]);
        assert_eq!(fa == fb, true);
    }
}
