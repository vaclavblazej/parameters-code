//! Trivial enums that do not contain complex structure
//! and so can be used from input till output.

use serde::{Serialize, Deserialize};

use crate::data::{data::PartialResult, preview::{PreviewRelation, PreviewSource}};


/// Refers to a page in a book or paper. If pdf is available it should refer its
/// page in pdf instead of the label.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Page{
    Pp(u32),
    Unknown,
    NotApplicable,
}

/// When a set is part of a transfer group T then we know that a relation between
/// A and B where both belong to the group also happens between T(A) and T(B).
// todo -- it is possible that the transfer assumption is too general and only inclusions should be transferred
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TransferGroup {
    DistanceTo,
    EdgeCover,
}

/// Points to the source of a citation.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum SourceKey {
    Bibtex {
        key: String,
        name: Option<String>,
        entry: Option<String>,
    },
    Online {
        url: String,
    },
    Other {
        name: String,
        description: String,
    },
}

/// Enum that makes inputting complexities more convenient.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cpx {
    Bounds(CpxTime, CpxTime),
    UpperBound(CpxTime),
    StrictUpperBound(CpxTime),
    Exactly(CpxTime),
    Equivalent(CpxTime, CpxTime),
    Equal,
    Exclusion,
    Incomparable,
    Todo,
}

/// High-level representation of values for computational complexity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpxTime {
    // with deeper processing, we would be able to devise these from the resulting equations directly
    Constant,   // O(1)
    Linear,     // O(N)
    Polynomial, // N^{O(1)}
    Exponential,// 2^{O(N)}
    Tower,      // 2^2^...^N
    Exists,     // f(N) where f is a computable function
}

