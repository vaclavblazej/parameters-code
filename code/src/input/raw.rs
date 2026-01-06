//! All the raw structures; these are used solely for keeping the input data
//! of the collection.

use std::collections::HashMap;

use crate::data::data::{NameCore, Tagged};
use crate::data::enums::*;
use crate::data::id::*;
use crate::input::build::CollectionBuilder;
use crate::input::raw_enums::*;

pub trait Defines<S> {
    fn defines(&self) -> S;
}

pub trait Concrete<N, S> {
    fn concretize(&self, value: N) -> S;
}

#[derive(Debug)]
pub struct RawTag {
    pub id: TagId,
    pub name: String,
    pub description: String,
}
impl HasPreviewId<PreviewTagId> for RawTag {}

pub struct RawLogicFragment {
    pub id: LogicFragmentId,
    pub name: String,
    pub description: Option<String>,
}
impl HasPreviewId<PreviewLogicFragmentId> for RawLogicFragment {}

#[derive(Debug)]
pub struct RawOperation {
    pub id: OperationId,
    pub name: NameCore,
    pub definition: RawOperationDefinition,
}

/// An undirected graph $G = (V,E)$.
#[derive(Debug)]
pub struct RawGraph {
    pub id: GraphId,
    pub names: NameCore,
    pub definition: Vec<String>,
}

/// A class of undirected graphs $\mathcal C = \{G_1,G_2,\dots\}$.
/// Each graph class is equivalent to a graph property via containment.
#[derive(Debug)]
pub struct RawGraphClass {
    pub id: GraphClassId,
    pub relevance: u32,
    pub names: NameCore,
    pub definition: RawGraphClassDefinition,
}
impl HasPreviewId<PreviewGraphClassId> for RawGraphClass {}

/// A graph property implies a graph class property
/// by defining over all its elements.
impl Defines<PreviewGraphClassPropertyId> for RawGraphClass {
    fn defines(&self) -> PreviewGraphClassPropertyId {
        todo!()
    }
}

/// A map from natural numbers to parameters.
#[derive(Debug)]
pub struct RawParametricParameter {
    pub id: ParametricParameterId,
    pub relevance: u32,
    pub names: NameCore,
    pub definition: RawParametricParameterDefinition,
    pub tags: Vec<PreviewTagId>,
}
impl HasPreviewId<PreviewParametricParameterId> for RawParametricParameter {}
impl Tagged for RawParametricParameter {}

impl Concrete<Value, PreviewGraphClassPropertyId> for RawParametricParameter {
    fn concretize(&self, value: Value) -> PreviewGraphClassPropertyId {
        todo!()
    }
}

/// A map from natural numbers to graphs.
#[derive(Debug)]
pub struct RawParametricGraphClass {
    pub id: ParametricGraphClassId,
    pub relevance: u32,
    pub names: NameCore,
    pub closed_under: PreviewGraphRelationId,
    pub tags: Vec<PreviewTagId>,
    pub definition: String,
}
impl HasPreviewId<PreviewParametricGraphClassId> for RawParametricGraphClass {}
impl Tagged for RawParametricGraphClass {}

impl Defines<PreviewGraphClassId> for RawParametricGraphClass {
    fn defines(&self) -> PreviewGraphClassId {
        todo!()
    }
}

impl Concrete<Value, PreviewGraphClassPropertyId> for RawParametricGraphClass {
    fn concretize(&self, value: Value) -> PreviewGraphClassPropertyId {
        todo!()
    }
}

/// Parameter over a graph class. Definable also as a parameter of a graph.
#[derive(Debug)]
pub struct RawParameter {
    pub id: ParameterId,
    pub relevance: u32,
    pub names: NameCore,
    pub definition: RawParameterDefinition,
    pub tags: Vec<PreviewTagId>,
}
impl HasPreviewId<PreviewParameterId> for RawParameter {}
impl Tagged for RawParameter {}

/// Property of a graph class, i.e. a map from graph classes to boolean.
/// Note that RawGraphProperty is equivalent to GraphClass and so was omitted.
#[derive(Debug)]
pub struct RawGraphClassProperty {
    pub id: GraphClassPropertyId,
    pub relevance: u32,
    pub names: NameCore,
    pub definition: RawGraphClassPropertyDefinition,
    pub own: RawOwn,
}
impl HasPreviewId<PreviewGraphClassPropertyId> for RawGraphClassProperty {}

#[derive(Debug)]
pub struct RawProvider {
    pub id: ProviderId,
    pub name: String,
    pub url: String,
}
impl HasPreviewId<PreviewProviderId> for RawProvider {}

#[derive(Debug)]
pub struct RawProviderLink {
    pub provider: PreviewProviderId,
    // pub set: Linkable,
    pub url: String,
}

#[derive(Debug)]
pub struct RawGraphRelation {
    pub id: GraphRelationId,
    pub name: String,
    pub displayed_definition: RawGraphRelationDefinition,
}

#[derive(Debug)]
pub struct RawGraphClassRelation {
    pub id: GraphClassRelationId,
    pub name: String,
    pub definition: RawGraphClassRelationDefinition,
}

/// Raw immutable datapoints
pub struct RawData {
    pub graph_class_relations: Vec<RawGraphClassRelation>,
    pub graph_classes: Vec<RawGraphClass>,
    pub graph_relations: Vec<RawGraphRelation>,
    pub graphs: Vec<RawGraph>,
    pub logic_fragments: Vec<RawLogicFragment>,
    pub operations: Vec<RawOperation>,
    pub parameters: Vec<RawParameter>,
    pub parametric_graph_class: Vec<RawParametricGraphClass>,
    pub parametric_parameters: Vec<RawParametricParameter>,
    pub providers: Vec<RawProvider>,
    pub tags: Vec<RawTag>,
}

impl RawData {
    pub fn new() -> Self {
        Self {
            graph_class_relations: Vec::new(),
            graph_classes: Vec::new(),
            graph_relations: Vec::new(),
            graphs: Vec::new(),
            logic_fragments: Vec::new(),
            operations: Vec::new(),
            parameters: Vec::new(),
            parametric_graph_class: Vec::new(),
            parametric_parameters: Vec::new(),
            providers: Vec::new(),
            tags: Vec::new(),
        }
    }
}
