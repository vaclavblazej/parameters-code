use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use anyhow::Result;
use biblatex::Bibliography;
use log::{debug, error, info, warn};

use crate::data::bibliography::load_bibliography;
use crate::data::data::{Data, GraphClass, Parameter};
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::{Link, Linkable};
use crate::data::preview::*;
use crate::general::cache::Cache;
use crate::general::file;
use crate::general::timer::Timer;
use crate::general::worker::Worker;
use crate::output::api;
use crate::output::dot::{DotEdge, DotGraph};
use crate::output::markdown::Markdown;
use crate::output::pages;
use crate::output::pages::TargetPage;
use crate::output::pages::add_content;
use crate::output::table::generate_relation_table;
use crate::work::processing::process_raw_data;

use super::Args;
use super::paths::Paths;

pub(crate) struct Computation {
    args: HashSet<Args>,
    time: Timer,
    pub(crate) paths: Paths,
    bibliography: Option<Bibliography>,
    hide_irrelevant_parameters_below: u32,
    simplified_hide_irrelevant_parameters_below: u32,
    some_data: Option<Data>,
    pub(crate) worker: Worker,
}

impl Computation {
    pub(crate) fn new() -> Self {
        let args = super::parse_args_and_init_logger();
        Self {
            args,
            time: Timer::new(),
            paths: Paths::new(),
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

    pub(crate) fn clear(&mut self) {
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

    pub(crate) fn retrieve_and_process_data(&mut self) {
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
        let mut rawdata = crate::collection::build_collection();
        self.time.print("processing data");
        let res = process_raw_data(rawdata, &self.bibliography);
        match data_cache.save(&res) {
            Ok(()) => {}
            Err(err) => info!("{:?}", err),
        }
        self.some_data = Some(res);
    }

    pub(crate) fn make_dots(&self) {
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
        let graphs: Vec<&GraphClass> = data.graph_classes.values().collect();
        for (name, displayed_parameters) in [
            ("parameters", parameters),
            ("parameters_simplified", simplified_parameters),
            // ("graphs", graphs), // todo
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

    pub(crate) fn make_api(&self) -> Result<()> {
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

    pub(crate) fn make_pages(&self) {
        if !self.args.contains(&Args::Pages) {
            return;
        }
        let data = self.get_data();
        self.time.print("fetching generated pages");
        let mut links: HashMap<String, Link> = HashMap::new();
        let mut generated_pages = HashMap::new();
        let Data {
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
            sorted_sources,
            factoids,
            drawings,
            arc_parameter_parameter,
            arc_lf_lf,
            arc_op_op,
            arc_graph_graph,
            arc_gc_gc,
            arc_graph_gc,
            arc_pargc_pargc,
            arc_gcprop_gcprop,
            arc_gc_gcprop,
            arc_parameter_gcprop,
            arc_problem_problem,
            arc_problem_gcprop,
            arc_problem_parameter,
        } = data;
        for (id, val) in parameters {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in graph_classes {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in graph_class_properties {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in sources {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in graphs {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in logic_fragments {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in operations {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in parametric_parameters {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in parametric_graph_class {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in providers {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in graph_relations {
            links.insert(id.to_string(), val.get_link());
        }
        for (id, val) in tags {
            links.insert(id.to_string(), val.get_link());
        }
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
            graph_class_properties.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(
            sources.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(
            graphs.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(
            logic_fragments.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(
            operations.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(
            parametric_parameters.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(
            parametric_graph_class.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(
            providers.values(),
            &self.paths.final_dir,
            &mut generated_pages,
        );
        add_content(
            graph_relations.values(),
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
        super::page_build::generate_pages(&pages, &markdown, &self.paths);
        self.time.print("pages done");
        markdown.worker.join();
    }

    pub(crate) fn interactive(&self) {
        if !self.args.contains(&Args::Interactive) {
            return;
        }
        let data = self.get_data();
        super::interactive::run_interactive(data);
    }

    pub(crate) fn make_relation_table(&self) {
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
