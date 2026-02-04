//! All the raw structures; these are used solely for keeping the input data
//! of the collection.

use std::collections::HashMap;

use crate::data::data::{NameCore, Named, Tagged};
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::{Link, Linkable};
use crate::input::build::CollectionBuilder;
use crate::input::raw_enums::*;
use crate::input::source::{RawFact, RawSource, RawWrote};
use crate::{named_impl, tagged_impl, tie_raw_to_previewid};

pub trait Defines<S> {
    fn defines(&self) -> S;
}

pub trait Concrete<N, S> {
    fn concretize(&self, value: N) -> S;
}

pub trait RawDataAddable {
    fn add(self, data: &mut RawData);
}

macro_rules! raw_data_addable {
    ($mytype:ident, $rawdatafield:ident) => {
        impl RawDataAddable for $mytype {
            fn add(self, data: &mut RawData) {
                data.$rawdatafield.push(self);
            }
        }
    };
}

#[derive(Debug)]
pub struct RawTag {
    pub id: TagId,
    pub name_core: NameCore,
    pub description: String,
}
tie_raw_to_previewid!(RawTag, PreviewTagId);
raw_data_addable!(RawTag, tags);

pub struct RawLogicFragment {
    pub id: LogicFragmentId,
    pub name_core: NameCore,
    pub description: Option<String>,
}
tie_raw_to_previewid!(RawLogicFragment, PreviewLogicFragmentId);
raw_data_addable!(RawLogicFragment, logic_fragments);

#[derive(Debug)]
pub struct RawOperation {
    pub id: OperationId,
    pub name_core: NameCore,
    pub definition: RawOperationDefinition,
}
raw_data_addable!(RawOperation, operations);

/// An undirected graph $G = (V,E)$.
#[derive(Debug)]
pub struct RawGraph {
    pub id: GraphId,
    pub score: u32,
    pub name_core: NameCore,
    pub definition: Vec<String>,
}
named_impl!(RawGraph);
raw_data_addable!(RawGraph, graphs);

/// A class of undirected graphs $\mathcal C = \{G_1,G_2,\dots\}$.
/// Each graph class is equivalent to a graph property via containment.
#[derive(Debug)]
pub struct RawGraphClass {
    pub id: GraphClassId,
    pub score: u32,
    pub name_core: NameCore,
    pub definition: RawGraphClassDefinition,
    pub tags: Vec<PreviewTagId>,
    pub variant: RawGraphClassVariant,
}
tie_raw_to_previewid!(RawGraphClass, PreviewGraphClassId);
tagged_impl!(RawGraphClass, PreviewTagId);
named_impl!(RawGraphClass);
raw_data_addable!(RawGraphClass, graph_classes);

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
    pub score: u32,
    pub name_core: NameCore,
    pub definition: RawParametricParameterDefinition,
    pub tags: Vec<PreviewTagId>,
}
tie_raw_to_previewid!(RawParametricParameter, PreviewParametricParameterId);
tagged_impl!(RawParametricParameter, PreviewTagId);
named_impl!(RawParametricParameter);
raw_data_addable!(RawParametricParameter, parametric_parameters);

impl Concrete<Value, PreviewGraphClassPropertyId> for RawParametricParameter {
    fn concretize(&self, value: Value) -> PreviewGraphClassPropertyId {
        todo!()
    }
}

/// A map from natural numbers to graphs.
#[derive(Debug)]
pub struct RawParametricGraphClass {
    pub id: ParametricGraphClassId,
    pub score: u32,
    pub name_core: NameCore,
    pub closed_under: PreviewGraphRelationId,
    pub tags: Vec<PreviewTagId>,
    pub definition: String,
}
tie_raw_to_previewid!(RawParametricGraphClass, PreviewParametricGraphClassId);
tagged_impl!(RawParametricGraphClass, PreviewTagId);
named_impl!(RawParametricGraphClass);
raw_data_addable!(RawParametricGraphClass, parametric_graph_class);

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
    pub score: u32,
    pub name_core: NameCore,
    pub definition: RawParameterDefinition,
    pub tags: Vec<PreviewTagId>,
}
tie_raw_to_previewid!(RawParameter, PreviewParameterId);
tagged_impl!(RawParameter, PreviewTagId);
named_impl!(RawParameter);
raw_data_addable!(RawParameter, parameters);

