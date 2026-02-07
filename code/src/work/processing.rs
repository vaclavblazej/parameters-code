//! Given raw data this module enriches and interconnects it.

use std::collections::{HashMap, HashSet, VecDeque};

use biblatex::{Bibliography, Chunk, DateValue, Entry, PermissiveType, Person, Spanned};
use log::info;
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
use crate::input::raw_enums::*;
use crate::input::source::Def;
use crate::input::source::DefKind;
use crate::input::source::Rel;
use crate::input::source::RelKind;
use crate::input::source::{RawFact, RawWrote};
use crate::input::source::{RawSource, RawSourceKey};
use crate::work::hierarchy::Relation;
use crate::work::preview_collection::PreviewCollection;

fn resolve_tags<Id: Eq>(
    entity_id: &Id,
    tag_set: &[(PreviewTagId, Id)],
    tag_map: &HashMap<PreviewTagId, PreviewTag>,
) -> Vec<PreviewTag> {
    tag_set
        .iter()
        .filter(|(_, id)| id == entity_id)
        .filter_map(|(tag_id, _)| tag_map.get(tag_id).cloned())
        .collect()
}

fn process_parameter(
    parameter: RawParameter,
    tag_set: &[(PreviewTagId, PreviewParameterId)],
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
    let tags = resolve_tags(&id.preview(), tag_set, tag_map);
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

fn process_graph_class(
    graph_class: RawGraphClass,
    tag_set: &[(PreviewTagId, PreviewGraphClassId)],
    tag_map: &HashMap<PreviewTagId, PreviewTag>,
    preview_collection: &PreviewCollection,
) -> GraphClass {
    let RawGraphClass {
        id,
        score,
        name_core,
        definition,
        tags: _,
        variant,
    } = graph_class;
    let tags = resolve_tags(&id.preview(), tag_set, tag_map);
    GraphClass {
        id,
        score,
        name_core,
        definition: GraphClassDefinition::from(definition, preview_collection),
        variant: GraphClassVariant::from(variant),
        tags,
    }
}

fn process_graph_class_property(
    gc_property: RawGraphClassProperty,
    tag_set: &[(PreviewTagId, PreviewGraphClassPropertyId)],
    tag_map: &HashMap<PreviewTagId, PreviewTag>,
    preview_collection: &PreviewCollection,
) -> GraphClassProperty {
    let RawGraphClassProperty {
        id,
        score,
        name_core,
        definition,
        own,
    } = gc_property;
    let tags = resolve_tags(&id.preview(), tag_set, tag_map);
    GraphClassProperty {
        id,
        score,
        name_core,
        definition: GraphClassPropertyDefinition::from(definition, preview_collection),
        own: Own::from(own),
    }
}

fn process_graph(graph: RawGraph) -> Graph {
    let RawGraph {
        id,
        score,
        name_core,
        definition,
    } = graph;
    Graph {
        id,
        score,
        name_core,
        definition,
    }
}

fn process_logic_fragment(lf: RawLogicFragment) -> LogicFragment {
    let RawLogicFragment {
        id,
        name_core,
        description,
    } = lf;
    LogicFragment {
        id,
        name_core,
        description,
    }
}

fn process_operation(op: RawOperation) -> Operation {
    let RawOperation {
        id,
        name_core,
        definition,
    } = op;
    let description = match definition {
        RawOperationDefinition::GraphOperation(text_as_id) => vec![text_as_id.to_string()],
        RawOperationDefinition::GraphClassOperation(text) => vec![text],
    };
    Operation {
        id,
        name_core,
        description,
    }
}

fn process_parametric_parameter(
    pp: RawParametricParameter,
    tag_set: &[(PreviewTagId, PreviewParametricParameterId)],
    tag_map: &HashMap<PreviewTagId, PreviewTag>,
) -> ParametricParameter {
    let RawParametricParameter {
        id,
        score,
        name_core,
        definition: _,
        tags: _,
    } = pp;
    let tags = resolve_tags(&id.preview(), tag_set, tag_map);
    ParametricParameter {
        id,
        score,
        name_core,
        tags,
    }
}

fn process_parametric_graph_class(
    pgc: RawParametricGraphClass,
    tag_set: &[(PreviewTagId, PreviewParametricGraphClassId)],
    tag_map: &HashMap<PreviewTagId, PreviewTag>,
    preview_collection: &PreviewCollection,
) -> ParametricGraphClass {
    let RawParametricGraphClass {
        id,
        score,
        name_core,
        closed_under,
        tags: _,
        definition: _,
    } = pgc;
    let tags = resolve_tags(&id.preview(), tag_set, tag_map);
    let closed_under = preview_collection
        .graph_relations_previews
        .get(&closed_under)
        .unwrap()
        .clone();
    ParametricGraphClass {
        id,
        score,
        name_core,
        closed_under,
        tags,
    }
}

fn process_graph_relation(
    gr: RawGraphRelation,
    preview_collection: &PreviewCollection,
) -> GraphRelation {
    let RawGraphRelation {
        id,
        name_core,
        displayed_definition,
    } = gr;
    GraphRelation {
        id,
        name_core,
        displayed_definition: GraphRelationDefinition::from(
            displayed_definition,
            preview_collection,
        ),
    }
}

impl RawSource {
    fn process(
        self,
        bibliography: &Option<Bibliography>,
        preview_collection: &PreviewCollection,
    ) -> Source {
        let RawSource {
            id,
            rawsourcekey,
            score,
        } = self;
        trace!("processing set {:?}", rawsourcekey);
        let mut time = Date::empty();
        let sourcekey: SourceKey = match &rawsourcekey {
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
            id,
            name_core: NameCore::new(&sourcekey.get_name()),
            sourcekey,
            wrote: showed,
            time,
            drawings: vec![],
            score,
            // drawings: source // todo
            //     .drawings
            //     .iter()
            //     .map(|drawing| Drawing::from(drawing, &preview_collection.preview_set_map))
            //     .collect(),
        }
    }
}

pub fn extract_tags<T>(item: T) -> Vec<(Link, PreviewTagId)>
where
    T: Linkable + Tagged<PreviewTagId>,
{
    let link = item.get_link();
    item.tag()
        .iter()
        .map(|tag| (link.clone(), tag.clone()))
        .collect()
}

pub fn process_raw_data(rawdata: RawData, bibliography: &Option<Bibliography>) -> Data {
    let preview_collection = PreviewCollection::new(&rawdata);
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
        graph_class_properties: raw_graph_class_properties,
        sources: raw_sources,
        factoids: raw_factoids,
        drawings: raw_drawings,
        provider_links: raw_provider_links,
        problems: raw_problems,
    } = rawdata;
    let raw_parameters_map = convert_to_id_map(raw_parameters);
    let tag_map = convert_to_id_map(raw_tags);
    let sources = convert_to_id_map(
        raw_sources
            .into_iter()
            .map(|source| source.process(bibliography, &preview_collection))
            .collect(),
    );
    let mut definitions_map: HashMap<DefKind, Vec<Def>> = HashMap::new();
    let mut relations_map: HashMap<RelKind, Vec<Rel>> = HashMap::new();
    for (source_id, raw_wrote) in raw_factoids {
        let RawWrote { text, page, facts } = raw_wrote;
        for (showed_id, wrote_status, fact) in facts {
            match fact {
                RawFact::Rel(r) => {
                    relations_map.entry(r.kind()).or_default().push(r);
                }
                RawFact::Def(d) => {
                    definitions_map.entry(d.kind()).or_default().push(d);
                }
            }
        }
    }
    let mut arc_parameter_parameter = Vec::new();
    let mut arc_lf_lf = Vec::new();
    let mut arc_op_op = Vec::new();
    let mut arc_graph_graph = Vec::new();
    let mut arc_gc_gc = Vec::new();
    let mut arc_graph_gc = Vec::new();
    let mut arc_pargc_pargc = Vec::new();
    let mut arc_gcprop_gcprop = Vec::new();
    let mut arc_gc_gcprop = Vec::new();
    let mut arc_parameter_gcprop = Vec::new();
    let mut arc_problem_problem = Vec::new();
    let mut arc_problem_gcprop = Vec::new();
    let mut arc_problem_parameter = Vec::new();
    for x in relations_map.get(&RelKind::ParPar).unwrap_or(&vec![]) {
        match x {
            Rel::LfLf(f, t, d) => arc_lf_lf.push((f.clone(), t.clone(), d.clone())),
            Rel::OpOp(f, t, d) => arc_op_op.push((f.clone(), t.clone(), d.clone())),
            Rel::GrGr(f, t, d) => arc_graph_graph.push((f.clone(), t.clone(), d.clone())),
            Rel::GcGc(f, t, d) => arc_gc_gc.push((f.clone(), t.clone(), d.clone())),
            Rel::GrGc(f, t, d) => arc_graph_gc.push((f.clone(), t.clone(), d.clone())),
            Rel::PgcPgc(f, t, d) => arc_pargc_pargc.push((f.clone(), t.clone(), d.clone())),
            Rel::ParPar(f, t, d) => arc_parameter_parameter.push((f.clone(), t.clone(), d.clone())),
            Rel::PropProp(f, t, d) => arc_gcprop_gcprop.push((f.clone(), t.clone(), d.clone())),
            Rel::GcProp(f, t, d) => arc_gc_gcprop.push((f.clone(), t.clone(), d.clone())),
            Rel::ParProp(f, t, d) => arc_parameter_gcprop.push((f.clone(), t.clone(), d.clone())),
            Rel::ProbProb(f, t, d) => arc_problem_problem.push((f.clone(), t.clone(), d.clone())),
            Rel::ProbProp(f, t, d) => arc_problem_gcprop.push((f.clone(), t.clone(), d.clone())),
            Rel::ProbPar(f, t, d) => arc_problem_parameter.push((f.clone(), t.clone(), d.clone())),
        }
    }
    let mut provider_links_map: HashMap<PreviewProviderId, Vec<ProviderLink>> = HashMap::new();
    for provider_link in raw_provider_links {
        provider_links_map
            .entry(provider_link.provider.clone())
            .or_default()
            .push(ProviderLink::from(provider_link));
    }
    let providers: Vec<Provider> = raw_providers
        .into_iter()
        .map(|x| {
            let links = provider_links_map
                .remove(&x.id.preview())
                .unwrap_or_default();
            Provider::from(x, links)
        })
        .collect();
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
    // extract_tags(raw_parameters_map.values()); // todo use
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
    // let (relations, partial_results) =
    //     process_relations(&composed_sets, &transfers, &sources, &preview_collection);
    let parameters = raw_parameters_map
        .into_values()
        .map(|parameter| {
            process_parameter(parameter, &tag_set, &tag_preview_map, &preview_collection)
        })
        .collect();
    let mut graph_class_tag_set: Vec<(PreviewTagId, PreviewGraphClassId)> = Vec::new();
    for gc in &raw_graph_classes {
        for preview_tag_id in &gc.tags {
            graph_class_tag_set.push((preview_tag_id.clone(), gc.id.preview()));
        }
    }
    let graph_classes = raw_graph_classes
        .into_iter()
        .map(|gc| {
            process_graph_class(
                gc,
                &graph_class_tag_set,
                &tag_preview_map,
                &preview_collection,
            )
        })
        .collect();
    let graph_class_properties = raw_graph_class_properties
        .into_iter()
        .map(|prop| {
            // todo fix tags
            process_graph_class_property(prop, &[], &tag_preview_map, &preview_collection)
        })
        .collect();
    let graphs: Vec<Graph> = raw_graphs.into_iter().map(process_graph).collect();
    let logic_fragments: Vec<LogicFragment> = raw_logic_fragments
        .into_iter()
        .map(process_logic_fragment)
        .collect();
    let operations: Vec<Operation> = raw_operations.into_iter().map(process_operation).collect();
    let graph_relations: Vec<GraphRelation> = raw_graph_relations
        .into_iter()
        .map(|gr| process_graph_relation(gr, &preview_collection))
        .collect();
    let mut pp_tag_set: Vec<(PreviewTagId, PreviewParametricParameterId)> = Vec::new();
    for pp in &raw_parametric_parameters {
        for preview_tag_id in &pp.tags {
            pp_tag_set.push((preview_tag_id.clone(), pp.id.preview()));
        }
    }
    let parametric_parameters: Vec<ParametricParameter> = raw_parametric_parameters
        .into_iter()
        .map(|pp| process_parametric_parameter(pp, &pp_tag_set, &tag_preview_map))
        .collect();
    let mut pgc_tag_set: Vec<(PreviewTagId, PreviewParametricGraphClassId)> = Vec::new();
    for pgc in &raw_parametric_graph_class {
        for preview_tag_id in &pgc.tags {
            pgc_tag_set.push((preview_tag_id.clone(), pgc.id.preview()));
        }
    }
    let parametric_graph_class: Vec<ParametricGraphClass> = raw_parametric_graph_class
        .into_iter()
        .map(|pgc| {
            process_parametric_graph_class(
                pgc,
                &pgc_tag_set,
                &tag_preview_map,
                &preview_collection,
            )
        })
        .collect();
    Data::new(DataFields {
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
        sources: sources.into_values().collect(),
        factoids: HashMap::new(),
        drawings: HashMap::new(),
        graph_class_properties,
        arc_parameter_parameter,
        arc_lf_lf,
        arc_op_op,
        arc_graph_graph,
        arc_gc_gc,
        arc_graph_gc,
        arc_pargc_pargc,
        arc_gcprop_gcprop,
        arc_gc_gcprop,
        arc_parameter_gcprop,
        arc_problem_problem,
        arc_problem_gcprop,
        arc_problem_parameter,
    })
}
