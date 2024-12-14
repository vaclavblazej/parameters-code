
use crate::general::enums::{Cpx::*, CpxTime::*, Page::*};
use crate::input::{raw::RawData, build::Builder};

pub fn build_collection() -> RawData {
    let mut create = Builder::new();
    let a = create.parameter("a", "a", 9).done();
    let b = create.parameter("b", "b", 8).done();
    let c = create.parameter("c", "c", 7).done();
    create.assumed_source()
        .showed("s_ab", NotApplicable, &a, &b, UpperBound(Linear), "")
        .showed("s_bc", NotApplicable, &b, &c, UpperBound(Linear), "")
        .done();
    create.build()
}
