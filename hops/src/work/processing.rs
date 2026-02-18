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
use crate::work::preview_collection::PreviewCollection;

fn process_parameter(
    parameter: RawParameter,
    preview_collection: &PreviewCollection,
    factoids: &Vec<(PreviewSourceId, Vec<Wrote>)>,
    source_map: &HashMap<PreviewSourceId, Source>,
) -> Parameter {
    let thisid = parameter.previewid();
    let RawParameter {
        id,
        score,
        name_core,
        definition: raw_definition,
        tags,
    } = parameter;
    let mut timeline: Vec<(PreviewSource, Vec<Wrote>)> = Vec::new();
    for (source_id, wrotes) in factoids {
        let mut ok_wrote: Vec<Wrote> = Vec::new();
        for Wrote { text, page, facts } in wrotes {
            let mut ok_facts: Vec<(PreviewShowedId, WroteStatus, Fact)> = Vec::new();
            for (showed_id, status, fact) in facts {
                let should_save = match fact {
                    Fact::Definition(def) => match def{
                        Definition::Parameter(preview) => preview.id == thisid,
                        _ => false,
                    }
                    Fact::Relation(rel) => match rel {
                        Relation::ParPar(pa, pb, _) => pa.id == thisid || pb.id == thisid,
                        Relation::PropPar(_, preview, _) 
                        | Relation::GcPar(_, preview, _)
                        | Relation::ParProp(preview, _, _)
                        | Relation::ProbPar(_, preview, _) => preview.id == thisid,
                        _ => false
                    },
                };
                if should_save {
                    ok_facts.push((showed_id.clone(), status.clone(), fact.clone()));
                }
            }
            if !ok_facts.is_empty() {
                ok_wrote.push(Wrote{
                    text: text.clone(),
                    page: page.clone(),
                    facts: ok_facts,
                });
            }
        }
        if !ok_wrote.is_empty() {
            timeline.push((source_map.get(source_id).unwrap().preview(), ok_wrote));
        }
    }
    Parameter {
        id,
        name_core,
        definition: ParameterDefinition::from(raw_definition, preview_collection),
        score,
        tags: tags
            .iter()
            .map(|x| preview_collection.tags_previews.get(x).unwrap().clone())
            .collect(),
        timeline,
    }
}

fn process_graph_class(
    graph_class: RawGraphClass,
    preview_collection: &PreviewCollection,
) -> GraphClass {
    let RawGraphClass {
        id,
        score,
        name_core,
        definition,
        tags,
        variant,
    } = graph_class;
    GraphClass {
        id,
        score,
        name_core,
        definition: GraphClassDefinition::from(definition, preview_collection),
        variant: GraphClassVariant::from(variant),
        tags: tags
            .iter()
            .map(|x| preview_collection.tags_previews.get(x).unwrap().clone())
            .collect(),
    }
}

fn process_graph_class_property(
    gc_property: RawGraphClassProperty,
    preview_collection: &PreviewCollection,
) -> GraphClassProperty {
    let RawGraphClassProperty {
        id,
        score,
        name_core,
        definition,
        own,
        tags,
    } = gc_property;
    GraphClassProperty {
        id,
        score,
        name_core,
        definition: GraphClassPropertyDefinition::from(definition, preview_collection),
        own: Own::from(own),
        tags: tags
            .iter()
            .map(|x| preview_collection.tags_previews.get(x).unwrap().clone())
            .collect(),
    }
}

