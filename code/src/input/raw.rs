//! Raw immutable datapoints

use std::collections::HashMap;

use crate::general::enums::{CpxInfo, Page, TransferGroup};


/// General identification of all database entities.
pub trait Id{
    fn get_id(&self) -> String;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RawType {
    Parameter,
    GraphClass,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Composition {
    None,
    Intersection(Vec<RawSet>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawSourceKey {
    Bibtex { key: String },
    Online { url: String },
    Other { name: String, description: String },
}

impl RawSourceKey {
    pub fn to_str(&self) -> String{
        match self {
            RawSourceKey::Bibtex { key } => key.clone(),
            RawSourceKey::Online { url } => url.clone(),
            RawSourceKey::Other { name, description: _ } => name.clone(),
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
    pub typ: RawType,
    pub composed: Composition,
    pub relevance: u32, // from 0 to 9
    pub hidden: bool,
    pub tags: Vec<RawTag>,
    pub aka: Vec<String>,
}

impl RawSet {
    pub fn new(id: String, name: String, typ: RawType, composed: Composition, relevance: u32) -> Self {
        Self {
            id,
            name,
            typ,
            composed,
            relevance,
            hidden: false,
            tags: vec![],
            aka: vec![],
        }
    }
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

impl RawRelation {
    pub fn new(subset: &RawSet, superset: &RawSet, cpx: CpxInfo) -> RawRelation {
        RawRelation{
            subset: subset.clone(),
            superset: superset.clone(),
            cpx,
        }
    }
}

impl Id for RawRelation {
    fn get_id(&self) -> String{
        format!("relation_{}_to_{}", self.subset.id, self.superset.id)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RawTag {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RawProvider {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RawProviderLink {
    pub set: RawSet,
    pub url: String,
}

pub struct RawData {
    pub sets: Vec<RawSet>,
    pub factoids: Vec<(RawSource, RawShowed)>,
    pub sources: Vec<RawSource>,
    pub provider_links: HashMap<RawProvider, Vec<RawProviderLink>>,
    pub tags: Vec<RawTag>,
    pub transfer: HashMap<TransferGroup, Vec<(RawSet, RawSet)>>,
}

impl RawData {
    pub fn new() -> Self {
        Self {
            sets: Vec::new(),
            factoids: Vec::new(),
            sources: Vec::new(),
            provider_links: HashMap::new(),
            tags: Vec::new(),
            transfer: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawShowed {
    pub id: String,
    pub text: String,
    pub fact: RawShowedFact,
    pub page: Page,
}

#[derive(Debug, Clone)]
pub enum RawShowedFact {
    Relation(RawRelation),
    Definition(RawSet),
    Citation(RawSource),
}
