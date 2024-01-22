//! First pass over the data which prepares the data for a more complex processing.

use std::collections::HashSet;

use crate::{raw::{RawSet, RawRelation}, complexity::CpxInfo, data::{ShowedFact, RawData}};


pub struct SimpleIndex {
    pub first_above_second: HashSet<(RawSet, RawSet)>,
    pub first_not_above_second: HashSet<(RawSet, RawSet)>,
}

impl SimpleIndex {
    pub fn new(rawdata: &RawData) -> SimpleIndex {
        let mut res = SimpleIndex{
            first_above_second: HashSet::new(),
            first_not_above_second: HashSet::new(),
        };
        for (_, showed) in &rawdata.factoids {
            if let ShowedFact::Relation(rel) = &showed.fact {
                res.add(rel);
            }
        }
        res
    }
    fn add(&mut self, relation: &RawRelation) {
        let element = (relation.superset.clone(), relation.subset.clone());
        match &relation.cpx {
            CpxInfo::Inclusion { mn: _, mx: _ } => {
                self.first_above_second.insert(element);
            },
            CpxInfo::Exclusion {} => {
                self.first_not_above_second.insert(element);
            },
            CpxInfo::Equivalence => {
                let (a, b) = element.clone();
                self.first_above_second.insert(element);
                self.first_above_second.insert((b, a));
            },
            CpxInfo::Unknown => {},
        }
    }
    pub fn get_subsets(&self, a: &RawSet) -> Vec<RawSet> {
        self.first_above_second.iter()
            .filter(|(_,sup)|sup==a)
            .map(|(sub,_)|sub.clone())
            .collect()
    }
    pub fn get_supersets(&self, a: &RawSet) -> Vec<RawSet> {
        self.first_above_second.iter()
            .filter(|(sub,_)|sub==a)
            .map(|(_,sup)|sup.clone())
            .collect()
    }
    pub fn get_antisubsets(&self, a: &RawSet) -> Vec<RawSet> {
        self.first_not_above_second.iter()
            .filter(|(_,sup)|sup==a)
            .map(|(sub,_)|sub.clone())
            .collect()
    }
    pub fn get_antisupersets(&self, a: &RawSet) -> Vec<RawSet> {
        self.first_not_above_second.iter()
            .filter(|(sub,_)|sub==a)
            .map(|(_,sup)|sup.clone())
            .collect()
    }
    pub fn is_below(&self, a: &RawSet, b: &RawSet) -> bool {
        self.first_above_second.contains(&(a.clone(), b.clone()))
    }
}

