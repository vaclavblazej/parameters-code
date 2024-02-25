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
