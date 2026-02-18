use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::path::PathBuf;

use anyhow::Result;
use biblatex::Bibliography;
use log::{debug, error, info, warn};

use crate::data::bibliography::load_bibliography;
use crate::data::data::{Data, GraphClass, Named, Parameter};
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::{Link, Linkable};
use crate::data::preview::*;
use crate::general::cache::Cache;
use crate::general::file;
use crate::general::timer::Timer;
use crate::general::worker::Worker;
use crate::input::raw::RawData;
use crate::output::api;
use crate::output::dot::{DotEdge, DotGraph};
use crate::output::markdown::{GeneratedPage, Markdown};
use crate::output::pages::TargetPage;
use crate::output::pages::{self, Substitute, add_content};
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

    pub(crate) fn retrieve_and_process_data(&mut self, collection_fn: Box<dyn Fn() -> RawData>) {
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
        let mut rawdata = collection_fn();
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
        let param_edges: Vec<(String, String)> = data
            .arc_parameter_parameter
            .iter()
            .map(|(f, t, _)| (f.id.to_string(), t.id.to_string()))
            .collect();
        self.make_single_dot("parameters", &parameters, &param_edges);
        self.make_single_dot(
            "parameters_simplified",
            &simplified_parameters,
            &param_edges,
        );
        let gc_edges: Vec<(String, String)> = data
            .arc_gc_gc
            .iter()
            .map(|(f, t, _)| (f.id.to_string(), t.id.to_string()))
            .collect();
        self.make_single_dot("graphs", &graphs, &gc_edges);
    }

    fn make_single_dot<T>(&self, name: &str, items: &[&T], edges: &[(String, String)])
    where
        T: Named + HasId,
    {
        let mut digraph: DotGraph<T> = DotGraph::new(name, None);
        for dp in items {
            digraph.add_vertex(*dp);
        }
        let displayed_ids: HashSet<String> = items.iter().map(|x| x.id()).collect();
        for (f, t) in edges {
            if displayed_ids.contains(f) && displayed_ids.contains(t) {
                digraph.add_edge(DotEdge {
                    from: f.clone(),
                    to: t.clone(),
                    data: HashSet::new(),
                });
            }
        }
        if let Ok(done_dot) = digraph.save_to_file(&self.paths.working_dir) {
            let final_dot = self.paths.html_dir.join(format!("{}.dot", name));
            info!("copy dot to {:?}", &final_dot);
            if let Err(err) = file::copy_file(&done_dot, &final_dot) {
                error!("{}", err);
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
            arc_parameter_parameter,
            arc_lf_lf,
            arc_op_op,
            arc_graph_graph,
            arc_gc_gc,
            arc_graph_gc,
            arc_pargc_pargc,
            arc_gcprop_gcprop,
            arc_gc_gcprop,
            arc_gc_par,
            arc_parameter_gcprop,
            arc_problem_problem,
            arc_problem_gcprop,
            arc_problem_parameter,
            arc_gcprop_parameter,
        } = data;
        fn add_to_links<IdT, T>(
            items: &HashMap<PreviewId<IdT>, T>,
            links: &mut HashMap<String, Link>,
        ) where
            T: Linkable,
        {
            for (id, val) in items {
                links.insert(id.to_string(), val.get_link());
            }
        }
        self.time.print("fetching handcrafted pages");
        add_to_links(parameters, &mut links);
        add_to_links(graph_classes, &mut links);
        add_to_links(graph_class_properties, &mut links);
        add_to_links(sources, &mut links);
        add_to_links(graphs, &mut links);
        add_to_links(logic_fragments, &mut links);
        add_to_links(operations, &mut links);
        add_to_links(parametric_parameters, &mut links);
        add_to_links(parametric_graph_class, &mut links);
        add_to_links(providers, &mut links);
        add_to_links(graph_relations, &mut links);
        add_to_links(tags, &mut links);
        fn add_to_content<'a, I, T>(
            items: &'a HashMap<PreviewId<I>, T>,
            paths: &Paths,
            generated_pages: &mut HashMap<PathBuf, Substitute<'a>>,
        ) where
            T: GeneratedPage + HasId + 'static,
        {
            add_content(items.values(), &paths.final_dir, generated_pages);
        }
        add_to_content(parameters, &self.paths, &mut generated_pages);
        add_to_content(graph_classes, &self.paths, &mut generated_pages);
        add_to_content(tags, &self.paths, &mut generated_pages);
        add_to_content(graph_class_properties, &self.paths, &mut generated_pages);
        add_to_content(sources, &self.paths, &mut generated_pages);
        add_to_content(graphs, &self.paths, &mut generated_pages);
        add_to_content(logic_fragments, &self.paths, &mut generated_pages);
        add_to_content(operations, &self.paths, &mut generated_pages);
        add_to_content(parametric_parameters, &self.paths, &mut generated_pages);
        add_to_content(parametric_graph_class, &self.paths, &mut generated_pages);
        add_to_content(providers, &self.paths, &mut generated_pages);
        add_to_content(graph_relations, &self.paths, &mut generated_pages);
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
