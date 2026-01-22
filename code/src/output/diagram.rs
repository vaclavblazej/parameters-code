use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time;

use crate::data::data::Data;
use crate::data::enums::*;
use crate::data::preview::HasPreview;
use crate::file;
use crate::output::color::{Color, interpolate_colors, relation_color};
use crate::output::dot::{Edge, Graph, SetColorCallback};
use crate::output::markdown::Markdown;
use crate::work::hide::filter_hidden;

pub fn bfs_limit_distance(set: &Set, data: &Data, distance: usize) -> HashMap<PreviewSet, usize> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    visited.insert(set.preview(), 0);
    queue.push_back((set.preview(), 0));
    while let Some((raw_set, current_distance)) = queue.pop_front() {
        let set = data.get_set(&raw_set);
        if current_distance >= distance {
            continue;
        }
        for bigset in [
            &set.related_sets.equivsets,
            &set.related_sets.subsets.minimal,
            &set.related_sets.supersets.maximal,
        ] {
            for sset in bigset {
                if !visited.contains_key(sset) {
                    let new_distance = current_distance + 1;
                    visited.insert(sset.clone(), new_distance);
                    queue.push_back((sset.clone(), new_distance));
                }
            }
        }
    }
    visited
}

// todo move this processing to utilities for diagrams on diagram structures
pub fn order_sets_from_sources(data: &Data, sets: &Vec<PreviewSet>) -> Vec<PreviewSet> {
    let mut predecesors: HashMap<PreviewSet, usize> = HashMap::new();
    let mut equivalent: HashSet<PreviewSet> = HashSet::new();
    let sets_set: HashSet<PreviewSet> = HashSet::from_iter(sets.iter().cloned());
    for preview in sets {
        predecesors.insert(preview.clone(), 0);
    }
    for preview in sets {
        let set = data.get_set(preview);
        for subset in &set.related_sets.supersets.all {
            if let Some(el) = predecesors.get_mut(subset) {
                *el += 1;
            }
        }
    }
    let mut queue: Vec<PreviewSet> = Vec::new();
    let mut eqqueue: Vec<PreviewSet> = Vec::new();
    for (set, count) in &predecesors {
        if *count == 0 {
            queue.push(set.clone());
        }
    }
    let mut resolved: HashSet<PreviewSetId> = HashSet::new();
    let mut result = Vec::new();
    loop {
        let current = match eqqueue.pop() {
            Some(c) => c,
            None => match queue.pop() {
                Some(c) => c,
                None => break,
            },
        };
        if resolved.contains(&current.id) {
            continue;
        }
        resolved.insert(current.id.clone());
        result.push(current.clone());
        let set = data.get_set(&current);
        for elem in &set.related_sets.equivsets {
            if predecesors.contains_key(elem) {
                eqqueue.push(elem.clone());
            }
        }
        let children: Vec<&PreviewSet> = set
            .related_sets
            .supersets
            .all
            .iter()
            .filter(|x| x.typ == PreviewType::Parameter)
            .collect();
        for neighbor in children {
            if let Some(mut x) = predecesors.get_mut(neighbor) {
                *x -= 1;
                if *x == 0 {
                    queue.push(neighbor.clone());
                }
            }
        }
    }
    assert_eq!(resolved.len(), sets.len());
    result
}

fn inclusion_edge_style(mx: &CpxTime) -> String {
    let mut res: String = "decorate=true lblstyle=\"above, sloped\"".into();
    res += match mx {
        CpxTime::Constant => " weight=\"100\" penwidth=\"3.0\"",
        CpxTime::Linear => " weight=\"100\" penwidth=\"2.0\"",
        CpxTime::Polynomial => " weight=\"20\" penwidth=\"0.7\"",
        CpxTime::Exponential => " style=\"dotted\" weight=\"1\" penwidth=\"1.0\"",
        CpxTime::Tower => " style=\"dotted\" weight=\"1\" penwidth=\"0.8\"",
        CpxTime::Exists => " color=\"gray\" weight=\"1\"",
    };
    res
}

// impl From<&Set> for Node {
//     fn from(set: &Set) -> Node {
//         let attributes = "shape=box".into();
//         Node {
//             id: set.id.to_string(),
//             label: set.name.clone(),
//             color: "#dddddd".into(),
//             attributes,
//         }
//     }
// }
//
// impl From<&PreviewRelation> for Edge {
//     fn from(prev: &PreviewRelation) -> Edge {
//         let attributes = String::new();
//         // attributes.append() ... todo
//         Edge {
//             from: prev.subset.id.to_string(),
//             to: prev.superset.id.to_string(),
//             label: "O".to_string(),
//             attributes,
//             url: prev.id.to_string(),
//         }
//     }
// }

