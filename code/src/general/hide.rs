//! Given a set of relations find out the essential subset from which
//! the other relations can be implied.

use std::collections::HashMap;

use crate::data::{data::{Data, Set}, preview::{PreviewRelation, PreviewSet}};

use super::enums::CpxInfo;



fn rel_can_be_implied_through(map: &HashMap<(&PreviewSet, &PreviewSet), PreviewRelation>, relation: &PreviewRelation, midset: &PreviewSet) -> bool {
    if let CpxInfo::Inclusion { mn: _, mx } = &relation.cpx {
        assert_ne!(relation.subset, relation.superset);
        assert_ne!(midset, &relation.superset);
        assert_ne!(midset, &relation.subset);
        if let (Some(upper_relation), Some(lower_relation))
            = (map.get(&(&relation.subset, midset)), map.get(&(midset, &relation.superset))) {
            // if we have sequence of inclusions that implies the current one hide it
            if let (CpxInfo::Inclusion { mn: _, mx: mxa },
                    CpxInfo::Inclusion { mn: _, mx: mxb })
                    = (&upper_relation.cpx, &lower_relation.cpx) {
                if !mx.is_better_than(&mxa.combine_serial(mxb)) {
                    return true;
                    // println!("excluded {:?} because of {:?} and {:?}", relation.preview, upper_relation.preview, lower_relation.preview);
                }
            }
        }
    }
    false
}

fn could_be_hidden(map: &HashMap<(&PreviewSet, &PreviewSet), PreviewRelation>, relation: &PreviewRelation, with_respect_to: &Vec<PreviewSet>) -> bool {
    if let CpxInfo::Inclusion { mn: _, mx } = &relation.cpx {
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
                    // can hide this relation if midset is more relevant than subset
                    if midset.relevance < relation.subset.relevance {
                        continue;
                    }
                    if midset.relevance == relation.subset.relevance
                        && midset.id < relation.subset.id{
                        continue;
                    }
                }
            }
            // case 2 -- superset and midset are mutually bounded
            if let Some(same) = map.get(&(&relation.subset, midset)){
                if rel_can_be_implied_through(map, same, &relation.superset){
                    // can hide this relation if midset is more relevant than superset
                    if midset.relevance < relation.superset.relevance {
                        continue;
                    }
                    if midset.relevance == relation.subset.relevance
                        && midset.id < relation.subset.id{
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
        if let CpxInfo::Inclusion { .. } = &relation.cpx {
            if !could_be_hidden(&map, &relation, displayed_sets) {
                drawn_relations.push(relation.clone());
            }
        }
    }
    drawn_relations
}
