use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Serialize;

use crate::input::raw::RawData;
use crate::input::source::{RawFact, Rel};

pub fn export_raw_data(raw: &RawData, json_dir: &Path) -> Result<()> {
    let structures_dir = json_dir.join("structures");
    let relations_dir = json_dir.join("relations");
    fs::create_dir_all(&structures_dir)?;
    fs::create_dir_all(&relations_dir)?;

    write_json(&structures_dir, "parameters", &raw.parameters)?;
    write_json(&structures_dir, "graph_classes", &raw.graph_classes)?;
    write_json(&structures_dir, "graph_class_properties", &raw.graph_class_properties)?;
    write_json(&structures_dir, "graphs", &raw.graphs)?;
    write_json(&structures_dir, "graph_relations", &raw.graph_relations)?;
    write_json(&structures_dir, "graph_class_relations", &raw.graph_class_relations)?;
    write_json(&structures_dir, "logic_fragments", &raw.logic_fragments)?;
    write_json(&structures_dir, "operations", &raw.operations)?;
    write_json(&structures_dir, "parametric_parameters", &raw.parametric_parameters)?;
    write_json(&structures_dir, "parametric_graph_class", &raw.parametric_graph_class)?;
    write_json(&structures_dir, "providers", &raw.providers)?;
    write_json(&structures_dir, "provider_links", &raw.provider_links)?;
    write_json(&structures_dir, "tags", &raw.tags)?;
    write_json(&structures_dir, "sources", &raw.sources)?;
    write_json(&structures_dir, "problems", &raw.problems)?;
    write_json(&structures_dir, "factoids", &raw.factoids)?;
    write_json(&structures_dir, "drawings", &raw.drawings)?;

    extract_relations(raw, &relations_dir)?;

    Ok(())
}

fn write_json<T: Serialize>(dir: &Path, name: &str, value: &T) -> Result<()> {
    let path = dir.join(format!("{}.json", name));
    let file = fs::File::create(&path)?;
    serde_json::to_writer_pretty(file, value)?;
    Ok(())
}

#[derive(Serialize)]
struct Arc<A, B, C> {
    from: A,
    to: B,
    relation: C,
}

fn write_arcs<A: Serialize, B: Serialize, C: Serialize>(
    relations_dir: &Path,
    name: &str,
    items: Vec<(A, B, C)>,
) -> Result<()> {
    let entries: Vec<Arc<A, B, C>> = items
        .into_iter()
        .map(|(f, t, r)| Arc { from: f, to: t, relation: r })
        .collect();
    write_json(relations_dir, name, &entries)
}

