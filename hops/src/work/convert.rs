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
use crate::input::source::Def;
use crate::input::source::RawFact;
use crate::input::source::RawNotedSource;
use crate::input::source::RawWrote;
use crate::input::source::RawWroteStatus;
use crate::input::source::Rel;
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

impl Wrote {
    pub fn from(item: RawWrote, source_map: &HashMap<PreviewSourceId, Source>, preview_collection: &PreviewCollection) -> Self {
        let RawWrote { text, page, facts } = item;
        let f: Vec<(PreviewShowedId, WroteStatus, Fact)> = facts.into_iter().map(|(id, wrote, fact)|{
            (id.preview(), WroteStatus::from(wrote, source_map), Fact::from(fact, preview_collection))
        }).collect();
        Self {
            text,
            page,
            facts: f,
        }
    }
}

impl Fact {
    pub fn from(item: RawFact, preview_collection: &PreviewCollection) -> Self {
        match item {
            RawFact::Def(def) => Self::Definition(Definition::from(def, preview_collection)),
            RawFact::Rel(rel) => Self::Relation(Relation::from(rel, preview_collection)),
        }
    }
}

impl Definition {
    pub fn from(item: Def, preview_collection: &PreviewCollection) -> Self {
        match item {
            Def::LogicFragment(id) => Definition::LogicFragment(preview_collection.logic_fragments_previews.get(&id).unwrap().clone()),
            Def::Parameter(id) => Definition::Parameter(preview_collection.parameters_previews.get(&id).unwrap().clone()),
            Def::Graph(id) => Definition::Graph(preview_collection.graphs_previews.get(&id).unwrap().clone()),
            Def::GraphClass(id) => Definition::GraphClass(preview_collection.graph_classes_previews.get(&id).unwrap().clone()),
            Def::Operation(id) => Definition::Operation(preview_collection.operations_previews.get(&id).unwrap().clone()),
            Def::Problem(id) => Definition::Problem(preview_collection.problem_previews.get(&id).unwrap().clone()),
            Def::ParametricParameter(id) => Definition::ParParameter(preview_collection.parametric_parameters_previews.get(&id).unwrap().clone()),
            Def::ParametricGraphClass(id) => Definition::ParGraphClass(preview_collection.parametric_graph_class_previews.get(&id).unwrap().clone()),
            Def::Property(id) => Definition::Property(preview_collection.graph_class_properties_previews.get(&id).unwrap().clone()),
        }
    }
}

impl Relation {
    pub fn from(item: Rel, preview_collection: &PreviewCollection) -> Self {
        match item {
            Rel::LfLf(ida, idb, d) => Relation::LfLf(preview_collection.logic_fragments_previews.get(&ida).unwrap().clone(), preview_collection.logic_fragments_previews.get(&idb).unwrap().clone(), d),
            Rel::OpOp(ida, idb, d) => Relation::OpOp(preview_collection.operations_previews.get(&ida).unwrap().clone(), preview_collection.operations_previews.get(&idb).unwrap().clone(), d),
            Rel::GrGr(ida, idb, d) => Relation::GrGr(preview_collection.graphs_previews.get(&ida).unwrap().clone(), preview_collection.graphs_previews.get(&idb).unwrap().clone(), d),
            Rel::GcGc(ida, idb, d) => Relation::GcGc(preview_collection.graph_classes_previews.get(&ida).unwrap().clone(), preview_collection.graph_classes_previews.get(&idb).unwrap().clone(), d),
            Rel::GrGc(ida, idb, d) => Relation::GrGc(preview_collection.graphs_previews.get(&ida).unwrap().clone(), preview_collection.graph_classes_previews.get(&idb).unwrap().clone(), d),
            Rel::PgcPgc(ida, idb, d) => Relation::PgcPgc(preview_collection.parametric_graph_class_previews.get(&ida).unwrap().clone(), preview_collection.parametric_graph_class_previews.get(&idb).unwrap().clone(), d),
            Rel::ParPar(ida, idb, d) => Relation::ParPar(preview_collection.parameters_previews.get(&ida).unwrap().clone(), preview_collection.parameters_previews.get(&idb).unwrap().clone(), d),
            Rel::PropProp(ida, idb, d) => Relation::PropProp(preview_collection.graph_class_properties_previews.get(&ida).unwrap().clone(), preview_collection.graph_class_properties_previews.get(&idb).unwrap().clone(), d),
            Rel::PropPar(ida, idb, d) => Relation::PropPar(preview_collection.graph_class_properties_previews.get(&ida).unwrap().clone(), preview_collection.parameters_previews.get(&idb).unwrap().clone(), d),
            Rel::GcProp(ida, idb, d) => Relation::GcProp(preview_collection.graph_classes_previews.get(&ida).unwrap().clone(), preview_collection.graph_class_properties_previews.get(&idb).unwrap().clone(), d),
            Rel::GcPar(ida, idb, d) => Relation::GcPar(preview_collection.graph_classes_previews.get(&ida).unwrap().clone(), preview_collection.parameters_previews.get(&idb).unwrap().clone(), d),
            Rel::ParProp(ida, idb, d) => Relation::ParProp(preview_collection.parameters_previews.get(&ida).unwrap().clone(), preview_collection.graph_class_properties_previews.get(&idb).unwrap().clone(), d),
            Rel::ProbProb(ida, idb, d) => Relation::ProbProb(preview_collection.problem_previews.get(&ida).unwrap().clone(), preview_collection.problem_previews.get(&idb).unwrap().clone(), d),
            Rel::ProbProp(ida, idb, d) => Relation::ProbProp(preview_collection.problem_previews.get(&ida).unwrap().clone(), preview_collection.graph_class_properties_previews.get(&idb).unwrap().clone(), d),
            Rel::ProbPar(ida, idb, d) => Relation::ProbPar(preview_collection.problem_previews.get(&ida).unwrap().clone(), preview_collection.parameters_previews.get(&idb).unwrap().clone(), d),
        }
    }
}

impl WroteStatus {
    pub fn from(item: RawWroteStatus, source_map: &HashMap<PreviewSourceId, Source>) -> Self {
        match item {
            RawWroteStatus::Assumed => Self::Assumed,
            RawWroteStatus::Conjectured => Self::Conjectured,
            RawWroteStatus::Original => Self::Original,
            RawWroteStatus::Derivative => Self::Derivative,
            RawWroteStatus::Noted(raw_noted_source) => Self::Noted(NotedSource::from(raw_noted_source, source_map)),
            RawWroteStatus::TodoStatus => Self::TodoStatus,
        }
    }
}

impl NotedSource {
    pub fn from(item: RawNotedSource, source_map: &HashMap<PreviewSourceId, Source>) -> Self {
        match item {
            RawNotedSource::SrcText(txt) => Self::SrcText(txt),
            RawNotedSource::Source(preview_id) => Self::Source({
                source_map.get(&preview_id).expect("referenced unexisting source id").preview()
            }),
            RawNotedSource::Omitted => Self::Omitted,
            RawNotedSource::SrcTodo => Self::SrcTodo,
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
