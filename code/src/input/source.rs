use log::warn;

use super::{build::Builder, raw::{
    BuiltRawSource, RawData, RawProvider, RawProviderLink, RawRelation, RawShowed, RawShowedFact, RawSource
}};
use crate::{
    data::id::{BaseId, PreviewRelationId, PreviewSetId, PreviewSourceId, RelationId, ShowedId},
    general::enums::{Cpx, CpxInfo::{self, *}, CpxTime, Page, RawDrawing},
};

pub struct RawDataSource {
    source: BuiltRawSource,
    factoids: Vec<RawShowed>,
    relations: Vec<RawRelation>,
}

pub struct CollectiveSource {
    raw: RawDataSource,
    page: Page,
    text: String,
}

pub struct RawDataProvider {
    provider: RawProvider,
    links: Vec<RawProviderLink>,
    format_url: Box<dyn Fn(&str) -> String>,
}

impl RawDataProvider {
    pub fn new(
        provider: RawProvider,
        format_url: Box<dyn Fn(&str) -> String>,
    ) -> Self {
        RawDataProvider {
            provider,
            links: Vec::new(),
            format_url,
        }
    }

    pub fn link(mut self, set: &PreviewSetId, id: &str) -> Self {
        let provider_id = self.provider.id.preview();
        let provider_link = RawProviderLink {
            provider: provider_id,
            set: set.clone(),
            url: (self.format_url)(id),
        };
        self.links.push(provider_link);
        self
    }

    pub fn done(self, builder: &mut Builder) {
        let RawDataProvider {
            provider,
            mut links,
            format_url: _,
        } = self;
        builder.data.providers.push(provider);
        builder.data.provider_links.append(&mut links);
    }
}

impl CollectiveSource {

    pub fn showed(
        mut self,
        id: &str,
        subset: &PreviewSetId,
        superset: &PreviewSetId,
        cpx: Cpx,
    ) -> Self {
        self.raw.ref_showed(id, self.page.clone(), subset, superset, cpx, self.text.as_str());
        self
    }

    pub fn done(self) -> RawDataSource {
        self.raw
    }

}

impl RawDataSource {

    pub fn new(source: BuiltRawSource) -> Self {
        RawDataSource {
            source,
            factoids: Vec::new(),
            relations: Vec::new(),
        }
    }

    pub fn defined(mut self, id: &str, page: Page, set: &PreviewSetId, text: &str) -> Self {
        self.ref_defined(id, page, set, text);
        self
    }

    fn ref_defined(&mut self, id: &str, page: Page, set: &PreviewSetId, text: &str) -> &mut Self {
        let showed = RawShowed {
            id: ShowedId::new(id.into()),
            text: text.into(),
            fact: RawShowedFact::Definition(set.clone()),
            page,
        };
        self.factoids.push(showed);
        self
    }

    fn relation(&mut self, subset: &PreviewSetId, superset: &PreviewSetId, cpx: CpxInfo) -> PreviewRelationId {
        let relation = RawRelation::new(subset, superset, cpx);
        let res = relation.id.preview();
        self.relations.push(relation);
        res
    }

    pub fn proper_graph_inclusion(
        &mut self,
        id: &str,
        subset: &PreviewSetId,
        superset: &PreviewSetId,
    ) -> &mut Self {
        let inclusion = RawShowed {
            id: ShowedId::new(id.into()),
            text: "".into(),
            fact: RawShowedFact::Relation(self.relation(
                    subset,
                    superset,
                    Inclusion {
                        mn: Some(CpxTime::Constant),
                        mx: Some(CpxTime::Constant),
                    }
            )),
            page: Page::NotApplicable,
        };
        let exclusion = RawShowed {
            id: ShowedId::new(id.into()),
            text: "".into(),
            fact: RawShowedFact::Relation(self.relation(
                superset,
                subset,
                Exclusion,
            )),
            page: Page::NotApplicable,
        };
        self.factoids.push(inclusion);
        self.factoids.push(exclusion);
        self
    }

    pub fn showed(
        mut self,
        id: &str,
        page: Page,
        subset: &PreviewSetId,
        superset: &PreviewSetId,
        cpx: Cpx,
        text: &str,
    ) -> Self {
        self.ref_showed(id, page, subset, superset, cpx, text);
        self
    }

