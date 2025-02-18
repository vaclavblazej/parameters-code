use crate::general::enums::{ComparisonResult, CpxInfo, CpxTime::{self, *}, SourcedCpxInfo};


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
            Self::UpperBound { .. } => 2,
            Self::LowerBound { .. } => 3,
            Self::Exclusion { .. } => 4,
            Self::Unknown => 5,
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
            (Self::Equal { .. }, Self::UpperBound { .. }) => ComparisonResult::Better,
            (Self::Equal { .. }, Self::LowerBound { .. }) => ComparisonResult::Better,
            (Self::Equal { .. }, Self::Exclusion { .. }) => panic!("impossible {:?} {:?}", a, b),
            (Self::Equal { .. }, Self::Unknown) => ComparisonResult::Better,
            (Self::Inclusion { mn: (mna, _), mx: (mxa, _) }, Self::Inclusion { mn: (mnb, _), mx: (mxb, _) }) => {
                if mna == mnb && mxa == mxb {
                    ComparisonResult::Equivalent
                } else if mna == mnb && mxa.is_smaller_than(mxb)
                       || mxa == mxb && mnb.is_smaller_than(mna)
                       || mnb.is_smaller_than(mna) && mxa.is_smaller_than(mxb) {
                    ComparisonResult::Better
                } else {
                    ComparisonResult::Worse
                }
            },
            (Self::Inclusion { mn, mx: (mxa, _) }, Self::UpperBound { mx: (mxb, _) }) => {
                if mxb.is_smaller_than(mxa) { ComparisonResult::Incomparable } else { ComparisonResult::Better }
            },
            (Self::Inclusion { mn: (mna, _), mx }, Self::LowerBound { mn: (mnb, _) }) => {
                if mna.is_smaller_than(mnb) { ComparisonResult::Incomparable } else { ComparisonResult::Better }
            },
            (Self::Inclusion { mn, mx }, Self::Exclusion { .. }) => panic!("impossible {:?} {:?}", a, b),
            (Self::Inclusion { mn, mx }, Self::Unknown) => ComparisonResult::Better,
            (Self::UpperBound { mx: (mxa, _) }, Self::UpperBound { mx: (mxb, _) }) => {
                if mxa == mxb { ComparisonResult::Equivalent } else if mxa.is_smaller_than(mxb) { ComparisonResult::Better } else { ComparisonResult::Worse }
            },
            (Self::UpperBound { mx }, Self::LowerBound { mn }) => ComparisonResult::Worse,
            (Self::UpperBound { mx }, Self::Exclusion { .. }) => panic!("impossible {:?} {:?}", a, b),
            (Self::UpperBound { mx }, Self::Unknown) => ComparisonResult::Better,
            (Self::LowerBound { mn: (mna, _) }, Self::LowerBound { mn: (mnb, _) }) => {
                if mna == mnb { ComparisonResult::Equivalent } else if mna.is_smaller_than(mnb) { ComparisonResult::Worse } else { ComparisonResult::Better }
            },
            (Self::LowerBound { mn }, Self::Exclusion { .. }) => panic!("impossible {:?} {:?}", a, b),
            (Self::LowerBound { mn }, Self::Unknown) => ComparisonResult::Better,
            (Self::Exclusion { .. }, Self::Exclusion { .. }) => ComparisonResult::Equivalent,
            (Self::Exclusion { .. }, Self::Unknown) => ComparisonResult::Better,
            (Self::Unknown, Self::Unknown) => ComparisonResult::Equivalent,
            _ => { panic!("impossible case which should have been handled by comparing and swapping before the comparison"); }
        };
        if should_flip_result {
            res.flip()
        } else {
            res
        }
    }

}
