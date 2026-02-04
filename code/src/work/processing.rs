//! Given raw data this module enriches and interconnects it.

use std::collections::{HashMap, HashSet, VecDeque};

use biblatex::{Bibliography, Chunk, DateValue, Entry, PermissiveType, Person, Spanned};
use log::{debug, error, trace, warn};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

use crate::data::bibliography::bibligraphy_to_source;
use crate::data::data::*;
use crate::data::date::Date;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::{Link, Linkable};
use crate::data::preview::*;
use crate::data::simple_index::SimpleIndex;
use crate::data::*;
use crate::general::file;
use crate::general::progress::ProgressDisplay;
use crate::input::raw::*;
use crate::input::source::Def;
use crate::input::source::DefKind;
use crate::input::source::Rel;
use crate::input::source::RelKind;
use crate::input::source::{RawFact, RawWrote};
use crate::input::source::{RawSource, RawSourceKey};
use crate::work::hierarchy::Relation;
use crate::work::preview_collection::PreviewCollection;
use crate::work::sets::{RelatedSets, prepare_extremes};

fn process_parameter(
    parameter: RawParameter,
    tag_set: &Vec<(PreviewTagId, PreviewParameterId)>,
    tag_map: &HashMap<PreviewTagId, PreviewTag>,
    preview_collection: &PreviewCollection,
) -> Parameter {
    let preview = parameter.preview();
    let RawParameter {
        id,
        score,
        name_core,
        definition: raw_definition,
        tags: raw_tags,
    } = parameter;
    // let mut timeline_map: HashMap<PreviewSourceId, Vec<PreviewWrote>> = HashMap::new(); // todo
    // for (source_id, showed) in &preview_collection.factoids {
    //     let should_save = match &showed.fact {
    //         ShowedFact::Relation(status, relation) => {
    //             let relation = preview_collection
    //                 .preview_relation_map
    //                 .get(&relation.id)
    //                 .unwrap();
    //             relation.superset.id == id.preview() || relation.subset.id == id.preview()
    //         }
    //         ShowedFact::Definition(status, defined_set_id) if defined_set_id == &id.preview() => {
    //             true
    //         }
    //         // ShowedFact::Citation( .. ) => false, // todo
    //         ShowedFact::Definition(..) => false,
    //     };
    //     if should_save {
    //         let mut arr = timeline_map.entry(source_id.clone()).or_default();
    //         arr.push(showed.clone());
    //     }
    // }
    // let mut timeline: Vec<SourceSubset> = timeline_map.into_iter()
    //     .map(|(source_id, showed_vec)| {
    //         let source = sources.get(&source_id).unwrap_or_else(
    //             ||panic!("A source id {} does not have a processed source. Use create.source() to add new sources.", source_id)
    //             );
    //         SourceSubset {
    //             preview: source.preview(),
    //             source: source.id.preview(),
    //             sourcekey: source.sourcekey.clone(),
    //             showed: showed_vec,
    //         }
    //     })
    // .collect();
    // timeline.sort_by_key(|subset| subset.preview.time.clone());
    // timeline.reverse();
    // let subsets = help.get_subsets(&preview);
    // let supersets = help.get_supersets(&preview);
    // let sub_exclusions = help.get_antisubsets(&preview);
    // let super_exclusions = help.get_antisupersets(&preview);
    // let mut unknown_map: HashSet<PreviewParameter> = HashSet::new();
    // for par in &preview_collection.preview_sets {
    //     unknown_map.insert(par.clone());
    // }
    // for s in &subsets {
    //     unknown_map.remove(s);
    // }
    // for s in &supersets {
    //     unknown_map.remove(s);
    // }
    // let unknown = unknown_map.iter().cloned().collect();
    // let providers = if let Some(content) = set_providers.get(&preview.id) {
    //     content.clone()
    // } else {
    //     vec![]
    // };
    // let transfers = HashMap::new(); // todo
    let mut tags: Vec<PreviewTag> = vec![];
    for (tag, set_id) in tag_set {
        if *set_id == id {
            tags.push(tag_map.get(tag).unwrap().clone());
        }
    }
    Parameter {
        id,
        name_core,
        definition: ParameterDefinition::from(raw_definition, preview_collection),
        score,
        tags,
        // related_sets: RelatedSets::new(
        //     help.get_eqsets(&preview),
        //     prepare_extremes(supersets, help),
        //     prepare_extremes(subsets, help),
        //     prepare_extremes(super_exclusions, help),
        //     prepare_extremes(sub_exclusions, help),
        //     prepare_extremes(unknown, help),
        // ),
    }
}

