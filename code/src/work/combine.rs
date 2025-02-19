use core::fmt;

use crate::{data::{data::{PartialResult, PartialResultsBuilder, Relation}, preview::{WorkRelation, PreviewSet}}, general::enums::{ComparisonResult, CpxInfo, CpxTime, CreatedBy, SourcedCpxInfo, SourcedCpxInfo::*}};
use log::{debug, error, trace};


#[derive(Debug)]
pub enum CombinationError {
    ExclusionInclusion(PartialResult, PartialResult),
    IncompatibleWithEquivalence(PartialResult, PartialResult),
}

impl fmt::Display for CombinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombinationError::ExclusionInclusion(a, b) => write!(f, "Tried to combine Exclusion with Inclusion relation in parallel:\n{:?}\n{:?}", a, b),
            CombinationError::IncompatibleWithEquivalence(a, b) => write!(f, "Tried to combine equivalence with an incompatible relation in parallel:\n{:?}\n{:?}", a, b),
        }
    }
}

impl PartialResultsBuilder {

    pub fn new() -> Self {
        Self {
            arr: vec![],
        }
    }

    pub fn partial_result(&mut self, created_by: CreatedBy, cpx: CpxInfo, relation: WorkRelation) -> PartialResult {
        let len = self.arr.len();
        let res = PartialResult{
            handle: len,
            created_by,
            cpx,
            relation,
        };
        self.arr.push(res.clone());
        res
    }

    pub fn done(self) -> Vec<PartialResult> {
        self.arr
    }

}

impl WorkRelation {

    pub fn new(subset: &PreviewSet, superset: &PreviewSet) -> Self {
        WorkRelation {
            subset: subset.clone(),
            superset: superset.clone(),
        }
    }

    pub fn combine_serial(&self, other: &Self) -> Self {
        assert_eq!(self.superset.id, other.subset.id);
        Self {
            subset: self.subset.clone(),
            superset: other.superset.clone(),
        }
    }

}

/// Out of two options give the one that is asymptotically smaller.
pub fn combine_parallel_min(a: (CpxTime, PartialResult), b: (CpxTime, PartialResult)) -> (CpxTime, PartialResult) {
    match (a, b) {
        ((CpxTime::Constant, a), _) | (_, (CpxTime::Constant, a)) => (CpxTime::Constant, a),
        ((CpxTime::Linear, a), _) | (_, (CpxTime::Linear, a)) => (CpxTime::Linear, a),
        ((CpxTime::Polynomial, a), _) | (_, (CpxTime::Polynomial, a)) => (CpxTime::Polynomial, a),
        ((CpxTime::Exponential, a), _) | (_, (CpxTime::Exponential, a)) => (CpxTime::Exponential, a),
        ((CpxTime::Tower, a), _) | (_, (CpxTime::Tower, a)) => (CpxTime::Tower, a),
        ((CpxTime::Exists, a), (CpxTime::Exists, _)) => (CpxTime::Exists, a),
    }
}

/// Out of two options give the one that is asymptotically bigger.
pub fn combine_parallel_max(a: (CpxTime, PartialResult), b: (CpxTime, PartialResult)) -> (CpxTime, PartialResult) {
    match (a, b) {
        ((CpxTime::Exists, a), _) | (_, (CpxTime::Exists, a)) => (CpxTime::Exists, a),
        ((CpxTime::Tower, a), _) | (_, (CpxTime::Tower, a)) => (CpxTime::Tower, a),
        ((CpxTime::Exponential, a), _) | (_, (CpxTime::Exponential, a)) => (CpxTime::Exponential, a),
        ((CpxTime::Polynomial, a), _) | (_, (CpxTime::Polynomial, a)) => (CpxTime::Polynomial, a),
        ((CpxTime::Linear, a), _) | (_, (CpxTime::Linear, a)) => (CpxTime::Linear, a),
        ((CpxTime::Constant, a), (CpxTime::Constant, _)) => (CpxTime::Constant, a),
    }
}

