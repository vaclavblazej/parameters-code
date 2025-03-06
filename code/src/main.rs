#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use anyhow::Result;
use biblatex::Bibliography;
use log::{debug, error, info, trace, warn};

use data::data::Data;
use data::data::Relation;
use data::data::Set;
use data::id::{Id, PreviewId};
use data::preview::PreviewSet;
use data::preview::PreviewType;
use data::simple_index::SimpleIndex;
use general::cache::Cache;
use general::enums::CpxInfo;
use general::enums::CpxTime;
use general::file;
use general::logger;
use general::progress::ProgressDisplay;
use output::diagram::make_drawing;
use output::markdown::Mappable;
use output::markdown::Markdown;
use output::markdown::{Address, Linkable};
use output::pages;
use output::pages::add_content;
use output::pages::TargetPage;
use output::table::render_table;
use work::bibliography::load_bibliography;
use work::processing::process_raw_data;

mod data {
    pub mod data;
    pub mod id;
    pub mod preview;
    pub mod simple_index;
}
mod general {
    pub mod cache;
    pub mod enums;
    pub mod file;
    pub mod hide;
    pub mod logger;
    pub mod progress;
}
mod work {
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
    pub mod all;
    pub mod collection;
}
mod collection;

fn build_page(
    page: &TargetPage,
    markdown: &Markdown,
    paths: &Paths,
    map: &HashMap<&str, Mappable>,
) -> anyhow::Result<()> {
    let content = match page.substitute {
        Some(substitute) => {
            // info!("generating {:?}", page.target);
            substitute.object.get_page(&markdown, paths)
        }
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
        }
        None => "".into(),
    };
    local_map.insert("handcrafted", Mappable::String(handcrafted_content));
    if page.target.exists() {
        fs::remove_file(&page.target)?;
    }
    if let Some(parent) = page.target.parent() {
        fs::create_dir_all(parent)?;
    }
    let altered_content = substitute(&content, &markdown, &local_map);
    file::write_file_content(page.target, &altered_content)?;
    Ok(())
}

fn generate_pages(
    pages: &Vec<TargetPage>,
    markdown: &Markdown,
    paths: &Paths,
    map: &HashMap<&str, Mappable>,
) -> anyhow::Result<()> {
    let mut progress = ProgressDisplay::new("generating pages", pages.len() as u32);
    // todo - maybe use par_iter for iterating parallely to speed up the computation?
    let res: Result<Vec<()>> = pages
        .iter()
        .map(|page| -> anyhow::Result<()> {
            progress.increase(1);
            build_page(page, markdown, paths, map)
        })
        .collect();
    progress.done();
    Ok(())
}

fn substitute(content: &String, markdown: &Markdown, map: &HashMap<&str, Mappable>) -> String {
    let altered_content = content
        .lines()
        .map(|line| {
            let mut strline = line.into();
            for _ in 1..10 {
                let newline = markdown.substitute_custom_markdown(&strline, map);
                if newline == strline {
                    return newline;
                }
                strline = newline;
            }
            strline
        })
        .collect::<Vec<String>>()
        .join("\n");
    altered_content
}

fn generate_relation_table(data: &Data, draw_sets: &Vec<PreviewSet>, paths: &Paths, name: &str) {
    let done_pdf = render_table(&data, draw_sets, &paths.table_tikz_folder).unwrap();
    let final_pdf = paths.html_dir.join(format!("{}.pdf", name));
    info!("copy the pdf to {:?}", &final_pdf);
    if let Err(err) = fs::copy(&done_pdf, &final_pdf){
        error!("{}", err);
    }
}

struct Timer {
    instant: Instant,
}

impl Timer {
    fn new() -> Self {
        Self {
            instant: Instant::now(),
        }
    }

    pub fn print(&self, message: &str) {
        info!("{:?} {}", self.instant.elapsed(), message);
    }
}

#[derive(Hash, PartialEq, Eq)]
enum Args {
    PREPROCESS,
    DOTS,
    PAGES,
    RELATIONS,
    TABLE,
    CLEAR,
    MOCK,
    DEBUG,
    TRACE,
}

struct Computation {
    args: HashSet<Args>,
    time: Timer,
    paths: Paths,
    bibliography: Option<Bibliography>,
    hide_irrelevant_parameters_below: u32,
    simplified_hide_irrelevant_parameters_below: u32,
    some_data: Option<Data>,
}

struct Paths {
    parent: PathBuf,
    table_tikz_folder: PathBuf,
    handcrafted_dir: PathBuf,
    bibliography_file: PathBuf,
    final_dir: PathBuf,
    hugo_public_dir: PathBuf,
    working_dir: PathBuf,
    html_dir: PathBuf,
    tmp_dir: PathBuf,
}

