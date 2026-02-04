//! Preview versions of the data.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::data::data::*;
use crate::data::date::Date;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::*;
use crate::input::raw::*;

pub trait HasPreview<T> {
    fn preview(&self) -> T;
}

#[macro_export]
macro_rules! tie_raw_to_previewid {
    ($mytype:ident, $previewid:ident) => {
        impl HasPreviewId for $mytype {
            type PreviewId = $previewid;
            fn previewid(&self) -> Self::PreviewId {
                self.id.preview()
            }
        }
        impl HasId for $mytype {
            fn id(&self) -> String {
                self.id.to_string()
            }
        }
    };
}

#[macro_export]
macro_rules! tie_data_to_previewid {
    ($mytype:ident, $previewid:ident) => {
        impl HasPreviewId for $mytype {
            type PreviewId = $previewid;
            fn previewid(&self) -> Self::PreviewId {
                self.id.preview()
            }
        }
        impl HasId for $mytype {
            fn id(&self) -> String {
                self.id.to_string()
            }
        }
        impl IsPreviewIdOf for $previewid {
            type MainStructure = $mytype;
        }
    };
}

macro_rules! tie_preview_to_previewid {
    ($mytype:ident, $previewid:ident) => {
        impl HasPreviewId for $mytype {
            type PreviewId = $previewid;
            fn previewid(&self) -> Self::PreviewId {
                self.id.clone()
            }
        }
        impl IsIdOfPreview for $previewid {
            type PreviewStructure = $mytype;
        }
    };
}

macro_rules! define_preview_id_name {
    ($main:ident, $rawmain:ident, $preview:ident, $previewid:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $preview {
            pub id: $previewid,
            pub name: NameCore,
        }
        tie_preview_to_previewid!($preview, $previewid);
        impl HasPreview<$preview> for $main {
            fn preview(&self) -> $preview {
                $preview {
                    id: self.id.preview(),
                    name: self.name_core.clone(),
                }
            }
        }
        impl HasPreview<$preview> for $rawmain {
            fn preview(&self) -> $preview {
                $preview {
                    id: self.id.preview(),
                    name: self.name_core.clone(),
                }
            }
        }
    };
}

macro_rules! define_preview_id_name_score {
    ($main:ident, $rawmain:ident, $preview:ident, $previewid:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
        pub struct $preview {
            pub id: $previewid,
            pub score: u32,
            pub name_core: NameCore,
        }
        tie_preview_to_previewid!($preview, $previewid);
        impl HasPreview<$preview> for &$main {
            fn preview(&self) -> $preview {
                $preview {
                    id: self.id.preview(),
                    score: self.score,
                    name_core: self.name_core.clone(),
                }
            }
        }
        impl HasPreview<$preview> for $rawmain {
            fn preview(&self) -> $preview {
                $preview {
                    id: self.id.preview(),
                    score: self.score,
                    name_core: self.name_core.clone(),
                }
            }
        }
    };
}

define_preview_id_name!(Tag, RawTag, PreviewTag, PreviewTagId);
define_preview_id_name!(
    LogicFragment,
    RawLogicFragment,
    PreviewLogicFragment,
    PreviewLogicFragmentId
);
define_preview_id_name!(
    Operation,
    RawOperation,
    PreviewOperation,
    PreviewOperationId
);
define_preview_id_name!(Provider, RawProvider, PreviewProvider, PreviewProviderId);
define_preview_id_name!(
    GraphRelation,
    RawGraphRelation,
    PreviewGraphRelation,
    PreviewGraphRelationId
);
define_preview_id_name!(
    GraphClassRelation,
    RawGraphClassRelation,
    PreviewGraphClassRelation,
    PreviewGraphClassRelationId
);

define_preview_id_name_score!(Graph, RawGraph, PreviewGraph, PreviewGraphId);
define_preview_id_name_score!(
    GraphClass,
    RawGraphClass,
    PreviewGraphClass,
    PreviewGraphClassId
);
define_preview_id_name_score!(
    ParametricParameter,
    RawParametricParameter,
    PreviewParametricParameter,
    PreviewParametricParameterId
);
define_preview_id_name_score!(
    ParametricGraphClass,
    RawParametricGraphClass,
    PreviewParametricGraphClass,
    PreviewParametricGraphClassId
);
define_preview_id_name_score!(
    Parameter,
    RawParameter,
    PreviewParameter,
    PreviewParameterId
);
define_preview_id_name_score!(
    GraphClassProperty,
    RawGraphClassProperty,
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

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct PreviewWrote {
    pub text: String,
    pub page: Page,
}

impl HasPreview<PreviewWrote> for Wrote {
    fn preview(&self) -> PreviewWrote {
        PreviewWrote {
            text: self.text.clone(),
            page: self.page.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct PreviewProblem {
    pub id: PreviewProblemId,
    pub name_core: NameCore,
}

impl HasPreview<PreviewProblem> for Problem {
    fn preview(&self) -> PreviewProblem {
        PreviewProblem {
            id: self.id.preview(),
            name_core: self.name_core.clone(),
        }
    }
}
