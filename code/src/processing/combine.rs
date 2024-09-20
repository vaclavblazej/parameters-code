use core::fmt;

use crate::general::enums::{CpxInfo, CpxTime};

#[derive(Debug)]
pub enum CombinationError {
    ExclusionInclusion(CpxInfo, CpxInfo),
    IncompatibleWithEquivalence(CpxInfo, CpxInfo),
}

impl fmt::Display for CombinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombinationError::ExclusionInclusion(a, b) => write!(f, "Tried to combine Exclusion with Inclusion relation in parallel:\n{:?}\n{:?}", a, b),
            CombinationError::IncompatibleWithEquivalence(a, b) => write!(f, "Tried to combine equivalence with an incompatible relation in parallel:\n{:?}\n{:?}", a, b),
        }
    }
}

impl CpxTime {

    /// What kind of complexity we get when we substitute k with b instead?
    pub fn combine_serial(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Exists, _) | (_, Self::Exists) => Self::Exists,
            (Self::Tower, _) | (_, Self::Tower) => Self::Tower,
            (Self::Exponential, Self::Exponential) => Self::Tower,
            (Self::Exponential, _) | (_, Self::Exponential) => Self::Exponential,
            (Self::Polynomial, _) | (_, Self::Polynomial) => Self::Polynomial,
            (Self::Linear, _) | (_, Self::Linear) => Self::Linear,
            (Self::Constant, Self::Constant) => Self::Constant,
        }
    }

    /// Out of two options give the one that is asymptotically smaller.
    pub fn combine_parallel_min(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Constant, _) | (_, Self::Constant) => Self::Constant,
            (Self::Linear, _) | (_, Self::Linear) => Self::Linear,
            (Self::Polynomial, _) | (_, Self::Polynomial) => Self::Polynomial,
            (Self::Exponential, _) | (_, Self::Exponential) => Self::Exponential,
            (Self::Tower, _) | (_, Self::Tower) => Self::Tower,
            (Self::Exists, Self::Exists) => Self::Exists,
        }
    }

    /// Out of two options give the one that is asymptotically bigger.
    pub fn combine_parallel_max(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Exists, _) | (_, Self::Exists) => Self::Exists,
            (Self::Tower, _) | (_, Self::Tower) => Self::Tower,
            (Self::Exponential, _) | (_, Self::Exponential) => Self::Exponential,
            (Self::Polynomial, _) | (_, Self::Polynomial) => Self::Polynomial,
            (Self::Linear, _) | (_, Self::Linear) => Self::Linear,
            (Self::Constant, Self::Constant) => Self::Constant, // constant & something is constant
        }
    }

}

impl CpxInfo {

    /// Combine the two complexities to represent the transitive complexity.
    pub fn combine_serial(&self, other: &Self) -> Self {
        match (self, other) {
            // (a = other) and (other rel c) => (a rel c)
            (Self::Equivalence, a) | (a, Self::Equivalence) => a.clone(),
            // No matter what, nothing can be deduced.
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            // Lower bounds are existential, i.e., there exists a graph
            // that has value other > f(A) which does not compose, because
            // existing graphs are not necessarily the same.
            (Self::Exclusion, _) | (_, Self::Exclusion) => Self::Unknown,
            (Self::LowerBound { .. }, _) | (_, Self::LowerBound { .. }) => Self::Unknown,
            (Self::Inclusion { mn: _, mx: mxa }, Self::Inclusion { mn: _, mx: mxb })
                => Self::Inclusion { mn: CpxTime::Constant, mx: mxa.combine_serial(&mxb) },
        }
    }

    /// Combine the two complexities' best results.
    pub fn combine_parallel(&self, other: &Self) -> Result<Self, CombinationError> {
        Ok(match (self, other) {
            // Prefer anything before taking Unknown.
            (Self::Unknown, a) | (a, Self::Unknown) => a.clone(),
            // Check equivalence is compatible with the other bound and if so, keep it.
            (Self::Equivalence, Self::Equivalence) => Self::Equivalence,
            (Self::Equivalence, Self::LowerBound { mn }) | (Self::LowerBound { mn }, Self::Equivalence) => {
                match mn {
                    CpxTime::Constant | CpxTime::Linear => Self::Equivalence,
                    _ => return Err(CombinationError::IncompatibleWithEquivalence(self.clone(), other.clone())),
                }
            },
            (Self::Equivalence, Self::Inclusion { mn, mx }) | (Self::Inclusion { mn, mx }, Self::Equivalence) => {
                match (mn, mx) {
                    (CpxTime::Constant | CpxTime::Linear,
                     CpxTime::Linear | CpxTime::Polynomial | CpxTime::Exponential | CpxTime::Tower | CpxTime::Exists)
                        => Self::Equivalence,
                        (_, _) => return Err(CombinationError::IncompatibleWithEquivalence(self.clone(), other.clone())),
                }
            },
            (Self::Exclusion, Self::Equivalence) | (Self::Equivalence, Self::Exclusion) => panic!("impossible"),
            // If both are inclusions or lower bounds we can nicely combine them.
            (Self::Inclusion { mn: mna, mx: mxa }, Self::Inclusion { mn: mnb, mx: mxb })
                => Self::Inclusion {
                    mn: mna.combine_parallel_max(&mnb),
                    mx: mxa.combine_parallel_min(&mxb),
                },
            (Self::Inclusion { mn: mna, mx: mxa }, Self::LowerBound { mn: mnb })
                | (Self::LowerBound { mn: mnb }, Self::Inclusion { mn: mna, mx: mxa })
                => Self::Inclusion {
                    mn: mna.combine_parallel_max(&mnb),
                    mx: mxa.clone(),
                },
            (Self::LowerBound { mn: mna }, Self::LowerBound { mn: mnb })
                => Self::LowerBound {
                    mn: mna.combine_parallel_max(&mnb),
                },
            // Lower bounds are weaker exclusions.
            (Self::Exclusion, Self::Exclusion)
                | (Self::LowerBound { .. }, Self::Exclusion)
                | (Self::Exclusion, Self::LowerBound { .. })
                => Self::Exclusion,
            // We cannot combine exclusion and inclusion as they are disjoint cases.
            (Self::Exclusion, Self::Inclusion { .. })
                | (Self::Inclusion { .. }, Self::Exclusion)
                => return Err(CombinationError::ExclusionInclusion(self.clone(), other.clone())),
        })
    }

}
