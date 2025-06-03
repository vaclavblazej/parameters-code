//! First pass over the data which prepares the data for a more complex processing.

use std::collections::HashSet;

use crate::general::enums::CpxInfo;

use super::{
    core::Relation,
    preview::{HasPreview, PreviewRelation, PreviewSet},
};

/// This structure keeps which sets are with relations with other sets.
pub struct SimpleIndex {
    pub first_subset_of_second: HashSet<(PreviewSet, PreviewSet)>,
    pub first_not_subset_of_second: HashSet<(PreviewSet, PreviewSet)>,
}

impl SimpleIndex {
    pub fn new(relations: &Vec<Relation>) -> SimpleIndex {
        let mut res = SimpleIndex {
            first_subset_of_second: HashSet::new(),
            first_not_subset_of_second: HashSet::new(),
        };
        for rel in relations {
            res.add(rel.preview());
        }
        res
    }

    fn add(&mut self, relation: PreviewRelation) {
        let element = (relation.subset.clone(), relation.superset.clone());
        match &relation.cpx {
            CpxInfo::Inclusion { mn: _, mx: Some(_) } => {
                self.first_subset_of_second.insert(element);
            }
            CpxInfo::Inclusion { .. } => {}
            CpxInfo::Exclusion => {
                self.first_not_subset_of_second.insert(element);
            }
            CpxInfo::Equal => {
                let (a, b) = element.clone();
                self.first_subset_of_second.insert(element);
                self.first_subset_of_second.insert((b, a));
            }
            CpxInfo::Unknown => {}
        }
    }

    fn get_all_subsets(&self, a: &PreviewSet) -> Vec<PreviewSet> {
        self.first_subset_of_second
            .iter()
            .filter(|(_, sup)| sup == a)
            .map(|(sub, _)| sub.clone())
            .collect()
    }

    fn get_all_supersets(&self, a: &PreviewSet) -> Vec<PreviewSet> {
        self.first_subset_of_second
            .iter()
            .filter(|(sub, _)| sub == a)
            .map(|(_, sup)| sup.clone())
            .collect()
    }

    pub fn get_eqsets(&self, a: &PreviewSet) -> Vec<PreviewSet> {
        let mut seta: HashSet<PreviewSet> = HashSet::from_iter(self.get_all_subsets(a));
        let mut setb: HashSet<PreviewSet> =
            HashSet::from_iter(self.get_all_supersets(a));
        seta.insert(a.clone());
        setb.insert(a.clone());
        let res: Vec<PreviewSet> = seta.intersection(&setb).cloned().collect();
        res
    }

    pub fn get_subsets(&self, a: &PreviewSet) -> Vec<PreviewSet> {
        let seta: HashSet<PreviewSet> = HashSet::from_iter(self.get_all_subsets(a));
        let setb: HashSet<PreviewSet> = HashSet::from_iter(self.get_eqsets(a));
        seta.difference(&setb).cloned().collect()
    }

    pub fn get_supersets(&self, a: &PreviewSet) -> Vec<PreviewSet> {
        let seta: HashSet<PreviewSet> = HashSet::from_iter(self.get_all_supersets(a));
        let setb: HashSet<PreviewSet> = HashSet::from_iter(self.get_eqsets(a));
        seta.difference(&setb).cloned().collect()
    }

    pub fn get_antisubsets(&self, a: &PreviewSet) -> Vec<PreviewSet> {
        self.first_not_subset_of_second
            .iter()
            .filter(|(_, sup)| sup == a)
            .map(|(sub, _)| sub.clone())
            .collect()
    }

    pub fn get_antisupersets(&self, a: &PreviewSet) -> Vec<PreviewSet> {
        self.first_not_subset_of_second
            .iter()
            .filter(|(sub, _)| sub == a)
            .map(|(_, sup)| sup.clone())
            .collect()
    }

    pub fn first_subset_of_second(&self, a: &PreviewSet, b: &PreviewSet) -> bool {
        self.first_subset_of_second
            .contains(&(a.clone(), b.clone()))
    }
}
