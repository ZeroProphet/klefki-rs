/// DOC: https://eips.ethereum.org/EIPS/eip-2494
use crate::algebra::groups::ecg;
use crate::curves::baby_jubjub::ff;
use num::{One, Zero};

pub const BABY_JUBJUB_A: u32 = 168700u32;
pub const BABY_JUBJUB_B: u32 = 168696u32;

#[derive(Debug, Clone)]
pub struct BabyJubJubCurve {
    pub x: ff::BabyJubJubField,
    pub y: ff::BabyJubJubField,
}

pub type BabyJubJubCurveGroup = Box<dyn ecg::CurvePoint<ff::BabyJubJubFieldEle, BabyJubJubCurve>>;

impl ecg::FromBigUint<ff::BabyJubJubFieldEle, BabyJubJubCurve> for BabyJubJubCurve {
    fn from(x: ff::BabyJubJubField, y: ff::BabyJubJubField) -> BabyJubJubCurveGroup {
        return box BabyJubJubCurve { x: x, y: y }
            as Box<dyn ecg::CurvePoint<ff::BabyJubJubFieldEle, BabyJubJubCurve>>;
    }
}

impl ecg::Op<ff::BabyJubJubFieldEle, BabyJubJubCurve> for BabyJubJubCurve {
    fn op(a: BabyJubJubCurveGroup, b: BabyJubJubCurveGroup) -> BabyJubJubCurveGroup {
        if a == BabyJubJubCurveGroup::zero() {
            return b;
        }
        if b == BabyJubJubCurveGroup::zero() {
            return a;
        }
        let m = ff::BabyJubJubField::from(BABY_JUBJUB_B) * a.x() * b.x() * a.y() * b.y();
        let x3 = (a.x() * b.y() + a.y() * b.x()) / (ff::BabyJubJubField::one() + m.clone());
        let y3 = (a.y() * b.y() - ff::BabyJubJubField::from(BABY_JUBJUB_A) * a.x() * b.x())
            / (ff::BabyJubJubField::one() - m);
        return box BabyJubJubCurve { x: x3, y: y3 };
    }
}

impl ecg::CurvePoint<ff::BabyJubJubFieldEle, BabyJubJubCurve> for BabyJubJubCurve {
    fn x(&self) -> ff::BabyJubJubField {
        return self.x.clone();
    }
    fn y(&self) -> ff::BabyJubJubField {
        return self.y.clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::traits::Scalar;
    use crate::curves::baby_jubjub::curve;
    use num_bigint::BigUint;
    use std::convert::TryFrom;

    #[test]
    fn test_addition() {
        let x1 = "17777552123799933955779906779655732241715742912184938656739573121738514868268";
        let y1 = "2626589144620713026669568689430873010625803728049924121243784502389097019475";
        let x2 = "16540640123574156134436876038791482806971768689494387082833631921987005038935";
        let y2 = "20819045374670962167435360035096875258406992893633759881276124905556507972311";
        let x3 = "7916061937171219682591368294088513039687205273691143098332585753343424131937";
        let y3 = "14035240266687799601661095864649209771790948434046947201833777492504781204499";

        let g1 = curve::BabyJubJubCurveGroup::try_from((x1, y1)).unwrap();
        let g2 = curve::BabyJubJubCurveGroup::try_from((x2, y2)).unwrap();
        let g3 = curve::BabyJubJubCurveGroup::try_from((x3, y3)).unwrap();
        assert_eq!(g1 + g2 == g3, true);
    }

    #[test]
    fn test_double() {
        let x1 = "17777552123799933955779906779655732241715742912184938656739573121738514868268";
        let y1 = "2626589144620713026669568689430873010625803728049924121243784502389097019475";
        let x3 = "6890855772600357754907169075114257697580319025794532037257385534741338397365";
        let y3 = "4338620300185947561074059802482547481416142213883829469920100239455078257889";
        let g1 = curve::BabyJubJubCurveGroup::try_from((x1.clone(), y1.clone())).unwrap();
        let g2 = curve::BabyJubJubCurveGroup::try_from((x1, y1)).unwrap();
        let g3 = curve::BabyJubJubCurveGroup::try_from((x3, y3)).unwrap();
        assert_eq!(g1.clone() + g2 == g3.clone(), true);
        assert_eq!(g1.scalar(2) == g3, true);
    }

    #[test]
    fn test_doubling_the_identity() {
        let x1 = 0u32;
        let y1 = 1u32;
        let x2 = 0u32;
        let y2 = 1u32;
        let x3 = 0u32;
        let y3 = 1u32;
        let g1 = curve::BabyJubJubCurveGroup::from((x1, y1));
        let g2 = curve::BabyJubJubCurveGroup::from((x2, y2));
        let g3 = curve::BabyJubJubCurveGroup::from((x3, y3));
        assert_eq!(g1 + g2 == g3, true);
    }

    #[test]
    fn test_base_point() {
        let bx = "5299619240641551281634865583518297030282874472190772894086521144482721001553";
        let by = "16950150798460657717958625567821834550301663161624707787222815936182638968203";
        let gx = "995203441582195749578291179787384436505546430278305826713579947235728471134";
        let gy = "5472060717959818805561601436314318772137091100104008585924551046643952123905";
        let g = curve::BabyJubJubCurveGroup::try_from((gx, gy)).unwrap();
        let b = curve::BabyJubJubCurveGroup::try_from((bx, by)).unwrap();
        assert_eq!(g.scalar(8) == b, true);
    }

    #[test]
    fn test_base_point_order() {
        let x = 0u32;
        let y = 1u32;
        let bx = "5299619240641551281634865583518297030282874472190772894086521144482721001553";
        let by = "16950150798460657717958625567821834550301663161624707787222815936182638968203";
        let l = "2736030358979909402780800718157159386076813972158567259200215660948447373041";

        let p = curve::BabyJubJubCurveGroup::from((x, y));
        let b = curve::BabyJubJubCurveGroup::try_from((bx, by)).unwrap();

        assert_eq!(
            b.scalar(BigUint::parse_bytes(l.as_bytes(), 10).unwrap()) == p,
            true
        );
    }
}
