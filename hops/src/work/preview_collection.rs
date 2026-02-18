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
    pub problem_previews: HashMap<PreviewProblemId, PreviewProblem>,
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
        process_to_preview_map!(graph_class_properties_previews, graph_class_properties, raw_data);
        process_to_preview_map!(graphs_previews, graphs, raw_data);
        process_to_preview_map!(logic_fragments_previews, logic_fragments, raw_data);
        process_to_preview_map!(operations_previews, operations, raw_data);
        process_to_preview_map!(parameters_previews, parameters, raw_data);
        process_to_preview_map!(parametric_graph_class_previews, parametric_graph_class, raw_data);
        process_to_preview_map!(parametric_parameters_previews, parametric_parameters, raw_data);
        process_to_preview_map!(providers_previews, providers, raw_data);
        process_to_preview_map!(tags_previews, tags, raw_data);
        process_to_preview_map!(graph_relations_previews, graph_relations, raw_data);
        process_to_preview_map!(problem_previews, problems, raw_data);
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
            problem_previews,
        }
    }
}