/// Returns complexity that we get when we substitute k with b
pub fn combine_serial((cpxa, a): (CpxTime, PartialResult), (cpxb, b): (CpxTime, PartialResult)) -> (PartialResult, PartialResult, CpxTime) {
    let relation = a.relation.combine_serial(&b.relation);
    match (cpxa, cpxb) {
        (CpxTime::Exists, _) | (_, CpxTime::Exists) => (a, b, CpxTime::Exists),
        (CpxTime::Tower, _) | (_, CpxTime::Tower) => (a, b, CpxTime::Tower),
        (CpxTime::Exponential, _) | (_, CpxTime::Exponential) => (a, b, CpxTime::Tower),
        (CpxTime::Polynomial, _) | (_, CpxTime::Polynomial) => (a, b, CpxTime::Polynomial),
        (CpxTime::Linear, _) | (_, CpxTime::Linear) => (a, b, CpxTime::Linear),
        (CpxTime::Constant, _) | (_, CpxTime::Constant) => (a, b, CpxTime::Constant),
    }
}


impl PartialResult {

    // todo - combine_parallel should be changed to find the simplest way to find the resulting complexity
    /// Combine the two complexities' best results. Returns Some if the result is better than self.
    pub fn combine_parallel(&self, other: &PartialResult, partial_result_builder: &mut PartialResultsBuilder) -> Option<PartialResult> {
        assert_eq!(self.relation, other.relation);
        trace!("\n{:?}\n{:?}", self.relation, other.relation);
        let original: SourcedCpxInfo = self.to_sourced();
        let res: Result<SourcedCpxInfo, CombinationError> = match (self.to_sourced(), other.to_sourced()) {
            // Prefer anything before taking Unknown.
            (Unknown, a) | (a, Unknown) => Ok(a.clone()),
            // Check equivalence is compatible with the other bound and if so, keep it.
            (Equal{ source }, Equal{ .. }) => Ok(Equal { source }),
            (Equal{ source }, LowerBound { mn: (mn, _) }) | (LowerBound { mn: (mn, _) }, Equal{ source }) => {
                match mn {
                    CpxTime::Constant | CpxTime::Linear => Ok(Equal { source }),
                    _ => Err(CombinationError::IncompatibleWithEquivalence(self.clone(), other.clone())),
                }
            },
            (Equal{ source }, Inclusion { mn: (mn, mns), mx: (mx, mxs) }) | (Inclusion { mn: (mn, mns), mx: (mx, mxs) }, Equal{ source }) => {
                match (mn, mx) {
                    (CpxTime::Constant | CpxTime::Linear,
                     CpxTime::Linear | CpxTime::Polynomial | CpxTime::Exponential | CpxTime::Tower | CpxTime::Exists)
                        => Ok(Equal { source }),
                    (_, _) => Err(CombinationError::IncompatibleWithEquivalence(self.clone(), other.clone())),
                }
            },
            (Equal{ source }, UpperBound { mx: (mx, mxs) }) | (UpperBound { mx: (mx, mxs) }, Equal{ source }) => {
                match mx {
                    CpxTime::Linear | CpxTime::Polynomial | CpxTime::Exponential | CpxTime::Tower | CpxTime::Exists
                        => Ok(Equal { source }),
                    _ => Err(CombinationError::IncompatibleWithEquivalence(self.clone(), other.clone())),
                }
            },
            (Equal{ .. }, Exclusion{ .. }) | (Exclusion{ .. }, Equal{ .. }) => panic!("impossible"),
            // If both are inclusions, upper bounds or lower bounds we can nicely combine them.
            (UpperBound {  mx: (mxa, sa) }, UpperBound { mx: (mxb, sb) })
                => Ok(UpperBound {
                    mx: combine_parallel_min((mxa, sa), (mxb, sb)),
                }),
            (UpperBound {  mx: (mxa, sxa) }, Inclusion { mn: (mnb, snb), mx: (mxb, sxb) })
                | (Inclusion { mn: (mnb, snb), mx: (mxb, sxb) }, UpperBound {  mx: (mxa, sxa) })
                => Ok(Inclusion {
                    mn: (mnb.clone(), snb.clone()),
                    mx: combine_parallel_min((mxa, sxa), (mxb, sxb)),
                }),
            (Inclusion { mn: (mna, sna), mx: (mxa, sxa) }, Inclusion { mn: (mnb, snb), mx: (mxb, sxb) })
                => Ok(Inclusion {
                    mn: combine_parallel_max((mna, sna), (mnb, snb)),
                    mx: combine_parallel_min((mxa, sxa), (mxb, sxb)),
                }),
            (Inclusion { mn: (mna, sna), mx }, LowerBound { mn: (mnb, snb) })
                | (LowerBound { mn: (mnb, snb) }, Inclusion { mn: (mna, sna), mx })
                => Ok(Inclusion {
                    mn: combine_parallel_max((mna, sna), (mnb, snb)),
                    mx: mx.clone(),
                }),
            (UpperBound { mx }, LowerBound { mn })
                | (LowerBound { mn }, UpperBound { mx })
                => Ok(Inclusion {
                    mn: mn.clone(),
                    mx: mx.clone(),
                }),
            (LowerBound { mn: (mna, sna) }, LowerBound { mn: (mnb, snb) })
                => Ok(LowerBound {
                    mn: combine_parallel_max((mna, sna), (mnb, snb)),
                }),
            // Lower bounds are weaker exclusions.
            (Exclusion { source }, Exclusion { .. })
                | (LowerBound { .. }, Exclusion { source })
                | (Exclusion { source }, LowerBound { .. })
                => Ok(Exclusion { source }),
            // We cannot combine exclusion and inclusion as they are disjoint cases.
            (Exclusion{ .. }, Inclusion { .. } | UpperBound { .. })
                | (Inclusion { .. } | UpperBound { .. }, Exclusion{ .. })
                => Err(CombinationError::ExclusionInclusion(self.clone(), other.clone())),
        };
        match res {
            Ok(res) => {
                match res.compare_to(&original) {
                    ComparisonResult::Better => {
                        Some(partial_result_builder.partial_result(CreatedBy::ParallelComposition(self.handle, other.handle), res.into(), self.relation.clone()))
                    },
                    _ => None,
                }
            },
            Err(err) => {
                error!("{}\n{:?}\n{:?}", err, self.relation, other.relation);
                None
            }
        }
    }

