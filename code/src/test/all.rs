
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
        let mut arr = vec![];
        let count = 10;
        let sequence = [0,1,2,3,4,6,7,8,9,5];
        for i in 0..=count {
            let str = format!("{}", i);
            arr.push(create.parameter(str.as_str(), str.as_str(), 9).done());
        }
        let mut source = create.assumed_source();
        for i in sequence {
            let id = format!("s_{}", i);
            let this = &arr[i];
            let next = &arr[i+1];
            source = source.showed(&id, NotApplicable, this, next, UpperBound(Linear), "");
        }
        source.done();
        let data = process_raw_data(&create.build(), &bibfile());
        // == test =============================================================
        let first = arr.first().unwrap();
        let last = arr.last().unwrap();
        let rel = data.get_relation(&first.clone().into(), &last.clone().into()).unwrap();
        assert!(matches!(rel.cpx, Inclusion{ .. }));
        assert_eq!(rel.cpx, Inclusion{ mn: Constant, mx: Linear });
    }

    #[test]
    fn exclusion_inclusion_transfer() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = create.parameter("a", "a", 9).done();
        let b = create.parameter("b", "b", 9).done();
        let c = create.parameter("c", "c", 9).done();
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
    fn equiv_correctly_created() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = create.parameter("a", "a", 9).done();
        let b = create.parameter("b", "b", 9).done();
        create.assumed_source()
            .showed("s_ab", NotApplicable, &a, &b, Cpx::Equivalence, "")
            .done();
        let data = process_raw_data(&create.build(), &bibfile());
        // == test =============================================================
        assert!(matches!(data.get_relation(&a.clone().into(), &b.clone().into()).unwrap().cpx, CpxInfo::Equivalence));
        assert!(matches!(data.get_relation(&b.clone().into(), &a.clone().into()).unwrap().cpx, CpxInfo::Equivalence));
    }

    #[test]
    fn equiv_inclusion_propagates() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = create.parameter("a", "a", 9).done();
        let b = create.parameter("b", "b", 9).done();
        let c = create.parameter("c", "c", 9).done();
        let d = create.parameter("d", "d", 9).done();
        create.assumed_source()
            .showed("s_ab", NotApplicable, &a, &b, Cpx::Equivalence, "")
            .showed("s_ac", NotApplicable, &a, &c, UpperBound(Linear), "")
            .showed("s_db", NotApplicable, &d, &b, UpperBound(Linear), "")
            .done();
        let data = process_raw_data(&create.build(), &bibfile());
        // == test =============================================================
        assert!(matches!(data.get_relation(&b.clone().into(), &c.clone().into()).unwrap().cpx, Inclusion{ .. }));
        assert!(matches!(data.get_relation(&d.clone().into(), &a.clone().into()).unwrap().cpx, Inclusion{ .. }));
    }

    #[test]
    fn combined_parameter_bound() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = create.parameter("a", "a", 9).done();
        let b = create.parameter("b", "b", 9).done();
        let c = create.parameter("c", "c", 9).done();
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
