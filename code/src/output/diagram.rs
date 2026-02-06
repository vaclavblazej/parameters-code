use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time;

use crate::data::data::{Data, Parameter};
use crate::data::digraph::Edge;
use crate::data::enums::*;
use crate::data::id::{HasPreviewId, PreviewParameterId};
use crate::data::preview::{HasPreview, PreviewParameter};
use crate::data::score::has_better_score_than;
use crate::general::file;
use crate::input::source::Cpx;
use crate::output::color::{Color, interpolate_colors};
use crate::output::dot::{DotEdge, DotEdgeAttribute, DotGraph, SetColorCallback};
use crate::output::markdown::Markdown;

fn inclusion_edge_style(mx: &CpxTime) -> HashSet<DotEdgeAttribute> {
    let mut res: HashSet<DotEdgeAttribute> = HashSet::new();
    res.insert(DotEdgeAttribute::Uncategorized(
        "decorate=true lblstyle=\"above, sloped\"".into(),
    ));
    match mx {
        CpxTime::Constant => {
            res.insert(DotEdgeAttribute::Weight(100));
            res.insert(DotEdgeAttribute::PenWidth(300));
        }
        CpxTime::Linear => {
            res.insert(DotEdgeAttribute::Weight(100));
            res.insert(DotEdgeAttribute::PenWidth(200));
        }
        CpxTime::Polynomial => {
            res.insert(DotEdgeAttribute::Weight(20));
            res.insert(DotEdgeAttribute::PenWidth(7));
        }
        CpxTime::Exponential => {
            res.insert(DotEdgeAttribute::Style("dotted".into()));
            res.insert(DotEdgeAttribute::Weight(1));
            res.insert(DotEdgeAttribute::PenWidth(100));
        }
        CpxTime::Tower => {
            res.insert(DotEdgeAttribute::Style("dotted".into()));
            res.insert(DotEdgeAttribute::Weight(1));
            res.insert(DotEdgeAttribute::PenWidth(8));
        }
        CpxTime::Exists => {
            res.insert(DotEdgeAttribute::Color(Color::Gray));
            res.insert(DotEdgeAttribute::Weight(1));
        }
    };
    res
}

// todo this was replaced with save_to_file within dotgraph, but the filtering logic should be
// moved to main or auxiliary function to remove edges that are among vertices where both are not
// in the diagram
// pub fn make_drawing(target_dir: &Path, digraph: DotGraph) -> anyhow::Result<PathBuf> {
// let mut displayed_sets_preview: HashSet<PreviewParameter> =
//     displayed_sets.iter().map(|(k, x)| x.preview()).collect();
// let mut remove_sets_preview: HashSet<PreviewParameter> = HashSet::new();
// for (subset_id, superset_id, cpx) in relations {
//     if let Some(superset) = displayed_sets.get(superset_id)
//         && let Some(subset) = displayed_sets.get(subset_id)
//         && let Cpx::Equal = &cpx
//         && has_better_score_than(superset, subset)
//     {
//         remove_sets_preview.insert(subset.preview());
//     }
// }
// for r in remove_sets_preview {
//     displayed_sets_preview.remove(&r);
// }
// for displayed_set_preview in &displayed_sets_preview {
//     if let Some(set) = displayed_sets.get(&displayed_set_preview.id) {
//         digraph.add_vertex(set);
//     }
// }
// let mut potential_relations = Vec::new();
// for relation in &data.relations {
//     if displayed_sets_preview.contains(&relation.subset)
//         && displayed_sets_preview.contains(&relation.superset)
//         && relation.preview().cpx.get_mx().is_some()
//     {
//         potential_relations.push(relation.preview())
//     }
// }
// hiding cannot be global as it is implied by the set of items shown in the drawing
// let drawn_relations = filter_hidden(
//     potential_relations,
//     &displayed_sets.iter().map(|x| x.preview()).collect(),
// );
// for relation in drawn_relations {
//     if let Some(mx) = relation.cpx.get_mx() {
//         let style = inclusion_edge_style(&mx);
//         let drawedge = DotEdge {
//             from: relation.subset.id.to_string(),
//             to: relation.superset.id.to_string(),
//             data,
//         };
//         digraph.add_edge(drawedge);
//     }
// }
//     let dot_str = digraph.to_dot();
//     let dot_target_file = target_dir.join(format!("{}.dot", digraph.info.name));
//     file::write_file_content(&dot_target_file, &dot_str)?;
//     Ok(dot_target_file)
// }

// pub fn make_focus_drawing(
//     filename: &str,
//     data: &Data,
//     digraph: DotGraph,
//     set: &Parameter,
//     distance: usize,
//     target_dir: &Path,
// ) -> anyhow::Result<PathBuf> {
//     let set_distances = digraph.bfs_get_distance(set.id.to_string());
//     let mut score_visibility: HashMap<u32, usize> = HashMap::new();
//     score_visibility.insert(0, 0);
//     score_visibility.insert(1, 0);
//     score_visibility.insert(2, 0);
//     score_visibility.insert(3, 0);
//     score_visibility.insert(4, 1);
//     score_visibility.insert(5, 1);
//     score_visibility.insert(6, 1);
//     score_visibility.insert(7, 2);
//     score_visibility.insert(8, 3);
//     score_visibility.insert(9, 4);
//     let preview_sets_to_draw: Vec<PreviewParameter> = set_distances
//         .iter()
//         .filter(|(x, y)| x.typ == set.typ)
//         .filter(|(x, y)| {
//             let mut visibility = *score_visibility.get(&x.score).unwrap();
//             if set.related_sets.subsets.all.contains(x)
//                 || set.related_sets.supersets.all.contains(x)
//             {
//                 visibility += 1;
//             }
//             if set.related_sets.equivsets.contains(x) {
//                 visibility += 10;
//             }
//             visibility >= **y
//         })
//         .map(|(x, y)| x)
//         .cloned()
//         .collect();
//     let sets_to_draw = data.get_sets(preview_sets_to_draw);
//     make_drawing(
//         data,
//         target_dir,
//         filename,
//         &sets_to_draw,
//         Some(mark_by_distance(set_distances, 3)),
//     )
// }

// pub fn make_subset_drawing<T>(
//     filename: &str,
//     data: &Data,
//     set: &T,
//     sets_to_draw: Vec<&T>,
//     target_dir: &Path,
// ) -> anyhow::Result<PathBuf> {
//     make_drawing(
//         data,
//         target_dir,
//         filename,
//         &sets_to_draw,
//         Some(mark_by_inclusions(set)),
//     )
// }

// fn mark_by_distance(
//     distances: HashMap<PreviewParameter, usize>,
//     max_dist: usize,
// ) -> Box<SetColorCallback> {
//     Box::new(move |set: &PreviewParameter| -> String {
//         let dist = distances
//             .get(&set.preview())
//             .expect("error getting distances");
//         let ratio = ((*dist as f32) / (max_dist as f32)).clamp(0.0, 1.0);
//         interpolate_colors("#78acff", "#dddde8", ratio)
//     })
// }
//
// fn mark_by_inclusions<T, PreviewT>(origin_set: &T) -> Box<SetColorCallback> {
//     let aset = origin_set.related_sets.clone();
//     let aid = origin_set.id.to_string();
//     Box::new(move |bset: &T| -> String {
//         // todo use HasColor
//         match relation_color(&aset, aid.clone(), &bset.preview()) {
//             Color::Gray => Color::Gray.hex(),
//             color => color.light(),
//         }
//     })
// }
