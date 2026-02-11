use std::collections::HashMap;

use crate::data::id::*;
use crate::data::preview::*;
use crate::input::raw::RawData;

pub struct PreviewCollection {
    pub graph_classes_previews: HashMap<PreviewGraphClassId, PreviewGraphClass>,
    pub graph_class_properties_previews:
        HashMap<PreviewGraphClassPropertyId, PreviewGraphClassProperty>,
    pub graphs_previews: HashMap<PreviewGraphId, PreviewGraph>,
    pub logic_fragments_previews: HashMap<PreviewLogicFragmentId, PreviewLogicFragment>,
    pub operations_previews: HashMap<PreviewOperationId, PreviewOperation>,
    pub parameters_previews: HashMap<PreviewParameterId, PreviewParameter>,
    pub parametric_graph_class_previews:
        HashMap<PreviewParametricGraphClassId, PreviewParametricGraphClass>,
    pub parametric_parameters_previews:
        HashMap<PreviewParametricParameterId, PreviewParametricParameter>,
    pub providers_previews: HashMap<PreviewProviderId, PreviewProvider>,
    pub tags_previews: HashMap<PreviewTagId, PreviewTag>,
    pub graph_relations_previews: HashMap<PreviewGraphRelationId, PreviewGraphRelation>,
}

macro_rules! process_to_preview_map {
    ($result:ident, $field:ident, $raw_data:ident) => {
        let $result = $raw_data
            .$field
            .iter()
            .map(|x| (x.id.preview(), x.preview()))
            .collect();
    };
}

impl PreviewCollection {
    pub fn new(raw_data: &RawData) -> Self {
        process_to_preview_map!(graph_classes_previews, graph_classes, raw_data);
        process_to_preview_map!(
            graph_class_properties_previews,
            graph_class_properties,
            raw_data
        );
        process_to_preview_map!(graphs_previews, graphs, raw_data);
        process_to_preview_map!(logic_fragments_previews, logic_fragments, raw_data);
        process_to_preview_map!(operations_previews, operations, raw_data);
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
        process_to_preview_map!(graph_relations_previews, graph_relations, raw_data);

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
            graph_relations_previews,
        }
    }
}
