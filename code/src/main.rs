#![allow(unused)]
// #![deny(clippy::unwrap_used)]
// #![deny(clippy::expect_used)]
// #![deny(clippy::panic)]
// #![deny(unused_must_use)]

use std::collections::{HashMap, HashSet, LinkedList};
use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use biblatex::Bibliography;
use general::worker::{Task, Worker};
use log::{debug, error, info, trace, warn};
use rayon::prelude::*;

use crate::data::bibliography::load_bibliography;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::{Link, Linkable};
use crate::data::preview::*;
use crate::data::*;
use crate::general::cache::Cache;
use crate::general::file;
use crate::general::logger;
use crate::general::progress::ProgressDisplay;
use crate::general::timer::Timer;

use crate::data::data::{Data, Parameter};
use crate::output::api;
use crate::output::dot::{DotEdge, DotGraph, DotVertex, DotVertexAttribute};
use crate::output::markdown::{Mappable, Markdown};
use crate::output::pages;
use crate::output::pages::TargetPage;
use crate::output::pages::add_content;
use crate::output::table::generate_relation_table;
use crate::work::processing::process_raw_data;

mod general {
    pub mod cache;
    pub mod file;
    pub mod logger;
    pub mod progress;
    pub mod strings;
    pub mod timer;
    pub mod worker;
}
mod input {
    pub mod build;
    pub mod builder;
    pub mod intersectable;
    pub mod provider;
    pub mod raw;
    pub mod raw_enums;
    pub mod source;
}
pub mod data {
    pub mod bibliography;
    pub mod data;
    pub mod date;
    pub mod digraph;
    pub mod enums;
    pub mod id;
    pub mod link;
    pub mod preview;
    pub mod score;
    pub mod simple_index;
}
mod work {
    pub mod combine;
    pub mod compare;
    pub mod convert;
    pub mod hide;
    pub mod hierarchy;
    pub mod preview_collection;
    pub mod processing;
}
mod output {
    pub mod api;
    pub mod color;
    pub mod diagram;
    pub mod dot;
    pub mod markdown;
    pub mod pages;
    pub mod table;
    pub mod to_markdown;
}
mod collection;

fn build_page(page: &TargetPage, markdown: &Markdown, paths: &Paths) -> anyhow::Result<()> {
    let content = match page.substitute {
        Some(substitute) => substitute.object.get_page(markdown, paths),
        None => "[[handcrafted]]".into(),
    };
    let mut local_map = HashMap::new();
    let handcrafted_content = match page.source {
        Some(source) => {
            if source.as_os_str().to_str().unwrap().ends_with(".md") {
                file::read_file_content(source)?
            } else {
                let target_folder = &page.target.parent().unwrap();
                file::copy_file(source, page.target)?;
                return Ok(());
            }
        }
        None => "".into(),
    };
    local_map.insert("handcrafted", Mappable::String(handcrafted_content));
    let mut altered_content = content;
    for _ in 0..2 {
        altered_content = substitute(&altered_content, markdown, &local_map);
    }
    file::write_file_content(page.target, &altered_content)?;
    Ok(())
}

fn generate_pages(
    pages: &Vec<TargetPage>,
    markdown: &Markdown,
    paths: &Paths,
) -> anyhow::Result<()> {
    let progress = ProgressDisplay::new("generating pages", pages.len() as u32);
    let res: Result<Vec<()>> = pages
        .par_iter()
        .map(|page| -> anyhow::Result<()> {
            progress.increase(1);
            build_page(page, markdown, paths)
        })
        .collect();
    progress.done();
    res?;
    Ok(())
}

fn substitute(content: &str, markdown: &Markdown, map: &HashMap<&str, Mappable>) -> String {
    content
        .lines()
        .map(|line| markdown.substitute_custom_markdown(line, map))
        .collect::<Vec<String>>()
        .join("\n")
}

