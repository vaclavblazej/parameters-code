use crate::general::enums::{Cpx::*, CpxTime::*, Page::*};
use crate::input::build::parameter;
use crate::input::{build::Builder, raw::RawData};

pub fn build_collection() -> RawData {
    let mut create = Builder::new();
    let a = parameter("a", "a", 9).done(&mut create);
    let b = parameter("b", "b", 8).done(&mut create);
    let c = parameter("c", "c", 7).done(&mut create);
    let bc = create.intersection("bc", &b, &c, "bc", 9).done(&mut create);
    create.assumed_source()
        .ref_showed("s_ab", NotApplicable, &a, &b, UpperBound(Linear), "")
        .ref_showed("s_bc", NotApplicable, &a, &c, UpperBound(Linear), "");
    create.build()
}