pub fn make_drawing(
    data: &Data,
    target_dir: &Path,
    name: &str,
    displayed_sets: &Vec<&Set>,
    color_fn: Option<Box<SetColorCallback>>,
) -> anyhow::Result<PathBuf> {
    let mut displayed_sets_preview: HashSet<PreviewSet> =
        displayed_sets.iter().map(|x| x.preview()).collect();
    let mut remove_sets_preview: HashSet<PreviewSet> = HashSet::new();
    for relation in &data.relations {
        if displayed_sets_preview.contains(&relation.subset)
            && displayed_sets_preview.contains(&relation.superset)
            && let SourcedCpxInfo::Equal { source: _ } = &relation.cpx
            && (relation.subset.relevance < relation.superset.relevance
                || (relation.subset.relevance == relation.superset.relevance
                    && relation.subset.id < relation.superset.id))
        {
            remove_sets_preview.insert(relation.subset.clone());
        }
    }
    for r in remove_sets_preview {
        displayed_sets_preview.remove(&r);
    }
    let mut graph = Graph::new(name, color_fn);
    for displayed_set_preview in &displayed_sets_preview {
        let set = data.get_set(displayed_set_preview);
        graph.add_node(set);
    }
    let mut potential_relations = Vec::new();
    for relation in &data.relations {
        if displayed_sets_preview.contains(&relation.subset)
            && displayed_sets_preview.contains(&relation.superset)
            && relation.preview().cpx.get_mx().is_some()
        {
            potential_relations.push(relation.preview())
        }
    }
    // hiding cannot be global as it is implied by the set of items shown in the drawing
    let drawn_relations = filter_hidden(
        potential_relations,
        &displayed_sets.iter().map(|x| x.preview()).collect(),
    );
    for relation in drawn_relations {
        if let Some(mx) = relation.cpx.get_mx() {
            let style = inclusion_edge_style(&mx);
            let drawedge = Edge {
                url: relation.id.to_string(),
                from: relation.subset.id.to_string(),
                to: relation.superset.id.to_string(),
                label: "".into(), // todo "o".into(),
                attributes: style,
            };
            graph.add_edge(drawedge);
        }
    }
    let dot_str = graph.to_dot();
    let dot_target_file = target_dir.join(format!("{}.dot", name));
    file::write_file_content(&dot_target_file, &dot_str)?;
    Ok(dot_target_file)
}

pub fn make_focus_drawing(
    filename: &str,
    data: &Data,
    set: &Set,
    distance: usize,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let set_distances = bfs_limit_distance(set, data, 20);
    let mut relevance_visibility: HashMap<u32, usize> = HashMap::new();
    relevance_visibility.insert(0, 0);
    relevance_visibility.insert(1, 0);
    relevance_visibility.insert(2, 0);
    relevance_visibility.insert(3, 0);
    relevance_visibility.insert(4, 1);
    relevance_visibility.insert(5, 1);
    relevance_visibility.insert(6, 1);
    relevance_visibility.insert(7, 2);
    relevance_visibility.insert(8, 3);
    relevance_visibility.insert(9, 4);
    let preview_sets_to_draw: Vec<PreviewSet> = set_distances
        .iter()
        .filter(|(x, y)| x.typ == set.typ)
        .filter(|(x, y)| {
            let mut visibility = *relevance_visibility.get(&x.relevance).unwrap();
            if set.related_sets.subsets.all.contains(x)
                || set.related_sets.supersets.all.contains(x)
            {
                visibility += 1;
            }
            if set.related_sets.equivsets.contains(x) {
                visibility += 10;
            }
            visibility >= **y
        })
        .map(|(x, y)| x)
        .cloned()
        .collect();
    let sets_to_draw = data.get_sets(preview_sets_to_draw);
    make_drawing(
        data,
        target_dir,
        filename,
        &sets_to_draw,
        Some(mark_by_distance(set_distances, 3)),
    )
}

pub fn make_subset_drawing(
    filename: &str,
    data: &Data,
    set: &Set,
    sets_to_draw: Vec<&Set>,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    make_drawing(
        data,
        target_dir,
        filename,
        &sets_to_draw,
        Some(mark_by_inclusions(set)),
    )
}

fn mark_by_distance(
    distances: HashMap<PreviewSet, usize>,
    max_dist: usize,
) -> Box<SetColorCallback> {
    Box::new(move |set: &Set| -> String {
        let dist = distances
            .get(&set.preview())
            .expect("error getting distances");
        let ratio = ((*dist as f32) / (max_dist as f32)).clamp(0.0, 1.0);
        interpolate_colors("#78acff", "#dddde8", ratio)
    })
}

fn mark_by_inclusions(origin_set: &Set) -> Box<SetColorCallback> {
    let aset = origin_set.related_sets.clone();
    let aid = origin_set.id.to_string();
    Box::new(move |bset: &Set| -> String {
        match relation_color(&aset, aid.clone(), &bset.preview()) {
            Color::Gray => Color::Gray.hex(),
            color => color.light(),
        }
    })
}
