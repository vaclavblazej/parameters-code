
use crate::data::data::{Showed, ShowedFact, SourceSubset};
use crate::data::preview::{PreviewRelation, PreviewSet, PreviewSource, PreviewSourceKey};
use crate::general::enums::{CpxInfo, CpxTime, Page, SourceKey};
use crate::input::raw::{RawRelation, RawSet};
use crate::data::preview::PreviewKind;

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

impl ToMarkdown for PreviewRelation {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        let subset_string = self.subset.to_markdown(builder).unwrap();
        let superset_string = self.superset.to_markdown(builder).unwrap();
        match &self.cpx {
            CpxInfo::Inclusion { mn, mx } => {
                let lb = mn.to_markdown(builder).unwrap();
                let ub = mx.to_markdown(builder).unwrap();
                match (&self.subset.kind, &self.superset.kind) {
                    (PreviewKind::Parameter, PreviewKind::Parameter) => {
                        if *mx == CpxTime::Constant { // preventing the trivial mn==mx on constant lb
                            Some(format!("{} upper bounds {} by {}", subset_string, superset_string, ub))
                        } else if mn == mx {
                            Some(format!("{} upper and lower bounds {} by {}", subset_string, superset_string, ub))
                        } else if *mn == CpxTime::Constant {
                            Some(format!("{} upper bounds {} by {}", subset_string, superset_string, ub))
                        } else {
                            Some(format!("{} upper bounds {} by {} and lower bounds it by {}", subset_string, superset_string, ub, lb))
                        }
                    },
                    (PreviewKind::GraphClass, PreviewKind::Parameter) => {
                        assert!(mx == &CpxTime::Constant);
                        Some(format!("graph class {} has constant {}", subset_string, superset_string))
                    },
                    (PreviewKind::Parameter, PreviewKind::GraphClass) => {
                        Some(format!("graphs with bounded {} are included in graph class {}", subset_string, superset_string))
                    },
                    (PreviewKind::GraphClass, PreviewKind::GraphClass) => {
                        Some(format!("graph class {} is included in graph class {}", subset_string, superset_string))
                    },
                }
            },
            CpxInfo::LowerBound { mn } => Some(format!("there exist cases where {} is $k$ but {} is at least {}", subset_string, superset_string, mn.to_markdown(builder).unwrap())),
            CpxInfo::Equivalence => Some(format!("{} is equal to {}", subset_string, superset_string)),
            CpxInfo::Exclusion => {
                match (&self.subset.kind, &self.superset.kind) {
                    (PreviewKind::Parameter, PreviewKind::Parameter) => {
                        Some(format!("bounded {} does not imply bounded {}", subset_string, superset_string))
                    },
                    (PreviewKind::GraphClass, PreviewKind::Parameter) => {
                        Some(format!("graph class {} has unbounded {}", subset_string, superset_string))
                    },
                    (PreviewKind::Parameter, PreviewKind::GraphClass) => {
                        Some(format!("graphs with bounded {} are not included in graph class {}", subset_string, superset_string))
                    },
                    (PreviewKind::GraphClass, PreviewKind::GraphClass) => {
                        Some(format!("graph class {} is not included in graph class {}", subset_string, superset_string))
                    },
                }
            },
            CpxInfo::Unknown => None,
        }
    }
}

impl ToMarkdown for PreviewSource {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        match &self.sourcekey {
            SourceKey::Bibtex { key, entry: _ } => Some(key.clone()),
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
                if let Some(val) = relation.to_markdown(&builder) {
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
        res += &format!("* {} [[{}]]\n", self.time.to_string(), &self.id);
        for showed in &self.showed {
            res += &format!("    * {}\n", showed.to_markdown(&builder).unwrap());
        }
        Some(res)
    }
}

