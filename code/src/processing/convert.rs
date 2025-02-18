
use std::collections::HashMap;

use biblatex::Entry;
use log::error;

use crate::data::data::{Date, PartialResultsBuilder, Provider, ProviderLink, Relation, Showed, ShowedFact, Source, Tag};
use crate::general::enums::{CreatedBy, Drawing, RawDrawing, SourceKey, SourcedCpxInfo};
use crate::input::raw::*;
use crate::data::preview::*;


impl Into<PreviewSourceKey> for RawSourceKey {
    fn into(self) -> PreviewSourceKey {
        match self {
            Self::Bibtex { key } => PreviewSourceKey::Bibtex { key },
            Self::Online { url } => PreviewSourceKey::Online { url },
            Self::Other { name, description: _ } => PreviewSourceKey::Other { name },
        }
    }
}

impl Into<PreviewType> for RawType {
    fn into(self) -> PreviewType {
        match self {
            Self::Parameter => PreviewType::Parameter,
            Self::GraphClass => PreviewType::GraphClass,
        }
    }
}

impl Into<Provider> for RawProvider {
    fn into(self) -> Provider {
        Provider {
            name: self.name,
            url: self.url,
        }
    }
}

impl RawProviderLink {
    pub fn preprocess(self, provider: &Provider) -> ProviderLink {
        ProviderLink {
            provider: provider.clone(),
            set: self.set.into(),
            url: self.url.into(),
        }
    }
}

impl Into<PreviewSet> for RawSet {
    fn into(self) -> PreviewSet {
        PreviewSet {
            id: self.id,
            name: self.name,
            typ: self.typ.into(),
            relevance: self.relevance,
        }
    }
}

impl RawShowed {
    pub fn preprocess(self, sourcekey: &SourceKey) -> Showed {
        Showed {
            id: self.id,
            text: self.text,
            fact: self.fact.preprocess(&sourcekey),
            page: self.page,
        }
    }
}

impl RawShowedFact {
    pub fn preprocess(self, sourcekey: &SourceKey) -> ShowedFact {
        match self {
            Self::Relation(x) => ShowedFact::Relation(x.into()),
            Self::Citation(x) => ShowedFact::Citation(x.preprocess(&sourcekey)),
            Self::Definition(x) => ShowedFact::Definition(x.into()),
        }
    }
}

impl RawTag {
    pub fn preprocess(self, sets: Vec<PreviewSet>) -> Tag {
        Tag {
            preview: self.clone().into(),
            id: self.id,
            name: self.name,
            description: self.description,
            sets,
        }
    }
}

impl Into<PreviewTag> for RawTag {
    fn into(self) -> PreviewTag {
        PreviewTag {
            id: self.id,
            name: self.name,
        }
    }
}

impl Into<PreviewRelation> for RawRelation {
    fn into(self) -> PreviewRelation {
        let preview_subset = self.subset.into();
        let preview_superset = self.superset.into();
        PreviewRelation {
            id: Relation::id(&preview_subset, &preview_superset),
            subset: preview_subset,
            superset: preview_superset,
            cpx: self.cpx,
        }
    }
}

fn str_to_preview_set(list: Vec<String>, preview_set_map: &HashMap<String, PreviewSet>) -> Vec<PreviewSet> {
    let mut res = vec![];
    for el in list {
        match preview_set_map.get(&el) {
            Some(x) => res.push(x.clone()),
            None => {
                error!("didn't find set with id {}", el);
            },
        }
    }
    res
}

impl RawDrawing {
    pub fn preprocess(&self, preview_set_map: &HashMap<String, PreviewSet>) -> Drawing {
        match self {
            RawDrawing::Table(q) => Drawing::Table(str_to_preview_set(q.clone(), preview_set_map)),
            RawDrawing::Hasse(q) => Drawing::Hasse(str_to_preview_set(q.clone(), preview_set_map)),
        }
    }
}

impl RawSource {
    pub fn preprocess(self, sourcekey: &SourceKey) -> PreviewSource {
        PreviewSource {
            id: self.id,
            sourcekey: sourcekey.clone(),
            time: Date::empty(),
        }
    }
}
