use log::warn;
use serde::{Deserialize, Serialize};

use crate::data::data::Named;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::Link;
use crate::input::build::CollectionBuilder;
use crate::input::raw::RawData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawSourceKey {
    Bibtex { key: String },
    Online { url: String },
    Other { name: String, description: String },
}

impl RawSourceKey {
    pub fn to_str(&self) -> String {
        match self {
            RawSourceKey::Bibtex { key } => key.clone(),
            RawSourceKey::Online { url } => url.clone(),
            RawSourceKey::Other {
                name,
                description: _,
            } => name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawShownRelation<F, T, D> {
    pub id: PreviewRelationId,
    pub subset: F,
    pub superset: T,
    pub data: D, // CpxInfo
}

#[derive(Debug)]
pub struct RawSource {
    pub id: SourceId,
    pub rawsourcekey: RawSourceKey,
    pub score: u32, // from 0 to 9
}

#[derive(Debug)]
pub struct RawWrote {
    pub text: String,
    pub page: Page,
    pub facts: Vec<(ShowedId, RawWroteStatus, RawFact)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquivalenceRelation {
    Equivalent,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImplicationRelation {
    Equivalent,
    Implies,
    Excludes,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassicalSolvability {
    Polynomial,
    NpHard,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParameterizedSolvability {
    Polynomial,
    Fpt,
    Whard,
    ParaNpHard,
}

/// Enum that makes inputting complexities more convenient.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cpx {
    Bounds(CpxTime, CpxTime),
    UpperBound(CpxTime),
    LowerBound(CpxTime),
    StrictUpperBound(CpxTime),
    Exactly(CpxTime),
    Equivalent(CpxTime, CpxTime),
    Equal,
    Exclusion,
    Incomparable,
    Todo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InclusionRelationUnderGraphRelation {
    pub relation: ImplicationRelation,
    pub graph_relation: PreviewGraphRelationId,
}

pub fn definition<T>(set: T) -> RawFact
where
    T: Definable,
{
    set.def()
}

pub fn relation<F, T, D>(fr: &F, to: &T, data: D) -> RawFact
where
    F: Definable,
    T: Definable,
    D: Relatable<F, T>,
{
    data.rel(fr, to)
}

pub trait Definable {
    fn def(&self) -> RawFact;
}

macro_rules! definable {
    ($mainid:ident, $enum:ident) => {
        impl Definable for $mainid {
            fn def(&self) -> RawFact {
                RawFact::Def(Def::$enum((*self).clone()))
            }
        }
        impl Definable for &$mainid {
            fn def(&self) -> RawFact {
                (*self).def()
            }
        }
    };
}

pub trait Relatable<F, T> {
    fn rel(self, fr: &F, to: &T) -> RawFact;
}

macro_rules! relatable {
    ($from:ident, $to:ident, $data:ident, $enum:ident) => {
        impl Relatable<$from, $to> for $data {
            fn rel(self, fr: &$from, to: &$to) -> RawFact {
                RawFact::Rel(Rel::$enum(fr.clone(), to.clone(), self))
            }
        }
    };
}

definable!(PreviewLogicFragmentId, LogicFragment);
definable!(PreviewGraphClassId, GraphClass);
definable!(PreviewGraphClassPropertyId, Property);
definable!(PreviewGraphId, Graph);
definable!(PreviewOperationId, Operation);
definable!(PreviewParameterId, Parameter);
definable!(PreviewParametricGraphClassId, ParametricGraphClass);
definable!(PreviewParametricParameterId, ParametricParameter);
definable!(PreviewProblemId, Problem);

relatable!(
    PreviewLogicFragmentId,
    PreviewLogicFragmentId,
    ImplicationRelation,
    LfLf
);
relatable!(
    PreviewOperationId,
    PreviewOperationId,
    ImplicationRelation,
    OpOp
);
relatable!(
    PreviewGraphId,
    PreviewGraphId,
    InclusionRelationUnderGraphRelation,
    GrGr
);
relatable!(
    PreviewGraphClassId,
    PreviewGraphClassId,
    InclusionRelationUnderGraphRelation,
    GcGc
);
relatable!(
    PreviewGraphId,
    PreviewGraphClassId,
    InclusionRelationUnderGraphRelation,
    GrGc
);
relatable!(
    PreviewParametricGraphClassId,
    PreviewParametricGraphClassId,
    ImplicationRelation,
    PgcPgc
);
relatable!(PreviewParameterId, PreviewParameterId, Cpx, ParPar);
relatable!(
    PreviewGraphClassPropertyId,
    PreviewGraphClassPropertyId,
    ImplicationRelation,
    PropProp
);
relatable!(
    PreviewGraphClassId,
    PreviewGraphClassPropertyId,
    EquivalenceRelation,
    GcProp
);
relatable!(
    PreviewParameterId,
    PreviewGraphClassPropertyId,
    EquivalenceRelation,
    ParProp
);
// relatable!(PreviewParametricGraphClassId, PreviewParameterId, EquivalenceRelation, ParProp); // todo as e.g. excluded minor
relatable!(
    PreviewProblemId,
    PreviewProblemId,
    ImplicationRelation,
    ProbProb
);
// relatable!(PreviewProblemId, PreviewGraphClassId, ClassicalSolvability, ProbGc); // through the trivial property
relatable!(
    PreviewProblemId,
    PreviewGraphClassPropertyId,
    ClassicalSolvability,
    ProbProp
);
relatable!(
    PreviewProblemId,
    PreviewParameterId,
    ParameterizedSolvability,
    ProbPar
);

#[derive(Debug)]
pub enum Def {
    LogicFragment(PreviewLogicFragmentId),
    Parameter(PreviewParameterId),
    Graph(PreviewGraphId),
    GraphClass(PreviewGraphClassId),
    Operation(PreviewOperationId),
    Problem(PreviewProblemId),
    ParametricParameter(PreviewParametricParameterId),
    ParametricGraphClass(PreviewParametricGraphClassId),
    Property(PreviewGraphClassPropertyId),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum DefKind {
    LogicFragment,
    Parameter,
    Graph,
    GraphClass,
    Operation,
    Problem,
    ParametricParameter,
    ParametricGraphClass,
    Property,
}

impl Def {
    pub fn kind(&self) -> DefKind {
        match self {
            Def::LogicFragment(..) => DefKind::LogicFragment,
            Def::Parameter(..) => DefKind::Parameter,
            Def::Graph(..) => DefKind::Graph,
            Def::GraphClass(..) => DefKind::GraphClass,
            Def::Operation(..) => DefKind::Operation,
            Def::Problem(..) => DefKind::Problem,
            Def::ParametricParameter(..) => DefKind::ParametricParameter,
            Def::ParametricGraphClass(..) => DefKind::ParametricGraphClass,
            Def::Property(..) => DefKind::Property,
        }
    }
}

#[derive(Debug)]
pub enum Rel {
    LfLf(
        PreviewLogicFragmentId,
        PreviewLogicFragmentId,
        ImplicationRelation,
    ),
    OpOp(PreviewOperationId, PreviewOperationId, ImplicationRelation),
    GrGr(
        PreviewGraphId,
        PreviewGraphId,
        InclusionRelationUnderGraphRelation,
    ),
    GcGc(
        PreviewGraphClassId,
        PreviewGraphClassId,
        InclusionRelationUnderGraphRelation,
    ),
    GrGc(
        PreviewGraphId,
        PreviewGraphClassId,
        InclusionRelationUnderGraphRelation,
    ),
    PgcPgc(
        PreviewParametricGraphClassId,
        PreviewParametricGraphClassId,
        ImplicationRelation,
    ),
    ParPar(PreviewParameterId, PreviewParameterId, Cpx),
    PropProp(
        PreviewGraphClassPropertyId,
        PreviewGraphClassPropertyId,
        ImplicationRelation,
    ),
    GcProp(
        PreviewGraphClassId,
        PreviewGraphClassPropertyId,
        EquivalenceRelation,
    ),
    ParProp(
        PreviewParameterId,
        PreviewGraphClassPropertyId,
        EquivalenceRelation,
    ),
    ProbProb(PreviewProblemId, PreviewProblemId, ImplicationRelation),
    ProbProp(
        PreviewProblemId,
        PreviewGraphClassPropertyId,
        ClassicalSolvability,
    ),
    ProbPar(
        PreviewProblemId,
        PreviewParameterId,
        ParameterizedSolvability,
    ),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum RelKind {
    LfLf,
    OpOp,
    GrGr,
    GcGc,
    GrGc,
    PgcPgc,
    ParPar,
    PropProp,
    GcProp,
    ParProp,
    ProbProb,
    ProbProp,
    ProbPar,
}

impl Rel {
    pub fn kind(&self) -> RelKind {
        match self {
            Rel::ParPar(..) => RelKind::ParPar,
            Rel::LfLf(..) => RelKind::LfLf,
            Rel::OpOp(..) => RelKind::OpOp,
            Rel::GrGr(..) => RelKind::GrGr,
            Rel::GcGc(..) => RelKind::GcGc,
            Rel::GrGc(..) => RelKind::GrGc,
            Rel::PgcPgc(..) => RelKind::PgcPgc,
            Rel::PropProp(..) => RelKind::PropProp,
            Rel::GcProp(..) => RelKind::GcProp,
            Rel::ParProp(..) => RelKind::ParProp,
            Rel::ProbProb(..) => RelKind::ProbProb,
            Rel::ProbProp(..) => RelKind::ProbProp,
            Rel::ProbPar(..) => RelKind::ProbPar,
        }
    }
}

#[derive(Debug)]
pub enum RawFact {
    Def(Def),
    Rel(Rel),
}

#[derive(Debug, Clone)]
pub enum RawWroteStatus {
    Assumed,               // taken as given by HOPS, mainly due to being out of project's scope
    Conjectured,           // posed as an open problem
    Original,              // first or independent
    Derivative,            // improvements or later proofs
    Noted(RawNotedSource), // results claimed to be somewhere else
    TodoStatus,
}

#[derive(Debug, Clone)]
pub enum RawNotedSource {
    SrcText(String),         // outside of HOPS
    Source(PreviewSourceId), // in HOPS
    Omitted,                 // source is not noted at all
    SrcTodo,                 // waiting to be added to HOPS
}

pub struct RawSourceData {
    source: RawSource,
    factoids: Vec<RawWrote>,
    drawings: Vec<Drawing>,
}

impl RawSourceData {
    pub fn new(source: RawSource) -> Self {
        RawSourceData {
            source,
            factoids: Vec::new(),
            drawings: Vec::new(),
        }
    }

    pub fn wrote(
        mut self,
        page: Page,
        text: &str,
        facts: Vec<(&str, RawWroteStatus, RawFact)>,
    ) -> Self {
        self.factoids.push(RawWrote {
            text: text.into(),
            page,
            facts: facts
                .into_iter()
                .map(|(id, st, fact)| (ShowedId::new(id), st, fact))
                .collect(),
        });
        self
    }

    /// Notes that a source contains a hasse diagram of the listed sets.
    /// This method recreates that diagram with results from HOPS.
    pub fn hasse(mut self, id: &str, page: Page, sets: Vec<Link>) -> Self {
        self.drawings.push(Drawing::Hasse(sets));
        self
    }

    /// Notes that a source has a complete comparison table of the listed sets.
    /// This recreates the same table from the results in HOPS.
    pub fn table(mut self, id: &str, page: Page, sets: Vec<Link>) -> Self {
        self.drawings.push(Drawing::Table(sets));
        self
    }

    pub fn todo_rest(mut self, builder: &mut CollectionBuilder) -> PreviewSourceId {
        warn!(
            "todo: rest of the source {} should be processed",
            self.source.id
        );
        self.done(builder)
    }

    pub fn done(mut self, builder: &mut CollectionBuilder) -> PreviewSourceId {
        self.data_done(&mut builder.data)
    }

    pub fn data_done(mut self, data: &mut RawData) -> PreviewSourceId {
        let RawSourceData {
            source,
            factoids,
            drawings,
        } = self;
        for factoid in factoids {
            data.factoids.insert(source.id.preview(), factoid);
        }
        for drawing in drawings {
            data.drawings.insert(source.id.preview(), drawing);
        }
        let res = source.id.preview();
        data.sources.push(source);
        res
    }
}

// fn relation(&mut self, subset: &PreviewSetId, superset: &PreviewSetId, cpx: CpxInfo) -> RawShownRelation {
// let relation = RawRelation::new(subset, superset, cpx.clone());
// let res = relation.id.preview();
// self.relations.push(relation);
// RawShownRelation {
// id: res,
// subset: subset.clone(),
// superset: superset.clone(),
// cpx,
// }
// }

// pub fn assumed_proper_inclusion(
// &mut self,
// id: &str,
// subset: &PreviewSetId,
// superset: &PreviewSetId,
// ) -> &mut Self {
// let inclusion = RawShowed {
// id: ShowedId::new(id.into()),
// text: "".into(),
// fact: RawShowedFact::Relation(RawShowedStatus::Assumed, self.relation(
// subset,
// superset,
// Inclusion {
// mn: Some(CpxTime::Constant),
// mx: Some(CpxTime::Constant),
// }
// )),
// page: Page::NotApplicable,
// };
// let exclusion = RawShowed {
// id: ShowedId::new(id.into()),
// text: "".into(),
// fact: RawShowedFact::Relation(RawShowedStatus::Assumed, self.relation(
// superset,
// subset,
// Exclusion,
// )),
// page: Page::NotApplicable,
// };
// self.factoids.push(inclusion);
// self.factoids.push(exclusion);
// self
// }

// pub fn noted_relation(
// mut self,
// id: &str,
// page: Page,
// subset: &PreviewSetId,
// superset: &PreviewSetId,
// cpx: Cpx,
// text: &str,
// source: RawNotedSource,
// ) -> Self {
// self.ref_noted_relation(id, page, subset, superset, cpx, text, source);
// self
// }

// pub fn ref_showed(
// &mut self,
// id: &str,
// page: Page,
// status: RawShowedStatus,
// subset: &PreviewSetId,
// superset: &PreviewSetId,
// cpx: Cpx,
// text: &str,
// ) -> &mut Self {
// let relations = match cpx {
// Cpx::Bounds(a, b) => vec![self.relation(
// subset,
// superset,
// Inclusion {
// mn: Some(a.clone()),
// mx: Some(b.clone()),
// },
// )],
// Cpx::Exactly(a) => vec![self.relation(
// subset,
// superset,
// Inclusion {
// mn: Some(a.clone()),
// mx: Some(a.clone()),
// },
// )],
// Cpx::UpperBound(b) => vec![self.relation(
// subset,
// superset,
// Inclusion {
// mn: None,
// mx: Some(b.clone()),
// },
// )],
// Cpx::LowerBound(b) => vec![self.relation(
// subset,
// superset,
// Inclusion {
// mn: Some(b.clone()),
// mx: None,
// },
// )],
// Cpx::Todo => vec![self.relation(
// subset,
// superset,
// Inclusion {
// mn: None,
// mx: Some(CpxTime::Exists),
// },
// )],
// Cpx::Equal => {
// if subset == superset {
// vec![
// self.relation(subset, superset, Equal),
// ]
// } else {
// vec![
// self.relation(subset, superset, Equal),
// self.relation(superset, subset, Equal),
// ]
// }
// },
// Cpx::Equivalent(first_to_second_cpx, second_to_first_cpx) => vec![
// self.relation(
// subset,
// superset,
// Inclusion {
// mn: None,
// mx: Some(first_to_second_cpx),
// },
// ),
// self.relation(
// superset,
// subset,
// Inclusion {
// mn: None,
// mx: Some(second_to_first_cpx),
// },
// ),
// ],
// Cpx::Exclusion => vec![self.relation(subset, superset, Exclusion)],
// Cpx::Incomparable => vec![
// self.relation(subset, superset, Exclusion),
// self.relation(superset, subset, Exclusion),
// ],
// Cpx::StrictUpperBound(a) => vec![
// self.relation(
// subset,
// superset,
// Inclusion {
// mn: None,
// mx: Some(a.clone()),
// },
// ),
// self.relation(superset, subset, Exclusion),
// ],
// };

// for relation in relations {
// let showed = RawShowed {
// id: ShowedId::new(id.into()),
// text: text.into(),
// fact: RawShowedFact::Relation(status.clone(), relation),
// page: page.clone(),
// };
// self.factoids.push(showed);
// }
// self
// }

// pub fn cited(self, id: &str, page: Page, who: &RawSource, text: &str) -> Self {
// let showed = RawShowed {
// id: id.into(),
// text: text.into(),
// fact: RawShowedFact::Citation(who.id.preview()),
// page,
// };
// self.data.factoids.push((self.source.id.clone(), showed));
// self
// }
