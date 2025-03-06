use crate::general::enums::{
    ComparisonResult, CpxInfo,
    CpxTime::{self, *},
    SourcedCpxInfo,
};

impl CpxTime {
    pub fn num(&self) -> u32 {
        match self {
            Constant => 0,
            Linear => 1,
            Polynomial => 2,
            Exponential => 3,
            Tower => 4,
            Exists => 5,
        }
    }

    pub fn is_smaller_than(&self, other: &Self) -> bool {
        self.num() < other.num()
    }
}

impl SourcedCpxInfo {
    fn num(&self) -> u32 {
        match self {
            Self::Equal { .. } => 0,
            Self::Inclusion { .. } => 1,
            Self::Exclusion { .. } => 2,
            Self::Unknown => 3,
        }
    }

    pub fn compare_to(&self, other: &SourcedCpxInfo) -> ComparisonResult {
        let (a, b, should_flip_result) = if self.num() < other.num() {
            (self, other, false)
        } else {
            (other, self, true)
        };
        let res: ComparisonResult = match (a, b) {
            (Self::Equal { .. }, Self::Equal { .. }) => ComparisonResult::Equivalent,
            (Self::Equal { .. }, Self::Inclusion { .. }) => ComparisonResult::Better,
            (Self::Equal { .. }, Self::Exclusion { .. }) => panic!("impossible {:?} {:?}", a, b),
            (Self::Equal { .. }, Self::Unknown) => ComparisonResult::Better,
            (Self::Inclusion { .. }, Self::Exclusion { .. }) => {
                panic!("impossible {:?} {:?}", a, b)
            }
            (Self::Inclusion { .. }, Self::Unknown) => ComparisonResult::Better,
            (Self::Inclusion { mn: mna, mx: mxa }, Self::Inclusion { mn: mnb, mx: mxb }) => {
                let res_mn: ComparisonResult = match (mna, mnb) {
                    (Some((a, sa)), Some((b, sb))) => {
                        if a == b {
                            ComparisonResult::Equivalent
                        } else if a.is_smaller_than(b) {
                            ComparisonResult::Worse
                        } else {
                            ComparisonResult::Better
                        }
                    }
                    (Some((a, sa)), None) => ComparisonResult::Better,
                    (None, Some((a, sa))) => ComparisonResult::Worse,
                    (None, None) => ComparisonResult::Equivalent,
                };
                let res_mx: ComparisonResult = match (mxa, mxb) {
                    (Some((a, sa)), Some((b, sb))) => {
                        if a == b {
                            ComparisonResult::Equivalent
                        } else if a.is_smaller_than(b) {
                            ComparisonResult::Better
                        } else {
                            ComparisonResult::Worse
                        }
                    }
                    (Some((a, sa)), None) => ComparisonResult::Better,
                    (None, Some((a, sa))) => ComparisonResult::Worse,
                    (None, None) => ComparisonResult::Equivalent,
                };
                match (res_mn, res_mx) {
                    (ComparisonResult::Equivalent, ComparisonResult::Equivalent) => {
                        ComparisonResult::Equivalent
                    }
                    (
                        ComparisonResult::Better | ComparisonResult::Equivalent,
                        ComparisonResult::Better | ComparisonResult::Equivalent,
                    ) => ComparisonResult::Better,
                    (
                        ComparisonResult::Worse | ComparisonResult::Equivalent,
                        ComparisonResult::Worse | ComparisonResult::Equivalent,
                    ) => ComparisonResult::Worse,
                    (ComparisonResult::Worse, ComparisonResult::Better)
                    | (ComparisonResult::Better, ComparisonResult::Worse) => {
                        ComparisonResult::Incomparable
                    }
                    (ComparisonResult::Incomparable, _) | (_, ComparisonResult::Incomparable) => {
                        ComparisonResult::Incomparable
                    }
                }
            }
            (Self::Exclusion { .. }, Self::Exclusion { .. }) => ComparisonResult::Equivalent,
            (Self::Exclusion { .. }, Self::Unknown) => ComparisonResult::Better,
            (Self::Unknown, Self::Unknown) => ComparisonResult::Equivalent,
            _ => {
                panic!("impossible case which should have been handled by comparing and swapping before the comparison");
            }
        };
        if should_flip_result {
            res.flip()
        } else {
            res
        }
    }
}
