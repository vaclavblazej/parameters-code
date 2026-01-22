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
    pub relevance: u32, // from 0 to 9
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
pub struct InclusionRelationUnderOperation {
    relation: ImplicationRelation,
    operation: PreviewOperationId,
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
    ($main:ident, $enum:ident) => {
        impl Definable for $main {
            fn def(&self) -> RawFact {
                RawFact::$enum(self.clone())
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
                RawFact::$enum(fr.clone(), to.clone(), self)
            }
        }
    };
}

definable!(PreviewLogicFragmentId, DefLogicFragment);
definable!(PreviewGraphClassId, DefGraphClass);
definable!(PreviewGraphClassPropertyId, DefProperty);
definable!(PreviewGraphId, DefGraph);
definable!(PreviewOperationId, DefOperation);
definable!(PreviewParameterId, DefParameter);
definable!(PreviewParametricGraphClassId, DefParGraphClass);
definable!(PreviewParametricParameterId, DefParParameter);
definable!(PreviewProblemId, DefProblem);

relatable!(
    PreviewLogicFragmentId,
    PreviewLogicFragmentId,
    ImplicationRelation,
    RelLfLf
);
relatable!(
    PreviewOperationId,
    PreviewOperationId,
    ImplicationRelation,
    RelOpOp
);
relatable!(
    PreviewGraphId,
    PreviewGraphId,
    InclusionRelationUnderOperation,
    RelGrGr
);
relatable!(
    PreviewGraphClassId,
    PreviewGraphClassId,
    InclusionRelationUnderOperation,
    RelGcGc
);
relatable!(
    PreviewGraphId,
    PreviewGraphClassId,
    InclusionRelationUnderOperation,
    RelGrGc
);
relatable!(
    PreviewParametricGraphClassId,
    PreviewParametricGraphClassId,
    ImplicationRelation,
    RelPgcPgc
);
relatable!(PreviewParameterId, PreviewParameterId, Cpx, RelParPar);
relatable!(
    PreviewGraphClassPropertyId,
    PreviewGraphClassPropertyId,
    ImplicationRelation,
    RelPropProp
);
relatable!(
    PreviewGraphClassId,
    PreviewGraphClassPropertyId,
    EquivalenceRelation,
    RelGcProp
);
relatable!(
    PreviewParameterId,
    PreviewGraphClassPropertyId,
    EquivalenceRelation,
    RelParProp
);
// relatable!(PreviewParametricGraphClassId, PreviewParameterId, EquivalenceRelation, RelParProp); // todo as e.g. excluded minor
relatable!(
    PreviewProblemId,
    PreviewProblemId,
    ImplicationRelation,
    RelProbProb
);
// relatable!(PreviewProblemId, PreviewGraphClassId, ClassicalSolvability, RelProbGc); // through the trivial property
relatable!(
    PreviewProblemId,
    PreviewGraphClassPropertyId,
    ClassicalSolvability,
    RelProbProp
);
relatable!(
    PreviewProblemId,
    PreviewParameterId,
    ParameterizedSolvability,
    RelProbPar
);

#[derive(Debug)]
pub enum RawFact {
    DefLogicFragment(PreviewLogicFragmentId),
    DefParameter(PreviewParameterId),
    DefGraph(PreviewGraphId),
    DefGraphClass(PreviewGraphClassId),
    DefOperation(PreviewOperationId),
    DefProblem(PreviewProblemId),
    DefParParameter(PreviewParametricParameterId),
    DefParGraphClass(PreviewParametricGraphClassId),
    DefProperty(PreviewGraphClassPropertyId),
    RelLfLf(
        PreviewLogicFragmentId,
        PreviewLogicFragmentId,
        ImplicationRelation,
    ),
    RelOpOp(PreviewOperationId, PreviewOperationId, ImplicationRelation),
    RelGrGr(
        PreviewGraphId,
        PreviewGraphId,
        InclusionRelationUnderOperation,
    ),
    RelGcGc(
        PreviewGraphClassId,
        PreviewGraphClassId,
        InclusionRelationUnderOperation,
    ),
    RelGrGc(
        PreviewGraphId,
        PreviewGraphClassId,
        InclusionRelationUnderOperation,
    ),
    RelPgcPgc(
        PreviewParametricGraphClassId,
        PreviewParametricGraphClassId,
        ImplicationRelation,
    ),
    RelParPar(PreviewParameterId, PreviewParameterId, Cpx),
    RelPropProp(
        PreviewGraphClassPropertyId,
        PreviewGraphClassPropertyId,
        ImplicationRelation,
    ),
    RelGcProp(
        PreviewGraphClassId,
        PreviewGraphClassPropertyId,
        EquivalenceRelation,
    ),
    RelParProp(
        PreviewParameterId,
        PreviewGraphClassPropertyId,
        EquivalenceRelation,
    ),
    RelProbProb(PreviewProblemId, PreviewProblemId, ImplicationRelation),
    RelProbProp(
        PreviewProblemId,
        PreviewGraphClassPropertyId,
        ClassicalSolvability,
    ),
    RelProbPar(
        PreviewProblemId,
        PreviewParameterId,
        ParameterizedSolvability,
    ),
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
