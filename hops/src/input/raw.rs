//! All the raw structures; these are used solely for keeping the input data
//! of the collection.

use std::collections::HashMap;

use hops_macros::{tagged, named, scored};

use crate::data::data::{NameCore, Named, Tagged};
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::{Link, Linkable};
use crate::data::score::Score;
use crate::input::build::CollectionBuilder;
use crate::input::builder::Builder;
use crate::input::raw_enums::*;
use crate::input::source::{RawFact, RawSource, RawWrote};
use crate::tie_raw_to_previewid;

pub trait Defines<S> {
    fn defines(&self) -> S;
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

#[named]
#[derive(Debug)]
pub struct RawTag {
    pub id: TagId,
    pub description: String,
}
tie_raw_to_previewid!(RawTag, PreviewTagId);
raw_data_addable!(RawTag, tags);

#[named]
#[derive(Debug)]
pub struct RawLogicFragment {
    pub id: LogicFragmentId,
    pub description: Option<String>,
}
tie_raw_to_previewid!(RawLogicFragment, PreviewLogicFragmentId);
raw_data_addable!(RawLogicFragment, logic_fragments);

#[named]
#[derive(Debug)]
pub struct RawOperation {
    pub id: OperationId,
    pub definition: RawOperationDefinition,
}
raw_data_addable!(RawOperation, operations);

/// An undirected graph $G = (V,E)$.
#[named]
#[scored]
#[tagged(PreviewTagId)]
#[derive(Debug)]
pub struct RawGraph {
    pub id: GraphId,
    pub definition: Vec<String>,
}
tie_raw_to_previewid!(RawGraph, PreviewGraphId);
raw_data_addable!(RawGraph, graphs);

/// A class of undirected graphs $\mathcal C = \{G_1,G_2,\dots\}$.
/// Each graph class is equivalent to a graph property via containment.
#[named]
#[scored]
#[tagged(PreviewTagId)]
#[derive(Debug)]
pub struct RawGraphClass {
    pub id: GraphClassId,
    pub definition: RawGraphClassDefinition,
    pub variant: RawGraphClassVariant,
}
tie_raw_to_previewid!(RawGraphClass, PreviewGraphClassId);
raw_data_addable!(RawGraphClass, graph_classes);

/// A graph property implies a graph class property
/// by defining over all its elements.
impl Defines<PreviewGraphClassPropertyId> for RawGraphClass {
    fn defines(&self) -> PreviewGraphClassPropertyId {
        todo!()
    }
}

/// A map from natural numbers to parameters.
#[named]
#[scored]
#[tagged(PreviewTagId)]
#[derive(Debug)]
pub struct RawParametricParameter {
    pub id: ParametricParameterId,
    pub definition: RawParametricParameterDefinition,
}
tie_raw_to_previewid!(RawParametricParameter, PreviewParametricParameterId);
raw_data_addable!(RawParametricParameter, parametric_parameters);

/// A map from natural numbers to graphs.
#[named]
#[scored]
#[tagged(PreviewTagId)]
#[derive(Debug)]
pub struct RawParametricGraphClass {
    pub id: ParametricGraphClassId,
    pub closed_under: PreviewGraphRelationId,
    pub definition: String,
}
tie_raw_to_previewid!(RawParametricGraphClass, PreviewParametricGraphClassId);
raw_data_addable!(RawParametricGraphClass, parametric_graph_class);

impl Defines<PreviewGraphClassId> for RawParametricGraphClass {
    fn defines(&self) -> PreviewGraphClassId {
        todo!()
    }
}

/// Parameter over a graph class. Definable also as a parameter of a graph.
#[named]
#[scored]
#[tagged(PreviewTagId)]
#[derive(Debug)]
pub struct RawParameter {
    pub id: ParameterId,
    pub definition: RawParameterDefinition,
}
tie_raw_to_previewid!(RawParameter, PreviewParameterId);
raw_data_addable!(RawParameter, parameters);

/// Property of a graph class, i.e. a map from graph classes to boolean.
/// Note that RawGraphProperty is equivalent to GraphClass and so was omitted.
#[named]
#[scored]
#[tagged(PreviewTagId)]
#[derive(Debug)]
pub struct RawGraphClassProperty {
    pub id: GraphClassPropertyId,
    pub definition: RawGraphClassPropertyDefinition,
    pub own: RawOwn,
}
tie_raw_to_previewid!(RawGraphClassProperty, PreviewGraphClassPropertyId);
raw_data_addable!(RawGraphClassProperty, graph_class_properties);

#[named]
#[derive(Debug)]
pub struct RawProvider {
    pub id: ProviderId,
    pub url: String,
}
tie_raw_to_previewid!(RawProvider, PreviewProviderId);
raw_data_addable!(RawProvider, providers);

#[derive(Debug)]
pub struct RawProviderLink {
    pub provider: PreviewProviderId,
    pub link: Link,
}

#[named]
#[derive(Debug)]
pub struct RawGraphRelation {
    pub id: GraphRelationId,
    pub displayed_definition: RawGraphRelationDefinition,
}
raw_data_addable!(RawGraphRelation, graph_relations);

#[named]
#[derive(Debug)]
pub struct RawGraphClassRelation {
    pub id: GraphClassRelationId,
    pub definition: RawGraphClassRelationDefinition,
}
raw_data_addable!(RawGraphClassRelation, graph_class_relations);

#[named]
#[derive(Debug)]
pub struct RawProblem {
    pub id: ProblemId,
    pub definition: RawProblemDefinition,
}
raw_data_addable!(RawProblem, problems);

/// Raw immutable datapoints
#[derive(Debug)]
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
    pub factoids: Vec<(PreviewSourceId, Vec<RawWrote>)>,
    pub drawings: Vec<(PreviewSourceId, Vec<Drawing>)>,
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
            factoids: Vec::new(),
            drawings: Vec::new(),
            problems: Vec::new(),
            provider_links: Vec::new(),
        }
    }
}
