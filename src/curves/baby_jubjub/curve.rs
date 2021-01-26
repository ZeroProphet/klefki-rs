use crate::algebra::groups::ecg;
use crate::curves::baby_jubjub::ff;
use num::One;

pub const BABY_JUBJUB_A: u32 = 168700u32;
pub const BABY_JUBJUB_B: u32 = 168696u32;

pub struct BabyJubJubCurve {
    x: ff::BabyJubJubField,
    y: ff::BabyJubJubField,
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

impl From<(u32, u32)> for BabyJubJubCurve {
    fn from(v: (u32, u32)) -> Self {
        return BabyJubJubCurve {
            x: ff::BabyJubJubField::from(v.0),
            y: ff::BabyJubJubField::from(v.1),
        };
    }
}
