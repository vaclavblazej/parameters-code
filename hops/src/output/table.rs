use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;

use log::{error, info};

use crate::cli::paths::Paths;
use crate::data::data::Data;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::Linkable;
use crate::data::preview::PreviewParameter;
use crate::general::file;
use crate::general::worker::{Task, Worker};
use crate::output::color::Color;

pub struct TableEntry {
    pub id: String,
    pub name: String,
}

fn table_format_par(i: usize, a: &TableEntry) -> String {
    format!("\\parname{{{}}}{{{}}}{{../{}}}", i + 1, a.name, a.id)
}

fn table_format_link(ai: usize, bi: usize, status: &str, link: &str) -> String {
    format!("\\cpxlink{{{}}}{{{}}}{{{}}}{{../{}}}", ai, bi, status, link)
}

// pub fn render_table(
//     set_info: &HashMap<TableEntry, RelatedSets>,
//     ordered_draw_sets: &[TableEntry],
//     table_folder: &PathBuf,
// ) -> anyhow::Result<PathBuf> {
//     let size_str = format!("\\def\\parlen{{{}}}\n", ordered_draw_sets.len());
//     let mut content = Vec::new();
//     for (i, a) in ordered_draw_sets.iter().enumerate() {
//         content.push(table_format_par(i, a));
//     }
//     for (ai, a) in ordered_draw_sets.iter().enumerate() {
//         for (bi, b) in ordered_draw_sets.iter().enumerate() {
//             let a_related = set_info.get(a).unwrap();
//             let color = relation_color(a_related, a.id.to_string(), b);
//             content.push(table_format_link(ai, bi, &color.name(), "todo"));
//         }
//     }
//     let template_string = file::read_file_content(&table_folder.join("template.tex"))?;
//     // let template = File::open(table_folder.join("template.tex"))?;
//     // let template_reader = BufReader::new(template);
//     let mut res = Vec::new();
//     for line in template_string.lines() {
//         let line = String::from(line);
//         if line == "%COLORS" {
//             for color in Color::list() {
//                 res.push(format!(
//                     "\\tikzset{{{}/.style={{fill={}}}}}",
//                     color.name(),
//                     color.tikz()
//                 ));
//             }
//         } else if line == "%SIZE" {
//             res.push(size_str.clone());
//         } else if line == "%CONTENT" {
//             res.extend(content.iter().map(|x| format!("    {}", x)));
//         } else {
//             res.push(line);
//         }
//     }
//
//     file::write_file_content(&table_folder.join("main.tex"), &res.join("\n"));
//
//     std::env::set_current_dir(table_folder);
//     let output = Command::new("pdflatex").arg("main.tex").output()?;
//     if !output.status.success() {
//         error!("Error executing pdflatex command: {:?}", output.stderr);
//     }
//
//     Ok(table_folder.join("main.pdf"))
// }
//
// pub struct CreateTable {
//     pub related_sets_map: HashMap<TableEntry, RelatedSets>,
//     pub ordered_draw_sets: Vec<TableEntry>,
//     pub paths: Box<Paths>,
//     pub name: String,
// }
//
// impl Task for CreateTable {
//     fn process(&self) -> anyhow::Result<()> {
//         let CreateTable {
//             related_sets_map,
//             ordered_draw_sets,
//             paths,
//             name,
//         } = self;
//         let tmp_folder = &paths
//             .table_tikz_folder
//             .parent()
//             .unwrap_or_else(|| panic!("the used path is not expected to be the root"))
//             .join(format!("thread_tmp_{}", name));
//         file::copy_folder(&paths.table_tikz_folder, tmp_folder);
//         let done_pdf = render_table(related_sets_map, ordered_draw_sets, tmp_folder)?;
//         let final_pdf = paths.html_dir.join(format!("{}.pdf", name));
//         info!("copy the pdf to {:?}", &final_pdf);
//         file::copy_file(&done_pdf, &final_pdf);
//         Ok(())
//     }
// }

pub fn generate_relation_table<T>(
    data: &Data,
    draw_sets: &[T],
    paths: &Paths,
    name: &str,
    worker: &Worker,
) where
    T: Linkable,
{
    // todo
    // let ordered_draw_sets = order_sets_from_sources(data, draw_sets);
    // let mut related_sets_map: HashMap<PreviewSet, RelatedSets> = HashMap::new();
    // for set in &ordered_draw_sets {
    //     related_sets_map.insert(set.clone(), data.get_set(set).related_sets.clone());
    // }
    // worker.send(CreateTable {
    //     related_sets_map: related_sets_map.clone(),
    //     ordered_draw_sets: ordered_draw_sets.clone(),
    //     paths: Box::new(paths.clone()),
    //     name: name.into(),
    // });
}

// // todo move this processing to utilities for diagrams on diagram structures
// pub fn order_sets_from_sources(
//     entities: &Vec<PreviewParameter>,
//     successors: &HashSet<PreviewParameterId, Vec<PreviewParameterId>>,
// ) -> Vec<PreviewParameterId> {
//     let mut predecesor_count: HashMap<PreviewParameterId, usize> = HashMap::new();
//     for preview in entities {
//         predecesor_count.insert(preview.previewid(), 0);
//     }
//     for preview in entities {
//         let set: &Vec<PreviewParameterId> = &successors.get(&preview.previewid()).unwrap();
//         for subset in set {
//             if let Some(el) = predecesor_count.get_mut(subset) {
//                 *el += 1;
//             }
//         }
//     }
//     let mut queue: Vec<PreviewParameterId> = Vec::new();
//     let mut eqqueue: Vec<PreviewParameterId> = Vec::new();
//     for (set, count) in &predecesor_count {
//         if *count == 0 {
//             queue.push(set.clone());
//         }
//     }
//     let mut resolved: HashSet<PreviewParameterId> = HashSet::new();
//     let mut result: Vec<PreviewParameterId> = Vec::new();
//     loop {
//         let current_id = match eqqueue.pop() {
//             Some(c) => c,
//             None => match queue.pop() {
//                 Some(c) => c,
//                 None => break,
//             },
//         };
//         if resolved.contains(&current_id) {
//             continue;
//         }
//         resolved.insert(current_id.clone());
//         result.push(current_id.clone());
//         let set = successors.get(&current_id);
//         for elem in &set.related_sets.equivsets {
//             if predecesor_count.contains_key(elem) {
//                 eqqueue.push(elem.clone());
//             }
//         }
//         let children: Vec<&PreviewParameterId> = set.related_sets.supersets.all.iter().collect();
//         for neighbor in children {
//             if let Some(mut x) = predecesor_count.get_mut(neighbor) {
//                 *x -= 1;
//                 if *x == 0 {
//                     queue.push(neighbor.clone());
//                 }
//             }
//         }
//     }
//     assert_eq!(resolved.len(), entities.len());
//     result
// }