impl RawSource {
    fn process(
        self,
        bibliography: &Option<Bibliography>,
        preview_collection: &PreviewCollection,
    ) -> Source {
        trace!("processing set {:?}", self.rawsourcekey);
        let mut time = Date::empty();
        let sourcekey: SourceKey = match &self.rawsourcekey {
            RawSourceKey::Bibtex { key } => {
                let (res, rtime) = bibligraphy_to_source(bibliography, key);
                time = rtime;
                res
            }
            RawSourceKey::Online { url } => SourceKey::Online { url: url.clone() },
            RawSourceKey::Other { name, description } => SourceKey::Other {
                name: name.clone(),
                description: description.clone(),
            },
        };
        let mut showed = vec![];
        // for (preview_source_id, preview_showed) in &preview_collection.factoids {
        //     if preview_source_id == &self.id.preview() {
        //         showed.push(preview_showed.clone());
        //     }
        // }
        Source {
            id: self.id,
            sourcekey,
            wrote: showed,
            time,
            drawings: vec![],
            // drawings: source // todo
            //     .drawings
            //     .iter()
            //     .map(|drawing| Drawing::from(drawing, &preview_collection.preview_set_map))
            //     .collect(),
        }
    }
}

// pub struct PartialResultsBuilder {
//     pub arr: Vec<PartialResult>,
// }

// impl Relation {
//     pub fn new(
//         subset: PreviewParameter,
//         superset: PreviewParameter,
//         cpx: SourcedCpxInfo,
//         handle: usize,
//     ) -> Self {
//         Self {
//             id: RelationId::new(&subset.id, &superset.id),
//             handle,
//             cpx,
//             subset,
//             superset,
//         }
//     }
// }

