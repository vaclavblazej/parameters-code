
use crate::general::enums::{Cpx::*, CpxTime::*, Page::*};
use crate::input::{raw::RawData, build::Builder};

pub fn build_collection() -> RawData {
    let mut create = Builder::new();
    let a = create.parameter("a", "a", 9);
    let b = create.parameter("b", "b", 8);
    let c = create.parameter("c", "c", 7);
    create.assumed_source()
        .showed("s_ab", NotApplicable, &a, &b, UpperBound(Linear), "")
        .showed("s_bc", NotApplicable, &b, &c, UpperBound(Linear), "")
        .done();
    create.build()
}
