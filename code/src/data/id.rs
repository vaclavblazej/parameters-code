use core::fmt;
use std::{fmt::Display, marker::PhantomData};

use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};

pub trait AbstractId<T> {
    fn id(&self) -> T;
    fn create(_: T) -> Self;
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct PreviewId<T> {
    pub code: String,
    _marker: PhantomData<T>,
}

impl<T> Display for PreviewId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

impl<T> From<String> for PreviewId<T> {
    fn from(code: String) -> PreviewId<T> {
        // todo check existence
        Self::create(code)
    }
}

impl<T> From<&str> for PreviewId<T> {
    fn from(code: &str) -> PreviewId<T> {
        Self::from(String::from(code))
    }
}

impl<T> AbstractId<String> for PreviewId<T> {
    fn id(&self) -> String {
        self.code.clone()
    }
    fn create(code: String) -> Self {
        PreviewId {
            code,
            _marker: PhantomData,
        }
    }
}

pub trait BaseId<T>: AbstractId<String>
where
    T: AbstractId<String>,
{
    fn get_tmp() -> Self;
    fn preview(&self) -> T;
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct Id<T> {
    code: String,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn preview(&self) -> PreviewId<T> {
        PreviewId::create(self.id())
    }
    pub fn new(code: String) -> Self {
        Id::create(code)
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

impl<T> AbstractId<String> for Id<T> {
    fn id(&self) -> String {
        self.code.clone()
    }
    fn create(code: String) -> Self {
        Self {
            code,
            _marker: PhantomData,
        }
    }
}

impl<T> BaseId<PreviewId<T>> for Id<T> {
    fn get_tmp() -> Self {
        Self {
            code: format!("_{}", Alphanumeric.sample_string(&mut rand::rng(), 5)),
            _marker: PhantomData,
        }
    }
    fn preview(&self) -> PreviewId<T> {
        PreviewId::create(self.id())
    }
}

impl<T> PartialEq<PreviewId<T>> for Id<T> {
    fn eq(&self, other: &PreviewId<T>) -> bool {
        self.code == other.code
    }
}

impl<T> PartialEq<Id<T>> for PreviewId<T> {
    fn eq(&self, other: &Id<T>) -> bool {
        self.code == other.code
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! define_type {
    ($typename:ident, $idname:ident, $prevewidname:ident) => {
        #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
        pub struct $typename;
        pub type $prevewidname = PreviewId<$typename>;
        pub type $idname = Id<$typename>;
    };
}

define_type!(TypeGraph, GraphId, PreviewGraphId);
define_type!(TypeGraphClass, GraphClassId, PreviewGraphClassId);
define_type!(TypeOperation, OperationId, PreviewOperationId);
define_type!(
    TypeGraphClassProperty,
    GraphClassPropertyId,
    PreviewGraphClassPropertyId
);
define_type!(
    TypeGraphClassRelation,
    GraphClassRelationId,
    PreviewGraphClassRelationId
);
define_type!(TypeGraphRelation, GraphRelationId, PreviewGraphRelationId);
define_type!(TypeLogicFragment, LogicFragmentId, PreviewLogicFragmentId);
define_type!(TypeParameter, ParameterId, PreviewParameterId);
define_type!(
    TypeParametricGraphClass,
    ParametricGraphClassId,
    PreviewParametricGraphClassId
);
define_type!(
    TypeParametricParameter,
    ParametricParameterId,
    PreviewParametricParameterId
);
define_type!(TypeProblem, ProblemId, PreviewProblemId);
define_type!(TypeProvider, ProviderId, PreviewProviderId);
define_type!(TypeShowed, ShowedId, PreviewShowedId);
define_type!(TypeSource, SourceId, PreviewSourceId);
define_type!(TypeSubset, SubsetId, PreviewSubsetId);
define_type!(TypeTag, TagId, PreviewTagId);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct TypeRelation;

pub type PreviewRelationId = PreviewId<TypeRelation>;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct RelationId<T> {
    code: String,
    _marker: PhantomData<T>,
}

impl<T> RelationId<T> {
    pub fn new(subset: &Id<T>, superset: &Id<T>) -> Self {
        RelationId::create(format!("{}_{}", subset.code, superset.code))
    }
}

impl<T> Display for RelationId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

impl<T> AbstractId<String> for RelationId<T> {
    fn id(&self) -> String {
        self.code.clone()
    }
    fn create(code: String) -> Self {
        RelationId {
            code,
            _marker: PhantomData,
        }
    }
}

impl<T> BaseId<PreviewRelationId> for RelationId<T> {
    fn get_tmp() -> Self {
        Self {
            code: format!(
                "_{}_{}",
                Alphanumeric.sample_string(&mut rand::rng(), 5),
                Alphanumeric.sample_string(&mut rand::rng(), 6)
            ),
            _marker: PhantomData,
        }
    }
    fn preview(&self) -> PreviewId<TypeRelation> {
        PreviewRelationId::create(self.id())
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait HasId {
    fn id(&self) -> String {
        self.id.to_string()
    }
}
impl<T> HasId for dyn HasPreviewId<T> {}

pub trait HasPreviewId<T> {
    fn preview(&self) -> T {
        self.id.preview()
    }
}
