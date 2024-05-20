#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{PathBuf,Path};
use std::process::Command;

use anyhow::Result;
use data::data::Data;
use data::data::Relation;
use data::data::Set;
use general::enums::CpxInfo;
use general::enums::CpxTime;
use output::markdown::Mappable;

use crate::data::preview::PreviewKind;
use crate::data::preview::PreviewSet;
use crate::output::diagram::make_drawing;
use crate::output::markdown::Address;
use crate::output::markdown::Markdown;
use crate::output::pages::TargetPage;
use crate::output::pages::add_content;
use crate::data::simpleindex::SimpleIndex;
use crate::output::table::render_table;
use crate::processing::processing::process_raw_data;
use crate::general::file;

mod data {
    pub mod data;
    pub mod preview;
    pub mod simpleindex;
}
mod general {
    pub mod enums;
    pub mod file;
}
mod processing {
    pub mod combine;
    pub mod compare;
    pub mod convert;
    pub mod date;
    pub mod processing;
}
mod input {
    pub mod build;
    pub mod raw;
    pub mod source;
}
mod output {
    pub mod color;
    pub mod diagram;
    pub mod dot;
    pub mod markdown;
    pub mod pages;
    pub mod table;
    pub mod to_markdown;
}
mod collection;

fn generate_pages(pages: &Vec<TargetPage>, markdown: &Markdown,
                  final_dir: &PathBuf, working_dir: &PathBuf,
                  map: &HashMap<&str, Mappable>) -> anyhow::Result<()> {
    println!("generating pages");
    for page in pages {
        let content = match page.substitute {
            Some(substitute) => {
                // println!("generating {:?}", page.target);
                substitute.object.get_page(&markdown, &final_dir, &working_dir)
            },
            None => "[[handcrafted]]".into(),
        };
        let mut local_map = map.clone();
        let handcrafted_content = match page.source {
            Some(source) => {
                if source.as_os_str().to_str().unwrap().ends_with(".md") {
                    println!("copy & processing {:?}", source);
                    file::read_file_content(source)?
                } else {
                    println!("copy {:?}", page.target);
                    let target_folder = &page.target.parent().unwrap();
                    fs::create_dir_all(target_folder)?;
                    fs::copy(&source, &page.target)?;
                    continue;
                }
            },
            None => { "".into() },
        };
        local_map.insert("handcrafted", Mappable::String(handcrafted_content));
        if page.target.exists() { fs::remove_file(&page.target)?; }
        if let Some(parent) = page.target.parent() {
            fs::create_dir_all(parent)?;
        }
        let altered_content = substitute(&content, &markdown, &local_map);
        file::write_file_content(page.target, &altered_content)?;
    }
    Ok(())
}

fn substitute(content: &String, markdown: &Markdown, map: &HashMap<&str, Mappable>) -> String {
    let altered_content = content.lines().map(|line| {
        let mut strline = line.into();
        for _ in 1..10 {
            let newline = markdown.substitute_custom_markdown(&strline, map);
            if newline == strline {
                return newline;
            }
            strline = newline;
        }
        strline
    }).collect::<Vec<String>>().join("\n");
    altered_content
}

fn generate_relation_table(data: &Data, parent: &Path) -> anyhow::Result<PathBuf> {
    println!("generating relation table");
    let table_folder = parent.join("scripts").join("table_tikz");
    let table_file = render_table(&data, &table_folder).unwrap();
    Ok(table_file)
}

fn main() {
    println!("retrieving data collection");
    let rawdata = collection::build_collection();
    println!("processing data");
    let current = env::current_dir().unwrap();
    let parent = current.parent().unwrap();
    let handcrafted_dir = parent.join("handcrafted");
    let bibliography_file = handcrafted_dir.join("main.bib");
    let data = process_raw_data(&rawdata, &bibliography_file);
    let final_dir = parent.join("web").join("content");
    let working_dir = current.join("target");
    println!("creating main page pdfs");
    let parameters: Vec<&Set> = data.sets.iter().filter(|x|x.kind == PreviewKind::Parameter).collect();
    if let Ok(done_pdf) = make_drawing(&data, &current.join("target"), "parameters", &parameters, None){
        let final_pdf = final_dir.join("html").join("parameters.pdf");
        println!("copy the pdf to {:?}", &final_pdf);
        fs::copy(&done_pdf, &final_pdf);
    }
    let graphs: Vec<&Set> = data.sets.iter().filter(|x|x.kind == PreviewKind::GraphClass).collect();
    if let Ok(done_pdf) = make_drawing(&data, &current.join("target"), "graphs", &graphs, None){
        let final_pdf = final_dir.join("html").join("graphs.pdf");
        println!("copy the pdf to {:?}", &final_pdf);
        fs::copy(&done_pdf, &final_pdf);
    }
    println!("fetching generated pages");
    let markdown = Markdown::new(&data);
    let mut generated_pages = HashMap::new();
    add_content(&data.sets, &final_dir, &mut generated_pages);
    add_content(&data.sources, &final_dir, &mut generated_pages);
    println!("fetching handcrafted pages");
    let mut handcrafted_pages: HashMap<PathBuf, PathBuf> = HashMap::new();
    for source in file::iterate_folder_recursively(&handcrafted_dir) {
        let relative = source.strip_prefix(&handcrafted_dir).unwrap();
        let target_file = final_dir.join(relative);
        if source.is_file() {
            handcrafted_pages.insert(target_file.clone(), source.clone());
        } else if source.is_dir() {
            println!("directory {:?}", target_file);
        } else {
            println!("unprocessable file {:?}", target_file);
        }
    }
    println!("merging generated and handcrafted pages");
    let mut target_keys = HashSet::new();
    for (k, _) in &generated_pages { target_keys.insert(k); }
    for (k, _) in &handcrafted_pages { target_keys.insert(k); }
    let mut pages = vec![];
    for target in target_keys {
        let substitute = generated_pages.get(&target.clone());
        let source = handcrafted_pages.get(&target.clone());
        pages.push(TargetPage{ target, substitute, source });
    }
    println!("building general substitution map");
    let mut map: HashMap<&str, Mappable> = HashMap::new();
    // todo
    map.insert("test", Mappable::Address(Address{name: "qq".into(), url: "hello.com".into()}));
    // println!("clearing the final directory");
    // fs::remove_dir_all(&final_dir);
    // fs::create_dir(&final_dir);
    generate_pages(&pages, &markdown, &final_dir, &working_dir, &map);
    // if let Ok(done_pdf) = generate_relation_table(&data, parent) { // todo generalize
        // let final_pdf = final_dir.join("html").join("table.pdf");
        // println!("copy the pdf to {:?}", &final_pdf);
        // fs::copy(&done_pdf, &final_pdf);
    // }
    println!("done");
}
