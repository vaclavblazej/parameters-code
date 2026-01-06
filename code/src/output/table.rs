use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;

use log::{error, info};

use crate::data::data::Data;
use crate::data::enums::*;
use crate::data::id::*;
use crate::file;
use crate::general::worker::Task;
use crate::output::color::{Color, relation_color};
use crate::work::sets::RelatedSets;

fn table_format_par(i: usize, a: &PreviewSet) -> String {
    format!("\\parname{{{}}}{{{}}}{{../{}}}", i + 1, a.name, a.id)
}

fn table_format_link(ai: usize, bi: usize, status: &str, link: &str) -> String {
    format!("\\cpxlink{{{}}}{{{}}}{{{}}}{{../{}}}", ai, bi, status, link)
}

pub fn render_table(
    set_info: &HashMap<PreviewSet, RelatedSets>,
    ordered_draw_sets: &[PreviewSet],
    table_folder: &PathBuf,
) -> anyhow::Result<PathBuf> {
    let size_str = format!("\\def\\parlen{{{}}}\n", ordered_draw_sets.len());
    let mut content = Vec::new();
    for (i, a) in ordered_draw_sets.iter().enumerate() {
        content.push(table_format_par(i, a));
    }
    for (ai, a) in ordered_draw_sets.iter().enumerate() {
        for (bi, b) in ordered_draw_sets.iter().enumerate() {
            let a_related = set_info.get(a).unwrap();
            let color = relation_color(a_related, a.id.to_string(), b);
            content.push(table_format_link(ai, bi, &color.name(), "todo"));
        }
    }
    let template_string = file::read_file_content(&table_folder.join("template.tex"))?;
    // let template = File::open(table_folder.join("template.tex"))?;
    // let template_reader = BufReader::new(template);
    let mut res = Vec::new();
    for line in template_string.lines() {
        let line = String::from(line);
        if line == "%COLORS" {
            for color in Color::list() {
                res.push(format!(
                    "\\tikzset{{{}/.style={{fill={}}}}}",
                    color.name(),
                    color.tikz()
                ));
            }
        } else if line == "%SIZE" {
            res.push(size_str.clone());
        } else if line == "%CONTENT" {
            res.extend(content.iter().map(|x| format!("    {}", x)));
        } else {
            res.push(line);
        }
    }

    file::write_file_content(&table_folder.join("main.tex"), &res.join("\n"));

    std::env::set_current_dir(table_folder);
    let output = Command::new("pdflatex").arg("main.tex").output()?;
    if !output.status.success() {
        error!("Error executing pdflatex command: {:?}", output.stderr);
    }

    Ok(table_folder.join("main.pdf"))
}

pub struct CreateTable {
    pub related_sets_map: HashMap<PreviewSet, RelatedSets>,
    pub ordered_draw_sets: Vec<PreviewSet>,
    pub paths: Box<Paths>,
    pub name: String,
}

impl Task for CreateTable {
    fn process(self) -> anyhow::Result<()> {
        let CreateTable {
            related_sets_map,
            ordered_draw_sets,
            paths,
            name,
        } = self;
        let tmp_folder = &paths
            .table_tikz_folder
            .parent()
            .unwrap_or_else(|| panic!("the used path is not expected to be the root"))
            .join(format!("thread_tmp_{}", name));
        file::copy_folder(&paths.table_tikz_folder, tmp_folder);
        let done_pdf = render_table(related_sets_map, ordered_draw_sets, tmp_folder)?;
        let final_pdf = paths.html_dir.join(format!("{}.pdf", name));
        info!("copy the pdf to {:?}", &final_pdf);
        file::copy_file(&done_pdf, &final_pdf);
        Ok(())
    }
}
