//! Collection of raw datapoints together with their mutual relations.
//! More complex structures use Raw structures to refer to each other.

use std::fmt;
use std::collections::HashMap;

use crate::{raw::{RawSet, RawTopic, RawRelation, RawSource, RawKind}, complexity::{CpxTime, CpxInfo}, processing::Sets};


pub trait Linkable {
    fn get_url(&self) -> String;
    fn get_name(&self) -> String;
}

pub trait HasId {
    fn id(&self) -> String;
}
impl HasId for Set { fn id(&self) -> String { self.id.clone() } }
impl HasId for Source { fn id(&self) -> String { self.id.clone() } }

/// Refers to a page in a book or paper.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Page{
    Pp(u32),
    Unknown,
    NotApplicable,
}

pub struct DataSource<'a> {
    source: RawSource,
    data: &'a mut RawData,
}

/// Enum that makes inputting complexities more convenient.
pub enum Cpx {
    Bounds(CpxTime, CpxTime),
    UpperBound(CpxTime),
    Exactly(CpxTime),
    Equivalence,
    Exclusion,
    Todo,
}

impl Cpx {
    fn expand(self) -> CpxInfo {
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
        } else {
            return write!(f, "{}", result)
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

impl<'a> DataSource<'a> {

    pub fn new(source: &RawSource, data: &'a mut RawData) -> Self {
        DataSource { source: source.clone(), data }
    }

    pub fn defined(self, id: &str, page: Page, set: &RawSet, text: &str) -> Self {
        let showed = Showed {
            id: id.into(),
            text: text.into(),
            fact: ShowedFact::Definition(set.clone()),
            page,
        };
        self.data.factoids.push((self.source.clone(), showed));
        self
    }

    pub fn showed(self, id: &str, page: Page, subset: &RawSet, superset: &RawSet, cpx: Cpx, text: &str) -> Self {
        let relation = RawRelation {
            subset: subset.clone(),
            superset: superset.clone(),
            cpx: cpx.expand(),
        };
        let showed = Showed {
            id: id.into(),
            text: text.into(),
            fact: ShowedFact::Relation(relation),
            page,
        };
        self.data.factoids.push((self.source.clone(), showed));
        self
    }

    pub fn cited(self, id: &str, page: Page, who: RawSource, text: &str) -> Self {
        let showed = Showed {
            id: id.into(),
            text: text.into(),
            fact: ShowedFact::Citation(who),
            page,
        };
        self.data.factoids.push((self.source.clone(), showed));
        self
    }

    pub fn done(self) -> RawSource {
        self.source
    }

}

#[derive(Debug, Clone)]
pub enum ShowedFact {
    Relation(RawRelation),
    Definition(RawSet),
    Citation(RawSource),
}

#[derive(Debug, Clone)]
pub struct Showed {
    pub id: String,
    pub text: String,
    pub fact: ShowedFact,
    pub page: Page,
}

pub struct Data {
    pub parameters: Vec<Set>,
    pub graph_classes: Vec<Set>,
    pub links: HashMap<String, Box<dyn Linkable>>,
    pub sources: Vec<Source>,
}

impl Data {
    pub fn get(&self, set: RawSet) -> &Set {
        for par in &self.parameters { if par.raw == set { return &par } }
        for gc in &self.graph_classes { if gc.raw == set { return &gc } }
        panic!("raw set not found {:?}", set);
    }
}

pub struct RawData {
    pub parameters: Vec<RawSet>,
    pub factoids: Vec<(RawSource, Showed)>,
    pub graph_classes: Vec<RawSet>,
    pub sources: Vec<RawSource>,
    pub isgci: Vec<(RawSet, u32)>,
    pub topics: Vec<RawTopic>,
    pub transfer: HashMap<TransferGroup, Vec<(RawSet, RawSet)>>,
}

// todo abbreviation

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TransferGroup {
    DistanceTo,
}
