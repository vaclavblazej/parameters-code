use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;

use crate::data::data::{Data, Set};
use crate::data::preview::{PreviewKind, PreviewSet};
use crate::general::enums::{CpxTime, CpxInfo::*};


fn table_format_par(i: usize, a: &PreviewSet) -> String {
    format!("\\parname{{{}}}{{{}}}{{../{}}}", i + 1, a.name, a.id)
}

fn table_format_link(ai: usize, bi: usize, status: &str, link: &str) -> String {
    format!("\\cpxlink{{{}}}{{{}}}{{{}}}{{../{}}}", ai, bi, status, link)
}

pub fn render_table(data: &Data, table_folder: &PathBuf) -> io::Result<PathBuf> {
    let draw_pars: Vec<&Set> = data.sets.iter().filter(|x|x.kind==PreviewKind::Parameter).collect();
    let size_str = format!("\\def\\parlen{{{}}}\n", draw_pars.len());

    let mut content = Vec::new();
    for (i, a) in draw_pars.iter().enumerate() {
        content.push(table_format_par(i, &a.preview));
    }

    for (ai, a) in draw_pars.iter().enumerate() {
        for (bi, b) in draw_pars.iter().enumerate() {
            let status = if a.id == b.id {
                "diagonal"
            } else {
                if let Some(relation) = data.get_relation(&a.preview, &b.preview) {
                    match &relation.cpx {
                        Inclusion { mx: _, mn: _ } | Equivalence => "bounded",
                        Exclusion => "unbounded",
                        _ => "unknown",
                    }
                    // todo bounded_derived and unbounded_derived
                } else {
                    "unknown"
                }
            };
            content.push(table_format_link(ai, bi, &status, "todo"));
        }
    }
    let template = File::open(table_folder.join("template.tex"))?;
    let template_reader = BufReader::new(template);
    let mut res = Vec::new();
    for line in template_reader.lines() {
        let line = line?;
        if line == "%SIZE" {
            res.push(size_str.clone());
        } else if line == "%CONTENT" {
            res.extend(content.iter().map(|x| format!("    {}", x)));
        } else {
            res.push(line);
        }
    }

    let mut file = File::create(table_folder.join("main.tex"))?;
    for line in res {
        writeln!(file, "{}", line)?;
    }

    std::env::set_current_dir(table_folder);
    let output = Command::new("pdflatex").arg("main.tex").output()?;
    if !output.status.success() {
        eprintln!("Error executing pdflatex command: {:?}", output.stderr);
    }

    Ok(table_folder.join("main.pdf"))
}
