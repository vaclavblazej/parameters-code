use std::collections::HashMap;

use biblatex::Entry;
use log::error;

use crate::data::core::{
    PartialResultsBuilder, PreviewShowed, Provider, ProviderLink, Relation, ShowedFact, Source, Tag
};
use crate::data::id::{Id, PreviewId, PreviewRelationId, PreviewSetId, RelationId};
use crate::data::preview::*;
use crate::general::enums::{CreatedBy, Drawing, RawDrawing, SourceKey, SourcedCpxInfo};
use crate::input::raw::*;
use crate::work::date::Date;

// impl Into<PreviewSourceKey> for RawSourceKey {
// fn into(self) -> PreviewSourceKey {
// match self {
// Self::Bibtex { key } => PreviewSourceKey::Bibtex { key },
// Self::Online { url } => PreviewSourceKey::Online { url },
// Self::Other { name, description: _ } => PreviewSourceKey::Other { name },
// }
// }
// }

impl From<&RawType> for PreviewType {
    fn from(raw: &RawType) -> PreviewType {
        match raw {
            RawType::Parameter => PreviewType::Parameter,
            RawType::GraphClass => PreviewType::GraphClass,
        }
    }
}

impl Provider {
    pub fn from(raw: RawProvider, links: Vec<ProviderLink>) -> Provider {
        Provider {
            id: raw.id,
            name: raw.name,
            url: raw.url,
            links,
        }
    }
}

impl ProviderLink {
    pub fn from(item: RawProviderLink, name: String) -> Self {
        ProviderLink {
            provider_name: name,
            set: item.set,
            url: item.url,
        }
    }
}

impl From<&RawSet> for PreviewSet {
    fn from(raw: &RawSet) -> Self {
        PreviewSet {
            id: raw.id.preview(),
            name: raw.name.clone(),
            typ: PreviewType::from(&raw.typ),
            relevance: raw.relevance,
        }
    }
}

impl From<&RawShowed> for PreviewShowed {
    fn from(raw: &RawShowed) -> PreviewShowed {
        PreviewShowed {
            id: raw.id.preview(),
            text: raw.text.clone(),
            fact: ShowedFact::from(&raw.fact),
            page: raw.page.clone(),
        }
    }
}

impl From<&RawShowedFact> for ShowedFact {
    fn from(raw: &RawShowedFact) -> ShowedFact {
        match raw {
            RawShowedFact::Relation(x) => ShowedFact::Relation(x.clone()),
            // Self::Citation(x) => ShowedFact::Citation(x.preprocess(&sourcekey)),
            RawShowedFact::Definition(x) => ShowedFact::Definition(x.clone()),
        }
    }
}

impl Tag {
    pub fn from(raw: RawTag, sets: Vec<PreviewSet>) -> Self {
        Self {
            preview: PreviewTag::from(&raw),
            id: raw.id,
            name: raw.name,
            description: raw.description,
            sets,
        }
    }
}

impl From<&RawTag> for PreviewTag {
    fn from(raw: &RawTag) -> PreviewTag {
        PreviewTag {
            id: raw.id.preview(),
            name: raw.name.clone(),
        }
    }
}

// impl From<&Relation> for WorkRelation {
// fn from(rel: &Relation) -> WorkRelation {
// WorkRelation {
// subset: rel.subset.clone(),
// superset: rel.superset.clone(),
// }
// }
// }

// impl From<PreviewRelation> for WorkRelation {
// fn from(raw: PreviewRelation) -> WorkRelation {
// WorkRelation {
// subset: raw.subset.clone(),
// superset: raw.superset.clone(),
// }
// }
// }

// impl From<&RawRelation> for PreviewRelation { // todo
    // fn from(raw: &RawRelation) -> PreviewRelation {
        // let preview_subset: PreviewSetId = raw.subset.into();
        // let preview_superset: PreviewSetId = raw.superset.into();
        // PreviewRelation {
            // id: PreviewRelationId::new(&preview_subset, &preview_superset),
            // subset: preview_subset,
            // superset: preview_superset,
            // cpx: raw.cpx,
        // }
    // }
// }

fn str_to_preview_set(list: Vec<PreviewSetId>, preview_set_map: &HashMap<PreviewSetId, PreviewSet>) -> Vec<PreviewSet> {
    let mut res = vec![];
    for el in list {
        match preview_set_map.get(&el) {
            Some(x) => res.push(x.clone()),
            None => {
                error!("didn't find set with id {}", el.to_string());
            },
        }
    }
    res
}

impl Drawing {
    pub fn from(raw: &RawDrawing, preview_set_map: &HashMap<PreviewSetId, PreviewSet>) -> Drawing {
        match raw {
            RawDrawing::Table(q) => Drawing::Table(str_to_preview_set(q.clone(), preview_set_map)),
            RawDrawing::Hasse(q) => Drawing::Hasse(str_to_preview_set(q.clone(), preview_set_map)),
        }
    }
}

// impl PreviewSource {
// pub fn from(raw: &RawSource, sourcekey: &SourceKey) -> PreviewSource {
// PreviewSource {
// id: raw.id.preview(),
// sourcekey: sourcekey.clone(),
// time: Date::empty(),
// }
// }
// }
