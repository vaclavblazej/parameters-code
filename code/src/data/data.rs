//! Collection of preview datapoints together with their mutual relations.
//! More complex structures use Preview structures to refer to each other.

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use log::trace;
use serde::{Deserialize, Serialize};

use crate::data::date::Date;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::Link;
use crate::data::preview::*;
use crate::data::score::Score;
use crate::input::source::ClassicalSolvability;
use crate::input::source::Cpx;
use crate::input::source::EquivalenceRelation;
use crate::input::source::ImplicationRelation;
use crate::input::source::InclusionRelationUnderOperation;
use crate::input::source::ParameterizedSolvability;
use crate::tie_data_to_previewid;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct NameCore {
    pub name: String,
    pub aka: Vec<String>,
    pub abbr: Option<String>,
}

impl NameCore {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            aka: Vec::new(),
            abbr: None,
        }
    }
}

pub trait Named {
    fn name_core(&self) -> &NameCore;
    fn name_core_mut(&mut self) -> &mut NameCore;

    fn name(&self) -> String {
        self.name_core().name.clone()
    }
    fn aka(&mut self, aka: String) {
        self.name_core_mut().aka.push(aka);
    }
    fn abbr(&mut self, abbr: String) {
        assert!(self.name_core_mut().abbr.is_none());
        self.name_core_mut().abbr = Some(abbr);
    }
}

#[macro_export]
macro_rules! named_impl {
    ($mytype:ident) => {
        impl Named for $mytype {
            fn name_core(&self) -> &NameCore {
                &self.name_core
            }
            fn name_core_mut(&mut self) -> &mut NameCore {
                &mut self.name_core
            }
        }
    };
}

pub trait Tagged<With> {
    fn tag(&self) -> &Vec<With>;
    fn tag_mut(&mut self) -> &mut Vec<With>;

    fn add_tag(&mut self, tag: With) {
        self.tag_mut().push(tag);
    }
}

#[macro_export]
macro_rules! tagged_impl {
    ($mytype:ident, $tagtype:ident) => {
        impl Tagged<$tagtype> for $mytype {
            fn tag(&self) -> &Vec<$tagtype> {
                &self.tags
            }
            fn tag_mut(&mut self) -> &mut Vec<$tagtype> {
                &mut self.tags
            }
        }
    };
}

macro_rules! score_impl {
    ($mytype:ident) => {
        impl Score for $mytype {
            fn score(&self) -> u32 {
                self.score
            }
            fn set_score(&mut self, new_score: u32) {
                self.score = new_score;
            }
        }
    };
}

pub trait DataRetrievable<'a, To> {
    fn get(&self, data: &'a Data) -> Option<&'a To>;
}

macro_rules! data_gettable {
    ($idtype:ident, $mytype:ident, $datafield:ident) => {
        impl<'a> DataRetrievable<'a, $mytype> for $idtype {
            // fn get(&self, key: &PreviewSet) -> &Set {
            //     trace!("get set {} {}", key.id, key.name);
            //     let idx: usize = *self
            //         .set_idx
            //         .get(key)
            //         .unwrap_or_else(|| panic!("preview set not found {:?}", key));
            //     &self.sets[idx]
            // }
            fn get(&self, data: &'a Data) -> Option<&'a $mytype> {
                data.$datafield.get(self)
            }
        }
    };
}

// trait TaggedRef: Tagged + Named + HasId {} // todo ?

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name_core: NameCore,
    pub description: String,
    pub sets: Vec<Link>,
}
named_impl!(Tag);
data_gettable!(PreviewTagId, Tag, tags);
tie_data_to_previewid!(Tag, PreviewTagId);

