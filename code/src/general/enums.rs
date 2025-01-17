//! Trivial enums that do not contain complex structure
//! and so can be used from input till output.

use serde::{Serialize, Deserialize};


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
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
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
    Tower, // 2^2^...^N
    Exists,     // f(N) where f is a computable function
}

/// What we know about parameter increase over a binary relation A with B.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpxInfo {
    /// Value of B is at least mn(A) and at most mx(A).
    Inclusion{mn: CpxTime, mx: CpxTime},
    /// Value of B is not bounded by any function of A.
    Exclusion,
    /// Values are the same, B = A; this relation is symmetric.
    /// Equal is a more refined Inclusion{Linear, Linear}.
    Equal,
    /// Value of B is at least mn(A) but upper bound is unknown.
    /// LowerBound can be further refined to Inclusion or Exclusion.
    LowerBound{mn: CpxTime},
    /// There is no information about whether B is bounded by the value of A.
    Unknown,
}

