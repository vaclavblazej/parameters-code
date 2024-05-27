use std::{collections::{HashMap, HashSet}, path::PathBuf, process::Command};

use crate::{data::{data::{Data, Relation, Set}, preview::PreviewSet}, general::hide::filter_hidden, output::color::interpolate_colors, processing::processing::bfs_limit_distance};
use crate::general::enums::{CpxInfo, CpxTime};
use crate::output::dot::{Edge, Graph};
use crate::file;
use std::{thread, time, fs};

use super::markdown::Markdown;


fn inclusion_edge_style(mx: &CpxTime) -> String {
    let mut res: String = "decorate=true lblstyle=\"above, sloped\"".into();
    res = res + match mx {
        CpxTime::Constant => &" weight=\"100\" penwidth=\"3.0\"",
        CpxTime::Linear => &" weight=\"100\" penwidth=\"2.0\"",
        CpxTime::Polynomial => &" weight=\"20\" penwidth=\"0.7\"",
        CpxTime::Exponential => &" style=\"dotted\" weight=\"1\" penwidth=\"1.0\"",
        CpxTime::Tower(_) => &" style=\"dotted\" weight=\"1\" penwidth=\"0.8\"",
        CpxTime::Exists => &" color=\"gray\" weight=\"1\"",
    };
    res
}

pub fn make_drawing(data: &Data, target_dir: &PathBuf, name: &str, displayed_sets: &Vec<&Set>, color_fn: Option<Box<dyn Fn(&Set) -> String>>) -> anyhow::Result<PathBuf> {
    println!("generating a dot pdf {:?} {}", target_dir, name);
    let mut displayed_sets_preview: HashSet<PreviewSet> = displayed_sets.into_iter().map(|x|x.preview.clone()).collect();
    let mut remove_sets_preview: HashSet<PreviewSet> = HashSet::new();
    for relation in &data.relations {
        if displayed_sets_preview.contains(&relation.subset) && displayed_sets_preview.contains(&relation.superset) {
            match &relation.cpx {
                CpxInfo::Equivalence => {
                    if relation.subset.popularity < relation.superset.popularity {
                        remove_sets_preview.insert(relation.subset.clone());
                    }
                },
                _ => {},
            }
        }
    }
    for r in remove_sets_preview {
        displayed_sets_preview.remove(&r);
    }
    let mut graph = Graph::new(color_fn);
    for displayed_set_preview in &displayed_sets_preview {
        let set = data.get_set(&displayed_set_preview);
        graph.add_node(set);
    }
    let mut potential_relations = Vec::new();
    for relation in &data.relations {
        if displayed_sets_preview.contains(&relation.subset) && displayed_sets_preview.contains(&relation.superset) {
            match &relation.cpx {
                CpxInfo::Inclusion { mn: _, mx: _ } => {
                    potential_relations.push(relation.preview.clone())
                },
                _ => {},
            }
        }
    }
    // hiding cannot be global as it is implied by the set of items shown in the drawing
    let mut drawn_relations = filter_hidden(potential_relations, &displayed_sets.iter().map(|x|x.preview.clone()).collect());
    for relation in drawn_relations {
        if let CpxInfo::Inclusion { mn, mx } = &relation.cpx {
            let style = inclusion_edge_style(&mx);
            let drawedge = Edge {
                url: relation.id.clone(),
                from: relation.subset.id.clone(),
                to: relation.superset.id.clone(),
                label: "".into(), // todo "o".into(),
                attributes: style,
            };
            graph.add_edge(drawedge);
        }
    }
    let dot_str = graph.to_dot();
    let dot_target_file = target_dir.join(format!("{}.dot", name));
    fs::create_dir_all(&target_dir);
    file::write_file_content(&dot_target_file, &dot_str)?;
    let pdf_target_file = target_dir.join(format!("{}.pdf", name));
    Command::new("dot").arg("-Tpdf").arg(&dot_target_file).arg("-o").arg(&pdf_target_file).output().expect("dot command failed");
    assert!(pdf_target_file.exists());
    Ok(pdf_target_file)
}

pub fn make_focus_drawing(data: &Data, set: &Set, distance: usize, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let set_distance_to_draw = bfs_limit_distance(set, &data, distance);
    let preview_sets_to_draw: Vec<PreviewSet> = set_distance_to_draw.iter().map(|(a,_)|a.clone()).filter(|x|x.kind == set.kind).collect();
    let sets_to_draw = data.get_sets(preview_sets_to_draw);
    let filename = &format!("local_{}", set.id);
    make_drawing(data, target_dir, filename, &sets_to_draw, Some(mark_by_distance(set_distance_to_draw, distance)))
}

pub fn make_subset_drawing(data: &Data, set: &Set, sets_to_draw: Vec<&Set>, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let filename = &format!("inclusions_{}", set.id);
    match set.kind { // todo polish this hacky solution
        crate::data::preview::PreviewKind::Parameter => {
            make_drawing(data, target_dir, filename, &sets_to_draw, Some(mark_by_subset(set)))
        },
        crate::data::preview::PreviewKind::GraphClass => {
            make_drawing(data, target_dir, filename, &sets_to_draw, Some(mark_by_superset(set)))
        },
    }
}

fn mark_by_distance(distances: HashMap<PreviewSet, usize>, max_dist: usize) -> Box<dyn Fn(&Set) -> String> {
    Box::new(move |set: &Set| -> String {
        let dist = distances.get(&set.preview).expect("error getting distances");
        let ratio = (*dist as f32) / (max_dist as f32);
        assert!(ratio >= 0.0 && ratio <= 1.0);
        interpolate_colors("#78acff", "#dddde8", ratio)
    })
}

fn mark_by_subset(origin_set: &Set) -> Box<dyn Fn(&Set) -> String> {
    let oset_copy = origin_set.clone();
    Box::new(move |set: &Set| -> String {
        if oset_copy.subsets.all.contains(&set.preview) {
            "#bbffbb".into()
        } else if oset_copy.sub_exclusions.all.contains(&set.preview) {
            "#ffbbbb".into()
        } else {
            "#dddddd".into()
        }
    })
}

fn mark_by_superset(origin_set: &Set) -> Box<dyn Fn(&Set) -> String> {
    let oset_copy = origin_set.clone();
    Box::new(move |set: &Set| -> String {
        if oset_copy.supersets.all.contains(&set.preview) {
            "#bbffbb".into()
        } else if oset_copy.super_exclusions.all.contains(&set.preview) {
            "#ffbbbb".into()
        } else {
            "#dddddd".into()
        }
    })
}

