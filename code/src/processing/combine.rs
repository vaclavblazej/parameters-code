use core::fmt;

use crate::{data::{data::Relation, preview::PreviewSet}, general::enums::{CpxInfo, CpxTime, CreatedBy, SourcedCpxInfo}};
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

/// What kind of complexity we get when we substitute k with b instead?
pub fn combine_serial(a: (CpxTime, CreatedBy), b: (CpxTime, CreatedBy)) -> (CpxTime, CreatedBy) {
    match (a, b) {
        ((CpxTime::Exists, a), (_, b)) | ((_, b), (CpxTime::Exists, a)) => (CpxTime::Exists, CreatedBy::TransitiveInclusion(a, b)),
        ((CpxTime::Tower, a), (_, b)) | ((_, b), (CpxTime::Tower, a)) => (CpxTime::Tower, a),
        ((CpxTime::Exponential, a), (_, b)) | ((_, b), (CpxTime::Exponential, a)) => (CpxTime::Tower, a),
        ((CpxTime::Exponential, a), (_, b)) | ((_, b), (CpxTime::Exponential, a)) => (CpxTime::Exponential, a),
        ((CpxTime::Polynomial, a), (_, b)) | ((_, b), (CpxTime::Polynomial, a)) => (CpxTime::Polynomial, a),
        ((CpxTime::Linear, a), (_, b)) | ((_, b), (CpxTime::Linear, a)) => (CpxTime::Linear, a),
        ((CpxTime::Constant, a), (CpxTime::Constant, _)) => (CpxTime::Constant, a),
    }
}

