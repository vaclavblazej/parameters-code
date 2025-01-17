use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;

use crate::data::data::{Data, Set};
use crate::data::preview::{PreviewType, PreviewSet};
use crate::general::enums::{CpxTime, CpxInfo::*};

use super::color::{relation_color, Color};


fn table_format_par(i: usize, a: &PreviewSet) -> String {
    format!("\\parname{{{}}}{{{}}}{{../{}}}", i + 1, a.name, a.id)
}

fn table_format_link(ai: usize, bi: usize, status: &str, link: &str) -> String {
    format!("\\cpxlink{{{}}}{{{}}}{{{}}}{{../{}}}", ai, bi, status, link)
}

fn order_sets_from_sources(data: &Data, sets: &Vec<PreviewSet>) -> Vec<PreviewSet> {
    let mut predecesors: HashMap<PreviewSet, usize> = HashMap::new();
    let mut equivalent: HashSet<PreviewSet> = HashSet::new();
    let sets_set: HashSet<PreviewSet> = HashSet::from_iter(sets.iter().cloned());
    for preview in sets {
        predecesors.insert(preview.clone(), 0);
    }
    for preview in sets {
        let set = data.get_set(&preview);
        for subset in &set.supersets.all {
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
    let mut resolved: HashSet<String> = HashSet::new();
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
        for elem in &set.equivsets {
            if predecesors.contains_key(elem) {
                eqqueue.push(elem.clone());
            }
        }
        let children: Vec<&PreviewSet> = set.supersets.all.iter().filter(|x|x.typ == PreviewType::Parameter).collect();
        for neighbor in children {
            if predecesors.contains_key(neighbor){
                *predecesors.get_mut(neighbor).unwrap() -= 1;
                if predecesors[&neighbor] == 0 {
                    queue.push(neighbor.clone());
                }
            }
        }
    }
    assert_eq!(resolved.len(), sets.len());
    result
}

pub fn render_table(data: &Data, draw_sets: &Vec<PreviewSet>, table_folder: &PathBuf) -> io::Result<PathBuf> {
    let size_str = format!("\\def\\parlen{{{}}}\n", draw_sets.len());
    let ordered_pars = order_sets_from_sources(data, draw_sets);

    let mut content = Vec::new();
    for (i, a) in ordered_pars.iter().enumerate() {
        content.push(table_format_par(i, &a));
    }

    for (ai, a) in ordered_pars.iter().enumerate() {
        for (bi, b) in ordered_pars.iter().enumerate() {
            let sa = data.get_set(a);
            let sb = data.get_set(b);
            let color = relation_color(sa, sb);
            content.push(table_format_link(ai, bi, &color.name(), "todo"));
        }
    }
    let template = File::open(table_folder.join("template.tex"))?;
    let template_reader = BufReader::new(template);
    let mut res = Vec::new();
    for line in template_reader.lines() {
        let line = line?;
        if line == "%COLORS" {
            for color in Color::list() {
                res.push(format!("\\tikzset{{{}/.style={{fill={}}}}}", color.name(), color.tikz()));
            }
        } else if line == "%SIZE" {
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
