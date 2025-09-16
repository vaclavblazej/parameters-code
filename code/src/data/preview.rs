//! Preview versions of the full structures.

use crate::{
    general::enums::{CpxInfo, Page, SourceKey, SourcedCpxInfo},
    work::date::Date,
};

use serde::{Deserialize, Serialize};

use super::{
    core::{Relation, Set, Source, Tag}, id::{
        BaseId, Id, PreviewId, PreviewRelationId, PreviewSetId, PreviewSourceId, PreviewTagId, RelationId
    }
};

pub trait HasPreview<T> {
    fn preview(&self) -> T;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum Own {
    Has,
    Is,
}

impl Own {

    pub fn to_string(&self, truth: bool, plural: bool) -> String {
        String::from(match (self, truth, plural) {
            (Own::Is, true, false) => "is",
            (Own::Is, true, true) => "are",
            (Own::Has, true, false) => "has",
            (Own::Has, true, true) => "have",
            (Own::Is, false, false) => "is not",
            (Own::Is, false, true) => "are not",
            (Own::Has, false, false) => "does not have",
            (Own::Has, false, true) => "do not have",
        })
    }

}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum PreviewType {
    Parameter,
    GraphClass,
    Property(Own),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewSourceKey {
    Bibtex { key: String },
    Online { url: String },
    Other { name: String },
}

impl PreviewSourceKey {
    pub fn to_str(&self) -> String {
        match self {
            PreviewSourceKey::Bibtex { key } => key.clone(),
            PreviewSourceKey::Online { url } => url.clone(),
            PreviewSourceKey::Other { name } => name.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct PreviewSource {
    pub id: PreviewSourceId,
    pub sourcekey: SourceKey,
    pub time: Date,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct PreviewSet {
    pub id: PreviewSetId,
    pub name: String,
    pub typ: PreviewType,
    pub relevance: u32,
}

impl PreviewSet {
    pub fn is_more_relevant_than(&self, other: &PreviewSet) -> bool {
        (self.relevance == other.relevance && self.id < other.id)
            || self.relevance > other.relevance
    }
}

impl HasPreview<PreviewSet> for Set {
    fn preview(&self) -> PreviewSet {
        PreviewSet {
            id: self.id.preview(),
            relevance: self.relevance,
            typ: self.typ.clone(),
            name: self.name.clone(),
        }
    }
}

impl HasPreview<PreviewTag> for Tag {
    fn preview(&self) -> PreviewTag {
        PreviewTag {
            id: self.id.preview(),
            name: self.name.clone(),
        }
    }
}

impl HasPreview<PreviewRelation> for Relation {
    fn preview(&self) -> PreviewRelation {
        PreviewRelation {
            id: self.id.preview(),
            subset: self.subset.clone(),
            superset: self.superset.clone(),
            cpx: CpxInfo::from(self.cpx.clone()),
        }
    }
}

impl HasPreview<PreviewSource> for Source {
    fn preview(&self) -> PreviewSource {
        PreviewSource {
            id: self.id.preview(),
            sourcekey: self.sourcekey.clone(),
            time: self.time.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PreviewRelation {
    pub id: PreviewRelationId,
    pub subset: PreviewSet,
    pub superset: PreviewSet,
    pub cpx: CpxInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkRelation {
    pub subset: PreviewSetId,
    pub superset: PreviewSetId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewTag {
    pub id: PreviewTagId,
    pub name: String,
}
