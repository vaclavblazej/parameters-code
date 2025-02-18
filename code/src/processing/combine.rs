use core::fmt;

use crate::{data::{data::{PartialResult, PartialResultsBuilder, Relation}, preview::PreviewSet}, general::enums::{ComparisonResult, CpxInfo, CpxTime, CreatedBy, SourcedCpxInfo}};
use log::{error, trace};
use SourcedCpxInfo::*;


#[derive(Debug)]
pub enum CombinationError {
    ExclusionInclusion(Relation, Relation),
    IncompatibleWithEquivalence(Relation, Relation),
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

    pub fn partial_result(&mut self, created_by: CreatedBy) -> PartialResult {
        let len = self.arr.len();
        let res = PartialResult{
            handle: len,
            created_by,
        };
        self.arr.push(res.clone());
        res
    }

    pub fn done(self) -> Vec<PartialResult> {
        self.arr
    }

    fn comb_trans(&mut self, a: PartialResult, b: PartialResult) -> PartialResult {
        self.partial_result(CreatedBy::TransitiveInclusion(a.handle, b.handle))
    }

    /// What kind of complexity we get when we substitute k with b instead?
    pub fn combine_serial(&mut self, a: (CpxTime, PartialResult), b: (CpxTime, PartialResult)) -> (CpxTime, PartialResult) {
        match (a, b) {
            ((CpxTime::Exists, a), (_, b)) | ((_, b), (CpxTime::Exists, a)) => (CpxTime::Exists, self.comb_trans(a, b)),
            ((CpxTime::Tower, a), (_, b)) | ((_, b), (CpxTime::Tower, a)) => (CpxTime::Tower, self.comb_trans(a, b)),
            ((CpxTime::Exponential, a), (_, b)) | ((_, b), (CpxTime::Exponential, a)) => (CpxTime::Tower, self.comb_trans(a, b)),
            ((CpxTime::Exponential, a), (_, b)) | ((_, b), (CpxTime::Exponential, a)) => (CpxTime::Exponential, self.comb_trans(a, b)),
            ((CpxTime::Polynomial, a), (_, b)) | ((_, b), (CpxTime::Polynomial, a)) => (CpxTime::Polynomial, self.comb_trans(a, b)),
            ((CpxTime::Linear, a), (_, b)) | ((_, b), (CpxTime::Linear, a)) => (CpxTime::Linear, self.comb_trans(a, b)),
            ((CpxTime::Constant, a), (CpxTime::Constant, b)) => (CpxTime::Constant, self.comb_trans(a, b)),
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


impl Relation {

    // todo - combine_parallel should be changed to find the simplest way to find the resulting complexity
    /// Combine the two complexities' best results.
    pub fn combine_parallel(&mut self, other: &Relation) -> bool {
        assert_eq!(self.superset, other.superset);
        assert_eq!(self.subset, other.subset);
        trace!("\n{:?}\n{:?}", self.preview, other.preview);
        let original = self.cpx.clone();
        let res: Result<SourcedCpxInfo, CombinationError> = match (self.cpx.clone(), other.cpx.clone()) {
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
                        self.preview.cpx = res.clone().into();
                        self.cpx = res;
                        true
                    },
                    _ => false,
                }
            },
            Err(err) => {
                error!("{}\n{:?}\n{:?}", err, self.preview, other.preview);
                false
            }
        }
    }

    /// Combine the two complexities to represent the transitive complexity.
    pub fn combine_serial(&self, other: &Relation, partial_results_builder: &mut PartialResultsBuilder) -> Relation {
        assert_eq!(self.superset, other.subset);
        let cpx = match (self.cpx.clone(), other.cpx.clone()) {
            // (a = other) and (other rel c) => (a rel c)
            (Equal{ .. }, a) | (a, Equal{ .. }) => a.clone(),
            // No matter what, nothing can be deduced.
            (Unknown, _) | (_, Unknown) => Unknown,
            // Lower bounds are existential, i.e., there exists a graph
            // that has value other > f(A) which does not compose, because
            // existing graphs are not necessarily the same.
            (Exclusion{ .. }, _) | (_, Exclusion{ .. }) => Unknown,
            (LowerBound { .. }, _) | (_, LowerBound { .. }) => Unknown,
            (Inclusion { mn: _, mx: (mxa, sra) } | UpperBound { mx: (mxa, sra) },
             Inclusion { mn: _, mx: (mxb, srb) } | UpperBound { mx: (mxb, srb) })
                => UpperBound {
                    mx: partial_results_builder.combine_serial((mxa, sra), (mxb, srb))
                },
        };
        Relation::new(
            &self.subset,
            &other.superset,
            cpx,
            1
            )
    }

    pub fn combine_plus(&self, other: &Relation) -> Relation {
        assert_eq!(self.subset, other.subset); // expected to be used for combined parameters only
        let cpx = match (self.cpx.clone(), other.cpx.clone()) {
            (Unknown, _) | (_, Unknown) => Unknown,
            (Exclusion { source }, _) | (_, Exclusion { source }) => Exclusion { source: source.clone() },
            (Equal { source }, a) | (a, Equal { source }) => a.clone(),
            (Inclusion { mn: _, mx: mxa } | UpperBound { mx: mxa },
             Inclusion { mn: _, mx: mxb } | UpperBound { mx: mxb })
                => UpperBound { mx: combine_parallel_max(mxa, mxb) },
            (LowerBound { .. }, _) | (_, LowerBound { .. }) => Unknown,
        };
        Relation::new(
            &self.subset,
            &self.superset,
            cpx,
            1
            )
    }

}

impl SourcedCpxInfo {

    pub fn combine_serial(&self, other: &Self) -> Self {
        let seta = PreviewSet::mock("a");
        let setb = PreviewSet::mock("b");
        let setc = PreviewSet::mock("c");
        let rela = Relation::new(&seta, &setb, self.clone(), 1);
        let relb = Relation::new(&setb, &setc, other.clone(), 2);
        let mut partial_results_builder = PartialResultsBuilder::new();
        rela.combine_serial(&relb, &mut partial_results_builder).cpx
    }

    pub fn combine_parallel(&self, other: &Self) -> Result<Self, CombinationError> {
        let seta = PreviewSet::mock("a");
        let setb = PreviewSet::mock("b");
        let mut rela = Relation::new(&seta, &setb, self.clone(), 1);
        let relb = Relation::new(&seta, &setb, other.clone(), 2);
        rela.combine_parallel(&relb);
        Ok(rela.cpx)
    }

    pub fn combine_plus(&self, other: &Self) -> Self {
        let seta = PreviewSet::mock("a");
        let setb = PreviewSet::mock("b");
        let setc = PreviewSet::mock("c");
        let setcb = PreviewSet::mock("cb");
        let relab = Relation::new(&seta, &setb, self.clone(), 1);
        let relac = Relation::new(&seta, &setc, other.clone(), 2);
        relab.combine_plus(&relac).cpx
    }

}