impl Computation {
    fn new() -> Self {
        let rawargs: Vec<String> = env::args().collect();
        let mut args = HashSet::new();
        for (i, arg) in rawargs.iter().enumerate() {
            if i == 0 {
                continue;
            }
            match arg.as_str() {
                "preprocess" => {
                    args.insert(Args::PREPROCESS);
                }
                "dots" => {
                    args.insert(Args::DOTS);
                }
                "pages" => {
                    args.insert(Args::PAGES);
                }
                "relations" => {
                    args.insert(Args::RELATIONS);
                }
                "table" => {
                    args.insert(Args::TABLE);
                }
                "clear" => {
                    args.insert(Args::CLEAR);
                }
                "mock" => {
                    args.insert(Args::MOCK);
                }
                "trace" => {
                    args.insert(Args::TRACE);
                }
                "debug" => {
                    args.insert(Args::DEBUG);
                }
                "fast" => {
                    args.insert(Args::CLEAR);
                    args.insert(Args::PREPROCESS);
                    args.insert(Args::DOTS);
                    args.insert(Args::PAGES);
                }
                "all" => {
                    args.insert(Args::CLEAR);
                    args.insert(Args::PREPROCESS);
                    args.insert(Args::DOTS);
                    args.insert(Args::PAGES);
                    args.insert(Args::RELATIONS);
                    args.insert(Args::TABLE);
                }
                other => panic!("unknown parameter: '{}'", other),
            }
        }
        logger::init(if args.contains(&Args::TRACE) {
            log::LevelFilter::Trace
        } else if args.contains(&Args::DEBUG) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        });
        let current = env::current_dir().unwrap();
        let parent = current.parent().unwrap();
        let table_tikz_folder = parent.join("scripts").join("table_tikz");
        let handcrafted_dir = parent.join("handcrafted");
        let bibliography_file = handcrafted_dir.join("main.bib");
        let final_dir = parent.join("web").join("content");
        let hugo_public_dir = parent.join("web").join("public");
        let working_dir = current.join("target");
        let html_dir = final_dir.join("html");
        let tmp_dir = current.join("tmp");
        Self {
            args,
            time: Timer::new(),
            paths: Paths {
                parent: parent.to_path_buf(),
                table_tikz_folder,
                handcrafted_dir,
                bibliography_file,
                final_dir,
                hugo_public_dir,
                working_dir,
                html_dir,
                tmp_dir,
            },
            bibliography: None,
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

    fn clear(&mut self) {
        if !self.args.contains(&Args::CLEAR) {
            return;
        }
        info!("clearing the final directory");
        info!(
            "removing folder of {}",
            self.paths.final_dir.to_str().unwrap()
        );
        fs::remove_dir_all(&self.paths.final_dir);
        fs::create_dir(&self.paths.final_dir);
        info!(
            "removing folder of {}",
            self.paths.hugo_public_dir.to_str().unwrap()
        );
        fs::remove_dir_all(&self.paths.hugo_public_dir);
    }

    fn retrieve_and_process_data(&mut self) {
        let mock = self.args.contains(&Args::MOCK);
        self.bibliography = load_bibliography(&self.paths.bibliography_file);
        let cch: Cache<Data> = Cache::new(&self.paths.tmp_dir.join("data.json"));
        if !mock && !self.args.contains(&Args::PREPROCESS) {
            if let Some(mut res) = cch.load() {
                info!("deserialized data");
                res.recompute();
                self.some_data = Some(res);
                return;
            }
        }
        self.time.print("retrieving data collection");
        let mut rawdata = match mock {
            false => collection::build_collection(),
            true => test::collection::build_collection(),
        };
        self.time.print("processing data");
        let res = process_raw_data(rawdata, &self.bibliography);
        if !mock {
            match cch.save(&res) {
                Ok(()) => {}
                Err(err) => info!("{:?}", err),
            }
        }
        self.some_data = Some(res);
    }

    fn make_dots(&self) {
        if !self.args.contains(&Args::DOTS) {
            return;
        }
        let data = self.get_data();
        self.time.print("creating main page dots");
        let parameters: Vec<&Set> = data
            .sets
            .iter()
            .filter(|x| x.typ == PreviewType::Parameter)
            .filter(|x| x.preview.relevance >= self.hide_irrelevant_parameters_below)
            .collect();
        let simplified_parameters: Vec<&Set> = data
            .sets
            .iter()
            .filter(|x| x.typ == PreviewType::Parameter)
            .filter(|x| x.preview.relevance >= self.simplified_hide_irrelevant_parameters_below)
            .collect();
        let graphs: Vec<&Set> = data
            .sets
            .iter()
            .filter(|x| x.typ == PreviewType::GraphClass)
            .collect();
        for (name, set) in [
            ("parameters", &parameters),
            ("graphs", &graphs),
            ("parameters_simplified", &simplified_parameters),
        ] {
            if let Ok(done_dot) = make_drawing(&data, &self.paths.working_dir, name, set, None) {
                let final_dot = self.paths.html_dir.join(format!("{}.dot", name));
                info!("copy dot to {:?}", &final_dot);
                if let Err(err) = file::copy_file(&done_dot, &final_dot) {
                    error!("{}", err);
                }
            }
        }
    }

    fn make_pages(&self) {
        if !self.args.contains(&Args::PAGES) {
            return;
        }
        let data = self.get_data();
        self.time.print("fetching generated pages");
        let mut linkable: HashMap<String, Box<dyn Linkable>> = HashMap::new(); // todo unified type for previews
        let mut generated_pages = HashMap::new();
        let Data {
            sets,
            relations,
            sources,
            providers,
            tags,
            partial_results,
            set_idx: _,
            set_id_idx: _,
            relation_idx: _,
            relation_id_idx: _,
        } = data;
        for set in sets {
            linkable.insert(set.id.to_string(), Box::new(set.preview.clone()));
        }
        if self.args.contains(&Args::RELATIONS) {
            for rel in relations {
                linkable.insert(rel.id.to_string(), Box::new(rel.preview.clone()));
            }
        }
        for source in sources {
            linkable.insert(source.id.to_string(), Box::new(source.preview.clone()));
        }
        for tag in tags {
            linkable.insert(tag.id.to_string(), Box::new(tag.preview.clone()));
        }
        add_content(sets, &self.paths.final_dir, &mut generated_pages);
        if self.args.contains(&Args::RELATIONS) {
            add_content(relations, &self.paths.final_dir, &mut generated_pages);
        }
        add_content(sources, &self.paths.final_dir, &mut generated_pages);
        add_content(tags, &self.paths.final_dir, &mut generated_pages);
        self.time.print("fetching handcrafted pages");
        let mut handcrafted_pages: HashMap<PathBuf, PathBuf> = HashMap::new();
        for source in file::iterate_folder_recursively(&self.paths.handcrafted_dir) {
            let relative = source.strip_prefix(&self.paths.handcrafted_dir).unwrap();
            let target_file = self.paths.final_dir.join(relative);
            if source.is_file() {
                handcrafted_pages.insert(target_file.clone(), source.clone());
            } else if source.is_dir() {
                info!("directory {:?}", target_file);
            } else {
                info!("unprocessable file {:?}", target_file);
            }
        }
        self.time.print("merging generated and handcrafted pages");
        let mut target_keys = HashSet::new();
        for (k, _) in &generated_pages {
            target_keys.insert(k);
        }
        for (k, _) in &handcrafted_pages {
            target_keys.insert(k);
        }
        let mut pages = vec![];
        for target in target_keys {
            let substitute = generated_pages.get(target);
            let source = handcrafted_pages.get(target);
            pages.push(TargetPage {
                target,
                substitute,
                source,
            });
        }
        self.time.print("building general substitution map");
        let mut map: HashMap<&str, Mappable> = HashMap::new();
        // todo add custom entries to the hash map for [[key]] notation in the handcrafted pages
        // map.insert("test", Mappable::Address(Address{name: "qq".into(), url: "hello.com".into()}));
        let markdown = Markdown::new(&data, linkable, &self.bibliography);
        generate_pages(&pages, &markdown, &self.paths, &map); // takes long
    }

    fn make_relation_table(&self) {
        if !self.args.contains(&Args::TABLE) {
            return;
        }
        let data = self.get_data();
        self.time.print("generating relation tables");
        let table_sets: Vec<PreviewSet> = data
            .sets
            .iter()
            .map(|x| x.preview.clone())
            .filter(|x| x.typ == PreviewType::Parameter)
            .filter(|x| x.relevance >= self.hide_irrelevant_parameters_below)
            .collect();
        let simplified_table_sets: Vec<PreviewSet> = data
            .sets
            .iter()
            .map(|x| x.preview.clone())
            .filter(|x| x.typ == PreviewType::Parameter)
            .filter(|x| x.relevance >= self.simplified_hide_irrelevant_parameters_below)
            .collect();
        for (name, set) in [
            ("table", &table_sets),
            ("table_simplified", &simplified_table_sets),
        ] {
            generate_relation_table(&data, set, &self.paths, name);
        }
    }
}

fn main() {
    let mut computation = Computation::new();
    computation.clear();
    computation.retrieve_and_process_data();
    computation.make_dots();
    computation.make_relation_table();
    computation.make_pages();
}
