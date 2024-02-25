//! Raw immutable datapoints

use std::collections::HashMap;

use crate::{complexity::info::CpxInfo, data::data::{Showed, TransferGroup}};

/// General identification of all database entities.
pub trait Id{
    fn get_id(&self) -> String;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RawKind {
    Parameter,
    GraphClass,
    Intersection(Vec<RawSet>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawSourceKey {
    Bibtex { key: String },
    Online { url: String },
    Unknown,
}

impl RawSourceKey {
    pub fn to_str(&self) -> String{
        match self {
            RawSourceKey::Bibtex { key } => key.clone(),
            RawSourceKey::Online { url } => url.clone(),
            RawSourceKey::Unknown => "unknown".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawSource {
    pub id: String,
    pub rawsourcekey: RawSourceKey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RawSet {
    pub id: String,
    pub name: String,
    pub kind: RawKind,
}

impl Id for RawSet {
    fn get_id(&self) -> String{
        self.id.clone()
    }
}


/// Holds into on whether bounded `subset` implies bounded `superset`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawRelation {
    /// If inclusion, then subset is the parameter above which is potentially bigger for the same graph.
    pub subset: RawSet,
    /// If inclusion, then superset is the parameter below which is potentially smaller for the same graph.
    pub superset: RawSet,
    pub cpx: CpxInfo,
}

impl Id for RawRelation {
    fn get_id(&self) -> String{
        format!("relation_{}_to_{}", self.subset.id, self.superset.id)
    }
}

impl RawRelation {

    fn combine_parallel(&self, bound: &RawRelation) -> RawRelation {
        assert_eq!(self.subset, bound.subset);
        assert_eq!(self.superset, bound.superset);
        RawRelation{
            subset: self.subset.clone(),
            superset: bound.superset.clone(),
            cpx: self.cpx.combine_parallel(&bound.cpx),
        }
    }

    fn combine_serial(&self, bound: &RawRelation) -> RawRelation {
        assert_eq!(self.superset, bound.subset);
        RawRelation {
            subset: self.subset.clone(),
            superset: bound.superset.clone(),
            cpx: self.cpx.combine_serial(&bound.cpx),
        }
    }

}

#[derive(Debug, Clone)]
pub struct RawTopic {
    pub id: String,
    pub name: String,
    pub description: String,
}

pub struct RawData {
    pub sets: Vec<RawSet>,
    pub factoids: Vec<(RawSource, Showed)>,
    pub sources: Vec<RawSource>,
    pub isgci: Vec<(RawSet, u32)>,
    pub topics: Vec<RawTopic>,
    pub transfer: HashMap<TransferGroup, Vec<(RawSet, RawSet)>>,
}

impl RawData {
    pub fn new() -> Self {
        Self {
            sets: Vec::new(),
            factoids: Vec::new(),
            sources: Vec::new(),
            isgci: Vec::new(),
            topics: Vec::new(),
            transfer: HashMap::new(),
        }
    }
}