fn extract_relations(raw: &RawData, relations_dir: &Path) -> Result<()> {
    use crate::data::id::*;
    use crate::input::source::{
        ClassicalSolvability, Cpx, EquivalenceRelation, ImplicationRelation,
        InclusionRelationUnderGraphRelation, ParameterizedSolvability,
    };

    let mut par_par: Vec<(PreviewParameterId, PreviewParameterId, Cpx)> = Vec::new();
    let mut gc_gc: Vec<(PreviewGraphClassId, PreviewGraphClassId, InclusionRelationUnderGraphRelation)> = Vec::new();
    let mut gc_par: Vec<(PreviewGraphClassId, PreviewParameterId, ImplicationRelation)> = Vec::new();
    let mut gc_prop: Vec<(PreviewGraphClassId, PreviewGraphClassPropertyId, EquivalenceRelation)> = Vec::new();
    let mut prop_par: Vec<(PreviewGraphClassPropertyId, PreviewParameterId, ImplicationRelation)> = Vec::new();
    let mut par_prop: Vec<(PreviewParameterId, PreviewGraphClassPropertyId, EquivalenceRelation)> = Vec::new();
    let mut gr_gr: Vec<(PreviewGraphId, PreviewGraphId, InclusionRelationUnderGraphRelation)> = Vec::new();
    let mut gr_gc: Vec<(PreviewGraphId, PreviewGraphClassId, InclusionRelationUnderGraphRelation)> = Vec::new();
    let mut lf_lf: Vec<(PreviewLogicFragmentId, PreviewLogicFragmentId, ImplicationRelation)> = Vec::new();
    let mut op_op: Vec<(PreviewOperationId, PreviewOperationId, ImplicationRelation)> = Vec::new();
    let mut pgc_pgc: Vec<(PreviewParametricGraphClassId, PreviewParametricGraphClassId, ImplicationRelation)> = Vec::new();
    let mut prop_prop: Vec<(PreviewGraphClassPropertyId, PreviewGraphClassPropertyId, ImplicationRelation)> = Vec::new();
    let mut prob_prob: Vec<(PreviewProblemId, PreviewProblemId, ImplicationRelation)> = Vec::new();
    let mut prob_prop: Vec<(PreviewProblemId, PreviewGraphClassPropertyId, ClassicalSolvability)> = Vec::new();
    let mut prob_par: Vec<(PreviewProblemId, PreviewParameterId, ParameterizedSolvability)> = Vec::new();

    for (_, wrotes) in &raw.factoids {
        for wrote in wrotes {
            for (_, _, fact) in &wrote.facts {
                if let RawFact::Rel(rel) = fact {
                    match rel {
                        Rel::ParPar(f, t, r) => par_par.push((f.clone(), t.clone(), r.clone())),
                        Rel::GcGc(f, t, r) => gc_gc.push((f.clone(), t.clone(), r.clone())),
                        Rel::GcPar(f, t, r) => gc_par.push((f.clone(), t.clone(), r.clone())),
                        Rel::GcProp(f, t, r) => gc_prop.push((f.clone(), t.clone(), r.clone())),
                        Rel::PropPar(f, t, r) => prop_par.push((f.clone(), t.clone(), r.clone())),
                        Rel::ParProp(f, t, r) => par_prop.push((f.clone(), t.clone(), r.clone())),
                        Rel::GrGr(f, t, r) => gr_gr.push((f.clone(), t.clone(), r.clone())),
                        Rel::GrGc(f, t, r) => gr_gc.push((f.clone(), t.clone(), r.clone())),
                        Rel::LfLf(f, t, r) => lf_lf.push((f.clone(), t.clone(), r.clone())),
                        Rel::OpOp(f, t, r) => op_op.push((f.clone(), t.clone(), r.clone())),
                        Rel::PgcPgc(f, t, r) => pgc_pgc.push((f.clone(), t.clone(), r.clone())),
                        Rel::PropProp(f, t, r) => prop_prop.push((f.clone(), t.clone(), r.clone())),
                        Rel::ProbProb(f, t, r) => prob_prob.push((f.clone(), t.clone(), r.clone())),
                        Rel::ProbProp(f, t, r) => prob_prop.push((f.clone(), t.clone(), r.clone())),
                        Rel::ProbPar(f, t, r) => prob_par.push((f.clone(), t.clone(), r.clone())),
                    }
                }
            }
        }
    }

    write_arcs(relations_dir, "parameter_to_parameter", par_par)?;
    write_arcs(relations_dir, "graph_class_to_graph_class", gc_gc)?;
    write_arcs(relations_dir, "graph_class_to_parameter", gc_par)?;
    write_arcs(relations_dir, "graph_class_to_graph_class_property", gc_prop)?;
    write_arcs(relations_dir, "graph_class_property_to_parameter", prop_par)?;
    write_arcs(relations_dir, "parameter_to_graph_class_property", par_prop)?;
    write_arcs(relations_dir, "graph_to_graph", gr_gr)?;
    write_arcs(relations_dir, "graph_to_graph_class", gr_gc)?;
    write_arcs(relations_dir, "logic_fragment_to_logic_fragment", lf_lf)?;
    write_arcs(relations_dir, "operation_to_operation", op_op)?;
    write_arcs(relations_dir, "parametric_graph_class_to_parametric_graph_class", pgc_pgc)?;
    write_arcs(relations_dir, "graph_class_property_to_graph_class_property", prop_prop)?;
    write_arcs(relations_dir, "problem_to_problem", prob_prob)?;
    write_arcs(relations_dir, "problem_to_graph_class_property", prob_prop)?;
    write_arcs(relations_dir, "problem_to_parameter", prob_par)?;

    Ok(())
}
