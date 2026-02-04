use std::fmt;

use crate::data::data::Own;
use crate::data::enums::*;
use crate::data::id::HasId;
use crate::data::preview::{HasPreview, PreviewSource, PreviewSourceKey};
use crate::output::markdown::Markdown;

pub trait ToMarkdown {
    fn to_markdown(&self, builder: &Markdown) -> Option<String>;
}

impl ToMarkdown for Page {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        let mut res = String::new();
        match self {
            Self::Pp(num) => Some(format!("page {}", num)),
            Self::NotApplicable => None,
            Self::Unknown => None,
        }
    }
}

impl ToMarkdown for CpxTime {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        Some(String::from(match self {
            // formed to continue line "upper bounded by ..."
            CpxTime::Constant => "a constant",
            CpxTime::Linear => "a linear function",
            CpxTime::Polynomial => "a polynomial function",
            CpxTime::Exponential => "an exponential function",
            CpxTime::Tower => "a tower function",
            CpxTime::Exists => "a computable function",
        }))
    }
}

impl<T> ToMarkdown for T
where
    T: HasId,
{
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        Some(format!("[[{}]]", &self.id()))
    }
}

impl Own {
    pub fn to_string(&self, truth: bool, plural: bool) -> String {
        String::from(match (self, truth, plural) {
            (Own::Is, true, false) => "is",
            (Own::Is, true, true) => "are",
            (Own::Has, true, false) => "has",
            (Own::Has, true, true) => "have",
            (Own::Is, false, false) => "is not",
            (Own::Is, false, true) => "are not",
            (Own::Has, false, false) => "does not have",
            (Own::Has, false, true) => "do not have",
        })
    }
}

// enum RelDescription {
//     UpperBound {
//         bound: CpxTime,
//     },
//     LowerBound {
//         bound: CpxTime,
//     },
//     BothBounds {
//         bound: CpxTime,
//     },
//     UpperLowerBound {
//         lower_bound: CpxTime,
//         upper_bound: CpxTime,
//     },
//     Equal,
//     IncludedIn(PreviewType, PreviewType),
//     Excludes(PreviewType, PreviewType),
//     Unknown,
// }

// fn relation_description(rel: &PreviewRelation, builder: &Markdown) -> RelDescription {
//     match &rel.cpx {
//         CpxInfo::Inclusion { mn: None, mx: None } => panic!("none none"),
//         CpxInfo::Inclusion {
//             mn: Some(mn),
//             mx: Some(mx),
//         } => {
//             match (&rel.subset.typ, &rel.superset.typ) {
//                 (PreviewType::Parameter, PreviewType::Parameter) => {
//                     if *mx == CpxTime::Constant {
//                         // preventing the trivial mn==mx on constant lb
//                         RelDescription::UpperBound { bound: mx.clone() }
//                     } else if mn == mx {
//                         RelDescription::BothBounds { bound: mx.clone() }
//                     } else if *mn == CpxTime::Constant {
//                         RelDescription::UpperBound { bound: mx.clone() }
//                     } else {
//                         RelDescription::UpperLowerBound {
//                             lower_bound: mn.clone(),
//                             upper_bound: mx.clone(),
//                         }
//                     }
//                 }
//                 (a, b) => RelDescription::IncludedIn(a.clone(), b.clone()),
//             }
//         }
//         CpxInfo::Inclusion {
//             mn: None,
//             mx: Some(mx),
//         } => RelDescription::UpperBound { bound: mx.clone() },
//         CpxInfo::Inclusion {
//             mn: Some(mn),
//             mx: None,
//         } => RelDescription::LowerBound { bound: mn.clone() },
//         CpxInfo::Equal => RelDescription::Equal,
//         CpxInfo::Exclusion => {
//             let (a, b) = (&rel.subset.typ, &rel.superset.typ);
//             RelDescription::Excludes(a.clone(), b.clone())
//         }
//         CpxInfo::Unknown => RelDescription::Unknown,
//     }
// }

