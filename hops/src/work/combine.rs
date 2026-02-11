use core::fmt;
use log::{debug, error, trace};

use crate::data::data::*;
use crate::data::enums::*;
use crate::data::id::*;

// #[derive(Debug)]
// pub enum CombinationError {
//     ExclusionInclusion(PartialResult, PartialResult),
//     IncompatibleWithEquivalence(PartialResult, PartialResult),
// }
//
// impl fmt::Display for CombinationError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             CombinationError::ExclusionInclusion(a, b) => write!(
//                 f,
//                 "Tried to combine Exclusion with Inclusion relation in parallel:\n{:?}\n{:?}",
//                 a, b
//             ),
//             CombinationError::IncompatibleWithEquivalence(a, b) => write!(
//                 f,
//                 "Tried to combine equivalence with an incompatible relation in parallel:\n{:?}\n{:?}",
//                 a, b
//             ),
//         }
//     }
// }
//
// impl PartialResultsBuilder {
//     pub fn new() -> Self {
//         Self { arr: vec![] }
//     }
//
//     pub fn partial_result(
//         &mut self,
//         created_by: CreatedBy,
//         cpx: CpxInfo,
//         relation: WorkRelation,
//     ) -> PartialResult {
//         let len = self.arr.len();
//         let res = PartialResult {
//             handle: len,
//             created_by,
//             cpx,
//             relation,
//         };
//         self.arr.push(res.clone());
//         res
//     }
//
//     pub fn done(self) -> Vec<PartialResult> {
//         self.arr
//     }
// }
//
// impl Default for PartialResultsBuilder {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// impl WorkRelation {
//     pub fn new(subset: &PreviewSetId, superset: &PreviewSetId) -> Self {
//         WorkRelation {
//             subset: subset.clone(),
//             superset: superset.clone(),
//         }
//     }
//
//     pub fn combine_serial(&self, other: &Self) -> Self {
//         assert_eq!(self.superset, other.subset);
//         Self {
//             subset: self.subset.clone(),
//             superset: other.superset.clone(),
//         }
//     }
// }
//
// /// Out of two options give the one that is asymptotically smaller.
// pub fn combine_parallel_min(
//     a: (CpxTime, PartialResult),
//     b: (CpxTime, PartialResult),
// ) -> (CpxTime, PartialResult) {
//     match (a, b) {
//         ((CpxTime::Constant, a), _) | (_, (CpxTime::Constant, a)) => (CpxTime::Constant, a),
//         ((CpxTime::Linear, a), _) | (_, (CpxTime::Linear, a)) => (CpxTime::Linear, a),
//         ((CpxTime::Polynomial, a), _) | (_, (CpxTime::Polynomial, a)) => (CpxTime::Polynomial, a),
//         ((CpxTime::Exponential, a), _) | (_, (CpxTime::Exponential, a)) => {
//             (CpxTime::Exponential, a)
//         }
//         ((CpxTime::Tower, a), _) | (_, (CpxTime::Tower, a)) => (CpxTime::Tower, a),
//         ((CpxTime::Exists, a), (CpxTime::Exists, _)) => (CpxTime::Exists, a),
//     }
// }
//
// /// Out of two options give the one that is asymptotically bigger.
// pub fn combine_parallel_max(
//     a: (CpxTime, PartialResult),
//     b: (CpxTime, PartialResult),
// ) -> (CpxTime, PartialResult) {
//     match (a, b) {
//         ((CpxTime::Exists, a), _) | (_, (CpxTime::Exists, a)) => (CpxTime::Exists, a),
//         ((CpxTime::Tower, a), _) | (_, (CpxTime::Tower, a)) => (CpxTime::Tower, a),
//         ((CpxTime::Exponential, a), _) | (_, (CpxTime::Exponential, a)) => {
//             (CpxTime::Exponential, a)
//         }
//         ((CpxTime::Polynomial, a), _) | (_, (CpxTime::Polynomial, a)) => (CpxTime::Polynomial, a),
//         ((CpxTime::Linear, a), _) | (_, (CpxTime::Linear, a)) => (CpxTime::Linear, a),
//         ((CpxTime::Constant, a), (CpxTime::Constant, _)) => (CpxTime::Constant, a),
//     }
// }
//
// /// Returns complexity that we get when we substitute k with b
// pub fn combine_serial(
//     (cpxa, a): (CpxTime, PartialResult),
//     (cpxb, b): (CpxTime, PartialResult),
// ) -> (PartialResult, PartialResult, CpxTime) {
//     // let relation = a.relation.combine_serial(&b.relation);
//     match (cpxa, cpxb) {
//         // constant is an exception here as it doesn't grow with growing parameter,
//         // hence, anything is composes with also results in a constant
//         (CpxTime::Constant, _) | (_, CpxTime::Constant) => (a, b, CpxTime::Constant),
//         // otherwise, we take the worst of the two options
//         (CpxTime::Exists, _) | (_, CpxTime::Exists) => (a, b, CpxTime::Exists),
//         (CpxTime::Tower, _) | (_, CpxTime::Tower) => (a, b, CpxTime::Tower),
//         (CpxTime::Exponential, _) | (_, CpxTime::Exponential) => (a, b, CpxTime::Tower),
//         (CpxTime::Polynomial, _) | (_, CpxTime::Polynomial) => (a, b, CpxTime::Polynomial),
//         (CpxTime::Linear, _) | (_, CpxTime::Linear) => (a, b, CpxTime::Linear),
//     }
// }
//
// impl PartialResult {
//     // todo - combine_parallel should be changed to find the simplest way to find the resulting complexity
//     /// Combine the two complexities' best results. Returns Some if the result is better than self.
//     pub fn combine_parallel(
//         &self,
//         other: &PartialResult,
//         partial_result_builder: &mut PartialResultsBuilder,
//     ) -> Option<PartialResult> {
//         assert_eq!(self.relation, other.relation);
//         trace!("\n{:?}\n{:?}", self.relation, other.relation);
//         let original: SourcedCpxInfo = self.to_sourced();
//         let res: Result<SourcedCpxInfo, CombinationError> =
//             match (self.to_sourced(), other.to_sourced()) {
//                 (
//                     Inclusion {
//                         mn: Option::None,
//                         mx: Option::None,
//                     },
//                     _,
//                 )
//                 | (
//                     _,
//                     Inclusion {
//                         mn: Option::None,
//                         mx: Option::None,
//                     },
//                 ) => panic!("impossible none none"),
//                 // Prefer anything before taking Unknown.
//                 (Unknown, a) | (a, Unknown) => Ok(a.clone()),
//                 // Check equivalence is compatible with the other bound and if so, keep it.
//                 (Equal { source }, Equal { .. }) => Ok(Equal { source }),
//                 (Equal { source }, Inclusion { mn, mx })
//                 | (Inclusion { mn, mx }, Equal { source }) => match (mn, mx) {
//                     (
//                         Option::None | Some((CpxTime::Constant | CpxTime::Linear, _)),
//                         Option::None
//                         | Some((
//                             CpxTime::Linear
//                             | CpxTime::Polynomial
//                             | CpxTime::Exponential
//                             | CpxTime::Tower
//                             | CpxTime::Exists,
//                             _,
//                         )),
//                     ) => Ok(Equal { source }),
//                     (_, _) => Err(CombinationError::IncompatibleWithEquivalence(
//                         self.clone(),
//                         other.clone(),
//                     )),
//                 },
//                 (Equal { .. }, Exclusion { .. }) | (Exclusion { .. }, Equal { .. }) => {
//                     panic!("impossible")
//                 }
//                 // If both are inclusions, upper bounds or lower bounds, we can nicely combine them.
//                 (Inclusion { mn: mna, mx: mxa }, Inclusion { mn: mnb, mx: mxb }) => Ok(Inclusion {
//                     mn: match (mna, mnb) {
//                         (Some((a, sa)), Some((b, sb))) => {
//                             Some(combine_parallel_max((a, sa), (b, sb)))
//                         }
//                         (Some((a, sa)), None) | (None, Some((a, sa))) => Some((a, sa)),
//                         (None, None) => None,
//                     },
//                     mx: match (mxa, mxb) {
//                         (Some((a, sa)), Some((b, sb))) => {
//                             Some(combine_parallel_min((a, sa), (b, sb)))
//                         }
//                         (Some((a, sa)), None) | (None, Some((a, sa))) => Some((a, sa)),
//                         (None, None) => None,
//                     },
//                 }),
//                 // Lower bounds are weaker exclusions.
//                 (Exclusion { source }, Exclusion { .. })
//                 | (
//                     Inclusion {
//                         mn: Some(_),
//                         mx: Option::None,
//                     },
//                     Exclusion { source },
//                 )
//                 | (
//                     Exclusion { source },
//                     Inclusion {
//                         mn: Some(_),
//                         mx: Option::None,
//                     },
//                 ) => Ok(Exclusion { source }),
//                 // We cannot combine exclusion and inclusion as they are disjoint cases.
//                 (Exclusion { .. }, Inclusion { mn: _, mx: Some(_) })
//                 | (Inclusion { mn: _, mx: Some(_) }, Exclusion { .. }) => Err(
//                     CombinationError::ExclusionInclusion(self.clone(), other.clone()),
//                 ),
//             };
//         match res {
//             Ok(res) => match res.compare_to(&original) {
//                 ComparisonResult::Better => Some(partial_result_builder.partial_result(
//                     CreatedBy::ParallelComposition(self.handle, other.handle),
//                     res.into(),
//                     self.relation.clone(),
//                 )),
//                 _ => None,
//             },
//             Err(err) => {
//                 error!("{}\n{:?}\n{:?}", err, self.relation, other.relation);
//                 None
//             }
//         }
//     }
//
//     pub fn to_sourced(&self) -> SourcedCpxInfo {
//         self.cpx.clone().into_sourced(self.clone())
//     }
// }
//
// impl SourcedCpxInfo {
//     pub fn combine_plus(&self, other: &Self) -> SourcedCpxInfo {
//         debug!("{:?} {:?}", self.clone(), other.clone());
//         match (self.clone(), other.clone()) {
//             (Unknown, _) | (_, Unknown) => Unknown,
//             (Equal { source }, a) | (a, Equal { source }) => a.clone(),
//             (
//                 Inclusion {
//                     mn: _,
//                     mx: Some(mxa),
//                 },
//                 Inclusion {
//                     mn: _,
//                     mx: Some(mxb),
//                 },
//             ) => Inclusion {
//                 mn: None,
//                 mx: Some(combine_parallel_max(mxa, mxb)),
//             },
//             (
//                 Exclusion { .. }
//                 | Inclusion {
//                     mn: _,
//                     mx: Option::None,
//                 },
//                 _,
//             )
//             | (
//                 _,
//                 Exclusion { .. }
//                 | Inclusion {
//                     mn: _,
//                     mx: Option::None,
//                 },
//             ) => Unknown,
//         }
//     }
// }
