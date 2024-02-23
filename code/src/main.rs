#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{PathBuf,Path};
use std::process::Command;

use anyhow::Result;
use data::Data;
use draw::Graph;
use markdown::Mappable;

use crate::markdown::Address;
use crate::markdown::Markdown;
use crate::pages::TargetPage;
use crate::pages::add_content;
use crate::simpleindex::SimpleIndex;
use crate::table::render_table;

mod build;
mod collection;
mod complexity;
mod data;
mod draw;
mod file;
mod markdown;
mod pages;
mod processing;
mod raw;
mod simpleindex;
mod table;

fn make_drawing(data: &Data, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    println!("generating dot pdf");
    let mut graph = Graph::new();
    for set in &data.parameters {
        graph.add_node(&data.get(set.raw.clone()).clone())
    }
    for above in &data.parameters {
        for below in &above.subsets.minimal {
            let attributes = "color=gray decorate=true lblstyle=\"above, sloped\" weight=1".into();
            let drawedge = draw::Edge{
                from: above.id.clone(),
                to: below.id.clone(),
                label: String::new(),
                attributes,
            };
            graph.add_edge(drawedge);
        }
    }
    let dot_str = graph.to_dot();
    let dot_target_file = target_dir.join("parameters.dot");
    file::write_file_content(&dot_target_file, &dot_str)?;
    let pdf_target_file = target_dir.join("parameters.pdf");
    Command::new("dot").arg("-Tpdf").arg(&dot_target_file).arg("-o").arg(&pdf_target_file).spawn()?;
    Ok(pdf_target_file)
}

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

fn generate_relation_table(data: &Data, parent: &Path) {
    println!("generating relation table");
    let table_folder = parent.join("scripts").join("table_tikz");
    render_table(&data.parameters, &table_folder).unwrap_or_else(|x|{
        println!("error producing relation table\n{}", x);
    });
}

fn main() -> Result<()> {
    println!("retrieving data collection");
    let rawdata = collection::build_collection();
    println!("processing data");
    let current = env::current_dir().unwrap();
    let parent = current.parent().unwrap();
    let handcrafted_dir = parent.join("handcrafted");
    let bibliography_file = handcrafted_dir.join("main.bib");
    let data = processing::process_raw_data(&rawdata, &bibliography_file);
    let final_dir = parent.join("web").join("content");
    let working_dir = current.join("target");
    println!("fetching generated pages");
    let markdown = Markdown::new(&data);
    let mut generated_pages = HashMap::new();
    add_content(&data.parameters, &final_dir, &mut generated_pages);
    add_content(&data.graph_classes, &final_dir, &mut generated_pages);
    add_content(&data.sources, &final_dir, &mut generated_pages);
    println!("fetching handcrafted pages");
    let mut handcrafted_pages: HashMap<PathBuf, PathBuf> = HashMap::new();
    for source in file::iterate_folder_recursively(&handcrafted_dir) {
        let relative = source.strip_prefix(&handcrafted_dir)?;
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
    println!("clearing the final directory");
    fs::remove_dir_all(&final_dir);
    fs::create_dir(&final_dir);
    // generate_pages(&pages, &markdown, &final_dir, &working_dir, &map);
    // generate_relation_table(&data, parent);
    make_drawing(&data, &current.join("target"));
    println!("done");
    Ok(())
}
