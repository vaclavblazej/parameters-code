//! Preview versions of the full structures.

use crate::general::enums::{CpxInfo, Page, SourceKey};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PreviewRelation {
    pub id: String,
    pub subset: PreviewSet,
    pub superset: PreviewSet,
    pub cpx: CpxInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewTopic {
    pub id: String,
    pub name: String,
}

