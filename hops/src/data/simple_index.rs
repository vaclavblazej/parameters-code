//! First pass over the data which prepares the data for a more complex processing.

use std::clone::Clone;
use std::collections::HashSet;
use std::hash::Hash;

use crate::data::enums::CpxInfo;
use crate::data::preview::HasPreview;

pub enum InclusionStatus {
    Inclusion,
    Exclusion,
}

/// This structure keeps which sets are with relations with other sets.
pub struct SimpleIndex<T> {
    pub first_subset_of_second: HashSet<(T, T)>,
    pub first_not_subset_of_second: HashSet<(T, T)>,
}

impl<T> SimpleIndex<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(relations: &Vec<(T, T, InclusionStatus)>) -> Self {
        let mut res = SimpleIndex {
            first_subset_of_second: HashSet::new(),
            first_not_subset_of_second: HashSet::new(),
        };
        for rel in relations {
            res.add(rel);
        }
        res
    }

    fn add(&mut self, relation: &(T, T, InclusionStatus)) {
        let (subset, superset, status) = relation;
        match status {
            // todo
            InclusionStatus::Inclusion => {
                self.first_subset_of_second
                    .insert((subset.clone(), superset.clone()));
            }
            InclusionStatus::Exclusion => {
                self.first_not_subset_of_second
                    .insert((subset.clone(), superset.clone()));
            }
        }
    }

    fn get_all_subsets(&self, a: &T) -> Vec<T> {
        self.first_subset_of_second
            .iter()
            .filter(|(_, sup)| sup == a)
            .map(|(sub, _)| sub.clone())
            .collect()
    }

    fn get_all_supersets(&self, a: &T) -> Vec<T> {
        self.first_subset_of_second
            .iter()
            .filter(|(sub, _)| sub == a)
            .map(|(_, sup)| sup.clone())
            .collect()
    }

    pub fn get_eqsets(&self, a: &T) -> Vec<T> {
        let mut seta: HashSet<T> = HashSet::from_iter(self.get_all_subsets(a));
        let mut setb: HashSet<T> = HashSet::from_iter(self.get_all_supersets(a));
        seta.insert(a.clone());
        setb.insert(a.clone());
        let res: Vec<T> = seta.intersection(&setb).cloned().collect();
        res
    }

    pub fn get_subsets(&self, a: &T) -> Vec<T> {
        let seta: HashSet<T> = HashSet::from_iter(self.get_all_subsets(a));
        let setb: HashSet<T> = HashSet::from_iter(self.get_eqsets(a));
        seta.difference(&setb).cloned().collect()
    }

    pub fn get_supersets(&self, a: &T) -> Vec<T> {
        let seta: HashSet<T> = HashSet::from_iter(self.get_all_supersets(a));
        let setb: HashSet<T> = HashSet::from_iter(self.get_eqsets(a));
        seta.difference(&setb).cloned().collect()
    }

    pub fn get_antisubsets(&self, a: &T) -> Vec<T> {
        self.first_not_subset_of_second
            .iter()
            .filter(|(_, sup)| sup == a)
            .map(|(sub, _)| sub.clone())
            .collect()
    }

    pub fn get_antisupersets(&self, a: &T) -> Vec<T> {
        self.first_not_subset_of_second
            .iter()
            .filter(|(sub, _)| sub == a)
            .map(|(_, sup)| sup.clone())
            .collect()
    }

    pub fn first_subset_of_second(&self, a: &T, b: &T) -> bool {
        self.first_subset_of_second
            .contains(&(a.clone(), b.clone()))
    }
}