/// Property of a graph class, i.e. a map from graph classes to boolean.
/// Note that RawGraphProperty is equivalent to GraphClass and so was omitted.
#[derive(Debug)]
pub struct RawGraphClassProperty {
    pub id: GraphClassPropertyId,
    pub score: u32,
    pub name_core: NameCore,
    pub definition: RawGraphClassPropertyDefinition,
    pub own: RawOwn,
}
tie_raw_to_previewid!(RawGraphClassProperty, PreviewGraphClassPropertyId);
named_impl!(RawGraphClassProperty);
raw_data_addable!(RawGraphClassProperty, graph_class_properties);

#[derive(Debug)]
pub struct RawProvider {
    pub id: ProviderId,
    pub name_core: NameCore,
    pub url: String,
}
tie_raw_to_previewid!(RawProvider, PreviewProviderId);
raw_data_addable!(RawProvider, providers);

// #[derive(Debug)]
pub struct RawProviderLink {
    pub provider: PreviewProviderId,
    pub link: Link,
}

#[derive(Debug)]
pub struct RawGraphRelation {
    pub id: GraphRelationId,
    pub name_core: NameCore,
    pub displayed_definition: RawGraphRelationDefinition,
}
raw_data_addable!(RawGraphRelation, graph_relations);

#[derive(Debug)]
pub struct RawGraphClassRelation {
    pub id: GraphClassRelationId,
    pub name_core: NameCore,
    pub definition: RawGraphClassRelationDefinition,
}
raw_data_addable!(RawGraphClassRelation, graph_class_relations);

#[derive(Debug)]
pub struct RawProblem {
    pub id: ProblemId,
    pub name_core: NameCore,
    pub definition: RawProblemDefinition,
}
raw_data_addable!(RawProblem, problems);

/// Raw immutable datapoints
pub struct RawData {
    pub graph_class_relations: Vec<RawGraphClassRelation>,
    pub graph_classes: Vec<RawGraphClass>,
    pub graph_relations: Vec<RawGraphRelation>,
    pub graph_class_properties: Vec<RawGraphClassProperty>,
    pub graphs: Vec<RawGraph>,
    pub logic_fragments: Vec<RawLogicFragment>,
    pub operations: Vec<RawOperation>,
    pub parameters: Vec<RawParameter>,
    pub parametric_graph_class: Vec<RawParametricGraphClass>,
    pub parametric_parameters: Vec<RawParametricParameter>,
    pub providers: Vec<RawProvider>,
    pub provider_links: Vec<RawProviderLink>,
    pub tags: Vec<RawTag>,
    pub sources: Vec<RawSource>,
    pub problems: Vec<RawProblem>,
    pub factoids: HashMap<PreviewSourceId, RawWrote>,
    pub drawings: HashMap<PreviewSourceId, Drawing>,
}

impl RawData {
    pub fn new() -> Self {
        Self {
            graph_class_relations: Vec::new(),
            graph_classes: Vec::new(),
            graph_relations: Vec::new(),
            graph_class_properties: Vec::new(),
            graphs: Vec::new(),
            logic_fragments: Vec::new(),
            operations: Vec::new(),
            parameters: Vec::new(),
            parametric_graph_class: Vec::new(),
            parametric_parameters: Vec::new(),
            providers: Vec::new(),
            tags: Vec::new(),
            sources: Vec::new(),
            factoids: HashMap::new(),
            drawings: HashMap::new(),
            problems: Vec::new(),
            provider_links: Vec::new(),
        }
    }
}
