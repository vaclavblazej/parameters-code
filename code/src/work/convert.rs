use std::collections::HashMap;

use biblatex::Entry;
use log::error;

use crate::data::data::{Named, Own, Provider, ProviderLink, Tag};
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::preview::*;
use crate::input::raw::{RawProvider, RawProviderLink, RawTag};
use crate::input::raw_enums::RawOwn;
use crate::input::source::RawNotedSource;
use crate::work::date::Date;

impl From<&RawOwn> for Own {
    fn from(raw: &RawOwn) -> Own {
        match raw {
            RawOwn::Has => Own::Has,
            RawOwn::Is => Own::Is,
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
    pub fn from(item: RawProviderLink, provider_id: PreviewProviderId) -> Self {
        ProviderLink {
            provider: provider_id,
            set: item.set_id,
            url: item.url,
        }
    }
}

impl Tag {
    pub fn from(raw: RawTag, sets: Vec<Box<dyn Named>>) -> Self {
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

// fn str_to_preview_set(
//     list: Vec<PreviewSetId>,
//     preview_set_map: &HashMap<PreviewSetId, PreviewSet>,
// ) -> Vec<PreviewSet> {
//     let mut res = vec![];
//     for el in list {
//         match preview_set_map.get(&el) {
//             Some(x) => res.push(x.clone()),
//             None => {
//                 error!("didn't find set with id {}", el);
//             }
//         }
//     }
//     res
// }
//
// impl Drawing {
//     pub fn from(raw: &RawDrawing, preview_set_map: &HashMap<PreviewSetId, PreviewSet>) -> Drawing {
//         match raw {
//             RawDrawing::Table(q) => Drawing::Table(str_to_preview_set(q.clone(), preview_set_map)),
//             RawDrawing::Hasse(q) => Drawing::Hasse(str_to_preview_set(q.clone(), preview_set_map)),
//         }
//     }
// }