pub fn process_raw_data(rawdata: RawData, bibliography: &Option<Bibliography>) -> Data {
    // previews ////////////////////////////////////////////////////////////////
    let preview_collection = PreviewCollection::new(&rawdata);
    // destruct to components //////////////////////////////////////////////////
    let RawData {
        graph_class_relations: raw_graph_class_relations,
        graph_classes: raw_graph_classes,
        graph_relations: raw_graph_relations,
        graphs: raw_graphs,
        logic_fragments: raw_logic_fragments,
        operations: raw_operations,
        parameters: raw_parameters,
        parametric_graph_class: raw_parametric_graph_class,
        parametric_parameters: raw_parametric_parameters,
        providers: raw_providers,
        tags: raw_tags,
        graph_class_properties: raw_graph_class_property,
        sources: raw_sources,
        factoids: raw_factoids,
        drawings: raw_drawings,
        provider_links: raw_provider_links,
        problems: raw_problems,
    } = rawdata;
    // maps ////////////////////////////////////////////////////////////////////
    let raw_parameters_map = convert_to_id_map(raw_parameters);
    let tag_map = convert_to_id_map(raw_tags);
    // sources /////////////////////////////////////////////////////////////////
    let sources = convert_to_id_map(
        raw_sources
            .into_iter()
            .map(|source| source.process(bibliography, &preview_collection))
            .collect(),
    );
    // wrote ///////////////////////////////////////////////////////////////////
    let mut definitions_map: HashMap<DefKind, Vec<Def>> = HashMap::new();
    let mut relations_map: HashMap<RelKind, Vec<Rel>> = HashMap::new();
    for (source_id, raw_wrote) in raw_factoids {
        let RawWrote { text, page, facts } = raw_wrote;
        for (showed_id, wrote_status, fact) in facts {
            match fact {
                RawFact::Rel(r) => {
                    relations_map
                        .entry(r.kind())
                        .or_insert_with(Vec::new)
                        .push(r);
                }
                RawFact::Def(d) => {
                    definitions_map
                        .entry(d.kind())
                        .or_insert_with(Vec::new)
                        .push(d);
                }
            }
        }
    }
    // // providers ///////////////////////////////////////////////////////////////
    // let mut provider_names = HashMap::new();
    // for raw_provider in &raw_providers {
    //     provider_names.insert(raw_provider.id.preview(), raw_provider.name.clone());
    // }
    // let mut provider_links: HashMap<PreviewProviderId, Vec<ProviderLink>> = HashMap::new();
    // for provider_link in raw_provider_links {
    //     let mut links = provider_links
    //         .entry(provider_link.provider.clone())
    //         .or_default();
    //     let name = provider_names.get(&provider_link.provider).unwrap().clone();
    //     links.push(ProviderLink::from(provider_link, name));
    // }
    // let providers: Vec<Provider> = raw_providers
    //     .into_iter()
    //     .filter_map(|x| {
    //         if let Some(links) = provider_links.get(&x.id.preview()) {
    //             Some(Provider::from(x, links.clone()))
    //         } else {
    //             error!("key not found in provider links {}", x.id);
    //             None
    //         }
    //     })
    //     .collect();
    // let mut set_providers: HashMap<PreviewParameterId, Vec<ProviderLink>> = HashMap::new();
    // for provider in &providers {
    //     for link in &provider.links {
    //         set_providers
    //             .entry(link.set.clone())
    //             .or_default()
    //             .push(link.clone());
    //     }
    // }
    // tags ////////////////////////////////////////////////////////////////////
    let parameter_links: HashMap<PreviewParameterId, Link> = raw_parameters_map
        .iter()
        .map(|(k, v)| (v.previewid(), v.get_link()))
        .collect();
    let mut tag_set: Vec<(PreviewTagId, PreviewParameterId)> = Vec::new();
    for (k, v) in &raw_parameters_map {
        for preview_tag_id in &v.tags {
            tag_set.push((preview_tag_id.clone(), k.clone()));
        }
    }
    // for val in raw_parameters_map.values() {
    //     for tagid in &val.tags {
    //         let mut sets: &mut Vec<Link> = tag_set.entry(tagid.clone()).or_insert(Vec::new());
    //         sets.push(
    //             preview_collection
    //                 .parameters_previews
    //                 .get(&val.id.preview())
    //                 .unwrap()
    //                 .get_link(),
    //         );
    //     }
    // }
    let tags: Vec<Tag> = tag_map
        .into_iter()
        .map(|(tagid, tag)| {
            // let tag = tag_map.remove(&tagid).unwrap();
            Tag::from(tag, vec![]) // todo sets.get_link()
        })
        .collect();
    let mut tag_preview_map: HashMap<PreviewTagId, PreviewTag> = HashMap::new();
    for tag in &tags {
        tag_preview_map.insert(tag.id.preview(), tag.preview());
    }
    // // relations ///////////////////////////////////////////////////////////////
    // let (relations, partial_results) =
    //     process_relations(&composed_sets, &transfers, &sources, &preview_collection);
    let parameters = raw_parameters_map
        .into_values()
        .into_iter()
        .map(|parameter| {
            process_parameter(parameter, &tag_set, &tag_preview_map, &preview_collection)
        })
        .collect();
    // // finalize ////////////////////////////////////////////////////////////////
    // let res_sources = ordered_sources
    //     .iter()
    //     .map(|x| sources.remove(x).unwrap())
    //     .collect();
    // todo
    Data::new(DataFields {
        graph_class_relations: vec![],
        tags,
        providers: vec![],
        parametric_parameters: vec![],
        parametric_graph_class: vec![],
        parameters,
        operations: vec![],
        logic_fragments: vec![],
        graphs: vec![],
        graph_relations: vec![],
        graph_classes: vec![],
        sources: vec![],
        factoids: HashMap::new(),
        drawings: HashMap::new(),
        graph_class_properties: vec![],
    })
}
