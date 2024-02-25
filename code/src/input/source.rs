use crate::general::enums::{Cpx, Page};

use super::raw::{RawData, RawRelation, RawSet, RawSource};



pub struct DataSource<'a> {
    source: RawSource,
    data: &'a mut RawData,
}

impl<'a> DataSource<'a> {

    pub fn new(source: &RawSource, data: &'a mut RawData) -> Self {
        DataSource { source: source.clone(), data }
    }

    pub fn defined(self, id: &str, page: Page, set: &RawSet, text: &str) -> Self {
        let showed = Showed {
            id: id.into(),
            text: text.into(),
            fact: ShowedFact::Definition(set.clone()),
            page,
        };
        self.data.factoids.push((self.source.clone(), showed));
        self
    }

    pub fn showed(self, id: &str, page: Page, subset: &RawSet, superset: &RawSet, cpx: Cpx, text: &str) -> Self {
        let relation = RawRelation {
            subset: subset.clone(),
            superset: superset.clone(),
            cpx: cpx.expand(),
        };
        let showed = Showed {
            id: id.into(),
            text: text.into(),
            fact: ShowedFact::Relation(relation),
            page,
        };
        self.data.factoids.push((self.source.clone(), showed));
        self
    }

    pub fn cited(self, id: &str, page: Page, who: RawSource, text: &str) -> Self {
        let showed = Showed {
            id: id.into(),
            text: text.into(),
            fact: ShowedFact::Citation(who),
            page,
        };
        self.data.factoids.push((self.source.clone(), showed));
        self
    }

    pub fn done(self) -> RawSource {
        self.source
    }

}

#[derive(Debug, Clone)]
pub struct Showed {
    pub id: String,
    pub text: String,
    pub fact: ShowedFact,
    pub page: Page,
}

#[derive(Debug, Clone)]
pub enum ShowedFact {
    Relation(RawRelation),
    Definition(RawSet),
    Citation(RawSource),
}
