//! Given raw data this module enriches and interconnects it.

use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Sub;
use std::path::PathBuf;

use biblatex::{Bibliography, Chunk, DateValue, Entry, PermissiveType, Person, Spanned};
use log::{debug, error, trace, warn};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

use crate::data::bibliography::bibligraphy_to_source;
use crate::data::data::Relevant;
use crate::data::date::Date;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::preview::*;
use crate::data::simple_index::SimpleIndex;
use crate::data::*;
use crate::general::file;
use crate::general::progress::ProgressDisplay;
use crate::input::build::{ASSUMED_SOURCE_ID, UNKNOWN_SOURCE_ID};
use crate::input::raw::*;
use crate::input::source::{RawSource, RawSourceKey};
use crate::work::combine::combine_serial;
use crate::work::sets::{RelatedSets, prepare_extremes};

pub fn is_more_relevant_than<A, B>(a: &dyn A, b: &dyn B) -> bool
where
    A: Relevant + HasId,
    B: Relevant + HasId,
{
    (a.relevance() == b.relevance() && a.id < b.id) || a.relevance() > b.relevance()
}

/// Given a RawSet create a full Set with all the information.
fn process_set(
    set: RawSet,
    help: &SimpleIndex,
    set_providers: &HashMap<PreviewSetId, Vec<ProviderLink>>,
    sources: &HashMap<PreviewSourceId, Source>,
    tag_set: &Vec<(PreviewTagId, PreviewSetId)>,
    tag_map: &HashMap<PreviewTagId, PreviewTag>,
    preview_collection: &PreviewCollection,
) -> Set {
    if set.displayed_definition.is_empty() {
        warn!("todo: set '{}' has no displayed definition", set.name)
    }
    let preview = PreviewSet::from(&set);
    let RawSet {
        id,
        name,
        typ,
        composed,
        relevance,
        aka,
        abbr,
        displayed_definition,
    } = set;
    let mut timeline_map: HashMap<PreviewSourceId, Vec<Showed>> = HashMap::new();
    for (source_id, showed) in &preview_collection.factoids {
        let should_save = match &showed.fact {
            ShowedFact::Relation(status, relation) => {
                let relation = preview_collection
                    .preview_relation_map
                    .get(&relation.id)
                    .unwrap();
                relation.superset.id == id.preview() || relation.subset.id == id.preview()
            }
            ShowedFact::Definition(status, defined_set_id) if defined_set_id == &id.preview() => {
                true
            }
            // ShowedFact::Citation( .. ) => false, // todo
            ShowedFact::Definition(..) => false,
        };
        if should_save {
            let mut arr = timeline_map.entry(source_id.clone()).or_default();
            arr.push(showed.clone());
        }
    }
    let mut timeline: Vec<SourceSubset> = timeline_map.into_iter()
        .map(|(source_id, showed_vec)| {
            let source = sources.get(&source_id).unwrap_or_else(
                ||panic!("A source id {} does not have a processed source. Use create.source() to add new sources.", source_id)
                );
            SourceSubset {
                preview: source.preview(),
                source: source.id.preview(),
                sourcekey: source.sourcekey.clone(),
                showed: showed_vec,
            }
        })
    .collect();
    timeline.sort_by_key(|subset| subset.preview.time.clone());
    timeline.reverse();
    let subsets = help.get_subsets(&preview);
    let supersets = help.get_supersets(&preview);
    let sub_exclusions = help.get_antisubsets(&preview);
    let super_exclusions = help.get_antisupersets(&preview);
    let mut unknown_map: HashSet<PreviewSet> = HashSet::new();
    for par in &preview_collection.preview_sets {
        unknown_map.insert(par.clone());
    }
    for s in &subsets {
        unknown_map.remove(s);
    }
    for s in &supersets {
        unknown_map.remove(s);
    }
    let unknown = unknown_map.iter().cloned().collect();
    let providers = if let Some(content) = set_providers.get(&preview.id) {
        content.clone()
    } else {
        vec![]
    };
    // let transfers = HashMap::new(); // todo
    let mut tags: Vec<PreviewTag> = vec![];
    for (tag, set_id) in tag_set {
        if set_id == id {
            tags.push(tag_map.get(tag).unwrap().clone());
        }
    }
    Set {
        id,
        name: preview.name.clone(),
        typ: preview.typ.clone(),
        providers,
        timeline,
        relevance,
        aka,
        abbr,
        tags,
        // transfers,
        displayed_definition,
        related_sets: RelatedSets::new(
            help.get_eqsets(&preview),
            prepare_extremes(supersets, help),
            prepare_extremes(subsets, help),
            prepare_extremes(super_exclusions, help),
            prepare_extremes(sub_exclusions, help),
            prepare_extremes(unknown, help),
        ),
    }
}

