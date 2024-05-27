//! Preview versions of the full structures.

use crate::general::enums::{CpxInfo, Page};

use super::data::{Date, Showed};


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PreviewKind {
    Parameter,
    GraphClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewSourceKey {
    Bibtex { key: String },
    Online { url: String },
    Unknown,
}

impl PreviewSourceKey {
    pub fn to_str(&self) -> String{
        match self {
            PreviewSourceKey::Bibtex { key } => key.clone(),
            PreviewSourceKey::Online { url } => url.clone(),
            PreviewSourceKey::Unknown => "unknown".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PreviewSource {
    pub id: String,
    pub sourcekey: PreviewSourceKey,
    pub time: Date,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PreviewSet {
    pub id: String,
    pub name: String,
    pub kind: PreviewKind,
    pub popularity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreviewRelation {
    pub id: String,
    pub subset: PreviewSet,
    pub superset: PreviewSet,
    pub cpx: CpxInfo,
}

#[derive(Debug, Clone)]
pub struct PreviewTopic {
    pub id: String,
    pub name: String,
}

