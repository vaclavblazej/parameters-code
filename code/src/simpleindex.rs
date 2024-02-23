//! First pass over the data which prepares the data for a more complex processing.

use std::collections::HashSet;

use crate::{raw::{RawSet, RawRelation}, complexity::CpxInfo, data::{ShowedFact, RawData}};


/// This structure keeps which parameters are directly above or below others.
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
            CpxInfo::LowerBound { mn: _ } => {
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
    // pub fn get_relation(&self, a: &RawSet, b: &RawSet) -> RawRelation {
        // // todo this implementation is temporary and terrible
        // let subsets = self.get_subsets(a);
        // let antisubsets = self.get_antisubsets(a);
        // let mut cpx = CpxInfo::Unknown;
        // // if subsets.contains(b) {
            // // cpx = CpxInfo::Inclusion{mn: crate::complexity::CpxTime::Constant, mx: crate::complexity::CpxTime::Exists};
        // // } else if antisubsets.contains(b) {
            // // cpx = CpxInfo::Exclusion;
        // // }
        // return RawRelation{
            // subset: a.clone(),
            // superset: b.clone(),
            // cpx,
        // };
    // }
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
    pub fn first_above_second(&self, a: &RawSet, b: &RawSet) -> bool {
        self.first_above_second.contains(&(a.clone(), b.clone()))
    }
}
