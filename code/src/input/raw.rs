//! Raw immutable datapoints

use std::collections::HashMap;

use crate::{
    data::id::{
        Id, PreviewId, PreviewProviderId, PreviewRelationId, PreviewSetId, PreviewSourceId, PreviewTagId, ProviderId, RelationId, SetId, ShowedId, SourceId, TagId, BaseId
    },
    general::enums::{CpxInfo, Drawing, Page, RawDrawing, TransferGroup},
};

#[derive(Debug, PartialEq, Clone)]
pub enum RawType {
    Parameter,
    GraphClass,
}

#[derive(Debug)]
pub enum Composition {
    None,
    Intersection(Vec<PreviewSetId>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawSourceKey {
    Bibtex { key: String },
    Online { url: String },
    Other { name: String, description: String },
}

impl RawSourceKey {
    pub fn to_str(&self) -> String {
        match self {
            RawSourceKey::Bibtex { key } => key.clone(),
            RawSourceKey::Online { url } => url.clone(),
            RawSourceKey::Other {
                name,
                description: _,
            } => name.clone(),
        }
    }
}

#[derive(Debug)]
pub struct RawSource {
    pub id: SourceId,
    pub rawsourcekey: RawSourceKey,
    pub relevance: u32, // from 0 to 9
    pub drawings: Vec<RawDrawing>,
}

#[derive(Debug)]
pub struct BuiltRawSource {
    pub id: SourceId,
    pub rawsourcekey: RawSourceKey,
    pub relevance: u32, // from 0 to 9
    pub drawings: Vec<RawDrawing>,
}

#[derive(Debug)]
pub struct BuiltRawSet {
    pub id: SetId,
    pub name: String,
    pub typ: RawType,
    pub composed: Composition,
    pub relevance: u32, // from 0 to 9
    pub aka: Vec<String>,
    pub abbr: Option<String>,
    pub tags: Vec<PreviewTagId>,
}

#[derive(Debug)]
pub struct RawSet {
    pub id: SetId,
    pub name: String,
    pub typ: RawType,
    pub composed: Composition,
    pub relevance: u32, // from 0 to 9
    pub aka: Vec<String>,
    pub abbr: Option<String>,
}

impl BuiltRawSet {
    pub fn new(
        id: String,
        name: String,
        typ: RawType,
        composed: Composition,
        relevance: u32,
    ) -> Self {
        Self {
            id: Id::new(id),
            name,
            typ,
            composed,
            relevance,
            tags: vec![],
            aka: vec![],
            abbr: None,
        }
    }
}

/// Holds into on whether bounded `subset` implies bounded `superset`.
#[derive(Debug)]
pub struct RawRelation {
    pub id: RelationId,
    /// If inclusion, then subset is the parameter above which is potentially bigger for the same graph.
    pub subset: PreviewSetId,
    /// If inclusion, then superset is the parameter below which is potentially smaller for the same graph.
    pub superset: PreviewSetId,
    pub cpx: CpxInfo,
}

impl RawRelation {
    pub fn new(subset: &PreviewSetId, superset: &PreviewSetId, cpx: CpxInfo) -> RawRelation {
        RawRelation {
            id: RelationId::new(subset, superset),
            subset: subset.clone(),
            superset: superset.clone(),
            cpx,
        }
    }
}

#[derive(Debug)]
pub struct RawTag {
    pub id: TagId,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct RawProvider {
    pub id: ProviderId,
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct RawProviderLink {
    pub provider: PreviewProviderId,
    pub set: PreviewSetId,
    pub url: String,
}

pub struct RawData {
    pub sets: Vec<RawSet>,
    pub relations: Vec<RawRelation>,
    pub factoids: Vec<(PreviewSourceId, RawShowed)>,
    pub sources: Vec<RawSource>,
    pub providers: Vec<RawProvider>,
    pub provider_links: Vec<RawProviderLink>,
    pub tags: Vec<RawTag>,
    pub tag_set: Vec<(PreviewTagId, PreviewSetId)>,
    pub transfer: HashMap<TransferGroup, Vec<(PreviewSetId, PreviewSetId)>>,
}

impl RawData {
    pub fn new() -> Self {
        Self {
            sets: Vec::new(),
            relations: Vec::new(),
            factoids: Vec::new(),
            sources: Vec::new(),
            providers: Vec::new(),
            provider_links: Vec::new(),
            tags: Vec::new(),
            tag_set: Vec::new(),
            transfer: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct RawShowed {
    pub id: ShowedId,
    pub text: String,
    pub fact: RawShowedFact,
    pub page: Page,
}

#[derive(Debug)]
pub enum RawShowedFact {
    Relation(PreviewRelationId),
    Definition(PreviewSetId),
    // Citation(RawSource),
}
