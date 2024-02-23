//! Utilities structures for handling computational complexity.


/// High-level representation of values for computational complexity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CpxTime {
    // with deeper processing, we would be able to devise these from the resulting equations directly
    Constant,   // O(1)
    Linear,     // O(N)
    Polynomial, // N^{O(1)}
    Exponential,// 2^{O(N)}
    Tower(u32), // 2^2^...^N of given length
    Exists,     // f(N) where f is a computable function
}

impl CpxTime {

    /// What kind of complexity we get when we substitute k with b instead?
    pub fn combine_serial(&self, b: &Self) -> Self {
        match (self, b) {
            (Self::Exists, _) | (_, Self::Exists) => Self::Exists,
            (Self::Tower(a), _) | (_, Self::Tower(a)) => Self::Tower(a.clone()),
            (Self::Exponential, Self::Exponential) => Self::Tower(2),
            (Self::Exponential, _) | (_, Self::Exponential) => Self::Exponential,
            (Self::Polynomial, _) | (_, Self::Polynomial) => Self::Polynomial,
            (Self::Linear, _) | (_, Self::Linear) => Self::Linear,
            (Self::Constant, Self::Constant) => Self::Constant,
        }
    }

    /// Out of two options give the one that is asymptotically smaller.
    pub fn combine_parallel_min(&self, b: &Self) -> Self {
        match (self, b) {
            (Self::Constant, _) | (_, Self::Constant) => Self::Constant,
            (Self::Linear, _) | (_, Self::Linear) => Self::Linear,
            (Self::Polynomial, _) | (_, Self::Polynomial) => Self::Polynomial,
            (Self::Exponential, _) | (_, Self::Exponential) => Self::Exponential,
            (Self::Tower(a), Self::Tower(b)) => Self::Tower(a.clone().min(b.clone())),
            (Self::Tower(a), _) | (_, Self::Tower(a)) => Self::Tower(a.clone()),
            (Self::Exists, Self::Exists) => Self::Exists,
        }
    }

    /// Out of two options give the one that is asymptotically bigger.
    pub fn combine_parallel_max(&self, b: &Self) -> Self {
        match (self, b) {
            (Self::Exists, _) | (_, Self::Exists) => Self::Exists,
            (Self::Tower(a), Self::Tower(b)) => Self::Tower(a.clone().max(b.clone())),
            (Self::Tower(a), _) | (_, Self::Tower(a)) => Self::Tower(a.clone()),
            (Self::Exponential, _) | (_, Self::Exponential) => Self::Exponential,
            (Self::Polynomial, _) | (_, Self::Polynomial) => Self::Polynomial,
            (Self::Linear, _) | (_, Self::Linear) => Self::Linear,
            (Self::Constant, Self::Constant) => Self::Constant,
        }
    }

}

/// What we know about parameter increase over a binary relation A with B.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CpxInfo{
    /// Value of B is at least mn(A) and at most mx(A).
    Inclusion{mn: CpxTime, mx: CpxTime},
    /// Value of B is not bounded by any function of A.
    Exclusion,
    /// Values are the same, B = A; this relation is symmetric.
    /// Equivalence is a more refined Inclusion{Linear, Linear}.
    Equivalence,
    /// Value of B is at least mn(A) but upper bound is unknown.
    /// LowerBound can be further refined to Inclusion or Exclusion.
    LowerBound{mn: CpxTime},
    /// There is no information about whether B is bounded by the value of A.
    Unknown,
}

impl CpxInfo {

    /// Combine the two complexities to represent the transitive complexity.
    pub fn combine_serial(&self, b: &Self) -> Self {
        match (self, b) {
            // (a = b) and (b rel c) => (a rel c)
            (Self::Equivalence, a) | (a, Self::Equivalence) => a.clone(),
            // No matter what, nothing can be deduced.
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            // Lower bounds are existential, i.e., there exists a graph
            // that has value B > f(A) which does not compose, because
            // existing graphs are not necessarily the same.
            (Self::Exclusion, _) | (_, Self::Exclusion) => Self::Unknown,
            (Self::LowerBound { .. }, _) | (_, Self::LowerBound { .. }) => Self::Unknown,
            (Self::Inclusion { mn: _, mx: mxa }, Self::Inclusion { mn: _, mx: mxb })
                => Self::Inclusion { mn: CpxTime::Constant, mx: mxa.combine_serial(&mxb) },
        }
    }

    /// Combine the two complexities' best results.
    pub fn combine_parallel(&self, b: &Self) -> Self {
        match (self, b) {
            // Prefer anything before taking Unknown.
            (Self::Unknown, a) | (a, Self::Unknown) => a.clone(),
            // Check equivalence is compatible with the other bound and if so, keep it.
            (Self::Equivalence, Self::Equivalence) => Self::Equivalence,
            (Self::Equivalence, Self::LowerBound { mn }) | (Self::LowerBound { mn }, Self::Equivalence) => {
                match mn {
                    CpxTime::Constant | CpxTime::Linear => Self::Equivalence,
                    _ => panic!("impossible"),
                }
            },
            (Self::Equivalence, Self::Inclusion { mn, mx }) | (Self::Inclusion { mn, mx }, Self::Equivalence) => {
                match (mn, mx) {
                    (CpxTime::Constant | CpxTime::Linear,
                     CpxTime::Linear | CpxTime::Polynomial | CpxTime::Exponential | CpxTime::Tower(_) | CpxTime::Exists)
                        => Self::Equivalence,
                        (_, _) => panic!("impossible"),
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
                => panic!("impossible"),
        }
    }

}
