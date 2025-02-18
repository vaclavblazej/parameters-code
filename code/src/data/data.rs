//! Collection of preview datapoints together with their mutual relations.
//! More complex structures use Preview structures to refer to each other.

use std::fmt;
use std::collections::HashMap;

use log::trace;
use serde::{Serialize, Deserialize};

use crate::data::preview::{PreviewType, PreviewRelation, PreviewSet, PreviewSource, PreviewTag};
use crate::general::enums::{Cpx, CpxInfo, CpxTime, CreatedBy, Drawing, Page, SourceKey, SourcedCpxInfo, TransferGroup};
use crate::processing::processing::Sets;


pub trait Linkable {
    fn get_url(&self) -> String;
    fn get_name(&self) -> String;
}

pub trait HasId {
    fn id(&self) -> String;
}
impl HasId for Set { fn id(&self) -> String { self.id.clone() } }
impl HasId for Relation { fn id(&self) -> String { self.id.clone() } }
impl HasId for Source { fn id(&self) -> String { self.id.clone() } }
impl HasId for Tag { fn id(&self) -> String { self.id.clone() } }

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
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
            return write!(f, "unknown")
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
        }
        write!(f, "{}", result)
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}", &self))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceSubset {
    pub preview: PreviewSource,
    pub id: String,
    pub sourcekey: SourceKey,
    pub showed: Vec<Showed>,
    pub time: Date,
}

/// A general structure for parameters, graph classes, any other structures.
// todo - Set is a heavy structure and ideally should not have #derive[clone]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    pub preview: PreviewSet,
    pub id: String,
    pub name: String,
    pub typ: PreviewType,
    pub aka: Vec<String>,
    pub abbr: Option<String>,
    pub tags: Vec<PreviewTag>,
    pub providers: Vec<ProviderLink>,
    pub timeline: Vec<SourceSubset>,
    pub equivsets: Vec<PreviewSet>,
    pub supersets: Sets,
    pub subsets: Sets,
    pub super_exclusions: Sets,
    pub sub_exclusions: Sets,
    pub unknown: Sets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderLink {
    pub provider: Provider,
    pub set: PreviewSet,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub preview: PreviewTag,
    pub id: String,
    pub name: String,
    pub description: String,
    pub sets: Vec<PreviewSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub preview: PreviewSource,
    pub id: String,
    pub sourcekey: SourceKey,
    pub showed: Vec<Showed>,
    pub time: Date,
    pub drawings: Vec<Drawing>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartialResult {
    pub handle: usize,
    pub created_by: CreatedBy,
}

pub struct PartialResultsBuilder {
    pub arr: Vec<PartialResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub sets: Vec<Set>,
    pub relations: Vec<Relation>,
    pub sources: Vec<Source>,
    pub providers: Vec<Provider>,
    pub tags: Vec<Tag>,
    pub partial_results: Vec<PartialResult>,
    #[serde(skip)]
    pub set_idx: HashMap<PreviewSet, usize>,
    #[serde(skip)]
    pub relation_idx: HashMap<(PreviewSet, PreviewSet), usize>,
}

pub fn compute_set_idx(sets: &Vec<Set>) -> HashMap<PreviewSet, usize> {
    trace!("computing set indices");
    let mut set_idx: HashMap<PreviewSet, usize> = HashMap::new();
    for (idx, set) in sets.iter().enumerate() {
        set_idx.insert(set.preview.clone(), idx);
    }
    set_idx
}

pub fn compute_relation_idx(relations: &Vec<Relation>) -> HashMap<(PreviewSet, PreviewSet), usize> {
    trace!("computing relation indices");
    let mut relation_idx: HashMap<(PreviewSet, PreviewSet), usize> = HashMap::new();
    for (idx, relation) in relations.iter().enumerate() {
        let pair = (relation.subset.clone(), relation.superset.clone());
        assert!(!relation_idx.contains_key(&pair));
        relation_idx.insert(pair, idx);
    }
    relation_idx
}

impl Data {
    pub fn new(mut sets: Vec<Set>, relations: Vec<Relation>, sources: Vec<Source>, providers: Vec<Provider>, tags: Vec<Tag>, partial_results: Vec<PartialResult>) -> Self {
        trace!("new data");
        sets.sort_by_key(|x|x.name.to_lowercase().clone());
        let set_idx = compute_set_idx(&sets);
        let relation_idx = compute_relation_idx(&relations);
        Self {
            sets,
            relations,
            sources,
            providers,
            tags,
            set_idx,
            relation_idx,
            partial_results,
        }
    }

    pub fn recompute(&mut self) {
        self.set_idx = compute_set_idx(&self.sets);
        self.relation_idx = compute_relation_idx(&self.relations);
    }

    pub fn get_sets<I>(&self, key: I) -> Vec<&Set>
        where I: IntoIterator<Item = PreviewSet>, {
        key.into_iter().map(|x|self.get_set(&x)).collect()
    }

    pub fn get_set(&self, key: &PreviewSet) -> &Set {
        trace!("get set {} {}", key.id, key.name);
        let idx: usize = *self.set_idx.get(&key).expect(&format!("preview set not found {:?}", key));
        &self.sets[idx]
    }

    pub fn get_relation(&self, subset: &PreviewSet, superset: &PreviewSet) -> Option<&Relation> {
        trace!("get relation from {} {} to {} {}", subset.id, subset.name, superset.id, superset.name);
        let key = (subset.clone(), superset.clone());
        match self.relation_idx.get(&key) {
            Some(&idx) => Some(&self.relations[idx]),
            None => None,
        }
    }

    pub fn get_partial_result(&self, handle: &usize) -> &PartialResult {
        self.partial_results.get(*handle).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: String,
    pub handle: usize,
    pub preview: PreviewRelation,
    /// If inclusion, then subset is the parameter above which is potentially bigger for the same graph.
    pub subset: PreviewSet,
    /// If inclusion, then superset is the parameter below which is potentially smaller for the same graph.
    pub superset: PreviewSet,
    pub cpx: SourcedCpxInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Showed {
    pub id: String,
    pub text: String,
    pub fact: ShowedFact,
    pub page: Page,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShowedFact {
    Relation(PreviewRelation),
    Definition(PreviewSet),
    Citation(PreviewSource),
}
