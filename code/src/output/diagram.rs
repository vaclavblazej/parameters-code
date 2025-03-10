use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    process::Command,
};

use crate::file;
use crate::general::enums::{CpxInfo, CpxTime};
use crate::output::dot::{Edge, Graph};
use crate::{
    data::{
        core::{Data, Relation, Set},
        preview::PreviewSet,
    },
    general::{enums::SourcedCpxInfo, hide::filter_hidden},
    output::color::interpolate_colors,
    work::processing::bfs_limit_distance,
};
use std::{fs, time};

use super::{
    color::{relation_color, Color}, dot::SetColorCallback, markdown::Markdown
};

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

pub fn make_drawing(
    data: &Data,
    target_dir: &Path,
    name: &str,
    displayed_sets: &Vec<&Set>,
    color_fn: Option<Box<SetColorCallback>>,
) -> anyhow::Result<PathBuf> {
    let mut displayed_sets_preview: HashSet<PreviewSet> =
        displayed_sets.iter().map(|x| x.preview.clone()).collect();
    let mut remove_sets_preview: HashSet<PreviewSet> = HashSet::new();
    for relation in &data.relations {
        if displayed_sets_preview.contains(&relation.subset)
            && displayed_sets_preview.contains(&relation.superset)
        {
            if let SourcedCpxInfo::Equal { source: _ } = &relation.cpx {
                if relation.subset.relevance < relation.superset.relevance
                    || (relation.subset.relevance == relation.superset.relevance
                        && relation.subset.id < relation.superset.id)
                    {
                        remove_sets_preview.insert(relation.subset.clone());
                    }
            }
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
                && relation.preview.cpx.get_mx().is_some() {
                    potential_relations.push(relation.preview.clone())
                }
    }
    // hiding cannot be global as it is implied by the set of items shown in the drawing
    let drawn_relations = filter_hidden(
        potential_relations,
        &displayed_sets.iter().map(|x| x.preview.clone()).collect(),
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
    // let pdf_target_file = target_dir.join(format!("{}.pdf", name));
    // Command::new("dot").arg("-Tpdf").arg(&dot_target_file).arg("-o").arg(&pdf_target_file).output().expect("dot command failed");
    // assert!(pdf_target_file.exists());
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
            .get(&set.preview)
            .expect("error getting distances");
        let ratio = ((*dist as f32) / (max_dist as f32)).clamp(0.0, 1.0);
        interpolate_colors("#78acff", "#dddde8", ratio)
    })
}

fn mark_by_inclusions(origin_set: &Set) -> Box<SetColorCallback> {
    let aset = origin_set.related_sets.clone();
    let aid = origin_set.id.to_string();
    Box::new(move |bset: &Set| -> String {
        match relation_color(&aset, aid.clone(), &bset.preview) {
            Color::Gray => Color::Gray.hex(),
            color => color.light(),
        }
    })
}