/// Out of two options give the one that is asymptotically smaller.
pub fn combine_parallel_min(a: (CpxTime, CreatedBy), b: (CpxTime, CreatedBy)) -> (CpxTime, CreatedBy) {
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
pub fn combine_parallel_max(a: (CpxTime, CreatedBy), b: (CpxTime, CreatedBy)) -> (CpxTime, CreatedBy) {
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

    // todo this should be changed to find the simplest way to find the resulting complexity
    /// Combine the two complexities' best results.
    pub fn combine_parallel(&mut self, other: &Relation) -> bool {
        assert_eq!(self.superset, other.superset);
        assert_eq!(self.subset, other.subset);
        // todo merging entries just via complexity is not good enough, they combine in a more nuanced way
        let original = self.cpx.clone();
        let res: Result<SourcedCpxInfo, CombinationError> = match (self.cpx, other.cpx) {
            // Prefer anything before taking Unknown.
            (Unknown, a) | (a, Unknown) => Ok(a.clone()),
            // Check equivalence is compatible with the other bound and if so, keep it.
            (Equal{ source }, Equal{ .. }) => Ok(Equal { source }),
            (Equal, LowerBound { mn: (mn, source) }) | (LowerBound { mn: (mn, source) }, Equal) => {
                match mn {
                    CpxTime::Constant | CpxTime::Linear => Ok(Equal),
                    _ => Err(CombinationError::IncompatibleWithEquivalence(self.clone(), other.clone())),
                }
            },
            (Equal, Inclusion { mn: (mn, mns), mx: (mx, mxs) }) | (Inclusion { mn: (mn, mns), mx: (mx, mxs) }, Equal) => {
                match (mn, mx) {
                    (CpxTime::Constant | CpxTime::Linear,
                     CpxTime::Linear | CpxTime::Polynomial | CpxTime::Exponential | CpxTime::Tower | CpxTime::Exists)
                        => Ok(Equal),
                    (_, _) => Err(CombinationError::IncompatibleWithEquivalence(self.clone(), other.clone())),
                }
            },
            (Equal, UpperBound { mx: (mx, mxs) }) | (UpperBound { mx: (mx, mxs) }, Equal) => {
                match mx {
                    CpxTime::Linear | CpxTime::Polynomial | CpxTime::Exponential | CpxTime::Tower | CpxTime::Exists
                        => Ok(Equal),
                    _ => Err(CombinationError::IncompatibleWithEquivalence(self.clone(), other.clone())),
                }
            },
            (Equal, Exclusion) | (Exclusion, Equal) => panic!("impossible"),
            // If both are inclusions, upper bounds or lower bounds we can nicely combine them.
            (UpperBound {  mx: (mxa, sa) }, UpperBound { mx: (mxb, sb) })
                => Ok(UpperBound {
                    mx: mxa.combine_parallel_min(&mxb),
                }),
            (UpperBound {  mx: (mxa, sxa) }, Inclusion { mn: (mnb, snb), mx: (mxb, sxb) })
                | (Inclusion { mn: (mnb, snb), mx: (mxb, sxb) }, UpperBound {  mx: (mxa, sxa) })
                => Ok(Inclusion {
                    mn: mxb.clone(),
                    mx: mxa.combine_parallel_min(&mxb),
                }),
            (Inclusion { mn: (mna, sna), mx: (mxa, sxa) }, Inclusion { mn: (mnb, snb), mx: (mxb, sxb) })
                => Ok(Inclusion {
                    mn: mna.combine_parallel_max(&mnb),
                    mx: mxa.combine_parallel_min(&mxb),
                }),
            (Inclusion { mn: (mna, sna), mx }, LowerBound { mn: (mnb, snb) })
                | (LowerBound { mn: (mnb, snb) }, Inclusion { mn: (mna, sna), mx })
                => Ok(Inclusion {
                    mn: mna.combine_parallel_max(&mnb),
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
                    mn: mna.combine_parallel_max(&mnb),
                }),
            // Lower bounds are weaker exclusions.
            (Exclusion { source }, Exclusion { .. })
                | (LowerBound { .. }, Exclusion { source })
                | (Exclusion { source }, LowerBound { .. })
                => Ok(Exclusion { source }),
            // We cannot combine exclusion and inclusion as they are disjoint cases.
            (Exclusion, Inclusion { .. } | UpperBound { .. })
                | (Inclusion { .. } | UpperBound { .. }, Exclusion)
                => Err(CombinationError::ExclusionInclusion(self.clone(), other.clone())),
        };
        match res {
            Ok(res) => {
                if original != res {
                    self.preview.cpx = res.clone().into();
                    self.cpx = res;
                    true
                } else {
                    false
                }
            },
            Err(err) => {
                eprintln!("{}\n{:?}\n{:?}", err, self.preview, other.preview);
                false
            }
        }
    }

    /// Combine the two complexities to represent the transitive complexity.
    pub fn combine_serial(&self, other: &Relation) -> Relation {
        assert_eq!(self.superset, other.subset);
        let cpx = match (&self.cpx, &other.cpx) {
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
                => UpperBound { mx: (mxa.combine_serial(&mxb), CreatedBy::TransitiveInclusion(self.preview.clone(), other.preview.clone())) },
        };
        Relation::new(
            &self.subset,
            &other.superset,
            cpx,
            )
    }

    pub fn combine_plus(&self, other: &Relation) -> Relation {
        assert_eq!(self.subset, other.subset); // expected to be used for combined parameters only
        match (self.cpx, other.cpx) {
            (Unknown, _) | (_, Unknown) => Unknown,
            (Exclusion { source }, _) | (_, Exclusion { source }) => Exclusion { source: source.clone() },
            (Equal { source }, a) | (a, Equal { source }) => a.clone(),
            (Inclusion { mn: _, mx: mxa } | UpperBound { mx: mxa },
             Inclusion { mn: _, mx: mxb } | UpperBound { mx: mxb })
                => UpperBound { mx: mxa.combine_parallel_max(&mxb) },
            (LowerBound { .. }, _) | (_, LowerBound { .. }) => Unknown,
        }
        // let cpx = self.cpx.combine_plus(&other.cpx);
        // Relation::new(
            // &self.subset,
            // &self.superset,
            // cpx,
            // // CreatedBy::TransitiveInclusion(self.preview.clone(), other.preview.clone())
            // )
    }

}

impl SourcedCpxInfo {

    pub fn combine_serial(&self, other: &Self) -> Self {
        let seta = PreviewSet::mock("a");
        let setb = PreviewSet::mock("b");
        let setc = PreviewSet::mock("c");
        let rela = Relation::new(&seta, &setb, self.clone());
        let relb = Relation::new(&setb, &setc, other.clone());
        rela.combine_serial(&relb).cpx
    }

    pub fn combine_parallel(&self, other: &Self) -> Result<Self, CombinationError> {
        let seta = PreviewSet::mock("a");
        let setb = PreviewSet::mock("b");
        let mut rela = Relation::new(&seta, &setb, self.clone());
        let relb = Relation::new(&seta, &setb, other.clone());
        rela.combine_parallel(&relb);
        Ok(rela.cpx)
    }

    pub fn combine_plus(&self, other: &Self) -> Self {
        let seta = PreviewSet::mock("a");
        let setb = PreviewSet::mock("b");
        let setc = PreviewSet::mock("c");
        let setcb = PreviewSet::mock("cb");
        let relab = Relation::new(&seta, &setb, self.clone());
        let relac = Relation::new(&seta, &setc, other.clone());
        relab.combine_plus(&relac).cpx
    }

}
