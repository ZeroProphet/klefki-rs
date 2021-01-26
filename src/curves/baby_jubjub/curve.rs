/// DOC: https://eips.ethereum.org/EIPS/eip-2494
use crate::algebra::groups::ecg;
use crate::curves::baby_jubjub::ff;
use num::One;

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
        let m = ff::BabyJubJubField::from(BABY_JUBJUB_B) * a.x() * b.x() * a.y() * b.y();
        let x3 = (a.x() * b.y() + a.y() * b.x()) / (ff::BabyJubJubField::one() + m.clone());
        let y3 = (a.y() * b.y() - ff::BabyJubJubField::from(BABY_JUBJUB_A) * a.x() * b.x()) / (ff::BabyJubJubField::one() - m);
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
    use std::convert::TryFrom;
    use crate::curves::baby_jubjub::curve;

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
}
