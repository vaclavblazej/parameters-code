#[cfg(test)]
mod tests {
    use std::env;

    use crate::data::core::Relation;
    use crate::data::id::{BaseId, RelationId};
    use crate::general::enums::{Cpx, CpxInfo};
    use crate::general::enums::{
        CpxTime::*,
        Page::*,
        SourcedCpxInfo::{self, *},
    };
    use crate::input::build::parameter;
    use crate::input::{build::Builder, raw::RawData};
    use crate::work::processing::process_raw_data;

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
        let sequence = [0, 1, 2, 3, 4, 6, 7, 8, 9, 5];
        for i in 0..=count {
            let str = format!("{}", i);
            arr.push(parameter(str.as_str(), str.as_str(), 9).done(&mut create));
        }
        let mut assumed_source = create.assumed_source();
        for i in sequence {
            let id = format!("s_{}", i);
            let this = &arr[i];
            let next = &arr[i + 1];
            assumed_source.ref_showed(&id, NotApplicable, this, next, Cpx::UpperBound(Linear), "");
        }
        let data = process_raw_data(create.build(), &None);
        // == test =============================================================
        let first = arr.first().unwrap();
        let last = arr.last().unwrap();
        let rel = data
            .get_relation_by_id(&RelationId::new(&first, &last).preview())
            .unwrap();
        assert!(matches!(rel.cpx, Inclusion { .. }));
        assert_eq!(
            CpxInfo::from(rel.cpx.clone()),
            CpxInfo::Inclusion { mn: None, mx: Some(Linear) }
        );
    }

    #[test]
    fn exclusion_inclusion_transfer() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = parameter("a", "a", 9).done(&mut create);
        let b = parameter("b", "b", 9).done(&mut create);
        let c = parameter("c", "c", 9).done(&mut create);
        create.assumed_source()
            .ref_showed("s_ab", NotApplicable, &a, &c, Cpx::Exclusion, "")
            .ref_showed("s_bc", NotApplicable, &b, &c, Cpx::UpperBound(Linear), "");
        let data = process_raw_data(create.build(), &None);
        // == test =============================================================
        let rel = data.get_relation_by_ids(&a, &b).unwrap();
        assert_eq!(CpxInfo::from(rel.cpx.clone()), CpxInfo::Exclusion);
    }

    #[test]
    fn equiv_correctly_created() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = parameter("a", "a", 9).done(&mut create);
        let b = parameter("b", "b", 9).done(&mut create);
        create.assumed_source()
            .ref_showed("s_ab", NotApplicable, &a, &b, Cpx::Equal, "");
        let data = process_raw_data(create.build(), &None);
        // == test =============================================================
        assert!(matches!(
            CpxInfo::from(data.get_relation_by_ids(&a, &b).unwrap().cpx.clone()),
            CpxInfo::Equal
        ));
        assert!(matches!(
            CpxInfo::from(data.get_relation_by_ids(&b, &a).unwrap().cpx.clone()),
            CpxInfo::Equal
        ));
    }

    #[test]
    fn equiv_inclusion_propagates() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = parameter("a", "a", 9).done(&mut create);
        let b = parameter("b", "b", 9).done(&mut create);
        let c = parameter("c", "c", 9).done(&mut create);
        let d = parameter("d", "d", 9).done(&mut create);
        create
            .assumed_source()
            .ref_showed("s_ab", NotApplicable, &a, &b, Cpx::Equal, "")
            .ref_showed("s_ac", NotApplicable, &a, &c, Cpx::UpperBound(Linear), "")
            .ref_showed("s_db", NotApplicable, &d, &b, Cpx::UpperBound(Linear), "");
        let data = process_raw_data(create.build(), &None);
        // == test =============================================================
        assert!(matches!(
            data.get_relation_by_ids(&b, &c)
                .unwrap()
                .cpx,
            Inclusion { .. }
        ));
        assert!(matches!(
            data.get_relation_by_ids(&d, &a).unwrap().cpx,
            Inclusion { .. }
        ));
    }

    #[test]
    fn combined_parameter_bound() {
        // == setup ============================================================
        let mut create = Builder::new();
        let a = parameter("a", "a", 9).done(&mut create);
        let b = parameter("b", "b", 9).done(&mut create);
        let c = parameter("c", "c", 9).done(&mut create);
        let bc = create.intersection("b+c", &b, &c, "b+c", 9).done(&mut create);
        create
            .assumed_source()
            .ref_showed("s_ab", NotApplicable, &a, &b, Cpx::UpperBound(Linear), "")
            .ref_showed("s_bc", NotApplicable, &a, &c, Cpx::UpperBound(Linear), "");
        let data = process_raw_data(create.build(), &None);
        // == test =============================================================
        let rel = data.get_relation_by_ids(&a, &bc).unwrap();
        assert!(matches!(rel.cpx, Inclusion { .. }));
    }
}
