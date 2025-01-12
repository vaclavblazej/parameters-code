
use crate::data::data::{Showed, ShowedFact, SourceSubset};
use crate::data::preview::{PreviewRelation, PreviewSet, PreviewSource, PreviewSourceKey};
use crate::general::enums::{CpxInfo, CpxTime, Page, SourceKey};
use crate::data::preview::PreviewType;

use super::markdown::Markdown;


pub trait ToMarkdown {
    fn to_markdown(&self, builder: &Markdown) -> Option<String>;
}

impl ToMarkdown for Page {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        let mut res = String::new();
        match self {
            Self::Pp(num) => Some(format!("page {}", num.to_string())),
            Self::NotApplicable => None,
            Self::Unknown => None,
        }
    }
}

impl ToMarkdown for CpxTime {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        Some(match self {
            // formed to continue line "upper bounded by ..."
            CpxTime::Constant => "a constant".into(),
            CpxTime::Linear => "a linear function".into(),
            CpxTime::Polynomial => "a polynomial function".into(),
            CpxTime::Exponential => "an exponential function".into(),
            CpxTime::Tower => "a tower function".into(),
            CpxTime::Exists => "a computable function".into(),
        })
    }
}

impl ToMarkdown for PreviewSet {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        Some(format!("[[{}]]", &self.id))
    }
}

enum RelDescription {
    UpperBound{bound: CpxTime},
    LowerBound{bound: CpxTime},
    BothBounds{bound: CpxTime},
    UpperLowerBound{lower_bound: CpxTime, upper_bound: CpxTime},
    Equal,
    GraphBoundedPar,
    GraphUnboundedPar,
    BoundedParGraph,
    ParExcludesGraph,
    GraphInclusion,
    GraphExclusion,
    ParExclusion,
    Unknown,
}

fn relation_description(rel: &PreviewRelation, builder: &Markdown) -> RelDescription {
    match &rel.cpx {
        CpxInfo::Inclusion { mn, mx } => {
            match (&rel.subset.typ, &rel.superset.typ) {
                (PreviewType::Parameter, PreviewType::Parameter) => {
                    if *mx == CpxTime::Constant { // preventing the trivial mn==mx on constant lb
                        RelDescription::UpperBound{bound: mx.clone()}
                    } else if mn == mx {
                        RelDescription::BothBounds{bound: mx.clone()}
                    } else if *mn == CpxTime::Constant {
                        RelDescription::UpperBound { bound: mx.clone() }
                    } else {
                        RelDescription::UpperLowerBound { lower_bound: mn.clone(), upper_bound: mx.clone() }
                    }
                },
                (PreviewType::GraphClass, PreviewType::Parameter) => {
                    // println!("{:?} {:?} {:?}\n{:?}\n", self.subset.name, self.superset.name, mx, self);
                    // assert!(mx == &CpxTime::Constant); // todo investigate and fix this
                    RelDescription::GraphBoundedPar
                },
                (PreviewType::Parameter, PreviewType::GraphClass) => {
                    RelDescription::BoundedParGraph
                },
                (PreviewType::GraphClass, PreviewType::GraphClass) => {
                    RelDescription::GraphInclusion
                },
            }
        },
        CpxInfo::LowerBound { mn } => RelDescription::LowerBound { bound: mn.clone() },
        CpxInfo::Equal => RelDescription::Equal,
        CpxInfo::Exclusion => {
            match (&rel.subset.typ, &rel.superset.typ) {
                (PreviewType::Parameter, PreviewType::Parameter) => {
                    RelDescription::ParExclusion
                },
                (PreviewType::GraphClass, PreviewType::Parameter) => {
                    RelDescription::GraphUnboundedPar
                },
                (PreviewType::Parameter, PreviewType::GraphClass) => {
                    RelDescription::ParExcludesGraph
                },
                (PreviewType::GraphClass, PreviewType::GraphClass) => {
                    RelDescription::GraphExclusion
                },
            }
        },
        CpxInfo::Unknown => RelDescription::Unknown,
    }
}

impl PreviewRelation {

