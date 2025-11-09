//! Collection of preview datapoints together with their mutual relations.
//! More complex structures use Preview structures to refer to each other.

use std::collections::HashMap;
use std::fmt;

use log::trace;
use serde::{Deserialize, Serialize};

use crate::data::preview::{HasPreview, PreviewRelation, PreviewSet, PreviewSource, PreviewTag, PreviewType};
use crate::general::enums::{
    Cpx, CpxInfo, CpxTime, CreatedBy, Drawing, Page, SourceKey, SourcedCpxInfo, TransferGroup,
};
use crate::work::date::Date;
use crate::work::processing::{RelatedSets, Sets};

use super::id::{
    Id, BaseId, PreviewId, PreviewProviderId, PreviewRelationId, PreviewSetId, PreviewShowedId, PreviewSourceId, PreviewSubsetId, ProviderId, RelationId, SetId, ShowedId, SourceId, TagId
};
use super::preview::WorkRelation;


#[derive(Debug, Serialize, Deserialize)]
pub struct SourceSubset {
    pub preview: PreviewSource,
    pub source: PreviewSourceId,
    pub sourcekey: SourceKey,
    pub showed: Vec<PreviewShowed>,
}

/// A general structure for parameters, graph classes, any other structures.
#[derive(Debug, Serialize, Deserialize)]
pub struct Set {
    pub id: SetId,
    pub name: String,
    pub typ: PreviewType,
    pub relevance: u32,
    pub aka: Vec<String>,
    pub abbr: Option<String>,
    pub tags: Vec<PreviewTag>,
    pub providers: Vec<ProviderLink>,
    pub timeline: Vec<SourceSubset>,
    pub related_sets: RelatedSets,
    pub displayed_definition: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub url: String,
    pub links: Vec<ProviderLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderLink {
    pub provider_name: String,
    pub set: PreviewSetId,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub description: String,
    pub sets: Vec<PreviewSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: SourceId,
    pub sourcekey: SourceKey,
    pub showed: Vec<PreviewShowed>,
    pub time: Date,
    pub drawings: Vec<Drawing>,
}

// todo, remove clone?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartialResult {
    pub handle: usize,
    pub created_by: CreatedBy,
    pub relation: WorkRelation,
    pub cpx: CpxInfo,
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
    pub set_id_idx: HashMap<PreviewSetId, usize>,
    #[serde(skip)]
    pub relation_idx: HashMap<(PreviewSet, PreviewSet), usize>,
    #[serde(skip)]
    pub relation_id_idx: HashMap<PreviewRelationId, usize>,
}

fn compute_set_idx(sets: &[Set]) -> HashMap<PreviewSet, usize> {
    trace!("computing set indices");
    let mut set_idx: HashMap<PreviewSet, usize> = HashMap::new();
    for (idx, set) in sets.iter().enumerate() {
        set_idx.insert(set.preview(), idx);
    }
    set_idx
}

fn compute_relation_idx(relations: &[Relation]) -> HashMap<(PreviewSet, PreviewSet), usize> {
    trace!("computing relation indices");
    let mut relation_idx: HashMap<(PreviewSet, PreviewSet), usize> = HashMap::new();
    for (idx, relation) in relations.iter().enumerate() {
        let pair = (relation.subset.clone(), relation.superset.clone());
        assert!(!relation_idx.contains_key(&pair));
        relation_idx.insert(pair, idx);
    }
    relation_idx
}

fn compute_relation_id_idx(relations: &[Relation]) -> HashMap<PreviewRelationId, usize> {
    let mut relation_id_idx = HashMap::new();
    for (idx, relation) in relations.iter().enumerate() {
        relation_id_idx.insert(relation.id.preview(), idx);
    }
    relation_id_idx
}

fn compute_set_id_idx(sets: &[Set]) -> HashMap<PreviewSetId, usize> {
    let mut set_id_idx = HashMap::new();
    for (idx, set) in sets.iter().enumerate() {
        set_id_idx.insert(set.id.preview(), idx);
    }
    set_id_idx
}

