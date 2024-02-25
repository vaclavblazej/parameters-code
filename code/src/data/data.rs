//! Collection of raw datapoints together with their mutual relations.
//! More complex structures use Raw structures to refer to each other.

use std::fmt;
use std::collections::HashMap;

use crate::{general::enums::{Cpx, CpxInfo, CpxTime}, input::{raw::{RawKind, RawRelation, RawSet, RawSource}, source::Showed}, processing::processing::Sets};


pub trait Linkable {
    fn get_url(&self) -> String;
    fn get_name(&self) -> String;
}

pub trait HasId {
    fn id(&self) -> String;
}
impl HasId for Set { fn id(&self) -> String { self.id.clone() } }
impl HasId for Source { fn id(&self) -> String { self.id.clone() } }

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Date {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

/// Points to the source of a citation.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum SourceKey {
    Bibtex {
        key: String,
        formatted_citation: String,
    },
    Online {
        url: String,
    },
    Unknown,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();
        if let Some(y) = self.year {
            result.push_str(&y.to_string());
        } else {
            return write!(f, "{}", result)
        }
        if let Some(m) = self.month {
            result.push('-');
            result.push_str(&m.to_string());
        } else {
            return write!(f, "{}", result)
        }
        if let Some(d) = self.day {
            result.push('-');
            result.push_str(&d.to_string());
        // } else {
            // return write!(f, "{}", result);
        }
        write!(f, "{}", result)
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}", &self))
    }
}

/// More processed version of RawSource that also includes auxiliary information
/// to show it but does not include everything. Meant to be presented as a link.
#[derive(Clone, Debug)]
pub struct SourceSubset {
    pub raw: RawSource,
    pub id: String,
    pub sourcekey: SourceKey,
    pub showed: Vec<Showed>,
    pub time: Date,
}

/// A general structure for parameters, graph classes, any other structures.
#[derive(Clone, Debug)]
pub struct Set {
    pub raw: RawSet,
    pub id: String,
    pub name: String,
    pub kind: RawKind,
    // pub providers: Vec<Provider>, // todo isgci, (and others?)
    pub timeline: Vec<SourceSubset>,
    pub supersets: Sets,
    pub subsets: Sets,
    pub super_exclusions: Sets,
    pub sub_exclusions: Sets,
    pub unknown: Sets,
    // pub transfers: HashMap<TransferGroup, Vec<RawSet>>,
}

#[derive(Clone)]
pub struct Source {
    pub raw: RawSource,
    pub id: String,
    pub sourcekey: SourceKey,
    pub showed: Vec<Showed>,
    pub time: Date,
}

pub struct Data {
    pub sets: Vec<Set>,
    pub relations: Vec<Relation>,
    pub urls: HashMap<String, Box<dyn Linkable>>,
    pub sources: Vec<Source>,
    pub set_idx: HashMap<RawSet, usize>,
    pub relation_idx: HashMap<(RawSet, RawSet), usize>,
}

impl Data {
    pub fn new(sets: Vec<Set>, relations: Vec<Relation>, urls: HashMap<String, Box<dyn Linkable>>, sources: Vec<Source>) -> Self {
        let mut set_idx: HashMap<RawSet, usize> = HashMap::new();
        for (idx, set) in sets.iter().enumerate() {
            set_idx.insert(set.raw.clone(), idx);
        }
        let mut relation_idx: HashMap<(RawSet, RawSet), usize> = HashMap::new();
        for (idx, relation) in relations.iter().enumerate() {
            relation_idx.insert((relation.subset.clone(), relation.superset.clone()), idx);
        }
        Self {
            sets,
            relations,
            urls,
            sources,
            set_idx,
            relation_idx,
        }
    }

    pub fn get_set(&self, key: &RawSet) -> &Set {
        let idx: usize = *self.set_idx.get(&key).expect(&format!("raw set not found {:?}", key));
        &self.sets[idx]
    }

    pub fn get_relation(&self, set_a: &RawSet, set_b: &RawSet) -> &Relation {
        let key = (set_a.clone(), set_b.clone());
        let idx: usize = *self.relation_idx.get(&key).expect(&format!("relation not found {:?}", key));
        &self.relations[idx]
    }
}

// todo abbreviation

pub struct Relation {
    pub raw: RawRelation,
    /// If inclusion, then subset is the parameter above which is potentially bigger for the same graph.
    pub subset: RawSet,
    /// If inclusion, then superset is the parameter below which is potentially smaller for the same graph.
    pub superset: RawSet,
    pub cpx: CpxInfo,
}

impl Relation {
    pub fn new(raw: &RawRelation) -> Self {
        Self {
            raw: raw.clone(),
            subset: raw.subset.clone(),
            superset: raw.superset.clone(),
            cpx: raw.cpx.clone(),
        }
    }
}

impl Cpx {
    pub fn expand(self) -> CpxInfo {
        match self {
            Cpx::Bounds(a, b) => CpxInfo::Inclusion{mn: a.clone(), mx: b.clone()},
            Cpx::Exactly(a) => CpxInfo::Inclusion{mn: a.clone(), mx: a.clone()},
            Cpx::UpperBound(b) => CpxInfo::Inclusion{mn: CpxTime::Constant, mx: b.clone()},
            Cpx::Todo => CpxInfo::Inclusion { mn: CpxTime::Constant, mx: CpxTime::Exists },
            Cpx::Equivalence => CpxInfo::Equivalence,
            Cpx::Exclusion => CpxInfo::Exclusion,
        }
    }
}
