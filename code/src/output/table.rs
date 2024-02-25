use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;

use crate::complexity::time::CpxTime;
use crate::data::data::{Data, Set};
use crate::input::raw::{Id, RawRelation, RawSet};
use crate::data::simpleindex::SimpleIndex;
use crate::complexity::info::CpxInfo::*;


fn table_format_par(i: usize, a: &RawSet) -> String {
    format!("\\parname{{{}}}{{{}}}{{../{}}}", i + 1, a.name, a.id)
}

fn table_format_link(ai: usize, bi: usize, status: &str, link: &str) -> String {
    format!("\\cpxlink{{{}}}{{{}}}{{{}}}{{../{}}}", ai, bi, status, link)
}

pub fn render_table(parameters: &Vec<Set>, table_folder: &PathBuf) -> io::Result<()> {
    let draw_pars = parameters;
    let size_str = format!("\\def\\parlen{{{}}}\n", draw_pars.len());

    let mut content = Vec::new();
    for (i, a) in draw_pars.iter().enumerate() {
        content.push(table_format_par(i, &a.raw));
    }

    for (ai, a) in draw_pars.iter().enumerate() {
        for (bi, b) in draw_pars.iter().enumerate() {
            // let bound = simpleindex.get_relation(&a, &b);
            let status = if a.id == b.id {
                "diagonal"
            } else {
                if a.subsets.maximal.contains(&b.raw) {
                    "bounded"
                } else if a.subsets.all.contains(&b.raw) {
                    "bounded_derived"
                }else if a.sub_exclusions.maximal.contains(&b.raw) {
                    "unbounded"
                }else if a.sub_exclusions.all.contains(&b.raw) {
                    "unbounded_derived"
                }else{
                    "unknown"
                }
                // match bound.cpx {
                    // Inclusion { mn: _, mx: _ } => "bounded",
                    // Exclusion => "unbounded",
                    // Equivalence => "bounded",
                    // LowerBound { mn: _ } => "unknown",
                    // Unknown => "unknown",
                // }
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

    Ok(())
}