impl Data {
    pub fn new(
        mut sets: Vec<Set>,
        relations: Vec<Relation>,
        sources: Vec<Source>,
        providers: Vec<Provider>,
        tags: Vec<Tag>,
        partial_results: Vec<PartialResult>,
    ) -> Self {
        trace!("new data");
        sets.sort_by_key(|x| x.name.to_lowercase().clone());
        let set_idx = compute_set_idx(&sets);
        let set_id_idx = compute_set_id_idx(&sets);
        let relation_idx = compute_relation_idx(&relations);
        let relation_id_idx = compute_relation_id_idx(&relations);
        Self {
            sets,
            relations,
            sources,
            providers,
            tags,
            partial_results,
            set_idx,
            set_id_idx,
            relation_idx,
            relation_id_idx,
        }
    }

    pub fn recompute(&mut self) {
        self.set_idx = compute_set_idx(&self.sets);
        self.set_id_idx = compute_set_id_idx(&self.sets);
        self.relation_idx = compute_relation_idx(&self.relations);
        self.relation_id_idx = compute_relation_id_idx(&self.relations);
    }

    pub fn get_sets<I>(&self, key: I) -> Vec<&Set>
    where
        I: IntoIterator<Item = PreviewSet>,
    {
        key.into_iter().map(|x| self.get_set(&x)).collect()
    }

    pub fn get_set(&self, key: &PreviewSet) -> &Set {
        trace!("get set {} {}", key.id, key.name);
        let idx: usize = *self
            .set_idx
            .get(key)
            .unwrap_or_else(||panic!("preview set not found {:?}", key));
        &self.sets[idx]
    }

    pub fn get_set_by_id(&self, id: &PreviewSetId) -> &Set {
        trace!("get set id {}", id);
        let idx: usize = *self
            .set_id_idx
            .get(id)
            .unwrap_or_else(||panic!("preview set not found {:?}", id));
        &self.sets[idx]
    }

    pub fn get_relation(&self, subset: &PreviewSet, superset: &PreviewSet) -> Option<&Relation> {
        trace!(
            "get relation from {} {} to {} {}",
            subset.id,
            subset.name,
            superset.id,
            superset.name
        );
        let key = (subset.clone(), superset.clone());
        match self.relation_idx.get(&key) {
            Some(&idx) => Some(&self.relations[idx]),
            None => None,
        }
    }

    pub fn get_relation_by_id(&self, id: &PreviewRelationId) -> Option<&Relation> {
        trace!("get relation id {}", id);
        match self.relation_id_idx.get(id) {
            Some(&idx) => Some(&self.relations[idx]),
            None => None,
        }
    }

    pub fn get_relation_by_ids(&self, a: &PreviewSetId, b: &PreviewSetId) -> Option<&Relation> {
        trace!("get relation ids {} {}", a, b);
        self.get_relation_by_id(&RelationId::new(a, b).preview())
    }

    pub fn get_partial_result(&self, handle: &usize) -> &PartialResult {
        self.partial_results.get(*handle).unwrap_or_else(||panic!("handle {} not found in the partial results", handle))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub id: RelationId,
    pub handle: usize,
    /// If inclusion, then subset is the parameter above which is potentially bigger for the same graph.
    pub subset: PreviewSet,
    /// If inclusion, then superset is the parameter below which is potentially smaller for the same graph.
    pub superset: PreviewSet,
    pub cpx: SourcedCpxInfo,
}

// showed is a lightweight structure, hence it has no preview variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewShowed {
    pub id: PreviewShowedId,
    pub text: String,
    pub fact: ShowedFact,
    pub page: Page,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShowedFact {
    Relation(ShowedStatus, PreviewRelation),
    Definition(ShowedStatus, PreviewSetId),
    // Citation(PreviewSource),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShowedStatus {
    Assumed,
    Original,
    Derivative,
    Noted(NotedSource),
    Conjectured,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotedSource {
    Text(String),
    Source(PreviewSourceId),
    Omitted,
    Todo,
}
