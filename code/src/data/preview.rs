//! Preview versions of the full structures.

use crate::general::enums::{CpxInfo, Page, SourceKey, SourcedCpxInfo};

use serde::{Serialize, Deserialize};

use super::data::{Date, Showed};


#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum PreviewType {
    Parameter,
    GraphClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewSourceKey {
    Bibtex { key: String },
    Online { url: String },
    Other { name: String },
}

impl PreviewSourceKey {
    pub fn to_str(&self) -> String{
        match self {
            PreviewSourceKey::Bibtex { key } => key.clone(),
            PreviewSourceKey::Online { url } => url.clone(),
            PreviewSourceKey::Other { name } => name.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct PreviewSource {
    pub id: String,
    pub sourcekey: SourceKey,
    pub time: Date,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct PreviewSet {
    pub id: String,
    pub name: String,
    pub typ: PreviewType,
    pub relevance: u32,
}

impl PreviewSet {
    pub fn mock(id: &str) -> PreviewSet{
        PreviewSet {
            id: id.into(),
            name: "".into(),
            typ: PreviewType::Parameter,
            relevance: 0,
        }
    }

    pub fn is_more_relevant_than(&self, other: &PreviewSet) -> bool {
        (self.relevance == other.relevance && self.id < other.id) || self.relevance > other.relevance
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PreviewRelation {
    pub id: String,
    pub subset: PreviewSet,
    pub superset: PreviewSet,
    pub cpx: CpxInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewTag {
    pub id: String,
    pub name: String,
}