    pub fn ref_showed(
        &mut self,
        id: &str,
        page: Page,
        subset: &PreviewSetId,
        superset: &PreviewSetId,
        cpx: Cpx,
        text: &str,
    ) -> &mut Self {
        let relations = match cpx {
            Cpx::Bounds(a, b) => vec![self.relation(
                subset,
                superset,
                Inclusion {
                    mn: Some(a.clone()),
                    mx: Some(b.clone()),
                },
            )],
            Cpx::Exactly(a) => vec![self.relation(
                subset,
                superset,
                Inclusion {
                    mn: Some(a.clone()),
                    mx: Some(a.clone()),
                },
            )],
            Cpx::UpperBound(b) => vec![self.relation(
                subset,
                superset,
                Inclusion {
                    mn: None,
                    mx: Some(b.clone()),
                },
            )],
            Cpx::LowerBound(b) => vec![self.relation(
                subset,
                superset,
                Inclusion {
                    mn: Some(b.clone()),
                    mx: None,
                },
            )],
            Cpx::Todo => vec![self.relation(
                subset,
                superset,
                Inclusion {
                    mn: None,
                    mx: Some(CpxTime::Exists),
                },
            )],
            Cpx::Equal => {
                if subset == superset {
                    vec![
                        self.relation(subset, superset, Equal),
                    ]
                } else {
                    vec![
                        self.relation(subset, superset, Equal),
                        self.relation(superset, subset, Equal),
                    ]
                }
            },
            Cpx::Equivalent(first_to_second_cpx, second_to_first_cpx) => vec![
                self.relation(
                    subset,
                    superset,
                    Inclusion {
                        mn: None,
                        mx: Some(first_to_second_cpx),
                    },
                ),
                self.relation(
                    superset,
                    subset,
                    Inclusion {
                        mn: None,
                        mx: Some(second_to_first_cpx),
                    },
                ),
            ],
            Cpx::Exclusion => vec![self.relation(subset, superset, Exclusion)],
            Cpx::Incomparable => vec![
                self.relation(subset, superset, Exclusion),
                self.relation(superset, subset, Exclusion),
            ],
            Cpx::StrictUpperBound(a) => vec![
                self.relation(
                    subset,
                    superset,
                    Inclusion {
                        mn: None,
                        mx: Some(a.clone()),
                    },
                ),
                self.relation(superset, subset, Exclusion),
            ],
        };

        for relation in relations {
            let showed = RawShowed {
                id: ShowedId::new(id.into()),
                text: text.into(),
                fact: RawShowedFact::Relation(relation),
                page: page.clone(),
            };
            self.factoids.push(showed);
        }
        self
    }

    pub fn asked(
        self,
        id: &str,
        page: Page,
        subset: &PreviewSetId,
        superset: &PreviewSetId,
        text: &str,
    ) -> Self {
        // todo - implement asked: source listed a relation as an open question
        self
    }

    pub fn collective(mut self, page: Page, text: &str) -> CollectiveSource {
        CollectiveSource {
            raw: self,
            page,
            text: text.into(),
        }
    }

    // pub fn cited(self, id: &str, page: Page, who: &RawSource, text: &str) -> Self {
    // let showed = RawShowed {
    // id: id.into(),
    // text: text.into(),
    // fact: RawShowedFact::Citation(who.id.preview()),
    // page,
    // };
    // self.data.factoids.push((self.source.id.clone(), showed));
    // self
    // }

    /// Notes that a source contains a hasse diagram of the listed sets.
    /// This method recreates that diagram with results from HOPS.
    pub fn hasse(mut self, id: &str, page: Page, sets: &Vec<&str>) -> Self {
        self.source.drawings.push(RawDrawing::Hasse(
            sets.iter().map(|x| PreviewSetId::from(x.to_string())).collect(),
        ));
        self
    }

    /// Notes that a source has a complete comparison table of the listed sets.
    /// This recreates the same table from the results in HOPS.
    pub fn table(mut self, id: &str, page: Page, sets: &Vec<&str>) -> Self {
        self.source.drawings.push(RawDrawing::Table(
            sets.iter().map(|x| PreviewSetId::from(x.to_string())).collect(),
        ));
        self
    }

    pub fn todo_rest(mut self, builder: &mut Builder) -> PreviewSourceId {
        warn!(
            "todo: rest of the source {} should be processed",
            self.source.id.to_string()
        );
        self.done(builder)
    }

    pub fn done(mut self, builder: &mut Builder) -> PreviewSourceId {
        self.data_done(&mut builder.data)
    }

    pub fn data_done(mut self, data: &mut RawData) -> PreviewSourceId {
        let RawDataSource {
            source,
            factoids,
            mut relations,
        } = self;
        for factoid in factoids {
            data.factoids.push((source.id.preview(), factoid));
        }
        data.relations.append(&mut relations);
        let res = source.id.preview();
        data.sources.push(RawSource::from(source));
        res
    }

}

impl From<BuiltRawSource> for RawSource {
    fn from(raw: BuiltRawSource) -> Self {
        RawSource {
            id: raw.id,
            rawsourcekey: raw.rawsourcekey,
            relevance: raw.relevance,
            drawings: raw.drawings,
        }
    }
}
