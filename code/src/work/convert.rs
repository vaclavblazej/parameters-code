use std::collections::HashMap;

use biblatex::Entry;
use log::error;

use crate::data::core::{
    PartialResultsBuilder, PreviewShowed, Provider, ProviderLink, Relation, ShowedFact, Tag, ShowedStatus, NotedSource
};
use crate::data::id::{Id, PreviewId, PreviewRelationId, PreviewSetId, RelationId};
use crate::data::preview::*;
use crate::general::enums::{CreatedBy, Drawing, RawDrawing, SourceKey, SourcedCpxInfo};
use crate::input::raw::*;
use crate::work::date::Date;

impl From<&RawOwn> for Own {
    fn from(raw: &RawOwn) -> Own {
        match raw {
            RawOwn::Has => Own::Has,
            RawOwn::Is => Own::Is,
        }
    }
}

impl From<&RawType> for PreviewType {
    fn from(raw: &RawType) -> PreviewType {
        match raw {
            RawType::Parameter => PreviewType::Parameter,
            RawType::GraphClass => PreviewType::GraphClass,
            RawType::Property(o) => PreviewType::Property(Own::from(o)),
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

impl From<&RawNotedSource> for NotedSource {
    fn from(raw: &RawNotedSource) -> NotedSource {
        match raw {
            RawNotedSource::Text(s) => NotedSource::Text(s.clone()),
            RawNotedSource::Source(s) => NotedSource::Source(s.clone()),
            RawNotedSource::Omitted => NotedSource::Omitted,
            RawNotedSource::SrcTodo => NotedSource::Todo,
        }
    }
}

impl From<&RawShowedStatus> for ShowedStatus {
    fn from(raw: &RawShowedStatus) -> ShowedStatus {
        match raw {
            RawShowedStatus::Assumed => ShowedStatus::Assumed,
            RawShowedStatus::Conjectured => ShowedStatus::Conjectured,
            RawShowedStatus::Original => ShowedStatus::Original,
            RawShowedStatus::Derivative => ShowedStatus::Derivative,
            RawShowedStatus::Noted(s) => ShowedStatus::Noted(NotedSource::from(s)),
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

impl Tag {
    pub fn from(raw: RawTag, sets: Vec<PreviewSet>) -> Self {
        Self {
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

fn str_to_preview_set(list: Vec<PreviewSetId>, preview_set_map: &HashMap<PreviewSetId, PreviewSet>) -> Vec<PreviewSet> {
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

impl Drawing {
    pub fn from(raw: &RawDrawing, preview_set_map: &HashMap<PreviewSetId, PreviewSet>) -> Drawing {
        match raw {
            RawDrawing::Table(q) => Drawing::Table(str_to_preview_set(q.clone(), preview_set_map)),
            RawDrawing::Hasse(q) => Drawing::Hasse(str_to_preview_set(q.clone(), preview_set_map)),
        }
    }
}
