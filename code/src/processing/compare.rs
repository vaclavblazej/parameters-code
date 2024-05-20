use crate::general::enums::{CpxInfo, CpxTime::{self, *}};


impl CpxTime {

    pub fn is_better_than(&self, other: &Self) -> bool {
        // todo define this linear order implicitly
        match (self, other) {
            (Constant, Linear | Polynomial | Exponential | Tower(_) | Exists)
            | (Linear, Polynomial | Exponential | Tower(_) | Exists)
            | (Polynomial, Exponential | Tower(_) | Exists)
            | (Exponential, Tower(_) | Exists)
            | (Tower(_), Exists)
            => { true },
            _ => { false },
        }
    }

}
