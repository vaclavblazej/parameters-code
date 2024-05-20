use crate::general::enums::{Cpx, Page};

use super::raw::{RawData, RawRelation, RawSet, RawShowed, RawShowedFact, RawSource};

pub struct RawDataSource<'a> {
    source: RawSource,
    data: &'a mut RawData,
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
                cpx: crate::general::enums::CpxInfo::Inclusion { mn: crate::general::enums::CpxTime::Constant, mx: crate::general::enums::CpxTime::Constant },
            }),
            page: Page::NotApplicable,
        };
        let exclusion = RawShowed {
            id: format!("{}_ex", id),
            text: "".into(),
            fact: RawShowedFact::Relation(RawRelation {
                subset: superset.clone(),
                superset: subset.clone(),
                cpx: crate::general::enums::CpxInfo::Exclusion,
            }),
            page: Page::NotApplicable,
        };
        self.data.factoids.push((self.source.clone(), inclusion));
        self.data.factoids.push((self.source.clone(), exclusion));
        self
    }

    pub fn showed(self, id: &str, page: Page, subset: &RawSet, superset: &RawSet, cpx: Cpx, text: &str) -> Self {
        let relation = RawRelation {
            subset: subset.clone(),
            superset: superset.clone(),
            cpx: cpx.expand(),
        };
        let showed = RawShowed {
            id: id.into(),
            text: text.into(),
            fact: RawShowedFact::Relation(relation),
            page,
        };
        self.data.factoids.push((self.source.clone(), showed));
        self
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

    pub fn todo(self) -> RawSource {
        self.source
    }

    pub fn done(self) -> RawSource {
        self.source
    }

}
