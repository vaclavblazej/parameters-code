//! Given a set of relations find out the essential subset from which
//! the other relations can be implied.

use std::collections::HashMap;

use crate::{data::{data::{Data, PartialResult, Set}, preview::{PreviewRelation, PreviewSet}}, general::enums::SourcedCpxInfo};

use super::enums::CpxInfo;



fn rel_can_be_implied_through(
    map: &HashMap<(&PreviewSet, &PreviewSet), PreviewRelation>,
    relation: &PreviewRelation,
    midset: &PreviewSet
    ) -> bool {
    if let Some(mx) = relation.cpx.get_mx() {
        assert_ne!(relation.subset, relation.superset);
        assert_ne!(midset, &relation.superset);
        assert_ne!(midset, &relation.subset);
        if let (Some(upper_relation), Some(lower_relation)) = (map.get(&(&relation.subset, midset)), map.get(&(midset, &relation.superset))) {
            let pa = PartialResult {
                handle: 0,
                created_by: crate::general::enums::CreatedBy::Todo
            };
            let sxa: SourcedCpxInfo = upper_relation.cpx.clone().to_sourced(pa.clone());
            let sxb: SourcedCpxInfo = lower_relation.cpx.clone().to_sourced(pa.clone());
            let sxc: SourcedCpxInfo = sxa.combine_serial(&sxb);
            let scc: CpxInfo = sxc.clone().into();
            if let Some(ccmx) = scc.get_mx() {
                if !mx.is_smaller_than(&ccmx) {
                    return true;
                }
            }
        }
    }
    false
}

fn could_be_hidden(map: &HashMap<(&PreviewSet, &PreviewSet), PreviewRelation>, relation: &PreviewRelation, with_respect_to: &Vec<PreviewSet>) -> bool {
    if relation.superset == relation.subset {
        return true;
    }
    if let Some(mx) = &relation.cpx.get_mx() {
        for midset in with_respect_to {
            if midset == &relation.superset || midset == &relation.subset {
                continue;
            }
            // cannot use this midset to hide the relation if it does not imply it
            if !rel_can_be_implied_through(map, &relation, midset){
                continue;
            }
            // even if connection through mid implies the relation we still need
            // to prevent mutual hiding for sets that bound each other
            // case 1 -- subset and midset are mutually bounded
            if let Some(same) = map.get(&(midset, &relation.superset)){
                if rel_can_be_implied_through(map, same, &relation.subset){
                    if relation.subset.is_more_relevant_than(midset) {
                        continue;
                    }
                }
            }
            // case 2 -- superset and midset are mutually bounded
            if let Some(same) = map.get(&(&relation.subset, midset)){
                if rel_can_be_implied_through(map, same, &relation.superset){
                    if relation.superset.is_more_relevant_than(midset) {
                        continue;
                    }
                }
            }
            return true;
        }
    }
    false
}

pub fn filter_hidden(potential_relations: Vec<PreviewRelation>, displayed_sets: &Vec<PreviewSet>) -> Vec<PreviewRelation> {
    let mut map = HashMap::new();
    for pr in &potential_relations {
        map.insert((&pr.subset, &pr.superset), pr.clone());
    }
    let mut drawn_relations = Vec::new();
    for relation in &potential_relations {
        if let Some(_) = &relation.cpx.get_mx() {
            if !could_be_hidden(&map, &relation, displayed_sets) {
                drawn_relations.push(relation.clone());
            }
        }
    }
    drawn_relations
}
