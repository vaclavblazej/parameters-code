
#[cfg(test)]
mod tests {
    use std::env;

    use crate::data::data::Relation;
    use crate::general::enums::CpxInfo;
    use crate::general::enums::{Cpx::{self, *}, CpxInfo::*, CpxTime::*, Page::*};
    use crate::input::{raw::RawData, build::Builder};
    use crate::processing::processing::process_raw_data;

    fn bibfile() -> std::path::PathBuf {
        let current = env::current_dir().unwrap();
        let parent = current.parent().unwrap();
        let handcrafted_dir = parent.join("handcrafted");
        let bibliography_file = handcrafted_dir.join("main.bib");
        bibliography_file
    }

    #[test]
    fn upper_bound_trnasitivity() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = create.parameter("a", "a", 9);
        let b = create.parameter("b", "b", 9);
        let c = create.parameter("c", "c", 9);
        create.assumed_source()
            .showed("s_ab", NotApplicable, &a, &b, UpperBound(Linear), "")
            .showed("s_bc", NotApplicable, &b, &c, UpperBound(Linear), "")
            .done();
        let data = process_raw_data(&create.build(), &bibfile());
        // == test =============================================================
        let rel = data.get_relation(&a.into(), &c.into()).unwrap();
        assert!(matches!(rel.cpx, Inclusion{ .. }));
        assert_eq!(rel.cpx, Inclusion{ mn: Constant, mx: Linear });
    }

    #[test]
    fn exclusion_inclusion_transfer() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = create.parameter("a", "a", 9);
        let b = create.parameter("b", "b", 9);
        let c = create.parameter("c", "c", 9);
        create.assumed_source()
            .showed("s_ab", NotApplicable, &a, &c, Cpx::Exclusion, "")
            .showed("s_bc", NotApplicable, &b, &c, UpperBound(Linear), "")
            .done();
        let data = process_raw_data(&create.build(), &bibfile());
        // == test =============================================================
        let rel = data.get_relation(&a.into(), &b.into()).unwrap();
        assert_eq!(rel.cpx, CpxInfo::Exclusion);
    }

    #[test]
    fn combined_parameter_bound() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = create.parameter("a", "a", 9);
        let b = create.parameter("b", "b", 9);
        let c = create.parameter("c", "c", 9);
        let bc = create.intersection("b+c", &b, &c, "b+c", 9);
        create.assumed_source()
            .showed("s_ab", NotApplicable, &a, &b, UpperBound(Linear), "")
            .showed("s_bc", NotApplicable, &a, &c, UpperBound(Linear), "")
            .done();
        let data = process_raw_data(&create.build(), &bibfile());
        // == test =============================================================
        let rel = data.get_relation(&a.into(), &bc.into()).unwrap();
        assert!(matches!(rel.cpx, Inclusion{ .. }));
    }

}