/// What we know about parameter increase over a binary relation A with B.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpxInfo {
    /// Values are the same, B = A; this relation is symmetric.
    Equal,
    /// Value of B is at least mn(A) and at most mx(A).
    Inclusion{mn: CpxTime, mx: CpxTime},
    /// Value of B is not bounded by any function of A.
    Exclusion,
    /// Value of B is at least mn(A) but upper bound is unknown.
    /// LowerBound can be further refined to Inclusion or Exclusion.
    LowerBound{mn: CpxTime},
    /// Value of B is at most mx(A).
    UpperBound{mx: CpxTime},
    /// There is no information about whether B is bounded by the value of A.
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BoundType {
    UpperBound { cpx: CpxTime },
    LowerBound { cpx: CpxTime },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SourcedBound {
    Direct { typ: BoundType, source: PreviewSource },
    Indirect { typ: BoundType, source: usize }, // idx to data
}

// Saved indices point to 'data.partial_results'
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreatedBy {
    TransitiveInclusion(usize, usize),
    TransitiveExclusion(usize, usize),
    SameThroughEquivalence(usize, usize),
    SumInclusion(usize, usize),
    TransferredFrom(TransferGroup, usize),
    Directly(PreviewSource),
    Todo,
}

/// A processed variant of CpxInfo which has links to sources that lead to a given result
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourcedCpxInfo {
    Equal{source: PartialResult},
    UpperBound{mx: (CpxTime, PartialResult)},
    Inclusion{mn: (CpxTime, PartialResult), mx: (CpxTime, PartialResult)},
    Exclusion{source: PartialResult},
    LowerBound{mn: (CpxTime, PartialResult)},
    Unknown,
}

impl CpxInfo {

    pub fn to_sourced(self, partial_result: PartialResult) -> SourcedCpxInfo {
        match self.clone() {
            CpxInfo::Equal => SourcedCpxInfo::Equal {
                source: partial_result
            },
            CpxInfo::Inclusion { mn, mx } => SourcedCpxInfo::Inclusion {
                mn: (mn, partial_result.clone()),
                mx: (mx, partial_result)
            },
            CpxInfo::UpperBound { mx } => SourcedCpxInfo::UpperBound {
                mx: (mx, partial_result)
            },
            CpxInfo::LowerBound { mn } => SourcedCpxInfo::LowerBound {
                mn: (mn, partial_result)
            },
            CpxInfo::Exclusion => SourcedCpxInfo::Exclusion { source: partial_result },
            CpxInfo::Unknown => SourcedCpxInfo::Unknown,
        }
    }

}

impl SourcedCpxInfo {

    fn num(&self) -> u32 {
        match self {
            Self::Equal { .. } => 0,
            Self::Inclusion { .. } => 1,
            Self::UpperBound { .. } => 2,
            Self::LowerBound { .. } => 3,
            Self::Exclusion { .. } => 4,
            Self::Unknown => 5,
        }
    }

    pub fn is_better_than(&self, other: &SourcedCpxInfo) -> bool {
        let (a, b) = if self.num() < other.num() {
            (self, other)
        } else {
            (other, self)
        };
        match (a, b) {
            (Self::Equal { .. }, Self::Equal { .. }) => false,
            (Self::Equal { .. }, Self::Inclusion { .. }) => true,
            (Self::Equal { .. }, Self::UpperBound { .. }) => true,
            (Self::Equal { .. }, Self::LowerBound { .. }) => true,
            (Self::Equal { .. }, Self::Exclusion { .. }) => panic!("impossible {:?} {:?}", a, b),
            (Self::Equal { .. }, Self::Unknown) => true,
            (Self::Inclusion { mn: (mna, _), mx: (mxa, _) }, Self::Inclusion { mn: (mnb, _), mx: (mxb, _) }) => {
                   mna == mnb && mxa.is_smaller_than(mxb)
                || mxa == mxb && mnb.is_smaller_than(mna)
                || mnb.is_smaller_than(mna) && mxa.is_smaller_than(mxb)
            },
            (Self::Inclusion { mn, mx: (mxa, _) }, Self::UpperBound { mx: (mxb, _) }) => !mxb.is_smaller_than(mxa),
            (Self::Inclusion { mn: (mna, _), mx }, Self::LowerBound { mn: (mnb, _) }) => !mna.is_smaller_than(mnb),
            (Self::Inclusion { mn, mx }, Self::Exclusion { .. }) => panic!("impossible {:?} {:?}", a, b),
            (Self::Inclusion { mn, mx }, Self::Unknown) => true,
            (Self::UpperBound { mx: (mxa, _) }, Self::UpperBound { mx: (mxb, _) }) => mxa.is_smaller_than(mxb),
            (Self::UpperBound { mx }, Self::LowerBound { mn }) => false,
            (Self::UpperBound { mx }, Self::Exclusion { .. }) => panic!("impossible {:?} {:?}", a, b),
            (Self::UpperBound { mx }, Self::Unknown) => true,
            (Self::LowerBound { mn: (mna, _) }, Self::LowerBound { mn: (mnb, _) }) => mnb.is_smaller_than(mna),
            (Self::LowerBound { mn }, Self::Exclusion { .. }) => panic!("impossible {:?} {:?}", a, b),
            (Self::LowerBound { mn }, Self::Unknown) => true,
            (Self::Exclusion { .. }, Self::Exclusion { .. }) => false,
            (Self::Exclusion { .. }, Self::Unknown) => true,
            (Self::Unknown, Self::Unknown) => false,
            _ => { panic!("impossible case which should have been handled by comparing and swapping before the comparison"); }
        }
    }

}

impl Into<CpxInfo> for SourcedCpxInfo {
    fn into(self) -> CpxInfo {
        match self {
            SourcedCpxInfo::Equal { source } => CpxInfo::Equal,
            SourcedCpxInfo::UpperBound { mx: (mx, _) } => CpxInfo::UpperBound { mx: mx.clone() },
            SourcedCpxInfo::Inclusion { mn: (mn, _), mx: (mx, _) } => CpxInfo::Inclusion { mn: mn.clone(), mx: mx.clone() },
            SourcedCpxInfo::Exclusion { source: _ } => CpxInfo::Exclusion,
            SourcedCpxInfo::Unknown => CpxInfo::Unknown,
            SourcedCpxInfo::LowerBound { mn: (mn, _) } => CpxInfo::LowerBound { mn: mn.clone() },
        }
    }
}