fn process_source(
    source: RawSource,
    bibliography: &Option<Bibliography>,
    preview_collection: &PreviewCollection,
) -> Source {
    trace!("processing set {:?}", source.rawsourcekey);
    let mut time = Date::empty();
    let sourcekey: SourceKey = match &source.rawsourcekey {
        RawSourceKey::Bibtex { key } => {
            let (res, rtime) = bibligraphy_to_source(bibliography, time, key);
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
    for (preview_source_id, preview_showed) in &preview_collection.factoids {
        if preview_source_id == &source.id.preview() {
            showed.push(preview_showed.clone());
        }
    }
    Source {
        id: source.id,
        sourcekey,
        wrote: showed,
        time,
        drawings: source
            .drawings
            .iter()
            .map(|drawing| Drawing::from(drawing, &preview_collection.preview_set_map))
            .collect(),
    }
}

fn add_and_update(
    result: PartialResult,
    relation_map: &mut HashMap<WorkRelation, PartialResult>,
    updated_relations: &mut VecDeque<WorkRelation>,
    partial_result_builder: &mut PartialResultsBuilder,
) {
    let res = if let Some(x) = relation_map.get_mut(&result.relation) {
        if let Some(res) = x.combine_parallel(&result, partial_result_builder) {
            trace!(
                "updated relation (replace) {} {}",
                x.relation.subset, x.relation.superset,
            );
            res
        } else {
            return;
        }
    } else {
        trace!(
            "updated relation (insert) {} {}",
            result.relation.subset, result.relation.superset,
        );
        result
    };
    updated_relations.push_back(res.relation.clone());
    relation_map.insert(res.relation.clone(), res);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkRelation {
    pub subset: PreviewSetId,
    pub superset: PreviewSetId,
}

// todo, remove clone?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartialResult {
    pub handle: usize,
    pub created_by: CreatedBy,
    pub relation: WorkRelation,
    pub cpx: CpxInfo,
}

pub struct PartialResultsBuilder {
    pub arr: Vec<PartialResult>,
}

fn process_relations(
    composed_sets: &Vec<(PreviewSetId, Vec<PreviewSet>)>,
    transfers: &HashMap<TransferGroup, HashMap<PreviewSetId, Vec<PreviewSet>>>,
    sources: &HashMap<PreviewSourceId, Source>,
    preview_collection: &PreviewCollection,
) -> (Vec<Relation>, Vec<PartialResult>) {
    trace!("processing relations");
    let mut partial_results: Vec<PartialResult> = vec![];
    let mut partial_results_builder = PartialResultsBuilder::new();
    for (raw_source_id, showed) in &preview_collection.factoids {
        match &showed.fact {
            ShowedFact::Relation(status, relation) => {
                if matches!(status, ShowedStatus::Conjectured) {
                    continue;
                }
                if let Some(source) = sources.get(raw_source_id) {
                    let work_relation =
                        WorkRelation::new(&relation.subset.id, &relation.superset.id);
                    let partial_result = partial_results_builder.partial_result(
                        CreatedBy::Directly(source.preview()),
                        relation.cpx.clone(),
                        work_relation.clone(),
                    );
                    partial_results.push(partial_result);
                } else {
                    panic!("source not found {:?}", raw_source_id);
                }
            }
            ShowedFact::Definition(_, _) => (),
        }
    }
    let mut res: HashMap<WorkRelation, PartialResult> = HashMap::new();
    let mut progress = ProgressDisplay::new("processing", 22113);
    for partial_result in partial_results {
        let pair = partial_result.relation.clone();
        debug!(
            "processing relation from {} to {}",
            pair.subset, pair.superset
        );
        let mut updated_relations: VecDeque<WorkRelation> = VecDeque::new();
        // todo add progress in history when the collection is more complete
        add_and_update(
            partial_result,
            &mut res,
            &mut updated_relations,
            &mut partial_results_builder,
        );
        let mut improved_relations = 0;
        while let Some(relation) = updated_relations.pop_front() {
            improved_relations += 1;
            if improved_relations >= 5000 {
                panic!("5k updates during processing probably means a bug");
            }
            // apply the new or improved relation
            for set in &preview_collection.preview_sets {
                if set.id == relation.subset || set.id == relation.superset {
                    continue;
                }
                // equivalence ab copies the new relation cd into ef
                let xx = &relation.subset.clone();
                let yy = &relation.superset.clone();
                let zz = &set.id.clone();
                for (x, y, z) in [
                    (xx, yy, zz),
                    (xx, zz, yy),
                    (yy, xx, zz),
                    (yy, zz, xx),
                    (zz, xx, yy),
                    (zz, yy, xx),
                ] {
                    let Some(ab) = res.get(&WorkRelation::new(x, y)) else {
                        continue;
                    };
                    match ab.to_sourced() {
                        SourcedCpxInfo::Equal { source } => {
                            for (c, d, e, f) in
                                [(z, x, z, y), (z, y, z, x), (x, z, y, z), (y, z, x, z)]
                            {
                                let Some(cd) = res.get(&WorkRelation::new(c, d)) else {
                                    continue;
                                };
                                let created_by =
                                    CreatedBy::SameThroughEquivalence(cd.handle, source.handle);
                                let partial_result = partial_results_builder.partial_result(
                                    created_by,
                                    cd.cpx.clone(),
                                    WorkRelation::new(e, f),
                                );
                                debug!("equivalence");
                                add_and_update(
                                    partial_result,
                                    &mut res,
                                    &mut updated_relations,
                                    &mut partial_results_builder,
                                );
                            }
                        }
                        _ => continue,
                    }
                }
                // inclusion ab and inclusion bc imply inclusion ac
                for (a, b, c) in [
                    (&set.id, &relation.subset, &relation.superset),
                    (&relation.subset, &relation.superset, &set.id),
                ] {
                    let Some(ab) = res.get(&WorkRelation::new(a, b)) else {
                        continue;
                    };
                    let Some(bc) = res.get(&WorkRelation::new(b, c)) else {
                        continue;
                    };
                    if let (
                        SourcedCpxInfo::Inclusion {
                            mn: _,
                            mx: Some((mxa, sra)),
                        },
                        SourcedCpxInfo::Inclusion {
                            mn: _,
                            mx: Some((mxb, srb)),
                        },
                    ) = (ab.to_sourced(), bc.to_sourced())
                    {
                        let rel = sra.relation.combine_serial(&srb.relation);
                        let (a, b, time) = combine_serial((mxa, sra), (mxb, srb));
                        let pr = partial_results_builder.partial_result(
                            CreatedBy::TransitiveInclusion(a.handle, b.handle),
                            CpxInfo::Inclusion {
                                mn: None,
                                mx: Some(time),
                            },
                            rel,
                        );
                        debug!(
                            "inclusions {} {} + {} = {}",
                            updated_relations.len(),
                            a.handle,
                            b.handle,
                            c
                        );
                        add_and_update(
                            pr,
                            &mut res,
                            &mut updated_relations,
                            &mut partial_results_builder,
                        );
                    };
                }
                // inclusion ab and exclusion cd implies exclusion ef
                for (a, b, c, d, e, f) in [
                    (
                        &relation.subset,
                        &relation.superset,
                        &set.id,
                        &relation.superset,
                        &set.id,
                        &relation.subset,
                    ),
                    (
                        &relation.subset,
                        &relation.superset,
                        &relation.subset,
                        &set.id,
                        &relation.superset,
                        &set.id,
                    ),
                    (
                        &set.id,
                        &relation.superset,
                        &relation.subset,
                        &relation.superset,
                        &relation.subset,
                        &set.id,
                    ),
                    (
                        &relation.subset,
                        &set.id,
                        &relation.subset,
                        &relation.superset,
                        &set.id,
                        &relation.superset,
                    ),
                ] {
                    let Some(ab) = res.get(&WorkRelation::new(a, b)) else {
                        continue;
                    };
                    let Some(cd) = res.get(&WorkRelation::new(c, d)) else {
                        continue;
                    };
                    let res_relation = WorkRelation::new(e, f);
                    match (&ab.to_sourced(), &cd.to_sourced()) {
                        (
                            SourcedCpxInfo::Inclusion {
                                mn: _,
                                mx: Some((_, smx)),
                            },
                            SourcedCpxInfo::Exclusion { source },
                        ) => {
                            let created_by =
                                CreatedBy::TransitiveExclusion(smx.handle, source.handle);
                            let partial_result = partial_results_builder.partial_result(
                                created_by,
                                CpxInfo::Exclusion,
                                res_relation,
                            );
                            debug!("exclusions");
                            add_and_update(
                                partial_result,
                                &mut res,
                                &mut updated_relations,
                                &mut partial_results_builder,
                            );
                        }
                        _ => continue,
                    }
                }
            }
            // inclusion ab implies inclusion f(a)f(b) for a transfer f
            if let Some(ab) = res.get(&relation) {
                let new_partial_results =
                    apply_transfers(transfers, ab, &mut partial_results_builder);
                for partial_result in new_partial_results {
                    debug!(
                        "transfer from ({},{}) to ({},{})",
                        relation.subset,
                        relation.superset,
                        partial_result.relation.subset,
                        partial_result.relation.superset,
                    );
                    add_and_update(
                        partial_result,
                        &mut res,
                        &mut updated_relations,
                        &mut partial_results_builder,
                    );
                }
            }
            // inclusion ab and ac imply inclusion a(b+c)
            for (composed_set, composed_elements) in composed_sets {
                if &relation.subset == composed_set {
                    continue;
                }
                let hash_components: HashSet<PreviewSetId> =
                    composed_elements.iter().map(|x| x.id.clone()).collect();
                if hash_components.contains(&relation.superset) {
                    debug!(
                        "attempting composition {} {}",
                        relation.subset, composed_set
                    );
                    let mut okay = true;
                    let opt_cpxs: Vec<SourcedCpxInfo> = composed_elements
                        .iter()
                        .map(|x| res.get(&WorkRelation::new(&relation.subset, &x.id)))
                        .filter_map(|x| {
                            if let Some(a) = x {
                                Some(a.to_sourced())
                            } else {
                                okay = false;
                                None
                            }
                        })
                        .collect();
                    if okay && !opt_cpxs.is_empty() {
                        let mut cpx: SourcedCpxInfo = opt_cpxs.first().unwrap().clone();
                        for i in 1..opt_cpxs.len() {
                            cpx = cpx.combine_plus(opt_cpxs.get(i).unwrap())
                        }
                        debug!("result: {:?}", cpx);
                        let handles: Vec<usize> = composed_elements
                            .iter()
                            .map(|x| res.get(&WorkRelation::new(&relation.subset, &x.id)))
                            .filter_map(|x| x.map(|a| a.handle))
                            .collect();
                        debug!("sum");
                        let key = WorkRelation::new(&relation.subset, composed_set);
                        let partial_result = partial_results_builder.partial_result(
                            CreatedBy::SumInclusion(handles),
                            cpx.into(),
                            key,
                        ); // todo check
                        add_and_update(
                            partial_result,
                            &mut res,
                            &mut updated_relations,
                            &mut partial_results_builder,
                        );
                    }
                }
            }
        }
        progress.increase(improved_relations);
    }
    progress.done();
    let result: Vec<Relation> = res
        .values()
        .map(|x: &PartialResult| {
            let subset = preview_collection
                .preview_set_map
                .get(&x.relation.subset)
                .unwrap()
                .clone();
            let superset = preview_collection
                .preview_set_map
                .get(&x.relation.superset)
                .unwrap()
                .clone();
            Relation::new(subset, superset, x.to_sourced(), x.handle)
        })
        .collect();
    (result, partial_results_builder.done())
}

// impl Relation {
//     pub fn new(
//         subset: PreviewSet,
//         superset: PreviewSet,
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
//
// fn apply_transfers(
//     transfers: &HashMap<TransferGroup, HashMap<PreviewSetId, Vec<PreviewSet>>>,
//     partial_result: &PartialResult,
//     partial_results_builder: &mut PartialResultsBuilder,
// ) -> Vec<PartialResult> {
//     let mut transferred_relations: Vec<PartialResult> = Vec::new();
//     let top = &partial_result.relation.subset;
//     let bot = &partial_result.relation.superset;
//     for (transfer_group, map) in transfers.iter() {
//         if let (Some(top_res), Some(bot_res)) = (map.get(top), map.get(bot)) {
//             let mut res_cpx: SourcedCpxInfo = partial_result.clone().to_sourced();
//             let okay = match res_cpx.clone() {
//                 SourcedCpxInfo::Inclusion { mn, mx } => {
//                     res_cpx = SourcedCpxInfo::Inclusion {
//                         mn,
//                         mx: match mx {
//                             // todo get rid of these exceptions via lambda that takes the result and transforms it
//                             Some((Constant, smx)) => Some((CpxTime::Linear, smx)),
//                             x => x,
//                         },
//                     };
//                     true
//                 }
//                 _ => false,
//             };
//             if okay {
//                 let created_by =
//                     CreatedBy::TransferredFrom(transfer_group.clone(), partial_result.handle);
//                 for tr in top_res {
//                     for br in bot_res {
//                         let key = WorkRelation::new(&tr.id, &br.id);
//                         let res = partial_results_builder.partial_result(
//                             created_by.clone(),
//                             res_cpx.clone().into(),
//                             key,
//                         );
//                         transferred_relations.push(res);
//                     }
//                 }
//             }
//         }
//     }
//     transferred_relations
// }
//
macro_rules! process_to_preview_map {
    ($result:ident, $field:ident, $raw_data:ident) => {
        let $result = $raw_data
            .$field
            .iter()
            .map(|x| (x.id.clone(), x.preview()))
            .collect();
    };
}

struct PreviewCollection {
    graph_classes_previews: HashMap<PreviewGraphClassId, PreviewGraphClass>,
    graph_class_properties_previews:
        HashMap<PreviewGraphClassPropertyId, PreviewGraphClassProperty>,
    graphs_previews: HashMap<PreviewGraphId, PreviewGraph>,
    logic_fragments_previews: HashMap<PreviewLogicFragmentId, PreviewLogicFragment>,
    operations_previews: HashMap<PreviewOperationId, PreviewOperation>,
    parameters_previews: HashMap<PreviewParameterId, PreviewParameter>,
    parametric_graph_class_previews:
        HashMap<PreviewParametricGraphClassId, PreviewParametricGraphClass>,
    parametric_parameters_previews:
        HashMap<PreviewParametricParameterId, PreviewParametricParameter>,
    providers_previews: HashMap<PreviewProviderId, PreviewProvider>,
    tags_previews: HashMap<PreviewTagId, PreviewTag>,
}

impl PreviewCollection {
    fn new(raw_data: &RawData) -> Self {
        process_to_preview_map!(graph_classes_previews, graph_classes, raw_data);
        process_to_preview_map!(
            graph_class_properties_previews,
            graph_class_properties,
            raw_data
        );
        process_to_preview_map!(graphs_previews, graphs, raw_data);
        process_to_preview_map!(logic_fragments_previews, logic_fragments, raw_data);
        process_to_preview_map!(operations_previews, logic_fragments, raw_data);
        process_to_preview_map!(parameters_previews, parameters, raw_data);
        process_to_preview_map!(
            parametric_graph_class_previews,
            parametric_graph_class,
            raw_data
        );
        process_to_preview_map!(
            parametric_parameters_previews,
            parametric_parameters,
            raw_data
        );
        process_to_preview_map!(providers_previews, providers, raw_data);
        process_to_preview_map!(tags_previews, tags, raw_data);
        // pub graph_relations: Vec<RawGraphRelation>,

        // todo
        // // previews ////////////////////////////////////////////////////////////////
        // let preview_sets: Vec<PreviewSet> = raw_sets.iter().map(PreviewSet::from).collect();
        // let preview_set_map: HashMap<PreviewSetId, PreviewSet> = preview_sets
        //     .iter()
        //     .map(|x| (x.id.clone(), x.clone()))
        //     .collect();
        // let mut preview_relation_map: HashMap<PreviewRelationId, PreviewRelation> = HashMap::new();
        // for raw_relation in raw_relations {
        //     let subset = preview_set_map.get(&raw_relation.subset).unwrap().clone();
        //     let superset = preview_set_map.get(&raw_relation.superset).unwrap().clone();
        //     let res = PreviewRelation {
        //         id: RelationId::new(&subset.id, &superset.id).preview(),
        //         subset,
        //         superset,
        //         cpx: raw_relation.cpx.clone(),
        //     };
        //     preview_relation_map.insert(res.id.clone(), res);
        // }
        // // factoids /////////////////////////////////////////////////////////////////
        // let mut factoids: Vec<(PreviewSourceId, Showed)> = Vec::new();
        // for (preview_source_id, raw_showed) in raw_factoids {
        //     let showed_fact = match &raw_showed.fact {
        //         RawShowedFact::Relation(s, raw_relation) => {
        //             let preview_relation = PreviewRelation {
        //                 id: raw_relation.id.clone(),
        //                 subset: preview_set_map.get(&raw_relation.subset).unwrap().clone(),
        //                 superset: preview_set_map.get(&raw_relation.superset).unwrap().clone(),
        //                 cpx: raw_relation.cpx.clone(),
        //             };
        //             ShowedFact::Relation(ShowedStatus::from(s), preview_relation)
        //         }
        //         RawShowedFact::Definition(s, x) => {
        //             ShowedFact::Definition(ShowedStatus::from(s), x.clone())
        //         }
        //     };
        //     let prev_showed = Showed {
        //         id: raw_showed.id.preview(),
        //         text: raw_showed.text.clone(),
        //         fact: showed_fact,
        //         page: raw_showed.page.clone(),
        //     };
        //     factoids.push((preview_source_id, prev_showed));
        // }
        Self {
            graph_classes_previews,
            graph_class_properties_previews,
            graphs_previews,
            logic_fragments_previews,
            operations_previews,
            parameters_previews,
            parametric_graph_class_previews,
            parametric_parameters_previews,
            providers_previews,
            tags_previews,
        }
    }
}

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
    } = rawdata;
    // sources /////////////////////////////////////////////////////////////////
    let sources = raw_sources
        .iter()
        .map(|x| process_source(x, bibliography, &preview_collection))
        .collect();
    // // list of composed sets ///////////////////////////////////////////////////
    // let mut composed_sets: Vec<(PreviewSetId, Vec<PreviewSet>)> = vec![];
    // for raw_graph_class in &raw_graph_classes {
    //     if let Composition::Intersection(ref vec) = raw_graph_class.composed {
    //         let comp_preview: Vec<PreviewSet> = vec
    //             .iter()
    //             .map(|x| preview_collection.preview_set_map.get(x).unwrap().clone())
    //             .collect();
    //         composed_sets.push((raw_graph_class.id.preview(), comp_preview));
    //     }
    // }
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
    // let mut set_providers: HashMap<PreviewSetId, Vec<ProviderLink>> = HashMap::new();
    // for provider in &providers {
    //     for link in &provider.links {
    //         set_providers
    //             .entry(link.set.clone())
    //             .or_default()
    //             .push(link.clone());
    //     }
    // }
    // tags ////////////////////////////////////////////////////////////////////
    let mut tagged_sets: HashMap<PreviewTagId, Box<dyn Named>> = HashMap::new();
    for field in vec![&raw_graph_classes, &raw_graphs] {
        for val in field {
            for tag in val.tags {
                let mut sets = tagged_sets.get(tag.preview()).or_insert(Vec::new());
                sets.push(Box::new(preview_collection.get(val.id)));
            }
        }
    }
    let tags = tagged_sets
        .iter()
        .map(|(tag, sets)| Tag::from(tag, sets))
        .collect();
    let mut tag_previews: HashMap<PreviewTagId, PreviewTag> = HashMap::new();
    for tag in &tags {
        tag_previews.insert(tag.id.preview(), tag.preview());
    }
    // // transfers ///////////////////////////////////////////////////////////////
    // let mut transfers: HashMap<TransferGroup, HashMap<PreviewSetId, Vec<PreviewSet>>> =
    //     HashMap::new();
    // for (key, raw_pairs) in &raw_transfer {
    //     let mut res: HashMap<PreviewSetId, Vec<PreviewSet>> = HashMap::new();
    //     for raw_pair in raw_pairs {
    //         let (from, to) = raw_pair.clone();
    //         let res_from: PreviewSet = preview_collection
    //             .preview_set_map
    //             .get(&from)
    //             .unwrap()
    //             .clone();
    //         let res_to: PreviewSet = preview_collection.preview_set_map.get(&to).unwrap().clone();
    //         res.entry(res_from.id).or_default().push(res_to);
    //     }
    //     transfers.insert(key.clone(), res);
    // }
    // // relations ///////////////////////////////////////////////////////////////
    // let (relations, partial_results) =
    //     process_relations(&composed_sets, &transfers, &sources, &preview_collection);
    // let simple_index = SimpleIndex::new(&relations);
    // let sets = raw_sets
    //     .into_iter()
    //     .map(|set| {
    //         process_set(
    //             set,
    //             &simple_index,
    //             &set_providers,
    //             &sources,
    //             &raw_tag_set,
    //             &tag_map,
    //             &preview_collection,
    //         )
    //     })
    //     .collect();
    // // finalize ////////////////////////////////////////////////////////////////
    // let res_sources = ordered_sources
    //     .iter()
    //     .map(|x| sources.remove(x).unwrap())
    //     .collect();
    Data::new(DataFields {
        graph_class_relations,
        tags,
        providers,
        parametric_parameters,
        parametric_graph_class,
        parameters,
        operations,
        logic_fragments,
        graphs,
        graph_relations,
        graph_classes,
        sources,
        factoids,
        drawings,
    })
}
