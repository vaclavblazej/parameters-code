//! todo


/// High-level representation of a mathematical equation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CpxTime {
    // with deeper processing, we would be able to devise these from the resulting equations directly
    Constant,
    Linear,
    Polynomial,
    Exponential,
    Tower,
    Exists,
}

impl CpxTime {

    /// What kind of complexity we get when we substitute k with b instead?
    pub fn combine_serial(&self, b: &Self) -> Self {
        match (self, b) {
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
    pub fn combine_parallel_min(&self, b: &Self) -> Self {
        match (self, b) {
            (Self::Constant, _) | (_, Self::Constant) => Self::Constant,
            (Self::Linear, _) | (_, Self::Linear) => Self::Linear,
            (Self::Polynomial, _) | (_, Self::Polynomial) => Self::Polynomial,
            (Self::Exponential, _) | (_, Self::Exponential) => Self::Exponential,
            (Self::Tower, _) | (_, Self::Tower) => Self::Tower,
            (Self::Exists, Self::Exists) => Self::Exists,
        }
    }

    /// Out of two options give the one that is asymptotically bigger.
    pub fn combine_parallel_max(&self, b: &Self) -> Self {
        match (self, b) {
            (Self::Exists, _) | (_, Self::Exists) => Self::Exists,
            (Self::Tower, _) | (_, Self::Tower) => Self::Tower,
            (Self::Exponential, _) | (_, Self::Exponential) => Self::Exponential,
            (Self::Polynomial, _) | (_, Self::Polynomial) => Self::Polynomial,
            (Self::Linear, _) | (_, Self::Linear) => Self::Linear,
            (Self::Constant, Self::Constant) => Self::Constant,
        }
    }

}

/// What we know about parameter increase over a binary relation inclusion
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CpxInfo{
    // todo: minor issue is that we can show mn while not showing mx, which still allows exclusion
    Inclusion{mn: CpxTime, mx: CpxTime},
    Exclusion,
    Equivalence,
    Unknown,
}

impl CpxInfo {

    /// Combine the two complexities to represent the transitive complexity.
    pub fn combine_serial(&self, b: &Self) -> Self {
        match (self, b) {
            (Self::Equivalence, a) | (a, Self::Equivalence) => a.clone(),
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            (Self::Exclusion, _) | (_, Self::Exclusion) => Self::Unknown,
            // note that serial composition does not give bounds on lower bound
            (Self::Inclusion { mn: _, mx: mxa }, Self::Inclusion { mn: _, mx: mxb })
                => Self::Inclusion { mn: CpxTime::Constant, mx: mxa.combine_serial(&mxb) },
        }
    }

    /// Combine the two complexities' best results.
    pub fn combine_parallel(&self, b: &Self) -> Self {
        match (self, b) {
            (Self::Unknown, a) | (a, Self::Unknown) => a.clone(),
            (Self::Inclusion { mn: mna, mx: mxa }, Self::Inclusion { mn: mnb, mx: mxb })
                => Self::Inclusion {
                    mn: mna.combine_parallel_max(&mnb),
                    mx: mxa.combine_parallel_min(&mxb),
                },
            (Self::Exclusion, Self::Exclusion) => Self::Exclusion,
            (Self::Exclusion, Self::Inclusion { .. } | Self::Equivalence) => panic!("impossible ex inc"),
            (Self::Inclusion { .. } | Self::Equivalence, Self::Exclusion) => panic!("impossible inc ex"),
            (Self::Equivalence, Self::Inclusion { .. }) => Self::Equivalence,
            (Self::Inclusion { .. }, Self::Equivalence) => Self::Equivalence,
            (Self::Equivalence, Self::Equivalence) => Self::Equivalence,
        }
    }

}
