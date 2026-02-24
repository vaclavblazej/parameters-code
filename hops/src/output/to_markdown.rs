use std::fmt;

use crate::data::data::{Definition, Fact, LogicFragment, NameCore, Own, Parameter, Relation, Wrote, WroteStatus};
use crate::data::enums::*;
use crate::data::id::{BaseId, HasId, HasPreviewId, Id, PreviewId, PreviewShowedId};
use crate::data::preview::{HasPreview, PreviewGraph, PreviewGraphClass, PreviewGraphClassProperty, PreviewLogicFragment, PreviewOperation, PreviewParameter, PreviewParametricGraphClass, PreviewParametricParameter, PreviewProblem, PreviewSource, PreviewSourceKey, PreviewTag};
use crate::input::source::{ClassicalSolvability, Cpx, EquivalenceRelation, ImplicationRelation, InclusionRelationUnderGraphRelation, ParameterizedSolvability};

pub trait MaybeToMarkdown {
    fn to_markdown(&self) -> Option<String>;
}

pub trait ToMarkdown {
    fn to_markdown(&self) -> String;
}

impl MaybeToMarkdown for Page {
    fn to_markdown(&self) -> Option<String> {
        let mut res = String::new();
        match self {
            Self::Pp(num) => Some(format!("page {}", num)),
            Self::NotApplicable => None,
            Self::PageTodo => None,
        }
    }
}

impl ToMarkdown for CpxTime {
    fn to_markdown(&self) -> String {
        String::from(match self {
            // formed to continue line "upper bounded by ..."
            CpxTime::Constant => "a constant",
            CpxTime::Linear => "a linear function",
            CpxTime::Polynomial => "a polynomial function",
            CpxTime::Exponential => "an exponential function",
            CpxTime::Tower => "a tower function",
            CpxTime::Exists => "a computable function",
        })
    }
}

