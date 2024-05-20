use crate::data::data::{Showed, ShowedFact, SourceSubset};
use crate::data::preview::{PreviewRelation, PreviewSet, PreviewSource, PreviewSourceKey};
use crate::general::enums::{CpxInfo, CpxTime, Page, SourceKey};
use crate::input::raw::{RawRelation, RawSet};

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
            CpxTime::Constant => "$\\mathcal O(1)$".into(),
            CpxTime::Linear => "$\\mathcal O(k)$".into(),
            CpxTime::Polynomial => "$k^{\\mathcal O(1)}$".into(),
            CpxTime::Exponential => "$2^{\\mathcal O(k)}$".into(),
            CpxTime::Tower(x) => "$k^k^k^k$".into(),
            CpxTime::Exists => "$f(k)$".into(),
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
                if mn == mx {
                    Some(format!("{} equal to $k$ implies that {} is equal to {}", subset_string, superset_string, ub)) // todo theta would be better
                } else if *mn == CpxTime::Constant {
                    Some(format!("{} equal to $k$ implies that {} is upper bounded by {}", subset_string, superset_string, ub))
                } else {
                    Some(format!("{} equal to $k$ implies that {} is lower bounded by {} and upper bounded by {}", subset_string, superset_string, lb, ub))
                }
            },
            CpxInfo::LowerBound { mn } => Some(format!("there exist cases where {} is bounded by $k$ but {} is at least {}", subset_string, superset_string, mn.to_markdown(builder).unwrap())),
            CpxInfo::Equivalence => Some(format!("{} is equivalent to {}", subset_string, superset_string)),
            CpxInfo::Exclusion => Some(format!("bounded {} does not imply bounded {}", subset_string, superset_string)),
            CpxInfo::Unknown => None,
        }
    }
}

impl ToMarkdown for PreviewSource {
    fn to_markdown(&self, builder: &Markdown) -> Option<String> {
        match &self.sourcekey {
            PreviewSourceKey::Bibtex { key } => Some(key.clone()),
            PreviewSourceKey::Online { url } => Some(url.clone()),
            PreviewSourceKey::Unknown => None,
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

