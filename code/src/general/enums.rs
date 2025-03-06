//! Trivial enums that do not contain complex structure
//! and so can be used from input till output.

use serde::{Deserialize, Serialize};

use crate::data::{
    data::PartialResult, id::PreviewSetId, preview::{PreviewSet, PreviewSource}
};

/// Refers to a page in a book or paper. If pdf is available it should refer its
/// page in pdf instead of the label.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Page {
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
        relevance: u32,
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
    Constant,    // O(1)
    Linear,      // O(N)
    Polynomial,  // N^{O(1)}
    Exponential, // 2^{O(N)}
    Tower,       // 2^2^...^N
    Exists,      // f(N) where f is a computable function
}

/// What we know about parameter increase over a binary relation A with B.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpxInfo {
    /// Values are the same, B = A; this relation is symmetric.
    Equal,
    /// Value of B is at least mn(A) and at most mx(A). Either mn or mx can be omitted
    /// if that bound is unknown, but at least one be should always present.
    Inclusion {
        mn: Option<CpxTime>,
        mx: Option<CpxTime>,
    },
    /// Value of B is not bounded by any function of A.
    Exclusion,
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
    Direct {
        typ: BoundType,
        source: PreviewSource,
    },
    Indirect {
        typ: BoundType,
        source: usize,
    }, // idx to data
}

// Saved indices point to 'data.partial_results'
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreatedBy {
    TransitiveInclusion(usize, usize),
    TransitiveExclusion(usize, usize),
    ParallelComposition(usize, usize),
    SameThroughEquivalence(usize, usize),
    SumInclusion(Vec<usize>),
    TransferredFrom(TransferGroup, usize),
    Directly(PreviewSource),
    Todo,
}

/// A processed variant of CpxInfo which has links to sources that lead to a given result
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourcedCpxInfo {
    Equal {
        source: PartialResult,
    },
    Inclusion {
        mn: Option<(CpxTime, PartialResult)>,
        mx: Option<(CpxTime, PartialResult)>,
    },
    Exclusion {
        source: PartialResult,
    },
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RawDrawing {
    Table(Vec<PreviewSetId>),
    Hasse(Vec<PreviewSetId>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Drawing {
    Table(Vec<PreviewSet>),
    Hasse(Vec<PreviewSet>),
}

impl CpxInfo {
    pub fn get_mx(&self) -> Option<CpxTime> {
        match self {
            CpxInfo::Equal => Some(CpxTime::Linear),
            CpxInfo::Inclusion {
                mn: _,
                mx: Some(ref x),
            } => Some(x.clone()),
            CpxInfo::Inclusion { .. } => None,
            CpxInfo::Exclusion => None,
            CpxInfo::Unknown => None,
        }
    }

    pub fn to_sourced(self, partial_result: PartialResult) -> SourcedCpxInfo {
        match self.clone() {
            CpxInfo::Equal => SourcedCpxInfo::Equal {
                source: partial_result,
            },
            CpxInfo::Inclusion { mn, mx } => SourcedCpxInfo::Inclusion {
                mn: match mn {
                    Option::Some(x) => Some((x, partial_result.clone())),
                    Option::None => None,
                },
                mx: match mx {
                    Option::Some(x) => Some((x, partial_result.clone())),
                    Option::None => None,
                },
            },
            CpxInfo::Exclusion => SourcedCpxInfo::Exclusion {
                source: partial_result,
            },
            CpxInfo::Unknown => SourcedCpxInfo::Unknown,
        }
    }
}

impl Into<CpxInfo> for SourcedCpxInfo {
    fn into(self) -> CpxInfo {
        match self {
            SourcedCpxInfo::Equal { source } => CpxInfo::Equal,
            SourcedCpxInfo::Inclusion { mn, mx } => CpxInfo::Inclusion {
                mn: match mn {
                    Some((x, _)) => Some(x),
                    Option::None => None,
                },
                mx: match mx {
                    Some((x, _)) => Some(x),
                    Option::None => None,
                },
            },
            SourcedCpxInfo::Exclusion { source: _ } => CpxInfo::Exclusion,
            SourcedCpxInfo::Unknown => CpxInfo::Unknown,
        }
    }
}

pub enum ComparisonResult {
    Better,
    Worse,
    Incomparable,
    Equivalent,
}

impl ComparisonResult {
    pub fn flip(self) -> Self {
        match self {
            Self::Better => Self::Worse,
            Self::Worse => Self::Better,
            Self::Incomparable => Self::Incomparable,
            Self::Equivalent => Self::Equivalent,
        }
    }
}
