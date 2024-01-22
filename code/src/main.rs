use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use markdown::Mappable;

use crate::markdown::Address;
use crate::markdown::Markdown;
use crate::pages::TargetPage;
use crate::pages::add_content;

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

fn main() -> Result<()> {
    println!("retrieving data collection");
    let rawdata = collection::build_collection();
    println!("processing data");
    let current = env::current_dir().unwrap();
    let parent = current.parent().unwrap();
    let handcrafted_dir = parent.join("handcrafted");
    let data = processing::process_raw_data(&rawdata, &handcrafted_dir);
    println!("listing generated pages");
    let final_dir = parent.join("web").join("content");
    let markdown = Markdown::new(&data);
    let mut generated_pages = HashMap::new();
    add_content(&data.parameters, &final_dir, &mut generated_pages);
    add_content(&data.graph_classes, &final_dir, &mut generated_pages);
    add_content(&data.sources, &final_dir, &mut generated_pages);
    println!("fetching the handcrafted pages");
    let mut handcrafted_pages: HashMap<PathBuf, PathBuf> = HashMap::new();
    for source in file::iterate_folder(&handcrafted_dir) {
        let relative = source.strip_prefix(&handcrafted_dir)?;
        let target_file = final_dir.join(relative);
        if source.is_file() {
            handcrafted_pages.insert(target_file.clone(), source.clone());
        } else if source.is_dir() {
            println!("dircetory {:?}", target_file);
        } else {
            println!("unprocessable file {:?}", target_file);
        }
    }
    println!("merging pages");
    let mut target_keys = HashSet::new();
    for (k, _) in &generated_pages { target_keys.insert(k); }
    for (k, _) in &handcrafted_pages { target_keys.insert(k); }
    let mut pages = vec![];
    for target in &target_keys {
        let substitute = generated_pages.get(target.clone());
        let source = handcrafted_pages.get(target.clone());
        pages.push(TargetPage{ target, substitute, source });
    }
    println!("building general substitution map");
    let mut map: HashMap<&str, Mappable> = HashMap::new();
    map.insert("test", Mappable::Address(Address{name: "qq".into(), url: "hello.com".into()}));
    println!("generating pages");
    for page in pages {
        let content = match page.substitute {
            Some(substitute) => {
                println!("generating {:?}", page.target);
                substitute.object.get_page(&markdown)
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
                    fs::copy(&source, &page.target)?;
                    continue;
                }
            },
            None => { "missing ...".into() },
        };
        local_map.insert("handcrafted", Mappable::String(handcrafted_content));
        if page.target.exists() { fs::remove_file(&page.target)?; }
        if let Some(parent) = page.target.parent() {
            fs::create_dir_all(parent)?;
        }
        let altered_content = substitute(&content, &markdown, &local_map);
        file::write_file_content(page.target, &altered_content)?;
    }
    println!("done");
    Ok(())
}