#[derive(Debug, Serialize, Deserialize)]
pub struct LogicFragment {
    pub id: LogicFragmentId,
    pub name_core: NameCore,
    pub description: Option<String>,
}
named_impl!(LogicFragment);
data_gettable!(PreviewLogicFragmentId, LogicFragment, logic_fragments);
tie_data_to_previewid!(LogicFragment, PreviewLogicFragmentId);

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    pub id: OperationId,
    pub name_core: NameCore,
    pub description: Vec<String>,
}
named_impl!(Operation);
data_gettable!(PreviewOperationId, Operation, operations);
tie_data_to_previewid!(Operation, PreviewOperationId);

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub id: GraphId,
    pub score: u32,
    pub name_core: NameCore,
    pub definition: Vec<String>,
}
named_impl!(Graph);
data_gettable!(PreviewGraphId, Graph, graphs);
tie_data_to_previewid!(Graph, PreviewGraphId);

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphClassDefinition {
    Text(Vec<String>),
    Intersection(Vec<PreviewGraphClass>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphClassVariant {
    GraphClass,
    GraphProperty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphClass {
    pub id: GraphClassId,
    pub score: u32,
    pub name_core: NameCore,
    pub definition: GraphClassDefinition,
    pub variant: GraphClassVariant,
}
named_impl!(GraphClass);
data_gettable!(PreviewGraphClassId, GraphClass, graph_classes);
tie_data_to_previewid!(GraphClass, PreviewGraphClassId);

#[derive(Debug, Serialize, Deserialize)]
pub struct ParametricParameter {
    pub id: ParametricParameterId,
    pub score: u32,
    pub name_core: NameCore,
    pub tags: Vec<PreviewTag>,
}
named_impl!(ParametricParameter);
tagged_impl!(ParametricParameter, PreviewTag);
score_impl!(ParametricParameter);
data_gettable!(
    PreviewParametricParameterId,
    ParametricParameter,
    parametric_parameters
);
tie_data_to_previewid!(ParametricParameter, PreviewParametricParameterId);

#[derive(Debug, Serialize, Deserialize)]
pub struct ParametricGraphClass {
    pub id: ParametricGraphClassId,
    pub score: u32,
    pub name_core: NameCore,
    pub closed_under: PreviewGraphRelation,
    pub tags: Vec<PreviewTag>,
}
named_impl!(ParametricGraphClass);
tagged_impl!(ParametricGraphClass, PreviewTag);
score_impl!(ParametricGraphClass);
data_gettable!(
    PreviewParametricGraphClassId,
    ParametricGraphClass,
    parametric_graph_class
);
tie_data_to_previewid!(ParametricGraphClass, PreviewParametricGraphClassId);

#[derive(Debug, Serialize, Deserialize)]
pub enum ParameterDefinition {
    Graph(String),
    GraphClass(String),
    BoundsAll(PreviewParametricParameter), // higher order parameter
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub id: ParameterId,
    pub score: u32,
    pub name_core: NameCore,
    pub definition: ParameterDefinition,
    pub tags: Vec<PreviewTag>,
}
named_impl!(Parameter);
tagged_impl!(Parameter, PreviewTag);
score_impl!(Parameter);
data_gettable!(PreviewParameterId, Parameter, parameters);
tie_data_to_previewid!(Parameter, PreviewParameterId);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum Own {
    Has,
    Is,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphClassPropertyDefinition {
    Text(String),
    FromGraphClass(PreviewGraphClassId),
    FromParameter(PreviewParameterId),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphClassProperty {
    pub id: GraphClassPropertyId,
    pub score: u32,
    pub name_core: NameCore,
    pub definition: GraphClassPropertyDefinition,
    pub own: Own,
}
named_impl!(GraphClassProperty);
tie_data_to_previewid!(GraphClassProperty, PreviewGraphClassPropertyId);
score_impl!(GraphClassProperty);
data_gettable!(
    PreviewGraphClassPropertyId,
    GraphClassProperty,
    graph_class_properties
);

#[derive(Debug)]
pub enum ProblemDefinition {
    Problem(String),
    ModelChecking(PreviewLogicFragment),
}

#[derive(Debug)]
pub struct Problem {
    pub id: ProblemId,
    pub name_core: NameCore,
    pub definition: ProblemDefinition,
}
named_impl!(Problem);

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub id: ProviderId,
    pub name_core: NameCore,
    pub url: String,
    pub links: Vec<ProviderLink>,
}
named_impl!(Provider);
tie_data_to_previewid!(Provider, PreviewProviderId);
data_gettable!(PreviewProviderId, Provider, providers);

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderLink {
    pub provider: PreviewProviderId,
    pub set: Link,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphRelationDefinition {
    Text(String),
    IsomorphicAfterOperations(Vec<PreviewOperation>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphRelation {
    pub id: GraphRelationId,
    pub name_core: NameCore,
    pub displayed_definition: GraphRelationDefinition,
}
named_impl!(GraphRelation);
data_gettable!(PreviewGraphRelationId, GraphRelation, graph_relations);
tie_data_to_previewid!(GraphRelation, PreviewGraphRelationId);

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphClassRelationDefinition {
    Text(String),
    GraphRelation(GraphRelationDefinition),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphClassRelation {
    pub id: GraphClassRelationId,
    pub name_core: NameCore,
    pub definition: GraphClassRelationDefinition,
}
named_impl!(GraphClassRelation);
data_gettable!(
    PreviewGraphClassRelationId,
    GraphClassRelation,
    graph_class_relations
);
tie_data_to_previewid!(GraphClassRelation, PreviewGraphClassRelationId);

pub enum KnowledgeState {
    // todo - check whether these are all the states
    UnknownToHOPS,
    Conjectured,
    Mentioned,
    Proved,
    Folklore,
    Assumed,
    Disproved,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: SourceId,
    pub sourcekey: SourceKey,
    pub wrote: Vec<PreviewWrote>,
    pub time: Date,
    pub drawings: Vec<Drawing>,
}
data_gettable!(PreviewSourceId, Source, sources);
tie_data_to_previewid!(Source, PreviewSourceId);

#[derive(Debug)]
pub struct Wrote {
    pub text: String,
    pub page: Page,
    pub facts: Vec<(ShowedId, WroteStatus, Fact)>,
}

#[derive(Debug, Clone)]
pub enum WroteStatus {
    Assumed,            // taken as given by HOPS, mainly due to being out of project's scope
    Conjectured,        // posed as an open problem
    Original,           // first or independent
    Derivative,         // improvements or later proofs
    Noted(NotedSource), // results claimed to be somewhere else
    TodoStatus,
}

#[derive(Debug, Clone)]
pub enum NotedSource {
    SrcText(String),
    Source(PreviewSource),
    Omitted,
    SrcTodo,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Definition {
    LogicFragment(PreviewLogicFragment),
    Parameter(PreviewParameter),
    Graph(PreviewGraph),
    GraphClass(PreviewGraphClass),
    Operation(PreviewOperation),
    Problem(PreviewProblem),
    ParParameter(PreviewParametricParameter),
    ParGraphClass(PreviewParametricGraphClass),
    Property(PreviewGraphClassProperty),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Relation {
    LfLf(
        PreviewLogicFragment,
        PreviewLogicFragment,
        ImplicationRelation,
    ),
    OpOp(PreviewOperation, PreviewOperation, ImplicationRelation),
    GrGr(PreviewGraph, PreviewGraph, InclusionRelationUnderOperation),
    GcGc(
        PreviewGraphClass,
        PreviewGraphClass,
        InclusionRelationUnderOperation,
    ),
    GrGc(
        PreviewGraph,
        PreviewGraphClass,
        InclusionRelationUnderOperation,
    ),
    PgcPgc(
        PreviewParametricGraphClass,
        PreviewParametricGraphClass,
        ImplicationRelation,
    ),
    ParPar(PreviewParameter, PreviewParameter, Cpx),
    PropProp(
        PreviewGraphClassProperty,
        PreviewGraphClassProperty,
        ImplicationRelation,
    ),
    GcProp(
        PreviewGraphClass,
        PreviewGraphClassProperty,
        EquivalenceRelation,
    ),
    ParProp(
        PreviewParameter,
        PreviewGraphClassProperty,
        EquivalenceRelation,
    ),
    ProbProb(PreviewProblem, PreviewProblem, ImplicationRelation),
    ProbProp(
        PreviewProblem,
        PreviewGraphClassProperty,
        ClassicalSolvability,
    ),
    ProbPar(PreviewProblem, PreviewParameter, ParameterizedSolvability),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Fact {
    Definition(Definition),
    Relation(Relation),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub graph_class_relations: HashMap<PreviewGraphClassRelationId, GraphClassRelation>,
    pub graph_classes: HashMap<PreviewGraphClassId, GraphClass>,
    pub graph_relations: HashMap<PreviewGraphRelationId, GraphRelation>,
    pub graph_class_properties: HashMap<PreviewGraphClassPropertyId, GraphClassProperty>,
    pub graphs: HashMap<PreviewGraphId, Graph>,
    pub logic_fragments: HashMap<PreviewLogicFragmentId, LogicFragment>,
    pub operations: HashMap<PreviewOperationId, Operation>,
    pub parameters: HashMap<PreviewParameterId, Parameter>,
    pub parametric_graph_class: HashMap<PreviewParametricGraphClassId, ParametricGraphClass>,
    pub parametric_parameters: HashMap<PreviewParametricParameterId, ParametricParameter>,
    pub providers: HashMap<PreviewProviderId, Provider>,
    pub tags: HashMap<PreviewTagId, Tag>,
    pub sources: HashMap<PreviewSourceId, Source>,
    pub factoids: HashMap<PreviewSourceId, Fact>,
    pub drawings: HashMap<PreviewSourceId, Drawing>,
    // pub relations: Vec<Relation>,
    // pub partial_results: Vec<PartialResult>,
    // #[serde(skip)]
    // pub set_idx: HashMap<PreviewSet, usize>,
    // #[serde(skip)]
    // pub set_id_idx: HashMap<PreviewSetId, usize>,
    // #[serde(skip)]
    // pub relation_idx: HashMap<(PreviewSet, PreviewSet), usize>,
    // #[serde(skip)]
    // pub relation_id_idx: HashMap<PreviewRelationId, usize>,
}

pub fn convert_to_id_map<D>(arr: Vec<D>) -> HashMap<D::PreviewId, D>
where
    D: HasPreviewId,
    D::PreviewId: Hash + Eq,
{
    arr.into_iter().map(|x| (x.previewid(), x)).collect()
}

pub struct DataFields {
    pub graph_class_relations: Vec<GraphClassRelation>,
    pub tags: Vec<Tag>,
    pub providers: Vec<Provider>,
    pub parametric_parameters: Vec<ParametricParameter>,
    pub parametric_graph_class: Vec<ParametricGraphClass>,
    pub parameters: Vec<Parameter>,
    pub operations: Vec<Operation>,
    pub logic_fragments: Vec<LogicFragment>,
    pub graphs: Vec<Graph>,
    pub graph_relations: Vec<GraphRelation>,
    pub graph_classes: Vec<GraphClass>,
    pub sources: Vec<Source>,
    pub factoids: HashMap<PreviewSourceId, Fact>,
    pub drawings: HashMap<PreviewSourceId, Drawing>,
    pub graph_class_properties: Vec<GraphClassProperty>,
}

impl Data {
    pub fn new(fields: DataFields) -> Self {
        trace!("new data");
        Self {
            graph_class_relations: convert_to_id_map(fields.graph_class_relations),
            graph_classes: convert_to_id_map(fields.graph_classes),
            graph_relations: convert_to_id_map(fields.graph_relations),
            graphs: convert_to_id_map(fields.graphs),
            logic_fragments: convert_to_id_map(fields.logic_fragments),
            operations: convert_to_id_map(fields.operations),
            parameters: convert_to_id_map(fields.parameters),
            parametric_graph_class: convert_to_id_map(fields.parametric_graph_class),
            parametric_parameters: convert_to_id_map(fields.parametric_parameters),
            providers: convert_to_id_map(fields.providers),
            tags: convert_to_id_map(fields.tags),
            sources: convert_to_id_map(fields.sources),
            factoids: fields.factoids,
            drawings: fields.drawings,
            graph_class_properties: convert_to_id_map(fields.graph_class_properties),
        }
    }
}
