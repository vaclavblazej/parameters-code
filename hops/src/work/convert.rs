use std::collections::HashMap;

use biblatex::Entry;
use log::error;

use crate::data::link::Link;
use crate::data::preview;
use crate::input::raw_enums::*;

use crate::data::data::*;
use crate::data::date::Date;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::preview::*;
use crate::input::raw::{RawProvider, RawProviderLink, RawTag};
use crate::input::raw_enums::{RawGraphClassDefinition, RawGraphClassVariant, RawOwn};
use crate::input::source::RawNotedSource;
use crate::input::source::definition;
use crate::work::preview_collection::PreviewCollection;

impl ParameterDefinition {
    pub fn from(item: RawParameterDefinition, preview_collection: &PreviewCollection) -> Self {
        match item {
            RawParameterDefinition::BoundsAll(parparid) => {
                let preview = preview_collection
                    .parametric_parameters_previews
                    .get(&parparid);
                Self::BoundsAll(preview.unwrap().clone())
            }
            RawParameterDefinition::GraphText(text) => Self::Graph(text),
            RawParameterDefinition::GraphClassText(text) => Self::GraphClass(text),
            RawParameterDefinition::DistanceToParameter(parid) => Self::DistanceToParameter(
                preview_collection
                    .parameters_previews
                    .get(&parid)
                    .unwrap()
                    .clone(),
            ),
            RawParameterDefinition::DistanceToGraphClass(gcid) => Self::DistanceToGraphClass(
                preview_collection
                    .graph_classes_previews
                    .get(&gcid)
                    .unwrap()
                    .clone(),
            ),
            RawParameterDefinition::IntersectionParameters(ids) => {
                let previews = ids
                    .iter()
                    .filter_map(|id| preview_collection.parameters_previews.get(id).cloned())
                    .collect();
                Self::Intersection(previews)
            }
            RawParameterDefinition::IntersectionParameterProperty(par_id, prop_id) => {
                Self::IntersectionParameterProperty(
                    preview_collection
                        .parameters_previews
                        .get(&par_id)
                        .unwrap()
                        .clone(),
                    preview_collection
                        .graph_class_properties_previews
                        .get(&prop_id)
                        .unwrap()
                        .clone(),
                )
            }
            RawParameterDefinition::IntersectionParameterGraphClass(par_id, gc_id) => {
                Self::IntersectionParameterGraphClass(
                    preview_collection
                        .parameters_previews
                        .get(&par_id)
                        .unwrap()
                        .clone(),
                    preview_collection
                        .graph_classes_previews
                        .get(&gc_id)
                        .unwrap()
                        .clone(),
                )
            }
            RawParameterDefinition::FromParametricParameter(preview_id) => {
                Self::FromParametricParameter(
                    preview_collection
                        .parametric_parameters_previews
                        .get(&preview_id)
                        .unwrap()
                        .clone(),
                )
            }
        }
    }
}

impl Own {
    pub fn from(raw: RawOwn) -> Own {
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
            name_core: raw.name_core,
            url: raw.url,
            links,
        }
    }
}

impl ProviderLink {
    pub fn from(item: RawProviderLink) -> Self {
        ProviderLink {
            provider: item.provider,
            set: item.link,
        }
    }
}

impl GraphRelationDefinition {
    pub fn from(item: RawGraphRelationDefinition, preview_collection: &PreviewCollection) -> Self {
        match item {
            RawGraphRelationDefinition::Text(text) => Self::Text(text),
            RawGraphRelationDefinition::IsomorphicAfterOperations(ids) => {
                let previews = ids
                    .iter()
                    .filter_map(|id| preview_collection.operations_previews.get(id).cloned())
                    .collect();
                Self::IsomorphicAfterOperations(previews)
            }
        }
    }
}

impl Tag {
    pub fn from(raw: RawTag, sets: Vec<Link>) -> Self {
        Self {
            id: raw.id,
            name_core: raw.name_core,
            description: raw.description,
            sets,
        }
    }
}

impl GraphClassDefinition {
    pub fn from(item: RawGraphClassDefinition, preview_collection: &PreviewCollection) -> Self {
        match item {
            RawGraphClassDefinition::Text(text) => Self::Text(vec![text]),
            RawGraphClassDefinition::IntersectionGraphClasses(ids) => {
                let previews = ids
                    .iter()
                    .filter_map(|id| preview_collection.graph_classes_previews.get(id).cloned())
                    .collect();
                Self::Intersection(previews)
            }
            RawGraphClassDefinition::IntersectionGraphClassProperty(gc_id, prop_id) => {
                Self::IntersectionGraphClassProperty(
                    preview_collection
                        .graph_classes_previews
                        .get(&gc_id)
                        .unwrap()
                        .clone(),
                    preview_collection
                        .graph_class_properties_previews
                        .get(&prop_id)
                        .unwrap()
                        .clone(),
                )
            }
            RawGraphClassDefinition::ParametricGraphClass(id) => {
                let preview = preview_collection
                    .parametric_graph_class_previews
                    .get(&id)
                    .unwrap()
                    .clone();
                Self::ParametricGraphClass(preview)
            }
            RawGraphClassDefinition::Parameter(id) => {
                let preview = preview_collection
                    .parameters_previews
                    .get(&id)
                    .unwrap()
                    .clone();
                Self::Parameter(preview)
            }
        }
    }
}

impl GraphClassPropertyDefinition {
    pub fn from(
        item: RawGraphClassPropertyDefinition,
        preview_collection: &PreviewCollection,
    ) -> Self {
        match item {
            RawGraphClassPropertyDefinition::Text(text) => GraphClassPropertyDefinition::Text(text),
            RawGraphClassPropertyDefinition::FromGraphClass(id) => {
                GraphClassPropertyDefinition::FromGraphClass(
                    preview_collection
                        .graph_classes_previews
                        .get(&id)
                        .unwrap()
                        .clone(),
                )
            }
            RawGraphClassPropertyDefinition::FromParameter(id) => {
                GraphClassPropertyDefinition::FromParameter(
                    preview_collection
                        .parameters_previews
                        .get(&id)
                        .unwrap()
                        .clone(),
                )
            }
        }
    }
}

impl GraphClassVariant {
    pub fn from(raw: RawGraphClassVariant) -> Self {
        match raw {
            RawGraphClassVariant::GraphClass => Self::GraphClass,
            RawGraphClassVariant::GraphProperty => Self::GraphProperty,
        }
    }
}

// impl From<&RawTag> for PreviewTag {
//     fn from(raw: &RawTag) -> PreviewTag {
//         PreviewTag {
//             id: raw.id.preview(),
//             name: raw.name.clone(),
//         }
//     }
// }

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
