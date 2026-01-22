use crate::input::build::CollectionBuilder;
use crate::input::raw::RawData;

pub fn build_collection() -> RawData {
    let mut create = CollectionBuilder::new();
    // let a = parameter("a", "a", 9).done(&mut create);
    // let b = parameter("b", "b", 8).done(&mut create);
    // let c = parameter("c", "c", 7).done(&mut create);
    // let bc = create.intersection("bc", &b, &c, "bc", 9).done(&mut create);
    // create.assumed_source()
    //     .ref_proved("s_ab", NotApplicable, &a, &b, UpperBound(Linear), "")
    //     .ref_proved("s_bc", NotApplicable, &a, &c, UpperBound(Linear), "");
    create.build()
}