impl ToMarkdown for NameCore {
    fn to_markdown(&self) -> String {
        let mut res = String::new();
        res += &format!("---\ntitle: \"{}\"\n---", self.name);
        res += &format!("# {}\n\n", self.name);
        if let Some(abbr) = &self.abbr {
            res += &format!("abbr: {}\n\n", abbr);
        }
        if !self.aka.is_empty() {
            res += &format!("aka: {}\n\n", self.aka.join(", "));
        }
        res
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

// fn relation_description(rel: &PreviewRelation) -> RelDescription {
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
//     pub fn long_description(&self) -> Option<String> {
//         let subset_string = self.subset.to_markdown().unwrap();
//         let superset_string = self.superset.to_markdown().unwrap();
//         match relation_description(self) {
//             RelDescription::UpperBound { bound } => Some(format!(
//                 "{} upper bounds {} by {}",
//                 subset_string,
//                 superset_string,
//                 bound.to_markdown().unwrap()
//             )),
//             RelDescription::LowerBound { bound } => Some(format!(
//                 "there exist cases where {} is $k$ but {} is at least {}",
//                 subset_string,
//                 superset_string,
//                 bound.to_markdown().unwrap()
//             )),
//             RelDescription::BothBounds { bound } => Some(format!(
//                 "{} upper and lower bounds {} by {}",
//                 subset_string,
//                 superset_string,
//                 bound.to_markdown().unwrap()
//             )),
//             RelDescription::UpperLowerBound {
//                 lower_bound,
//                 upper_bound,
//             } => Some(format!(
//                 "{} upper bounds {} by {} and lower bounds it by {}",
//                 subset_string,
//                 superset_string,
//                 upper_bound.to_markdown().unwrap(),
//                 lower_bound.to_markdown().unwrap()
//             )),
//             RelDescription::IncludedIn(PreviewType::Parameter, PreviewType::Parameter) => {
//                 panic!("this pair was designed to be processed in a function that calls this")
//             }
//             RelDescription::IncludedIn(from, to) | RelDescription::Excludes(from, to) => {
//                 let is_inclusion = matches!(
//                     relation_description(self),
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
//     pub fn short_description(&self) -> String {
//         let subset_string = self.subset.to_markdown().unwrap();
//         let superset_string = self.superset.to_markdown().unwrap();
//         match relation_description(self) {
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

impl ToMarkdown for PreviewSource {
    fn to_markdown(&self) -> String {
        let res = match &self.sourcekey {
            SourceKey::Bibtex { entry_key, name, entry_content } => {
                name.clone().unwrap_or(entry_key.clone())
            },
            SourceKey::Online { url } => url.clone(),
            SourceKey::Other { name, description } => name.clone(),
        };
        format!("* {}\n", res)
    }
}

impl ToMarkdown for Wrote {
    fn to_markdown(&self) -> String {
        let page_str: String = match self.page.to_markdown() {
            Some(page) => format!("{} : ", page),
            None => "".into(),
        };
        let mut res = String::new();
        let text = if self.text.is_empty() {
            "empty text".into()
        } else {
            self.text.clone()
        };
        res += &format!("  * {}{}\n", &page_str, &text);
        for fact in &self.facts {
            res += &format!("    * {}\n", &fact.to_markdown());
        }
        res
    }
}

impl ToMarkdown for Definition {
    fn to_markdown(&self) -> String {
        let mut res = String::new();
        res += "definition of ";
        res += &match self {
            Definition::LogicFragment(p) => p.to_markdown().clone(),
            Definition::Parameter(p) => p.to_markdown().clone(),
            Definition::Graph(p) => p.to_markdown().clone(),
            Definition::GraphClass(p) => p.to_markdown().clone(),
            Definition::Operation(p) => p.to_markdown().clone(),
            Definition::Problem(p) => p.to_markdown().clone(),
            Definition::ParParameter(p) => p.to_markdown().clone(),
            Definition::ParGraphClass(p) => p.to_markdown().clone(),
            Definition::Property(p) => p.to_markdown().clone(),
        };
        res.to_string()
    }
}

impl<T> ToMarkdown for Id<T> {
    fn to_markdown(&self) -> String {
        format!("[[{}]]", self)
    }
}

impl<T> ToMarkdown for PreviewId<T> {
    fn to_markdown(&self) -> String {
        format!("[[{}]]", self)
    }
}

impl ToMarkdown for PreviewParameter {
    fn to_markdown(&self) -> String {
        self.previewid().to_markdown()
    }
}

impl ToMarkdown for PreviewLogicFragment {
    fn to_markdown(&self) -> String {
        self.previewid().to_markdown()
    }
}

impl ToMarkdown for PreviewGraphClass {
    fn to_markdown(&self) -> String {
        self.previewid().to_markdown()
    }
}

impl ToMarkdown for PreviewProblem {
    fn to_markdown(&self) -> String {
        self.previewid().to_markdown()
    }
}

impl ToMarkdown for PreviewParametricParameter {
    fn to_markdown(&self) -> String {
        self.previewid().to_markdown()
    }
}

impl ToMarkdown for PreviewParametricGraphClass {
    fn to_markdown(&self) -> String {
        self.previewid().to_markdown()
    }
}

impl ToMarkdown for PreviewGraphClassProperty {
    fn to_markdown(&self) -> String {
        self.previewid().to_markdown()
    }
}

impl ToMarkdown for PreviewOperation {
    fn to_markdown(&self) -> String {
        self.previewid().to_markdown()
    }
}

impl ToMarkdown for PreviewGraph {
    fn to_markdown(&self) -> String {
        self.previewid().to_markdown()
    }
}

impl ToMarkdown for (String, String, ImplicationRelation) {
    fn to_markdown(&self) -> String {
        let (a, b, r) = self.clone();
        match r {
            ImplicationRelation::Equivalent => format!("{} is equivalent to {}", a, b),
            ImplicationRelation::Implies => format!("{} implies {}", a, b),
            ImplicationRelation::Excludes => format!("{} excludes {}", a, b),
            ImplicationRelation::Unknown => format!("unknown relation of {} to {}", a, b),
        }
    }
}

impl ToMarkdown for (String, String, InclusionRelationUnderGraphRelation) {
    fn to_markdown(&self) -> String {
        let (a, b, r) = self.clone();
        let res = match r.relation {
            ImplicationRelation::Equivalent => format!("{} is equivalent to {}", a, b),
            ImplicationRelation::Implies => format!("{} implies {}", a, b),
            ImplicationRelation::Excludes => format!("{} excludes {}", a, b),
            ImplicationRelation::Unknown => format!("unknown relation of {} to {}", a, b),
        };
        format!("{} under {}", res, r.graph_relation)
    }
}

impl ToMarkdown for (String, String, Cpx) {
    fn to_markdown(&self) -> String {
        let (a, b, r) = self.clone();
        match r {
            Cpx::Bounds(lb, ub) => format!("{} bounds {} between {} and {}", a, b, lb.to_markdown(), ub.to_markdown()),
            Cpx::UpperBound(ub) => format!("{} upper bounds {} by {}", a, b, ub.to_markdown()),
            Cpx::LowerBound(lb) => format!("if {} upper bounds {} then the function is at least {}", a, b, lb.to_markdown()),
            Cpx::StrictUpperBound(ub) => format!("{} upper bounds {} by {}", a, b, ub.to_markdown()),
            Cpx::Exactly(bound) => format!("{} upper bounds {} by {} (tight)", a, b, bound.to_markdown()),
            Cpx::Equivalent(abub, baub) => format!("{} upper bounds {} by {} and {} upper bounds {} by {}", a, b, abub.to_markdown(), b, a, baub.to_markdown()),
            Cpx::Equal => format!("{} equals {}", a, b),
            Cpx::Exclusion => format!("{} excludes {}", a, b),
            Cpx::Incomparable => format!("{} is incomparable to {}", a, b),
            Cpx::Todo => panic!("Cpx::Todo at to_markdown is not allowed"),
        }
    }
}

impl ToMarkdown for (String, String, EquivalenceRelation) {
    fn to_markdown(&self) -> String {
        let (a, b, r) = self.clone();
        match r {
            EquivalenceRelation::Equivalent => format!("{} is equivalent to {}", a, b),
        }
    }
}

impl ToMarkdown for (String, String, ClassicalSolvability) {
    fn to_markdown(&self) -> String {
        let (a, b, r) = self.clone();
        match r {
            ClassicalSolvability::Polynomial => format!("{} is polynomial for {}", a, b),
            ClassicalSolvability::NpHard => format!("{} is NP-hard for {}", a, b),
        }
    }
}

impl ToMarkdown for (String, String, ParameterizedSolvability) {
    fn to_markdown(&self) -> String {
        let (a, b, r) = self.clone();
        match r {
            ParameterizedSolvability::Polynomial => format!("{} is polynomial-time solvable for {}", a, b),
            ParameterizedSolvability::Fpt => format!("{} is FPT for {}", a, b),
            ParameterizedSolvability::Whard => format!("{} is W[1]-hard for {}", a, b),
            ParameterizedSolvability::ParaNpHard => format!("{} is para-NP-hard for {}", a, b),
        }
    }
}

impl ToMarkdown for Relation {
    fn to_markdown(&self) -> String {
        match self.clone() {
            Relation::LfLf(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::OpOp(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::GrGr(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::GcGc(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::GrGc(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::PgcPgc(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::ParPar(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::PropProp(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::GcProp(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::GcPar(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::ParProp(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::ProbProb(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::ProbProp(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::ProbPar(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
            Relation::PropPar(f, t, d) => (f.to_markdown(), t.to_markdown(), d).to_markdown(),
        }
    }
}

impl ToMarkdown for (PreviewShowedId, WroteStatus, Fact) {
    fn to_markdown(&self) -> String {
        let (showedid, status, fact) = self;
        match fact {
            Fact::Definition(definition) => definition.to_markdown(),
            Fact::Relation(relation) => relation.to_markdown(),
        }
    }
}
