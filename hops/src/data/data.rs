//! Collection of preview datapoints together with their mutual relations.
//! More complex structures use Preview structures to refer to each other.

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use log::trace;
use serde::{Deserialize, Serialize};
use hops_macros::{tagged, named, scored};

use crate::data::date::Date;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::Link;
use crate::data::preview::*;
use crate::data::score::Score;
use crate::input::source::ClassicalSolvability;
use crate::input::source::Cpx;
use crate::input::source::DefKind;
use crate::input::source::EquivalenceRelation;
use crate::input::source::ImplicationRelation;
use crate::input::source::InclusionRelationUnderGraphRelation;
use crate::input::source::ParameterizedSolvability;
use crate::input::source::RelKind;
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

pub trait Tagged<With> {
    fn tag(&self) -> &Vec<With>;
    fn tag_mut(&mut self) -> &mut Vec<With>;

    fn add_tag(&mut self, tag: With) {
        self.tag_mut().push(tag);
    }
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

#[named]
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub description: String,
    pub sets: Vec<Link>,
}
data_gettable!(PreviewTagId, Tag, tags);
tie_data_to_previewid!(Tag, PreviewTagId);

#[named]
#[derive(Debug, Serialize, Deserialize)]
pub struct LogicFragment {
    pub id: LogicFragmentId,
    pub description: Option<String>,
}
data_gettable!(PreviewLogicFragmentId, LogicFragment, logic_fragments);
tie_data_to_previewid!(LogicFragment, PreviewLogicFragmentId);

#[named]
#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    pub id: OperationId,
    pub description: Vec<String>,
}
data_gettable!(PreviewOperationId, Operation, operations);
tie_data_to_previewid!(Operation, PreviewOperationId);

#[named]
#[scored]
#[tagged(PreviewTag)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub id: GraphId,
    pub definition: Vec<String>,
}
data_gettable!(PreviewGraphId, Graph, graphs);
tie_data_to_previewid!(Graph, PreviewGraphId);

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphClassDefinition {
    Text(Vec<String>),
    Intersection(Vec<PreviewGraphClass>),
    IntersectionGraphClassProperty(PreviewGraphClass, PreviewGraphClassProperty),
    ParametricGraphClass(PreviewParametricGraphClass),
    Parameter(PreviewParameter),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphClassVariant {
    GraphClass,
    GraphProperty,
}

#[named]
#[scored]
#[tagged(PreviewTag)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphClass {
    pub id: GraphClassId,
    pub definition: GraphClassDefinition,
    pub variant: GraphClassVariant,
}
data_gettable!(PreviewGraphClassId, GraphClass, graph_classes);
tie_data_to_previewid!(GraphClass, PreviewGraphClassId);

#[named]
#[scored]
#[tagged(PreviewTag)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ParametricParameter {
    pub id: ParametricParameterId,
}
data_gettable!(
    PreviewParametricParameterId,
    ParametricParameter,
    parametric_parameters
);
tie_data_to_previewid!(ParametricParameter, PreviewParametricParameterId);

#[named]
#[scored]
#[tagged(PreviewTag)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ParametricGraphClass {
    pub id: ParametricGraphClassId,
    pub closed_under: PreviewGraphRelation,
}
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
    DistanceToParameter(PreviewParameter),
    DistanceToGraphClass(PreviewGraphClass),
    Intersection(Vec<PreviewParameter>),
    IntersectionParameterProperty(PreviewParameter, PreviewGraphClassProperty),
    IntersectionParameterGraphClass(PreviewParameter, PreviewGraphClass),
    FromParametricParameter(PreviewParametricParameter),
}

#[named]
#[scored]
#[tagged(PreviewTag)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub id: ParameterId,
    pub definition: ParameterDefinition,
    pub timeline: Vec<(PreviewSource, Vec<Wrote>)>,
}
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
    FromGraphClass(PreviewGraphClass),
    FromParameter(PreviewParameter),
}

#[named]
#[scored]
#[tagged(PreviewTag)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphClassProperty {
    pub id: GraphClassPropertyId,
    pub definition: GraphClassPropertyDefinition,
    pub own: Own,
}
tie_data_to_previewid!(GraphClassProperty, PreviewGraphClassPropertyId);
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

#[named]
#[derive(Debug)]
pub struct Problem {
    pub id: ProblemId,
    pub definition: ProblemDefinition,
}

#[named]
#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub id: ProviderId,
    pub url: String,
    pub links: Vec<ProviderLink>,
}
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

#[named]
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphRelation {
    pub id: GraphRelationId,
    pub displayed_definition: GraphRelationDefinition,
}
data_gettable!(PreviewGraphRelationId, GraphRelation, graph_relations);
tie_data_to_previewid!(GraphRelation, PreviewGraphRelationId);

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphClassRelationDefinition {
    Text(String),
    GraphRelation(GraphRelationDefinition),
}

