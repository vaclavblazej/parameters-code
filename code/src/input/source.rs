use crate::general::enums::{Cpx, Page};

use super::raw::{RawData, RawRelation, RawSet, RawShowed, RawShowedFact, RawSource};
use crate::general::enums::CpxInfo::*;
use crate::general::enums::CpxTime;

pub struct RawDataSource<'a> {
    source: RawSource,
    data: &'a mut RawData,
}

pub struct CollectiveSource<'a> {
    raw: RawDataSource<'a>,
    page: Page,
    text: &'a str,
}

impl<'a> CollectiveSource<'a> {

    pub fn showed(mut self, id: &str, subset: &RawSet, superset: &RawSet, cpx: Cpx, ) -> Self {
        self.raw = self.raw.showed(id, self.page.clone(), subset, superset, cpx, self.text);
        self
    }

    pub fn done(self) -> RawDataSource<'a> {
        self.raw
    }

}

impl<'a> RawDataSource<'a> {

    pub fn new(source: &RawSource, data: &'a mut RawData) -> Self {
        RawDataSource { source: source.clone(), data }
    }

    pub fn defined(self, id: &str, page: Page, set: &RawSet, text: &str) -> Self {
        let showed = RawShowed {
            id: id.into(),
            text: text.into(),
            fact: RawShowedFact::Definition(set.clone()),
            page,
        };
        self.data.factoids.push((self.source.clone(), showed));
        self
    }

    pub fn proper_graph_inclusion(self, id: &str, subset: &RawSet, superset: &RawSet) -> Self {
        let inclusion = RawShowed {
            id: format!("{}_in", id),
            text: "".into(),
            fact: RawShowedFact::Relation(RawRelation {
                subset: subset.clone(),
                superset: superset.clone(),
                cpx: Inclusion { mn: CpxTime::Constant, mx: CpxTime::Constant },
            }),
            page: Page::NotApplicable,
        };
        let exclusion = RawShowed {
            id: format!("{}_ex", id),
            text: "".into(),
            fact: RawShowedFact::Relation(RawRelation {
                subset: superset.clone(),
                superset: subset.clone(),
                cpx: Exclusion,
            }),
            page: Page::NotApplicable,
        };
        self.data.factoids.push((self.source.clone(), inclusion));
        self.data.factoids.push((self.source.clone(), exclusion));
        self
    }

    pub fn showed(self, id: &str, page: Page, subset: &RawSet, superset: &RawSet, cpx: Cpx, text: &str) -> Self {
        let relations = match cpx {
            Cpx::Bounds(a, b) => vec![RawRelation::new(subset, superset, Inclusion{mn: a.clone(), mx: b.clone()})],
            Cpx::Exactly(a) => vec![RawRelation::new(subset, superset, Inclusion{mn: a.clone(), mx: a.clone()})],
            Cpx::UpperBound(b) => vec![RawRelation::new(subset, superset, Inclusion{mn: CpxTime::Constant, mx: b.clone()})],
            Cpx::Todo => vec![RawRelation::new(subset, superset, Inclusion { mn: CpxTime::Constant, mx: CpxTime::Exists })],
            Cpx::Equivalence => vec![RawRelation::new(subset, superset, Equivalence)],
            Cpx::Exclusion => vec![RawRelation::new(subset, superset, Exclusion)],
            Cpx::Incomparable => vec![
                RawRelation::new(subset, superset, Exclusion),
                RawRelation::new(superset, subset, Exclusion),
            ],
            Cpx::StrictUpperBound(a) => vec![
                RawRelation::new(subset, superset, Inclusion{mn: CpxTime::Constant, mx: a.clone()}),
                RawRelation::new(superset, subset, Exclusion),
            ],
        };

        for relation in relations {
            let showed = RawShowed {
                id: id.into(),
                text: text.into(),
                fact: RawShowedFact::Relation(relation),
                page: page.clone(),
            };
            self.data.factoids.push((self.source.clone(), showed));
        }
        self
    }

    pub fn collective(self, page: Page, text: &'a str) -> CollectiveSource<'a> {
        CollectiveSource { raw: self, page, text }
    }

    pub fn cited(self, id: &str, page: Page, who: RawSource, text: &str) -> Self {
        let showed = RawShowed {
            id: id.into(),
            text: text.into(),
            fact: RawShowedFact::Citation(who),
            page,
        };
        self.data.factoids.push((self.source.clone(), showed));
        self
    }

    pub fn hasse(self, id: &str, page: Page, sets: &Vec<&str>) -> Self {
        // todo
        self
    }

    pub fn table(self, id: &str, page: Page, sets: &Vec<&str>) -> Self {
        // todo
        self
    }

    pub fn todo(self) -> RawSource {
        self.source
    }

    pub fn done(self) -> RawSource {
        self.source
    }

}
