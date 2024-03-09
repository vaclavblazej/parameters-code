//! Collection of preview datapoints together with their mutual relations.
//! More complex structures use Preview structures to refer to each other.

use std::fmt;
use std::collections::HashMap;

use crate::data::preview::{PreviewKind, PreviewRelation, PreviewSet, PreviewSource};
use crate::general::enums::{Cpx, CpxInfo, CpxTime, Page, SourceKey};
use crate::processing::processing::Sets;


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
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
}

impl Date {
    pub fn empty() -> Date {
        Date {
            year: None,
            month: None,
            day: None,
        }
    }
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
            result.push('/');
            result.push_str(&format!("{:0>2}", m.to_string()));
        } else {
            return write!(f, "{}", result)
        }
        if let Some(d) = self.day {
            result.push('/');
            result.push_str(&format!("{:0>2}", d.to_string()));
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

#[derive(Clone, Debug)]
pub struct SourceSubset {
    pub preview: PreviewSource,
    pub id: String,
    pub sourcekey: SourceKey,
    pub showed: Vec<Showed>,
    pub time: Date,
}

/// A general structure for parameters, graph classes, any other structures.
// todo remove clone
#[derive(Debug, Clone)]
pub struct Set {
    pub preview: PreviewSet,
    pub id: String,
    pub name: String,
    pub kind: PreviewKind,
    // pub providers: Vec<Provider>, // todo isgci, (and others?)
    pub timeline: Vec<SourceSubset>,
    pub supersets: Sets,
    pub subsets: Sets,
    pub super_exclusions: Sets,
    pub sub_exclusions: Sets,
    pub unknown: Sets,
    // pub transfers: HashMap<TransferGroup, Vec<PreviewSet>>,
}

#[derive(Clone)]
pub struct Source {
    pub preview: PreviewSource,
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
    pub set_idx: HashMap<PreviewSet, usize>,
    pub relation_idx: HashMap<(PreviewSet, PreviewSet), usize>,
}

impl Data {
    pub fn new(sets: Vec<Set>, relations: Vec<Relation>, urls: HashMap<String, Box<dyn Linkable>>, sources: Vec<Source>) -> Self {
        let mut set_idx: HashMap<PreviewSet, usize> = HashMap::new();
        for (idx, set) in sets.iter().enumerate() {
            set_idx.insert(set.preview.clone(), idx);
        }
        let mut relation_idx: HashMap<(PreviewSet, PreviewSet), usize> = HashMap::new();
        for (idx, relation) in relations.iter().enumerate() {
            let pair = (relation.subset.clone(), relation.superset.clone());
            assert!(!relation_idx.contains_key(&pair));
            relation_idx.insert(pair, idx);
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

    pub fn get_sets<I>(&self, key: I) -> Vec<&Set>
        where I: IntoIterator<Item = PreviewSet>, {
        key.into_iter().map(|x|self.get_set(&x)).collect()
    }

    pub fn get_set(&self, key: &PreviewSet) -> &Set {
        let idx: usize = *self.set_idx.get(&key).expect(&format!("preview set not found {:?}", key));
        &self.sets[idx]
    }

    pub fn get_relation(&self, subset: &PreviewSet, superset: &PreviewSet) -> Option<&Relation> {
        let key = (subset.clone(), superset.clone());
        match self.relation_idx.get(&key) {
            Some(&idx) => Some(&self.relations[idx]),
            None => None,
        }
        // let idx: usize = *self.relation_idx.get(&key).expect(&format!("relation not found {:?}", key));
        // Some(&self.relations[idx])
    }
}

// todo abbreviation

#[derive(Debug)]
pub struct Relation {
    pub id: String,
    pub preview: PreviewRelation,
    /// If inclusion, then subset is the parameter above which is potentially bigger for the same graph.
    pub subset: PreviewSet,
    /// If inclusion, then superset is the parameter below which is potentially smaller for the same graph.
    pub superset: PreviewSet,
    pub cpx: CpxInfo,
    pub combined_from: Option<(PreviewRelation, PreviewRelation)>,
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

#[derive(Debug, Clone)]
pub struct Showed {
    pub id: String,
    pub text: String,
    pub fact: ShowedFact,
    pub page: Page,
}

#[derive(Debug, Clone)]
pub enum ShowedFact {
    Relation(PreviewRelation),
    Definition(PreviewSet),
    Citation(PreviewSource),
}