// impl PreviewRelation {
//     pub fn long_description(&self, builder: &Markdown) -> Option<String> {
//         let subset_string = self.subset.to_markdown(builder).unwrap();
//         let superset_string = self.superset.to_markdown(builder).unwrap();
//         match relation_description(self, builder) {
//             RelDescription::UpperBound { bound } => Some(format!(
//                 "{} upper bounds {} by {}",
//                 subset_string,
//                 superset_string,
//                 bound.to_markdown(builder).unwrap()
//             )),
//             RelDescription::LowerBound { bound } => Some(format!(
//                 "there exist cases where {} is $k$ but {} is at least {}",
//                 subset_string,
//                 superset_string,
//                 bound.to_markdown(builder).unwrap()
//             )),
//             RelDescription::BothBounds { bound } => Some(format!(
//                 "{} upper and lower bounds {} by {}",
//                 subset_string,
//                 superset_string,
//                 bound.to_markdown(builder).unwrap()
//             )),
//             RelDescription::UpperLowerBound {
//                 lower_bound,
//                 upper_bound,
//             } => Some(format!(
//                 "{} upper bounds {} by {} and lower bounds it by {}",
//                 subset_string,
//                 superset_string,
//                 upper_bound.to_markdown(builder).unwrap(),
//                 lower_bound.to_markdown(builder).unwrap()
//             )),
//             RelDescription::IncludedIn(PreviewType::Parameter, PreviewType::Parameter) => {
//                 panic!("this pair was designed to be processed in a function that calls this")
//             }
//             RelDescription::IncludedIn(from, to) | RelDescription::Excludes(from, to) => {
//                 let is_inclusion = matches!(
//                     relation_description(self, builder),
//                     RelDescription::IncludedIn(..)
//                 );
//                 let (from_str, plural) = match &from {
//                     PreviewType::GraphClass => (format!("graph class {}", subset_string), false),
//                     PreviewType::Parameter => (
//                         format!("graph classes with bounded {}", subset_string),
//                         true,
//                     ),
//                     PreviewType::Property(s) => (
//                         format!(
//                             "graph classes that {} {}",
//                             s.clone().to_string(true, true),
//                             subset_string
//                         ),
//                         true,
//                     ),
//                     PreviewType::ParametricGraphClass => todo!(),
//                 };
//                 let bnd = if let PreviewType::Parameter = from {
//                     "bounded"
//                 } else {
//                     "constant"
//                 };
//                 let (to_str, own) = match to {
//                     PreviewType::GraphClass => {
//                         (format!("contained in {}", superset_string), Own::Has)
//                     }
//                     PreviewType::Parameter => (format!("{} {}", bnd, superset_string), Own::Is),
//                     PreviewType::Property(s) => (superset_string, s),
//                     PreviewType::ParametricGraphClass => todo!(),
//                 };
//                 let join = own.to_string(is_inclusion, plural);
//                 Some(format!("{} {} {}", from_str, join, to_str))
//             }
//             RelDescription::Equal => Some(format!(
//                 "{} is equivalent to {}",
//                 subset_string, superset_string
//             )),
//             RelDescription::Unknown => None,
//         }
//     }
//
//     pub fn short_description(&self, builder: &Markdown) -> String {
//         let subset_string = self.subset.to_markdown(builder).unwrap();
//         let superset_string = self.superset.to_markdown(builder).unwrap();
//         match relation_description(self, builder) {
//             RelDescription::UpperBound { bound } => "upper bound",
//             RelDescription::LowerBound { bound } => "only lower bound",
//             RelDescription::BothBounds { bound } => "tight bounds",
//             RelDescription::UpperLowerBound {
//                 lower_bound,
//                 upper_bound,
//             } => "non-tight bounds",
//             RelDescription::IncludedIn(PreviewType::Parameter, PreviewType::Parameter) => panic!(),
//             RelDescription::IncludedIn(PreviewType::GraphClass, PreviewType::Parameter) => {
//                 "constant"
//             }
//             RelDescription::IncludedIn(PreviewType::Parameter, PreviewType::GraphClass) => {
//                 "inclusion"
//             }
//             RelDescription::IncludedIn(PreviewType::GraphClass, PreviewType::GraphClass) => {
//                 "inclusion"
//             }
//             RelDescription::IncludedIn(PreviewType::Property(_), PreviewType::Property(_)) => {
//                 "implies"
//             }
//             RelDescription::IncludedIn(PreviewType::Property(_), PreviewType::GraphClass) => {
//                 "inclusion"
//             }
//             RelDescription::IncludedIn(PreviewType::Property(_), PreviewType::Parameter) => {
//                 "inclusion"
//             }
//             RelDescription::IncludedIn(PreviewType::GraphClass, PreviewType::Property(_)) => "has",
//             RelDescription::IncludedIn(PreviewType::Parameter, PreviewType::Property(_)) => "has",
//             RelDescription::IncludedIn(PreviewType::ParametricGraphClass, _)
//             | RelDescription::IncludedIn(_, PreviewType::ParametricGraphClass) => todo!(),
//             RelDescription::Excludes(PreviewType::Parameter, PreviewType::Parameter) => "exclusion",
//             RelDescription::Excludes(PreviewType::GraphClass, PreviewType::Parameter) => {
//                 "unbounded"
//             }
//             RelDescription::Excludes(PreviewType::Parameter, PreviewType::GraphClass) => {
//                 "exclusion"
//             }
//             RelDescription::Excludes(PreviewType::GraphClass, PreviewType::GraphClass) => {
//                 "exclusion"
//             }
//             RelDescription::Excludes(PreviewType::Property(_), PreviewType::Property(_)) => {
//                 "avoids"
//             }
//             RelDescription::Excludes(PreviewType::Property(_), PreviewType::GraphClass) => {
//                 "exclusion"
//             }
//             RelDescription::Excludes(PreviewType::Property(_), PreviewType::Parameter) => {
//                 "exclusion"
//             }
//             RelDescription::Excludes(PreviewType::GraphClass, PreviewType::Property(_)) => "avoids",
//             RelDescription::Excludes(PreviewType::Parameter, PreviewType::Property(_)) => "avoids",
//             RelDescription::Excludes(PreviewType::ParametricGraphClass, _)
//             | RelDescription::Excludes(_, PreviewType::ParametricGraphClass) => todo!(),
//             RelDescription::Equal => "equal",
//             RelDescription::Unknown => "unknown to HOPS",
//         }
//         .into()
//     }
// }

