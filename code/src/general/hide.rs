//! Given a set of relations find out the essential subset from which
//! the other relations can be implied.

use std::collections::HashMap;

use crate::{
    data::{
        core::{Data, PartialResult, Set},
        preview::{PreviewRelation, PreviewSet, WorkRelation},
    },
    general::enums::SourcedCpxInfo,
    work::combine::combine_serial,
};

use super::enums::CpxInfo;

fn rel_can_be_implied_through(
    map: &HashMap<WorkRelation, PreviewRelation>,
    relation: &PreviewRelation,
    midset: &PreviewSet,
) -> bool {
    if let Some(mx) = relation.cpx.get_mx() {
        assert_ne!(relation.subset, relation.superset);
        assert_ne!(midset, &relation.superset);
        assert_ne!(midset, &relation.subset);
        let upper_work = WorkRelation::new(&relation.subset.id, &midset.id);
        let lower_work = WorkRelation::new(&midset.id, &relation.superset.id);
        if let (Some(upper_relation), Some(lower_relation)) =
            (map.get(&upper_work), map.get(&lower_work))
        {
            let mock_a = PartialResult {
                handle: 0,
                created_by: crate::general::enums::CreatedBy::Todo,
                cpx: CpxInfo::Unknown,
                relation: upper_work.clone(),
            };
            let mock_b = PartialResult {
                handle: 0,
                created_by: crate::general::enums::CreatedBy::Todo,
                cpx: CpxInfo::Unknown,
                relation: lower_work.clone(),
            };
            // todo fix
            if let (Some(a), Some(b)) = (upper_relation.cpx.get_mx(), lower_relation.cpx.get_mx()) {
                let (c, d, e) = combine_serial((a, mock_a.clone()), (b, mock_b.clone()));
                if !mx.is_smaller_than(&e) {
                    return true;
                }
            }
        }
    }
    false
}

fn could_be_hidden(
    map: &HashMap<WorkRelation, PreviewRelation>,
    relation: &PreviewRelation,
    with_respect_to: &Vec<PreviewSet>,
) -> bool {
    if relation.superset == relation.subset {
        return true;
    }
    if let Some(mx) = &relation.cpx.get_mx() {
        for midset in with_respect_to {
            if midset == &relation.superset || midset == &relation.subset {
                continue;
            }
            // cannot use this midset to hide the relation if it does not imply it
            if !rel_can_be_implied_through(map, relation, midset) {
                continue;
            }
            // even if connection through mid implies the relation we still need
            // to prevent mutual hiding for sets that bound each other
            // case 1 -- subset and midset are mutually bounded
            if let Some(same) = map.get(&WorkRelation::new(&midset.id, &relation.superset.id)) {
                if rel_can_be_implied_through(map, same, &relation.subset)
                    && relation.subset.is_more_relevant_than(midset) {
                        continue;
                }
            }
            // case 2 -- superset and midset are mutually bounded
            if let Some(same) = map.get(&WorkRelation::new(&relation.subset.id, &midset.id)) {
                if rel_can_be_implied_through(map, same, &relation.superset)
                    && relation.superset.is_more_relevant_than(midset) {
                        continue;
                }
            }
            return true;
        }
    }
    false
}

pub fn filter_hidden(
    potential_relations: Vec<PreviewRelation>,
    displayed_sets: &Vec<PreviewSet>,
) -> Vec<PreviewRelation> {
    let mut map: HashMap<WorkRelation, PreviewRelation> = HashMap::new();
    for pr in &potential_relations {
        map.insert(WorkRelation::new(&pr.subset.id, &pr.superset.id), pr.clone());
    }
    let mut drawn_relations = Vec::new();
    for relation in &potential_relations {
        if relation.cpx.get_mx().is_some()
            && !could_be_hidden(&map, relation, displayed_sets) {
                drawn_relations.push(relation.clone());
            }
    }
    drawn_relations
}