// #[named]
// #[derive(Debug, Serialize, Deserialize)]
// pub struct GraphClassRelation {
//     pub id: GraphClassRelationId,
//     pub definition: GraphClassRelationDefinition,
// }
// data_gettable!(
//     PreviewGraphClassRelationId,
//     GraphClassRelation,
//     graph_class_relations
// );
// tie_data_to_previewid!(GraphClassRelation, PreviewGraphClassRelationId);

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

#[named]
#[scored]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wrote {
    pub text: String,
    pub page: Page,
    pub facts: Vec<(PreviewShowedId, WroteStatus, Fact)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WroteStatus {
    Assumed,            // taken as given by HOPS, mainly due to being out of project's scope
    Conjectured,        // posed as an open problem
    Original,           // first or independent
    Derivative,         // improvements or later proofs
    Noted(NotedSource), // results claimed to be somewhere else
    TodoStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotedSource {
    SrcText(String),
    Source(PreviewSource),
    Omitted,
    SrcTodo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Definition {
    pub fn kind(&self) -> DefKind {
        match self {
            Definition::LogicFragment(..) => DefKind::LogicFragment,
            Definition::Parameter(..) => DefKind::Parameter,
            Definition::Graph(..) => DefKind::Graph,
            Definition::GraphClass(..) => DefKind::GraphClass,
            Definition::Operation(..) => DefKind::Operation,
            Definition::Problem(..) => DefKind::Problem,
            Definition::ParParameter(..) => DefKind::ParametricParameter,
            Definition::ParGraphClass(..) => DefKind::ParametricGraphClass,
            Definition::Property(..) => DefKind::Property,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Relation {
    LfLf(PreviewLogicFragment, PreviewLogicFragment, ImplicationRelation),
    OpOp(PreviewOperation, PreviewOperation, ImplicationRelation),
    GrGr(PreviewGraph, PreviewGraph, InclusionRelationUnderGraphRelation),
    GcGc(PreviewGraphClass, PreviewGraphClass, InclusionRelationUnderGraphRelation),
    GrGc(PreviewGraph, PreviewGraphClass, InclusionRelationUnderGraphRelation),
    PgcPgc(PreviewParametricGraphClass, PreviewParametricGraphClass, ImplicationRelation),
    ParPar(PreviewParameter, PreviewParameter, Cpx),
    PropProp(PreviewGraphClassProperty, PreviewGraphClassProperty, ImplicationRelation),
    PropPar(PreviewGraphClassProperty, PreviewParameter, ImplicationRelation),
    GcProp(PreviewGraphClass, PreviewGraphClassProperty, EquivalenceRelation),
    GcPar(PreviewGraphClass, PreviewParameter, ImplicationRelation),
    ParProp(PreviewParameter, PreviewGraphClassProperty, EquivalenceRelation),
    ProbProb(PreviewProblem, PreviewProblem, ImplicationRelation),
    ProbProp(PreviewProblem, PreviewGraphClassProperty, ClassicalSolvability),
    ProbPar(PreviewProblem, PreviewParameter, ParameterizedSolvability),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fact {
    Definition(Definition),
    Relation(Relation),
}

impl Relation {
    pub fn kind(&self) -> RelKind {
        match self {
            Relation::ParPar(..) => RelKind::ParPar,
            Relation::LfLf(..) => RelKind::LfLf,
            Relation::OpOp(..) => RelKind::OpOp,
            Relation::GrGr(..) => RelKind::GrGr,
            Relation::GcGc(..) => RelKind::GcGc,
            Relation::GrGc(..) => RelKind::GrGc,
            Relation::PgcPgc(..) => RelKind::PgcPgc,
            Relation::PropProp(..) => RelKind::PropProp,
            Relation::GcProp(..) => RelKind::GcProp,
            Relation::GcPar(..) => RelKind::GcPar,
            Relation::PropPar(..) => RelKind::PropPar,
            Relation::ParProp(..) => RelKind::ParProp,
            Relation::ProbProb(..) => RelKind::ProbProb,
            Relation::ProbProp(..) => RelKind::ProbProp,
            Relation::ProbPar(..) => RelKind::ProbPar,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
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
    pub sorted_sources: Vec<PreviewSourceId>,
    pub arc_parameter_parameter: Vec<(PreviewParameter, PreviewParameter, Cpx)>,
    pub arc_lf_lf: Vec<(PreviewLogicFragment, PreviewLogicFragment, ImplicationRelation)>,
    pub arc_op_op: Vec<(PreviewOperation, PreviewOperation, ImplicationRelation)>,
    pub arc_graph_graph: Vec<(PreviewGraph, PreviewGraph, InclusionRelationUnderGraphRelation)>,
    pub arc_gc_gc: Vec<(PreviewGraphClass, PreviewGraphClass, InclusionRelationUnderGraphRelation)>,
    pub arc_graph_gc: Vec<(PreviewGraph, PreviewGraphClass, InclusionRelationUnderGraphRelation)>,
    pub arc_pargc_pargc: Vec<(PreviewParametricGraphClass, PreviewParametricGraphClass, ImplicationRelation)>,
    pub arc_gcprop_gcprop: Vec<(PreviewGraphClassProperty, PreviewGraphClassProperty, ImplicationRelation)>,
    pub arc_gc_gcprop: Vec<(PreviewGraphClass, PreviewGraphClassProperty, EquivalenceRelation)>,
    pub arc_gc_par: Vec<(PreviewGraphClass, PreviewParameter, ImplicationRelation)>,
    pub arc_gcprop_parameter: Vec<(PreviewGraphClassProperty, PreviewParameter, ImplicationRelation)>,
    pub arc_parameter_gcprop: Vec<(PreviewParameter, PreviewGraphClassProperty, EquivalenceRelation)>,
    pub arc_problem_problem: Vec<(PreviewProblem, PreviewProblem, ImplicationRelation)>,
    pub arc_problem_gcprop: Vec<(PreviewProblem, PreviewGraphClassProperty, ClassicalSolvability)>,
    pub arc_problem_parameter: Vec<(PreviewProblem, PreviewParameter, ParameterizedSolvability)>,
}

pub fn convert_to_id_map<D>(arr: Vec<D>) -> HashMap<D::PreviewId, D>
where
    D: HasPreviewId,
    D::PreviewId: Hash + Eq,
{
    arr.into_iter().map(|x| (x.previewid(), x)).collect()
}

pub struct DataFields {
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
    pub graph_class_properties: Vec<GraphClassProperty>,
    pub arc_parameter_parameter: Vec<(PreviewParameter, PreviewParameter, Cpx)>,
    pub arc_lf_lf: Vec<(PreviewLogicFragment, PreviewLogicFragment, ImplicationRelation)>,
    pub arc_op_op: Vec<(PreviewOperation, PreviewOperation, ImplicationRelation)>,
    pub arc_graph_graph: Vec<(PreviewGraph, PreviewGraph, InclusionRelationUnderGraphRelation)>,
    pub arc_gc_gc: Vec<(PreviewGraphClass, PreviewGraphClass, InclusionRelationUnderGraphRelation)>,
    pub arc_graph_gc: Vec<(PreviewGraph, PreviewGraphClass, InclusionRelationUnderGraphRelation)>,
    pub arc_pargc_pargc: Vec<(PreviewParametricGraphClass, PreviewParametricGraphClass, ImplicationRelation)>,
    pub arc_gcprop_gcprop: Vec<(PreviewGraphClassProperty, PreviewGraphClassProperty, ImplicationRelation)>,
    pub arc_gc_gcprop: Vec<(PreviewGraphClass, PreviewGraphClassProperty, EquivalenceRelation)>,
    pub arc_gc_par: Vec<(PreviewGraphClass, PreviewParameter, ImplicationRelation)>,
    pub arc_gcprop_parameter: Vec<(PreviewGraphClassProperty, PreviewParameter, ImplicationRelation)>,
    pub arc_parameter_gcprop: Vec<(PreviewParameter, PreviewGraphClassProperty, EquivalenceRelation)>,
    pub arc_problem_problem: Vec<(PreviewProblem, PreviewProblem, ImplicationRelation)>,
    pub arc_problem_gcprop: Vec<(PreviewProblem, PreviewGraphClassProperty, ClassicalSolvability)>,
    pub arc_problem_parameter: Vec<(PreviewProblem, PreviewParameter, ParameterizedSolvability)>,
}

impl Data {
    pub fn new(fields: DataFields) -> Self {
        trace!("new data");
        Self {
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
            sorted_sources: fields.sources.iter().map(|x| x.previewid()).collect(),
            sources: convert_to_id_map(fields.sources),
            graph_class_properties: convert_to_id_map(fields.graph_class_properties),
            arc_parameter_parameter: fields.arc_parameter_parameter,
            arc_lf_lf: fields.arc_lf_lf,
            arc_op_op: fields.arc_op_op,
            arc_graph_graph: fields.arc_graph_graph,
            arc_gc_gc: fields.arc_gc_gc,
            arc_gc_par: fields.arc_gc_par,
            arc_gcprop_parameter: fields.arc_gcprop_parameter,
            arc_graph_gc: fields.arc_graph_gc,
            arc_pargc_pargc: fields.arc_pargc_pargc,
            arc_gcprop_gcprop: fields.arc_gcprop_gcprop,
            arc_gc_gcprop: fields.arc_gc_gcprop,
            arc_parameter_gcprop: fields.arc_parameter_gcprop,
            arc_problem_problem: fields.arc_problem_problem,
            arc_problem_gcprop: fields.arc_problem_gcprop,
            arc_problem_parameter: fields.arc_problem_parameter,
        }
    }
}