// impl ToMarkdown for PreviewSource {
//     fn to_markdown(&self, builder: &Markdown) -> Option<String> {
//         match &self.sourcekey {
//             SourceKey::Bibtex {
//                 key,
//                 name,
//                 entry: _,
//                 score: _,
//             } => Some(name.clone().unwrap_or(key.clone())),
//             SourceKey::Online { url } => Some(url.clone()),
//             SourceKey::Other {
//                 name,
//                 description: _,
//             } => Some(name.clone()),
//         }
//     }
// }

// impl ToMarkdown for ShowedFact {
//     fn to_markdown(&self, builder: &Markdown) -> Option<String> {
//         let mut res = String::new();
//         match self {
//             Self::Relation(status, relation) => {
//                 if let Some(val) = relation.long_description(builder) {
//                     res += &val;
//                 }
//             }
//             Self::Definition(status, preview_set) => {
//                 let set = builder.data.get_set_by_id(preview_set);
//                 if let Some(val) = set.preview().to_markdown(builder) {
//                     res += &val;
//                 }
//             } // Self::Citation(citation) => {
//               // if let Some(val) = citation.to_markdown(&builder) {
//               // res += &val;
//               // }
//               // }
//         }
//         Some(res)
//     }
// }
//
// impl ToMarkdown for Showed {
//     fn to_markdown(&self, builder: &Markdown) -> Option<String> {
//         let mut res = String::new();
//         if let Some(val) = self.page.to_markdown(builder) {
//             res += &format!("{} : ", val);
//         }
//         if let Some(val) = self.fact.to_markdown(builder) {
//             res += &val;
//         }
//         if !res.is_empty() && !self.text.is_empty() {
//             res += " -- ";
//         }
//         res += &self.text;
//         Some(res)
//     }
// }
//
// impl ToMarkdown for SourceSubset {
//     fn to_markdown(&self, builder: &Markdown) -> Option<String> {
//         let mut res = String::new();
//         res += "*";
//         if self.preview.time.year.is_some() {
//             res += &format!(" {}", self.preview.time);
//         };
//         res += &format!(" [[{}]]\n", &self.preview.id.to_string());
//         for showed in &self.showed {
//             res += &format!("    * {}\n", showed.to_markdown(builder).unwrap());
//         }
//         Some(res)
//     }
// }