    pub fn to_sourced(&self) -> SourcedCpxInfo {
        self.cpx.clone().to_sourced(self.clone())
    }

}

impl SourcedCpxInfo {

    // todo
    // pub fn combine_serial(&self, other: &Self) -> Self {
        // let seta = PreviewSet::mock("a");
        // let setb = PreviewSet::mock("b");
        // let setc = PreviewSet::mock("c");
        // let rela = Relation::new(&seta, &setb, self.clone(), 1);
        // let relb = Relation::new(&setb, &setc, other.clone(), 2);
        // let mut partial_results_builder = PartialResultsBuilder::new();
        // rela.combine_serial(&relb, &mut partial_results_builder).cpx
    // }
    // pub fn combine_parallel(&self, other: &Self) -> Result<Self, CombinationError> {
        // let seta = PreviewSet::mock("a");
        // let setb = PreviewSet::mock("b");
        // let mut rela = Relation::new(&seta, &setb, self.clone(), 1);
        // let relb = Relation::new(&seta, &setb, other.clone(), 2);
        // rela.combine_parallel(&relb);
        // Ok(rela.cpx)
    // }

    pub fn combine_plus(&self, other: &Self) -> SourcedCpxInfo {
        debug!("{:?} {:?}", self.clone(), other.clone());
        let cpx = match (self.clone(), other.clone()) {
            (Unknown, _) | (_, Unknown) => Unknown,
            (Equal { source }, a) | (a, Equal { source }) => a.clone(),
            (Inclusion { mn: _, mx: mxa } | UpperBound { mx: mxa },
             Inclusion { mn: _, mx: mxb } | UpperBound { mx: mxb })
                => UpperBound { mx: combine_parallel_max(mxa, mxb) },
            (Exclusion { .. } | LowerBound { .. }, _) | (_, Exclusion { .. } | LowerBound { .. }) => Unknown,
        };
        cpx
    }

}