#[derive(Hash, PartialEq, Eq)]
enum Args {
    Preprocess,
    Dots,
    Pages,
    Table,
    Api,
    Clear,
    Interactive,
    Debug,
    Trace,
}

struct Computation {
    args: HashSet<Args>,
    time: Timer,
    paths: Paths,
    bibliography: Option<Bibliography>,
    hide_irrelevant_parameters_below: u32,
    simplified_hide_irrelevant_parameters_below: u32,
    some_data: Option<Data>,
    worker: Worker,
}

#[derive(Clone)]
struct Paths {
    parent: PathBuf,
    table_tikz_folder: PathBuf,
    handcrafted_dir: PathBuf,
    bibliography_file: PathBuf,
    final_dir: PathBuf,
    hugo_public_dir: PathBuf,
    working_dir: PathBuf,
    html_dir: PathBuf,
    api_dir: PathBuf,
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
                    args.insert(Args::Preprocess);
                }
                "dots" => {
                    args.insert(Args::Dots);
                }
                "pages" => {
                    args.insert(Args::Pages);
                }
                "table" => {
                    args.insert(Args::Table);
                }
                "clear" => {
                    args.insert(Args::Clear);
                }
                "trace" => {
                    args.insert(Args::Trace);
                }
                "debug" => {
                    args.insert(Args::Debug);
                }
                "fast" => {
                    args.insert(Args::Clear);
                    args.insert(Args::Preprocess);
                    args.insert(Args::Dots);
                    args.insert(Args::Pages);
                }
                "api" => {
                    args.insert(Args::Api);
                }
                "all" => {
                    args.insert(Args::Clear);
                    args.insert(Args::Preprocess);
                    args.insert(Args::Dots);
                    args.insert(Args::Pages);
                    args.insert(Args::Api);
                    args.insert(Args::Table);
                }
                "interactive" | "i" => {
                    args.insert(Args::Interactive);
                }
                other => panic!("unknown parameter: '{}'", other),
            }
        }
        logger::init(if args.contains(&Args::Trace) {
            log::LevelFilter::Trace
        } else if args.contains(&Args::Debug) {
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
        let temp_dir = env::temp_dir();
        let working_dir = temp_dir.join("target");
        let html_dir = final_dir.join("html");
        let api_dir = final_dir.join("api");
        let tmp_dir = temp_dir.join("tmp");
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
                api_dir,
                tmp_dir,
            },
            bibliography: None,
            hide_irrelevant_parameters_below: 1,
            simplified_hide_irrelevant_parameters_below: 5,
            some_data: None,
            worker: Worker::new(),
        }
    }

    fn get_data(&self) -> &Data {
        match self.some_data {
            Some(ref data) => data,
            None => panic!("unwrap empty data"),
        }
    }

    fn clear(&mut self) {
        if !self.args.contains(&Args::Clear) {
            return;
        }
        info!("clearing the final directory");
        info!(
            "removing folder of {}",
            self.paths.final_dir.to_str().unwrap()
        );
        file::clear_folder(&self.paths.final_dir);
        info!(
            "removing folder of {}",
            self.paths.hugo_public_dir.to_str().unwrap()
        );
        file::clear_folder(&self.paths.hugo_public_dir);
    }

    fn retrieve_and_process_data(&mut self) {
        self.bibliography = match load_bibliography(&self.paths.bibliography_file) {
            Ok(x) => Some(x),
            Err(err) => {
                error!("{}", err);
                None
            }
        };
        let data_cache: Cache<Data> = Cache::new(&self.paths.tmp_dir.join("data.json"));
        if !self.args.contains(&Args::Preprocess)
            && let Some(mut res) = data_cache.load()
        {
            info!("deserialized data");
            self.some_data = Some(res);
            return;
        }
        self.time.print("retrieving data collection");
        let mut rawdata = collection::build_collection();
        self.time.print("processing data");
        let res = process_raw_data(rawdata, &self.bibliography);
        match data_cache.save(&res) {
            Ok(()) => {}
            Err(err) => info!("{:?}", err),
        }
        self.some_data = Some(res);
    }

    fn make_dots(&self) {
        if !self.args.contains(&Args::Dots) {
            return;
        }
        let data = self.get_data();
        self.time.print("creating main page dots");
        let parameters: Vec<&Parameter> = data
            .parameters
            .values()
            .filter(|x| x.score >= self.hide_irrelevant_parameters_below)
            .collect();
        let simplified_parameters: Vec<&Parameter> = data
            .parameters
            .values()
            .filter(|x| x.score >= self.simplified_hide_irrelevant_parameters_below)
            .collect();
        for (name, displayed_parameters) in [
            ("parameters", parameters),
            ("parameters_simplified", simplified_parameters),
        ] {
            let mut digraph = DotGraph::new(name, None);
            for dp in &displayed_parameters {
                digraph.add_vertex(dp);
            }
            let displayed_ids: HashSet<PreviewParameterId> = displayed_parameters
                .iter()
                .map(|&x| x.previewid())
                .collect();
            for (f, t, cpx) in &data.arc_parameter_parameter {
                if displayed_ids.contains(f) && displayed_ids.contains(t) {
                    digraph.add_edge(DotEdge {
                        from: f.to_string(),
                        to: t.to_string(),
                        data: HashSet::new(),
                    });
                }
            }
            // todo get parameter relations from data and filter them so that only those between
            // displayed_parameters are preserved; then add them to digraph as edges
            if let Ok(done_dot) = digraph.save_to_file(&self.paths.working_dir) {
                let final_dot = self.paths.html_dir.join(format!("{}.dot", name));
                info!("copy dot to {:?}", &final_dot);
                if let Err(err) = file::copy_file(&done_dot, &final_dot) {
                    error!("{}", err);
                }
            }
        }
    }

    fn make_api(&self) -> Result<()> {
        if !self.args.contains(&Args::Api) {
            return Ok(());
        }
        let data = self.get_data();
        self.time.print("generating api");
        // data.sets.iter().map(|x|api_data.push(Box::new(x)));
        api::create_set_api(data, &self.paths.api_dir)?;
        api::create_simple_api(data, &self.paths.api_dir)?;
        Ok(())
    }

    fn make_pages(&self) {
        if !self.args.contains(&Args::Pages) {
            return;
        }
        let data = self.get_data();
        self.time.print("fetching generated pages");
        let mut links: HashMap<String, Link> = HashMap::new();
        let mut generated_pages = HashMap::new();
        let Data {
            graph_class_relations,
            graph_classes,
            graph_relations,
            graph_class_properties,
            graphs,
            logic_fragments,
            operations,
            parameters,
            parametric_graph_class,
            parametric_parameters,
            providers,
            tags,
            sources,
            factoids,
            drawings,
            arc_parameter_parameter,
        } = data;
        for (id, val) in parameters {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in graph_classes {
            links.insert(id.to_string(), val.get_link());
        }
        // for source in sources {
        //     links.insert(source.id.to_string(), Box::new(source.preview()));
        // }
        // for tag in tags {
        //     links.insert(tag.id.to_string(), Box::new(tag.preview()));
        // }
        add_content(
            parameters.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(
            graph_classes.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(tags.values(), &self.paths.final_dir, &mut generated_pages);
        add_content(
            sources.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        self.time.print("fetching handcrafted pages");
        let mut handcrafted_pages: HashMap<PathBuf, PathBuf> = HashMap::new();
        for source in file::iterate_folder_recursively(&self.paths.handcrafted_dir) {
            let relative = source.strip_prefix(&self.paths.handcrafted_dir).unwrap();
            let target_file = self.paths.final_dir.join(relative);
            if source.is_file() {
                debug!("file {:?}", target_file);
                handcrafted_pages.insert(target_file.clone(), source.clone());
            } else if source.is_dir() {
                debug!("directory {:?}", target_file);
            } else {
                warn!("unprocessable file {:?}", target_file);
            }
        }
        self.time.print("merging generated and handcrafted pages");
        let mut target_keys = HashSet::new();
        for k in generated_pages.keys() {
            target_keys.insert(k);
        }
        for k in handcrafted_pages.keys() {
            target_keys.insert(k);
        }
        let mut pages = Vec::new();
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
        let markdown = Markdown::new(data, links, &self.bibliography);
        generate_pages(&pages, &markdown, &self.paths);
        self.time.print("pages done");
        markdown.worker.join();
    }

    fn process_command(&self, data: &Data, mut command: LinkedList<String>) -> bool {
        if let Some(cmd) = command.pop_front() {
            match cmd.as_str() {
                "help" => {
                    println!("help:");
                    println!(
                        "    hasse <par_id> [<par_id> ...] - draws hasse diagram of the listed parameters"
                    );
                    println!("    exit - end the interactive prompt");
                }
                // "hasse" => {
                //     let mut sets = Vec::new();
                //     for i in command.iter() {
                //         let set_id: PreviewParameterId = PreviewParameterId::from(i.clone());
                //         sets.push(data.get(&set_id));
                //     }
                //     let target_dir = &self.paths.tmp_dir;
                //     let name = "drawing";
                //     let res_dot_target_file = make_drawing(data, target_dir, name, &sets, None);
                //     if let Ok(dot_target_file) = res_dot_target_file {
                //         println!("dot drawing created at '{:?}'", dot_target_file);
                //         let pdf_target_file = target_dir.join(format!("{}.pdf", name));
                //         Command::new("dot")
                //             .arg("-Tpdf")
                //             .arg(&dot_target_file)
                //             .arg("-o")
                //             .arg(&pdf_target_file)
                //             .output()
                //             .expect("dot command failed");
                //         assert!(pdf_target_file.exists());
                //         println!("pdf generated at '{:?}'", pdf_target_file);
                //     }
                // }
                "exit" => return false,
                x => warn!("unknown command '{}'", x),
            }
        }
        true
    }

    fn interactive(&self) {
        if !self.args.contains(&Args::Interactive) {
            return;
        }
        let data = self.get_data();
        let mut buffer = String::new();
        let stdin = io::stdin();
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            match stdin.read_line(&mut buffer) {
                Ok(_okcode) => {
                    let mut chars = buffer.as_str().chars();
                    chars.next_back(); // remove the last \n character
                    buffer = String::from(chars.as_str());
                    let mut commands: LinkedList<String> = LinkedList::new();
                    for word in buffer.as_str().split(' ') {
                        commands.push_back(String::from(word));
                    }
                    if !self.process_command(data, commands) {
                        break;
                    }
                    buffer = String::new();
                }
                Err(err) => {
                    error!("{:?}", err);
                }
            }
        }
    }

    fn make_relation_table(&self) {
        if !self.args.contains(&Args::Table) {
            return;
        }
        let data = self.get_data();
        self.time.print("generating relation tables");
        let table_sets: Vec<PreviewParameter> = data
            .parameters
            .values()
            .map(|x| x.preview())
            .filter(|x| x.score >= self.hide_irrelevant_parameters_below)
            .collect();
        let simplified_table_sets: Vec<PreviewParameter> = data
            .parameters
            .values()
            .map(|x| x.preview())
            .filter(|x| x.score >= self.simplified_hide_irrelevant_parameters_below)
            .collect();
        let (name, parameter) = ("table_simplified", &simplified_table_sets);
        generate_relation_table(data, parameter, &self.paths, name, &self.worker);
    }
}

fn main() {
    let mut computation = Computation::new();
    computation.clear();
    computation.retrieve_and_process_data();
    computation.make_dots();
    computation.make_relation_table();
    computation.make_api();
    computation.make_pages();
    computation.interactive();
    computation.worker.join();
}