    pub fn long_description(&self, builder: &Markdown) -> Option<String> {
        let subset_string = self.subset.to_markdown(builder).unwrap();
        let superset_string = self.superset.to_markdown(builder).unwrap();
        match relation_description(&self, &builder) {
            RelDescription::UpperBound { bound } => Some(format!("{} upper bounds {} by {}", subset_string, superset_string, bound.to_markdown(builder).unwrap())),
            RelDescription::LowerBound { bound } => Some(format!("there exist cases where {} is $k$ but {} is at least {}", subset_string, superset_string, bound.to_markdown(builder).unwrap())),
            RelDescription::BothBounds { bound } => Some(format!("{} upper and lower bounds {} by {}", subset_string, superset_string, bound.to_markdown(builder).unwrap())),
            RelDescription::UpperLowerBound { lower_bound, upper_bound } => Some(format!("{} upper bounds {} by {} and lower bounds it by {}", subset_string, superset_string, upper_bound.to_markdown(builder).unwrap(), lower_bound.to_markdown(builder).unwrap())),
            RelDescription::GraphBoundedPar => Some(format!("graph class {} has constant {}", subset_string, superset_string)),
            RelDescription::BoundedParGraph => Some(format!("graphs with bounded {} are included in graph class {}", subset_string, superset_string)),
            RelDescription::GraphInclusion => Some(format!("graph class {} is included in graph class {}", subset_string, superset_string)),
            RelDescription::Equal => Some(format!("{} is equivalent to {}", subset_string, superset_string)),
            RelDescription::Unknown => None,
            RelDescription::ParExclusion => Some(format!("bounded {} does not imply bounded {}", subset_string, superset_string)),
            RelDescription::GraphUnboundedPar => Some(format!("graph class {} has unbounded {}", subset_string, superset_string)),
            RelDescription::ParExcludesGraph => Some(format!("graphs with bounded {} are not included in graph class {}", subset_string, superset_string)),
            RelDescription::GraphExclusion => Some(format!("graph class {} is not included in graph class {}", subset_string, superset_string)),
        }
    }

    pub fn short_description(&self, builder: &Markdown) -> Option<String> {
        let subset_string = self.subset.to_markdown(builder).unwrap();
        let superset_string = self.superset.to_markdown(builder).unwrap();
        match relation_description(&self, &builder) {
            RelDescription::UpperBound { bound } => Some(format!("upper bound")),
            RelDescription::LowerBound { bound } => Some(format!("only lower bound")),
            RelDescription::BothBounds { bound } => Some(format!("tight bounds")),
            RelDescription::UpperLowerBound { lower_bound, upper_bound } => Some(format!("non-tight bounds")),
            RelDescription::GraphBoundedPar => Some(format!("constant")),
            RelDescription::BoundedParGraph => Some(format!("inclusion")),
            RelDescription::GraphInclusion => Some(format!("inclusion")),
            RelDescription::Equal => Some(format!("equal")),
            RelDescription::ParExclusion => Some(format!("exclusion")),
            RelDescription::GraphUnboundedPar => Some(format!("unbounded")),
            RelDescription::ParExcludesGraph => Some(format!("exclusion")),
            RelDescription::GraphExclusion => Some(format!("exclusion")),
            RelDescription::Unknown => None,
        }
    }

}

impl ToMarkdown for PreviewSource {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        match &self.sourcekey {
            SourceKey::Bibtex { key, name, entry: _ } => {
                Some(name.clone().unwrap_or(key.clone()))
            },
            SourceKey::Online { url } => Some(url.clone()),
            SourceKey::Other { name, description: _ } => Some(name.clone()),
        }
    }
}

impl ToMarkdown for ShowedFact {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        let mut res = String::new();
        match self {
            Self::Relation(relation) => {
                if let Some(val) = relation.long_description(&builder) {
                    res += &val;
                }
            },
            Self::Definition(definition) => {
                if let Some(val) = definition.to_markdown(&builder) {
                    res += &val;
                }
            },
            Self::Citation(citation) => {
                if let Some(val) = citation.to_markdown(&builder) {
                    res += &val;
                }
            },
        }
        Some(res)
    }
}

impl ToMarkdown for Showed {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        let mut res = String::new();
        if let Some(val) = self.page.to_markdown(&builder) {
            res += &format!("{} : ", val);
        }
        if let Some(val) = self.fact.to_markdown(&builder) {
            res += &val;
        }
        if !res.is_empty() && !self.text.is_empty() {
            res += " -- ";
        }
        res += &self.text;
        Some(res)
    }
}

impl ToMarkdown for SourceSubset {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        let mut res = String::new();
        res += "*";
        if self.time.year != None {
            res += &format!(" {}", self.time.to_string());
        };
        res += &format!(" [[{}]]\n", &self.id);
        for showed in &self.showed {
            res += &format!("    * {}\n", showed.to_markdown(&builder).unwrap());
        }
        Some(res)
    }
}

