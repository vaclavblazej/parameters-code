use core::fmt;
use std::{fmt::Display, marker::PhantomData};

use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};

use super::{
    core::{Relation, Set, Source, Tag},
    preview::PreviewSet,
};

// todo eq between Id and PreviewId

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

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct TypeRelation;
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct TypeSource;
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct TypeSet;
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct TypeTag;
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct TypeProvider;
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct TypeShowed;
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct TypeSubset;

pub type PreviewRelationId = PreviewId<TypeRelation>;
pub type PreviewSourceId = PreviewId<TypeSource>;
pub type PreviewSetId = PreviewId<TypeSet>;
pub type PreviewTagId = PreviewId<TypeTag>;
pub type PreviewProviderId = PreviewId<TypeProvider>;
pub type PreviewSubsetId = PreviewId<TypeSubset>;
pub type PreviewShowedId = PreviewId<TypeShowed>;

// pub type RelationId = Id<TypeRelation>;
pub type SourceId = Id<TypeSource>;
pub type SetId = Id<TypeSet>;
pub type TagId = Id<TypeTag>;
pub type ProviderId = Id<TypeProvider>;
pub type ShowedId = Id<TypeShowed>;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct RelationId {
    code: String,
}

impl RelationId {
    pub fn new(subset: &PreviewSetId, superset: &PreviewSetId) -> Self {
        RelationId::create(format!("{}_{}", subset.code, superset.code))
    }
}

impl Display for RelationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

impl AbstractId<String> for RelationId {
    fn id(&self) -> String {
        self.code.clone()
    }
    fn create(code: String) -> Self {
        RelationId { code }
    }
}

impl BaseId<PreviewRelationId> for RelationId {
    fn get_tmp() -> Self {
        Self {
            code: format!(
                "_{}_{}",
                Alphanumeric.sample_string(&mut rand::rng(), 5),
                Alphanumeric.sample_string(&mut rand::rng(), 6)
            ),
        }
    }
    fn preview(&self) -> PreviewId<TypeRelation> {
        PreviewRelationId::create(self.id())
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait HasId {
    fn id(&self) -> String;
}
impl HasId for Set {
    fn id(&self) -> String {
        self.id.to_string()
    }
}
impl HasId for Relation {
    fn id(&self) -> String {
        self.id.to_string()
    }
}
impl HasId for Source {
    fn id(&self) -> String {
        self.id.to_string()
    }
}
impl HasId for Tag {
    fn id(&self) -> String {
        self.id.to_string()
    }
}
