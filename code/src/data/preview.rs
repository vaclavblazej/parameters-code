//! Preview versions of the data.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::data::data::*;
use crate::data::enums::*;
use crate::data::id::*;
use crate::work::date::Date;

pub trait HasPreview<T> {
    fn preview(&self) -> T;
}

macro_rules! define_preview_id_name {
    ($main:ident, $preview:ident, $previewid:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $preview {
            pub id: $previewid,
            pub name: String,
        }
        impl HasPreviewId<$previewid> for $preview {}
        impl HasPreview<$preview> for $main {
            fn preview(&self) -> $preview {
                $preview {
                    id: self.id.preview(),
                    name: self.name.clone(),
                }
            }
        }
    };
}

macro_rules! define_preview_id_name_relevance {
    ($main:ident, $preview:ident, $previewid:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $preview {
            pub id: $previewid,
            pub relevance: u32,
            pub name: String,
        }
        impl HasPreviewId<$previewid> for $preview {}
        impl HasPreview<$preview> for $main {
            fn preview(&self) -> $preview {
                $preview {
                    id: self.id.preview(),
                    relevance: self.relevance,
                    name: self.core.name.clone(),
                }
            }
        }
    };
}

define_preview_id_name!(Tag, PreviewTag, PreviewTagId);
define_preview_id_name!(LogicFragment, PreviewLogicFragment, PreviewLogicFragmentId);
define_preview_id_name!(Operation, PreviewOperation, PreviewOperationId);
define_preview_id_name!(Provider, PreviewProvider, PreviewProviderId);
define_preview_id_name!(GraphRelation, PreviewGraphRelation, PreviewGraphRelationId);
define_preview_id_name!(
    GraphClassRelation,
    PreviewGraphClassRelation,
    PreviewGraphClassRelationId
);

define_preview_id_name_relevance!(Graph, PreviewGraph, PreviewGraphId);
define_preview_id_name_relevance!(GraphClass, PreviewGraphClass, PreviewGraphClassId);
define_preview_id_name_relevance!(
    HigherOrderParameter,
    PreviewHigherOrderParameter,
    PreviewHigherOrderParameterId
);
define_preview_id_name_relevance!(
    ParametricParameter,
    PreviewParametricParameter,
    PreviewParametricParameterId
);
define_preview_id_name_relevance!(
    ParametricGraphClass,
    PreviewParametricGraphClass,
    PreviewParametricGraphClassId
);
define_preview_id_name_relevance!(Parameter, PreviewParameter, PreviewParameterId);
define_preview_id_name_relevance!(
    GraphClassProperty,
    PreviewGraphClassProperty,
    PreviewGraphClassPropertyId
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewSourceKey {
    Bibtex { key: String },
    Online { url: String },
    Other { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct PreviewSource {
    pub id: PreviewSourceId,
    pub sourcekey: SourceKey,
    pub time: Date,
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

// impl Display for PreviewSourceKey {
// fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// write!(f, "{}", match self {
// PreviewSourceKey::Bibtex { key } => key,
// PreviewSourceKey::Online { url } => url,
// PreviewSourceKey::Other { name } => name,
// })
// }
// }

// #[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
// pub struct PreviewSet {
// pub id: PreviewSetId,
// pub name: String,
// pub typ: PreviewType,
// pub relevance: u32,
// }

// impl HasPreview<PreviewRelation> for Relation {
// fn preview(&self) -> PreviewRelation {
// PreviewRelation {
// id: self.id.preview(),
// subset: self.subset.clone(),
// superset: self.superset.clone(),
// cpx: CpxInfo::from(self.cpx.clone()),
// }
// }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct PreviewRelation {
// pub id: PreviewRelationId,
// pub subset: PreviewSet,
// pub superset: PreviewSet,
// pub cpx: CpxInfo,
// }