fn process_graph(graph: RawGraph, preview_collection: &PreviewCollection) -> Graph {
    let RawGraph {
        id,
        score,
        name_core,
        definition,
        tags,
    } = graph;
    Graph {
        id,
        score,
        name_core,
        definition,
        tags: tags
            .iter()
            .map(|x| preview_collection.tags_previews.get(x).unwrap().clone())
            .collect(),
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
    preview_collection: &PreviewCollection,
) -> ParametricParameter {
    let RawParametricParameter {
        id,
        score,
        name_core,
        definition,
        tags,
    } = pp;
    ParametricParameter {
        id,
        score,
        name_core,
        tags: tags
            .iter()
            .map(|x| preview_collection.tags_previews.get(x).unwrap().clone())
            .collect(),
    }
}

fn process_parametric_graph_class(
    pgc: RawParametricGraphClass,
    preview_collection: &PreviewCollection,
) -> ParametricGraphClass {
    let RawParametricGraphClass {
        id,
        score,
        name_core,
        closed_under,
        tags,
        definition,
    } = pgc;
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
        tags: tags
            .iter()
            .map(|x| preview_collection.tags_previews.get(x).unwrap().clone())
            .collect(),
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
    let sources = convert_to_id_map(
        raw_sources
            .into_iter()
            .map(|source| source.process(bibliography, &preview_collection))
            .collect(),
    );
    let factoids: Vec<(PreviewSourceId, Vec<Wrote>)> = raw_factoids.into_iter().map(|(source_id, wrotes)|{
        (source_id, wrotes.into_iter().map(|wrote|{
            Wrote::from(wrote, &sources, &preview_collection)
        }).collect())
    }).collect();    let raw_parameters_map = convert_to_id_map(raw_parameters);
    let mut definitions_map: HashMap<DefKind, Vec<Definition>> = HashMap::new();
    let mut relations_map: HashMap<RelKind, Vec<Relation>> = HashMap::new();
    for (source_id, wrotes) in &factoids {
        for wrote in wrotes {
            let Wrote { text, page, facts } = wrote;
            for (showed_id, wrote_status, fact) in facts {
                match fact {
                    Fact::Relation(r) => {
                        relations_map.entry(r.kind()).or_default().push(r.clone());
                    }
                    Fact::Definition(d) => {
                        definitions_map.entry(d.kind()).or_default().push(d.clone());
                    }
                }
            }
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
    let tags: Vec<Tag> = raw_tags.into_iter().map(|t| Tag::from(t, vec![])).collect();
    let mut tag_map: HashMap<PreviewTagId, Tag> = convert_to_id_map(tags);
    fn add_tag_links<T>(items: &[T], tag_map: &mut HashMap<PreviewTagId, Tag>)
    where
        T: Linkable + Tagged<PreviewTagId>,
    {
        for item in items {
            let link = item.get_link();
            for tag_id in item.tag() {
                if let Some(tag) = tag_map.get_mut(tag_id) {
                    tag.sets.push(link.clone());
                }
            }
        }
    }
    for parameter in raw_parameters_map.values() {
        let link = parameter.get_link();
        for tag_id in &parameter.tags {
            if let Some(tag) = tag_map.get_mut(tag_id) {
                tag.sets.push(link.clone());
            }
        }
    }
    add_tag_links(&raw_graph_classes, &mut tag_map);
    add_tag_links(&raw_graph_class_properties, &mut tag_map);
    add_tag_links(&raw_graphs, &mut tag_map);
    add_tag_links(&raw_parametric_parameters, &mut tag_map);
    add_tag_links(&raw_parametric_graph_class, &mut tag_map);
    // let (relations, partial_results) =
    //     process_relations(&composed_sets, &transfers, &sources, &preview_collection);
    let parameters = raw_parameters_map
        .into_values()
        .map(|parameter| process_parameter(parameter, &preview_collection, &factoids, &sources))
        .collect();
    let graph_classes = raw_graph_classes
        .into_iter()
        .map(|gc| process_graph_class(gc, &preview_collection))
        .collect();
    let graph_class_properties = raw_graph_class_properties
        .into_iter()
        .map(|prop| process_graph_class_property(prop, &preview_collection))
        .collect();
    let graphs: Vec<Graph> = raw_graphs
        .into_iter()
        .map(|graph| process_graph(graph, &preview_collection))
        .collect();
    let logic_fragments: Vec<LogicFragment> = raw_logic_fragments
        .into_iter()
        .map(process_logic_fragment)
        .collect();
    let operations: Vec<Operation> = raw_operations.into_iter().map(process_operation).collect();
    let graph_relations: Vec<GraphRelation> = raw_graph_relations
        .into_iter()
        .map(|gr| process_graph_relation(gr, &preview_collection))
        .collect();
    let parametric_parameters: Vec<ParametricParameter> = raw_parametric_parameters
        .into_iter()
        .map(|pp| process_parametric_parameter(pp, &preview_collection))
        .collect();
    let parametric_graph_class: Vec<ParametricGraphClass> = raw_parametric_graph_class
        .into_iter()
        .map(|pgc| process_parametric_graph_class(pgc, &preview_collection))
        .collect();
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
    let mut arc_gc_par = Vec::new();
    let mut arc_problem_problem = Vec::new();
    let mut arc_problem_gcprop = Vec::new();
    let mut arc_problem_parameter = Vec::new();
    let mut arc_gcprop_parameter = Vec::new();
    for (k, col) in relations_map {
        for x in col {
            match x {
                Relation::LfLf(f, t, d) => arc_lf_lf.push((f.clone(), t.clone(), d.clone())),
                Relation::OpOp(f, t, d) => arc_op_op.push((f.clone(), t.clone(), d.clone())),
                Relation::GrGr(f, t, d) => arc_graph_graph.push((f.clone(), t.clone(), d.clone())),
                Relation::GcGc(f, t, d) => arc_gc_gc.push((f.clone(), t.clone(), d.clone())),
                Relation::GrGc(f, t, d) => arc_graph_gc.push((f.clone(), t.clone(), d.clone())),
                Relation::PgcPgc(f, t, d) => arc_pargc_pargc.push((f.clone(), t.clone(), d.clone())),
                Relation::ParPar(f, t, d) => arc_parameter_parameter.push((f.clone(), t.clone(), d.clone())),
                Relation::PropProp(f, t, d) => arc_gcprop_gcprop.push((f.clone(), t.clone(), d.clone())),
                Relation::GcProp(f, t, d) => arc_gc_gcprop.push((f.clone(), t.clone(), d.clone())),
                Relation::GcPar(f, t, d) => arc_gc_par.push((f.clone(), t.clone(), d.clone())),
                Relation::ParProp(f, t, d) => arc_parameter_gcprop.push((f.clone(), t.clone(), d.clone())),
                Relation::ProbProb(f, t, d) => arc_problem_problem.push((f.clone(), t.clone(), d.clone())),
                Relation::ProbProp(f, t, d) => arc_problem_gcprop.push((f.clone(), t.clone(), d.clone())),
                Relation::ProbPar(f, t, d) => arc_problem_parameter.push((f.clone(), t.clone(), d.clone())),
                Relation::PropPar(f, t, d) => arc_gcprop_parameter.push((f.clone(), t.clone(), d.clone())),
            }
        }
    }
    Data::new(DataFields {
        tags: tag_map.into_values().collect(),
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
        arc_gc_par,
        arc_parameter_gcprop,
        arc_problem_problem,
        arc_problem_gcprop,
        arc_problem_parameter,
        arc_gcprop_parameter,
    })
}
