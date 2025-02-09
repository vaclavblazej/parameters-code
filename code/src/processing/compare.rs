use crate::general::enums::{CpxInfo, CpxTime::{self, *}};


impl CpxTime {

    pub fn is_smaller_than(&self, other: &Self) -> bool {
        // todo define this linear order implicitly
        match (self, other) {
            (Constant, Linear | Polynomial | Exponential | Tower | Exists)
            | (Linear, Polynomial | Exponential | Tower | Exists)
            | (Polynomial, Exponential | Tower | Exists)
            | (Exponential, Tower | Exists)
            | (Tower, Exists)
            => { true },
            _ => { false },
        }
    }

}
