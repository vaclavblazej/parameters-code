use std::{collections::HashSet, path::PathBuf, process::Command};

use crate::{data::{data::{Data, Relation, Set}, preview::PreviewSet}, processing::processing::bfs_limit_distance};
use crate::general::enums::{CpxInfo, CpxTime};
use crate::output::dot::{Edge, Graph};
use crate::file;

use super::markdown::Markdown;


fn relation_style(relation: &Relation) -> Option<String> {
    let mut res: String = "decorate=true lblstyle=\"above, sloped\"".into();
    res = match &relation.cpx {
        CpxInfo::Inclusion {mn, mx} => {
            res + match mx {
                CpxTime::Constant | CpxTime::Linear => &" weight=\"100\" penwidth=\"2.0\"",
                CpxTime::Polynomial => &" weight=\"20\" penwidth=\"0.8\"",
                CpxTime::Exponential => &" style=\"dotted\" weight=\"1\" penwidth=\"1.0\"",
                CpxTime::Tower(_) => &" style=\"dotted\" weight=\"1\" penwidth=\"0.8\"",
                CpxTime::Exists => &" color=\"gray\" weight=\"1\"",
            }
        },
        CpxInfo::Exclusion => return None,
        CpxInfo::Unknown => return None,
        CpxInfo::LowerBound { mn: _ } => "".into(),
        CpxInfo::Equivalence => res + "",
    };
    Some(res)
}

pub fn make_drawing(data: &Data, target_dir: &PathBuf, name: &str, displayed_sets: &Vec<&Set>) -> anyhow::Result<PathBuf> {
    // todo add coloring lambda function
    println!("generating a dot pdf {:?} {}", target_dir, name);
    let mut graph = Graph::new();
    for set in displayed_sets {
        graph.add_node(set);
    }
    let displayed_sets_preview: HashSet<PreviewSet> = displayed_sets.into_iter().map(|x|x.preview.clone()).collect();
    for relation in &data.relations {
        if displayed_sets_preview.contains(&relation.subset)
            && displayed_sets_preview.contains(&relation.superset) {
            if let Some(attributes) = relation_style(relation) {
                let drawedge = Edge {
                    url: relation.id.clone(),
                    from: relation.subset.id.clone(),
                    to: relation.superset.id.clone(),
                    label: "".into(), // "o".into(),
                    attributes,
                };
                graph.add_edge(drawedge);
            }
        }
    }
    let dot_str = graph.to_dot();
    let dot_target_file = target_dir.join(format!("{}.dot", name));
    file::write_file_content(&dot_target_file, &dot_str)?;
    let pdf_target_file = target_dir.join(format!("{}.pdf", name));
    Command::new("dot").arg("-Tpdf").arg(&dot_target_file).arg("-o").arg(&pdf_target_file).spawn()?;
    Ok(pdf_target_file)
}

pub fn make_focus_drawing(data: &Data, set: &Set, builder: &Markdown, distance: usize, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let pairs_to_draw = bfs_limit_distance(set, &builder.data, distance);
    let preview_sets_to_draw: Vec<PreviewSet> = pairs_to_draw.into_iter().map(|(a,b)|a.clone()).collect();
    let sets_to_draw = data.get_sets(preview_sets_to_draw);
    let filename = &format!("local_{}", set.id);
    make_drawing(data, target_dir, filename, &sets_to_draw)
}

