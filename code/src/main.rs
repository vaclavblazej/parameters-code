#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{PathBuf,Path};
use std::process::Command;
use std::time::Instant;

use anyhow::Result;
use biblatex::Bibliography;
use data::data::Linkable;
use general::cache::Cache;
use general::progress::ProgressDisplay;
use processing::bibliography::load_bibliography;
use rayon::prelude::*;
use data::data::Data;
use data::data::Relation;
use data::data::Set;
use general::enums::CpxInfo;
use general::enums::CpxTime;
use output::markdown::Mappable;

use crate::data::preview::PreviewType;
use crate::data::preview::PreviewSet;
use crate::output::diagram::make_drawing;
use crate::output::markdown::Address;
use crate::output::markdown::Markdown;
use crate::output::pages;
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
    pub mod cache;
    pub mod enums;
    pub mod file;
    pub mod hide;
    pub mod progress;
}
mod processing {
    pub mod bibliography;
    pub mod combine;
    pub mod compare;
    pub mod convert;
    pub mod date;
    pub mod processing;
}
mod input {
    pub mod build;
    pub mod raw;
    pub mod set;
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
mod test {
    pub mod collection;
    pub mod all;
}
mod collection;

fn build_page(page: &TargetPage,
              markdown: &Markdown,
              final_dir: &PathBuf,
              working_dir: &PathBuf,
              map: &HashMap<&str, Mappable>) -> anyhow::Result<()> {
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
                file::read_file_content(source)?
            } else {
                let target_folder = &page.target.parent().unwrap();
                fs::create_dir_all(target_folder)?;
                fs::copy(&source, &page.target)?;
                return Ok(());
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
    Ok(())
}

fn generate_pages(pages: &Vec<TargetPage>,
                  markdown: &Markdown,
                  final_dir: &PathBuf,
                  working_dir: &PathBuf,
                  map: &HashMap<&str, Mappable>) -> anyhow::Result<()> {
    let mut progress = ProgressDisplay::new("generating pages", pages.len() as u32);
    // todo par_iter ?
    let res: Result<Vec<()>> = pages.iter().map(|page| -> anyhow::Result<()>{
        progress.increase(1);
        build_page(page, markdown, final_dir, working_dir, map)
    }).collect();
    progress.done();
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

fn generate_relation_table(data: &Data, draw_sets: &Vec<PreviewSet>,  parent: &Path) -> anyhow::Result<PathBuf> {
    let table_folder = parent.join("scripts").join("table_tikz");
    let table_file = render_table(&data, draw_sets, &table_folder).unwrap();
    Ok(table_file)
}

struct Timer {
    instant: Instant,
}

impl Timer {
    fn new() -> Self {
        Self{ instant: Instant::now(), }
    }

    pub fn print(&self, message: &str) {
        println!("{:?} {}", self.instant.elapsed(), message);
    }
}

#[derive(Hash, PartialEq, Eq)]
enum ComputationPhases {
    PREPROCESS,
    DOTS,
    PAGES,
    TABLE,
    MOCK,
}

struct Computation {
    args: HashSet<ComputationPhases>,
    time: Timer,
    parent: PathBuf,
    handcrafted_dir: PathBuf,
    bibliography_file: PathBuf,
    bibliography: Option<Bibliography>,
    final_dir: PathBuf,
    working_dir: PathBuf,
    html_dir: PathBuf,
    tmp_dir: PathBuf,
    hide_irrelevant_parameters_below: u32,
    simplified_hide_irrelevant_parameters_below: u32,
    some_data: Option<Data>,
}

impl Computation {

    fn new() -> Self {
        let rawargs: Vec<String> = env::args().collect();
        let mut args = HashSet::new();
        for (i, arg) in rawargs.iter().enumerate() {
            if i == 0 { continue; }
            match arg.as_str() {
                "preprocess" => {args.insert(ComputationPhases::PREPROCESS);},
                "dots" => {args.insert(ComputationPhases::DOTS);},
                "pages" => {args.insert(ComputationPhases::PAGES);},
                "table" => {args.insert(ComputationPhases::TABLE);},
                "mock" => {args.insert(ComputationPhases::MOCK);},
                "all" => {
                    args.insert(ComputationPhases::PREPROCESS);
                    args.insert(ComputationPhases::DOTS);
                    args.insert(ComputationPhases::PAGES);
                    args.insert(ComputationPhases::TABLE);
                },
                other => eprintln!("unknown parameter: '{}'", other),
            }
        }
        let current = env::current_dir().unwrap();
        let parent = current.parent().unwrap();
        let handcrafted_dir = parent.join("handcrafted");
        let bibliography_file = handcrafted_dir.join("main.bib");
        let final_dir = parent.join("web").join("content");
        let working_dir = current.join("target");
        let html_dir = final_dir.join("html");
        let tmp_dir = current.join("tmp");
        Self {
            args,
            time: Timer::new(),
            parent: parent.to_path_buf(),
            handcrafted_dir,
            bibliography_file,
            bibliography: None,
            final_dir,
            working_dir,
            html_dir,
            tmp_dir,
            hide_irrelevant_parameters_below: 1,
            simplified_hide_irrelevant_parameters_below: 5,
            some_data: None,
        }
    }

    fn get_data(&self) -> &Data {
        match self.some_data {
            Some(ref data) => &data,
            None => panic!("unwrap empty data"),
        }
    }

    fn retrieve_and_process_data(&mut self) {
        let mock = self.args.contains(&ComputationPhases::MOCK);
        self.bibliography = load_bibliography(&self.bibliography_file);
        let cch: Cache<Data> = Cache::new(&self.tmp_dir.join("data.json"));
        if !mock && !self.args.contains(&ComputationPhases::PREPROCESS) {
            if let Some(mut res) = cch.load(){
                println!("deserialized data");
                res.recompute();
                self.some_data = Some(res);
                return;
            }
        }
        self.time.print("retrieving data collection");
        let rawdata = match mock {
            false => collection::build_collection(),
            true => test::collection::build_collection(),
        };
        self.time.print("processing data");
        let res = process_raw_data(&rawdata, &self.bibliography);
        if !mock {
            match cch.save(&res){
                Ok(()) => {},
                Err(err) => println!("{:?}", err),
            }
        }
        self.some_data = Some(res);
    }

    fn make_dots(&self) {
        if !self.args.contains(&ComputationPhases::DOTS) {
            return;
        }
        let data = self.get_data();
        self.time.print("creating main page dots");
        let parameters: Vec<&Set> = data.sets.iter()
            .filter(|x|x.typ == PreviewType::Parameter)
            .filter(|x|x.preview.relevance >= self.hide_irrelevant_parameters_below)
            .collect();
        let simplified_parameters: Vec<&Set> = data.sets.iter()
            .filter(|x|x.typ == PreviewType::Parameter)
            .filter(|x|x.preview.relevance >= self.simplified_hide_irrelevant_parameters_below)
            .collect();
        let graphs: Vec<&Set> = data.sets.iter().filter(|x|x.typ == PreviewType::GraphClass).collect();
        self.time.print("drawing parameters & graphs");
        for (name, set) in [
            ("parameters", &parameters),
            ("graphs", &graphs),
            ("parameters_simplified", &simplified_parameters),
        ] {
            if let Ok(done_dot) = make_drawing(&data, &self.working_dir, name, set, None){
                let final_dot = self.html_dir.join(format!("{}.dot", name));
                println!("copy dot to {:?}", &final_dot);
                fs::copy(&done_dot, &final_dot);
            }
        }
    }

    fn make_pages(&self) {
        if !self.args.contains(&ComputationPhases::PAGES) {
            return;
        }
        let data = self.get_data();
        self.time.print("fetching generated pages");
        let mut linkable: HashMap<String, Box<dyn Linkable>> = HashMap::new();
        for set in &data.sets {
            linkable.insert(set.id.clone(), Box::new(set.preview.clone()));
        }
        for rel in &data.relations {
            linkable.insert(rel.id.clone(), Box::new(rel.preview.clone()));
        }
        for source in &data.sources {
            linkable.insert(source.id.clone(), Box::new(source.preview.clone()));
        }
        for tag in &data.tags {
            linkable.insert(tag.id.clone(), Box::new(tag.preview.clone()));
        }
        let markdown = Markdown::new(&data, linkable, &self.bibliography);
        let mut generated_pages = HashMap::new();
        add_content(&data.sets, &self.final_dir, &mut generated_pages);
        add_content(&data.relations, &self.final_dir, &mut generated_pages);
        add_content(&data.sources, &self.final_dir, &mut generated_pages);
        add_content(&data.tags, &self.final_dir, &mut generated_pages);
        self.time.print("fetching handcrafted pages");
        let mut handcrafted_pages: HashMap<PathBuf, PathBuf> = HashMap::new();
        for source in file::iterate_folder_recursively(&self.handcrafted_dir) {
            let relative = source.strip_prefix(&self.handcrafted_dir).unwrap();
            let target_file = self.final_dir.join(relative);
            if source.is_file() {
                handcrafted_pages.insert(target_file.clone(), source.clone());
            } else if source.is_dir() {
                println!("directory {:?}", target_file);
            } else {
                println!("unprocessable file {:?}", target_file);
            }
        }
        self.time.print("merging generated and handcrafted pages");
        let mut target_keys = HashSet::new();
        for (k, _) in &generated_pages { target_keys.insert(k); }
        for (k, _) in &handcrafted_pages { target_keys.insert(k); }
        let mut pages = vec![];
        for target in target_keys {
            let substitute = generated_pages.get(target);
            let source = handcrafted_pages.get(target);
            pages.push(TargetPage{ target, substitute, source });
        }
        self.time.print("building general substitution map");
        let mut map: HashMap<&str, Mappable> = HashMap::new();
        // todo
        map.insert("test", Mappable::Address(Address{name: "qq".into(), url: "hello.com".into()}));
        // println!("clearing the final directory");
        // fs::remove_dir_all(&self.final_dir);
        // fs::create_dir(&self.final_dir);
        generate_pages(&pages, &markdown, &self.final_dir, &self.working_dir, &map); // takes long
    }

    fn make_relation_table(&self) {
        if !self.args.contains(&ComputationPhases::TABLE) {
            return;
        }
        let data = self.get_data();
        self.time.print("generating relation tables");
        let table_sets: Vec<PreviewSet> = data.sets.iter()
            .map(|x|x.preview.clone())
            .filter(|x|x.typ==PreviewType::Parameter)
            .filter(|x|x.relevance >= self.hide_irrelevant_parameters_below)
            .filter(|x|!x.hidden)
            .collect();
        let simplified_table_sets: Vec<PreviewSet> = data.sets.iter()
            .map(|x|x.preview.clone())
            .filter(|x|x.typ==PreviewType::Parameter)
            .filter(|x|x.relevance >= self.simplified_hide_irrelevant_parameters_below)
            .filter(|x|!x.hidden)
            .collect();
        for (name, set) in [
            ("table", &table_sets),
            ("table_simplified", &simplified_table_sets),
        ] {
            if let Ok(done_pdf) = generate_relation_table(&data, set, &self.parent) { // todo generalize
                let final_pdf = self.final_dir.join("html").join(format!("{}.pdf", name));
                println!("copy the pdf to {:?}", &final_pdf);
                fs::copy(&done_pdf, &final_pdf);
            }
        }
    }

}

fn main() {
    let mut computation = Computation::new();
    computation.retrieve_and_process_data();
    computation.make_dots();
    computation.make_pages();
    computation.make_relation_table();
}

