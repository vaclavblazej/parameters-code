//! Given the processed data generate markdown pages.

use std::collections::{HashMap, HashSet, LinkedList, VecDeque};
use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use biblatex::Bibliography;
use log::{error, info, trace};
use rand::seq::IndexedRandom;
use regex::Regex;

use crate::data::data::*;
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::link::{Link, Linkable};
use crate::data::preview::*;
use crate::general::progress;
use crate::output::color::Color;
use crate::output::to_markdown::ToMarkdown;
use crate::{Paths, Worker, file};

type Result<T> = std::result::Result<T, MarkdownError>;

#[derive(Debug)]
pub enum MarkdownError {
    IdNotFound(String),
    MissingId,
    EmptyKey,
    ErrSubstitutingId(String),
    ErrSubstituting,
}

impl fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MarkdownError::IdNotFound(key) => write!(f, "id [[{}]] not found", key),
            MarkdownError::MissingId => write!(f, "id is missing"),
            MarkdownError::EmptyKey => write!(f, "error key is empty"),
            MarkdownError::ErrSubstitutingId(key) => write!(f, "error substituting [[{}]]", key),
            MarkdownError::ErrSubstituting => write!(f, "error substituting"),
        }
    }
}

fn base(id: &String) -> String {
    format!("{{{{< base >}}}}{}", id)
}

pub trait GeneratedPage: Sync + Send {
    fn get_page(&self, builder: &Markdown, path: &Paths) -> String;
}

fn copy_file_to_final_location(file: &PathBuf, to_directory: &Path) {
    let filename = file
        .file_name()
        .expect("Result file has no name")
        .to_owned();
    let final_path = to_directory.join(&filename);
    file::copy_file(file, &final_path).expect("Failed to copy result to final directory");
}

fn include_dot_file(drawing: anyhow::Result<PathBuf>, final_dir: &Path) -> String {
    match drawing {
        Ok(result_pdf_file) => {
            copy_file_to_final_location(&result_pdf_file, &final_dir.join("html"));
            let filename = result_pdf_file.file_name().unwrap().to_string_lossy();
            format!("[[zoomdot ../{}]]", filename)
        }
        Err(e) => {
            error!("{:?}", e);
            format!("{:?}", e)
        }
    }
}

impl GeneratedPage for Parameter {
    fn get_page(&self, builder: &Markdown, paths: &Paths) -> String {
        let mut res = String::new();
        res += &format!("---\ntitle: \"{}\"\n---", self.name_core.name);
        res += &format!("# {}\n\n", self.name_core.name);
        if let Some(abbr) = &self.name_core.abbr {
            res += &format!("abbr: {}\n\n", abbr);
        }
        if !self.name_core.aka.is_empty() {
            res += &format!("aka: {}\n\n", self.name_core.aka.join(", "));
        }
        if !self.tags.is_empty() {
            let tag_strings: Vec<String> = self
                .tags
                .iter()
                .map(|x| builder.linkto(&x.get_link()))
                .collect();
            res += &format!("tags: {}\n\n", tag_strings.join(", "));
        }
        // let equivalent_strings: Vec<String> = self
        //     .related_sets
        //     .equivsets
        //     .iter()
        //     .filter(|x| x.id != self.id.preview())
        //     .map(|x| builder.linkto(x))
        //     .collect();
        // if !equivalent_strings.is_empty() {
        //     res += &format!(
        //         "functionally equivalent to: {}\n\n",
        //         equivalent_strings.join(", ")
        //     );
        // }
        // if !self.providers.is_empty() {
        //     let provider_strings: Vec<String> =
        //         self.providers.iter().map(|x| builder.linkto(x)).collect();
        //     res += &format!("providers: {}\n\n", provider_strings.join(", "));
        // }
        let definition_string = match &self.definition {
            ParameterDefinition::Graph(text) => text.clone(),
            ParameterDefinition::GraphClass(text) => text.clone(),
            ParameterDefinition::BoundsAll(preview_parametric_parameter) => {
                format!(
                    "Parameter is at most $k$ if value of every {} is at most $k$.",
                    preview_parametric_parameter.name_core.name
                )
            }
        };
        res += &format!("**Definition:** {}\n\n", definition_string);
        res += "[[handcrafted]]\n\n";
        // for drawing_path in [
        //     make_focus_drawing(
        //         &format!("local_{}", self.id),
        //         builder.data,
        //         digraph,
        //         self,
        //         2,
        //         &paths.working_dir,
        //     ),
        //     make_subset_drawing(
        //         &format!("graph_property_inclusions_{}", self.id),
        //         builder.data,
        //         self,
        //         builder.data.parameters.values().collect(),
        //         &paths.working_dir,
        //     ),
        //     make_subset_drawing(
        //         &format!("parameter_inclusions_{}", self.id),
        //         builder.data,
        //         self,
        //         builder
        //             .data
        //             .parameters
        //             .values()
        //             .filter(|x| x.score > 0)
        //             .collect(),
        //         &paths.working_dir,
        //     ),
        // ] {
        //     res += &include_dot_file(drawing_path, &paths.final_dir);
        // }
        // todo - having parameters and graphs both as sets means maximal doesn't show what was expected
        // let subs = &self.subsets.maximal;
        // if !subs.is_empty() {
        // res += &format!("## Maximal subsets\n\n");
        // for s in subs {
        // res += &format!("* {}\n", builder.linkto(s));
        // }
        // res += "\n";
        // }
        // let sups = &self.supersets.minimal;
        // if !sups.is_empty() {
        // res += &format!("## Minimal supersets\n\n");
        // for s in sups {
        // res += &format!("* {}\n", builder.linkto(s));
        // }
        // res += "\n";
        // }
        // res += "---\n\n## Relations\n\n";
        // let mut relation_table = Table::new(vec!["Other", "", "Relation from", "Relation to"]);
        // for set in &builder.data.parameters {
        //     let relation_fr_el: String =
        //         match builder.data.get_relation(&set.preview(), &self.preview()) {
        //             None => "unknown to HOPS".into(),
        //             Some(rel) => {
        //                 // let name = rel.preview().short_description(builder); // todo
        //                 // format!("[{}]({})", name, rel.preview().get_url())
        //                 rel.preview().short_description(builder)
        //             }
        //         };
        //     let relation_to_el: String =
        //         match builder.data.get_relation(&self.preview(), &set.preview()) {
        //             None => "unknown to HOPS".into(),
        //             Some(rel) => {
        //                 // let name = rel.preview().short_description(builder);
        //                 // format!("[{}]({})", name, rel.preview().get_url())
        //                 rel.preview().short_description(builder)
        //             }
        //         };
        //     let colorname = relation_color(&self.related_sets, set.id(), &set.preview()).name();
        //     relation_table.add(vec![
        //         builder.linkto(&set.preview()),
        //         // the hidden span makes the color column sortable
        //         format!(
        //             "<span style=\"display:none\">{}</span>[[color {}]]",
        //             colorname, colorname
        //         ),
        //         relation_fr_el,
        //         relation_to_el,
        //     ]);
        // }
        // res += builder.make_table(relation_table).as_str();
        res += "\n";
        // make_subset_drawing
        // let sources_timeline = &self.timeline;
        // if !sources_timeline.is_empty() {
        //     res += "---\n\n## Results\n\n";
        //     for source in sources_timeline {
        //         if let Some(val) = source.to_markdown(builder) {
        //             res += &val;
        //         }
        //     }
        //     res += "\n";
        // }
        res
    }
}

impl GeneratedPage for Source {
    fn get_page(&self, builder: &Markdown, paths: &Paths) -> String {
        let mut res = String::new();
        match &self.sourcekey {
            SourceKey::Bibtex {
                entry_key,
                name,
                entry_content,
            } => {
                // res += &format!("# {}\n\n", self.preview().get_name());
                if let Some(somebib) = builder.bibliography {
                    if let Some(val) = somebib.get(entry_key) {
                        if let Ok(doi) = val.doi() {
                            let doi_url = format!("https://www.doi.org/{}", doi);
                            res += &format!("[{}]({})\n\n", doi_url, doi_url);
                        } else if let Ok(url) = val.url() {
                            res += &format!("[{}]({})\n\n", url, url);
                        }
                        // todo - print the original (unformatted) biblatex citation from main.bib
                        res += &format!("```bibtex\n{}\n```\n", val.to_biblatex_string());
                    } else {
                        error!("unable to load {} from main.bib", entry_key);
                        res += &format!(
                            "an error occured while loading the bibtex entry for `{}`",
                            entry_key
                        );
                    }
                }
            }
            SourceKey::Other { name, description } => {
                res += &format!("# {}\n\n", name);
                res += &format!("{}\n\n", description);
            }
            SourceKey::Online { url } => {
                res += &format!("# Online source {}\n\n", self.id);
            }
        }
        for (idx, drawing) in self.drawings.iter().enumerate() {
            let name = &format!("drawing_{}_{}", self.id, idx);
            match drawing {
                Drawing::Table(list) => {
                    // generate_relation_table(builder.data, list, paths, name, &builder.worker);
                    // res += &format!("[[pdf ../{}.pdf]]\n\n", name);
                }
                Drawing::Hasse(list) => {
                    // let drawing_path = make_drawing(
                    //     builder.data,
                    //     &paths.final_dir,
                    //     name,
                    //     &builder.data.get(list.clone()),
                    //     None,
                    // );
                    // res += &include_dot_file(drawing_path, &paths.final_dir);
                }
            };
        }
        // res += &format!("{:?} {}", self.sourcekey, self.time);
        // for s in &self.wrote {
        //     if let Some(val) = s.to_markdown(builder) {
        //         res += &format!("* {}\n", val);
        //     }
        // }
        res
    }
}

fn format_created_by(data: &Data, created_by: &CreatedBy) -> String {
    match &created_by {
        CreatedBy::TransferredFrom(transfer_group, handle) => {
            format!("transferred from {}", handle)
        }
        CreatedBy::TransitiveInclusion(a, b) => {
            format!("by {} and {}", a, b)
        }
        CreatedBy::TransitiveExclusion(a, b) => {
            format!("by {} and {}", a, b)
        }
        CreatedBy::ParallelComposition(a, b) => {
            format!("parallel composition of {} and {}", a, b)
        }
        CreatedBy::SameThroughEquivalence(a, b) => {
            format!("due to equivalence {} and relation {}", a, b)
        }
        CreatedBy::SumInclusion(sumincl) => {
            let sumstr: String = sumincl
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            format!("implied by inclusion of summands {}", sumstr)
        }
        CreatedBy::TransferredFrom(group, a) => {
            // todo name the rule group
            format!("transferred from {}", a)
        }
        CreatedBy::Directly(source) => {
            format!("by [[{}]]", source.id)
        }
        CreatedBy::Todo => "todo".to_string(),
    }
}

// fn format_complexity(data: &Data, cpx: &SourcedCpxInfo) -> String {
//     let (res, children) = match &cpx {
//         SourcedCpxInfo::Unknown => ("unknown to HOPS", "".into()),
//         SourcedCpxInfo::Equal { source } => {
//             let format_a = format_created_by(data, &source.created_by);
//             ("equal", format_a)
//         }
//         SourcedCpxInfo::Inclusion { mn, mx } => {
//             let mut children: Vec<String> = vec![];
//             if let Some((a, sa)) = mn {
//                 children.push(format!(
//                     "lower bound {}",
//                     format_created_by(data, &sa.created_by)
//                 ));
//             }
//             if let Some((a, sa)) = mx {
//                 children.push(format!(
//                     "upper bound {}",
//                     format_created_by(data, &sa.created_by)
//                 ));
//             };
//             ("inclusion", children.join("\n"))
//         }
//         SourcedCpxInfo::Exclusion { source } => {
//             let format_a = format_created_by(data, &source.created_by);
//             ("exclusion", format_a)
//         }
//     };
//     format!("{}: {}", res, children)
// }

// fn relations_list(builder: &Markdown) -> String {
//     let mut res = String::new();
//     for relation in &builder.data.relations {
//         let this_anchor = format!("<span id=\"{}\"></span>", relation.preview().get_url());
//         let this_el = format!("[$]({})", relation.preview().get_url());
//         let join_el = if let Some(reverse_relation) = builder
//             .data
//             .get_relation(&relation.superset, &relation.subset)
//         {
//             format!("[→]({})", reverse_relation.preview().get_url())
//         } else {
//             "→".to_string()
//         };
//         let sub = builder.data.get_set(&relation.subset);
//         let sup = builder.data.get_set(&relation.superset);
//         // let color = &format!( // todo
//         // "color: [[color {}]]\n\n",
//         // relation_color(&sub.related_sets, sub.id.to_string(), &sup.preview()).name()
//         // );
//         res += &format!(
//             "\n{}{} {} {} {}\n",
//             // "\n{}{} {} {} {} -- {}\n",
//             this_anchor,
//             this_el,
//             builder.linkto(&relation.subset),
//             join_el,
//             builder.linkto(&relation.superset),
//             // &format_complexity(builder.data, &relation.cpx)
//         );
//         res += "\n";
//     }
//     res
// }

impl GeneratedPage for GraphClass {
    fn get_page(&self, builder: &Markdown, _paths: &Paths) -> String {
        let mut res = String::new();
        res += &format!("---\ntitle: \"{}\"\n---", self.name_core.name);
        res += &format!("# {}\n\n", self.name_core.name);
        if let Some(abbr) = &self.name_core.abbr {
            res += &format!("abbr: {}\n\n", abbr);
        }
        if !self.name_core.aka.is_empty() {
            res += &format!("aka: {}\n\n", self.name_core.aka.join(", "));
        }
        if !self.tags.is_empty() {
            let tag_strings: Vec<String> = self
                .tags
                .iter()
                .map(|x| builder.linkto(&x.get_link()))
                .collect();
            res += &format!("tags: {}\n\n", tag_strings.join(", "));
        }
        let definition_string = match &self.definition {
            GraphClassDefinition::Text(texts) => texts.join("\n\n"),
            GraphClassDefinition::Intersection(classes) => {
                let class_links: Vec<String> = classes
                    .iter()
                    .map(|c| builder.linkto(&c.get_link()))
                    .collect();
                format!("Intersection of: {}", class_links.join(", "))
            }
            GraphClassDefinition::ParametricGraphClass(pgc) => {
                format!("Instance of {}", builder.linkto(&pgc.get_link()))
            }
            GraphClassDefinition::Parameter(param) => {
                format!(
                    "Graphs where {} is bounded",
                    builder.linkto(&param.get_link())
                )
            }
        };
        res += &format!("**Definition:** {}\n\n", definition_string);
        res += "[[handcrafted]]\n\n";
        res += "\n";
        res
    }
}

impl GeneratedPage for Tag {
    fn get_page(&self, builder: &Markdown, paths: &Paths) -> String {
        let mut res = String::new();
        res += &format!("# {}\n\n", self.name_core.name);
        res += &format!("{}\n\n", self.description);
        let mut table = Table::new(vec!["has this tag"]);
        for w in &self.sets {
            table.add(vec![builder.linkto(w)]);
        }
        res += builder.make_table(table).as_str();
        res
    }
}

pub struct Table {
    pub head: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(head: Vec<&str>) -> Self {
        Self {
            head: head.iter().map(|x| (*x).into()).collect(),
            rows: vec![],
        }
    }
    pub fn add(&mut self, row: Vec<String>) {
        assert_eq!(row.len(), self.head.len());
        self.rows.push(row);
    }
}

pub struct Markdown<'a> {
    pub data: &'a Data,
    pub urls: HashMap<String, Link>,
    pub bibliography: &'a Option<Bibliography>,
    pub worker: Worker,
}

#[derive(Clone, Debug)]
pub enum Mappable {
    Address(Link),
    String(String),
}

impl<'a> Markdown<'a> {
    pub fn new(
        data: &'a Data,
        urls: HashMap<String, Link>,
        bibliography: &'a Option<Bibliography>,
    ) -> Markdown<'a> {
        Markdown {
            data,
            urls,
            bibliography,
            worker: Worker::new(),
        }
    }

    pub fn substitute_custom_markdown(&self, line: &str, map: &HashMap<&str, Mappable>) -> String {
        let pattern = Regex::new(r"\[\[(?P<capturegroup>[^\]]+)\]\]").unwrap();
        let result = pattern.replace_all(line, |caps: &regex::Captures| {
            let part = caps.name("capturegroup");
            match part {
                Some(raw_name) => {
                    let key = raw_name.as_str();
                    match self.process_key(key, map) {
                        Ok(res) => res,
                        Err(error) => {
                            error!("  {}", error);
                            format!("<< substitution error for key {} >>", key)
                        }
                    }
                }
                None => {
                    error!("substitution error");
                    "<< substitution error unparsed key >>".to_string()
                }
            }
        });
        result.into()
    }

    pub fn link_id(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let some_id = keys.pop_front();
        if let Some(id) = some_id {
            match self.urls.get(&id) {
                Some(link) => Ok(format!("[{}]({})", link.name, link.url)),
                _ => Err(MarkdownError::ErrSubstitutingId(id)),
            }
        } else {
            Err(MarkdownError::ErrSubstituting)
        }
    }

    pub fn process_list_key(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let mut content = String::new();
        if let Some(key) = keys.pop_front() {
            match key.as_str() {
                "parameters" => {
                    let mut pars = self.data.parameters.values().collect::<Vec<&Parameter>>();
                    pars.sort_by_key(|x| x.name_core.name.to_lowercase());
                    let mut table = Table::new(vec![
                        "Parameter",
                        &format!(
                            "<a href=\"{}\">*</a>Score",
                            base(&("docs/legend/#score").into())
                        ),
                    ]);
                    for set in &pars {
                        let relstring = progress::bar(set.score, 9);
                        table.add(vec![self.linkto(&set.get_link()), relstring]);
                    }
                    content += self.make_table(table).as_str();
                }
                // "relations" => {
                //     content += &relations_list(self);
                // }
                "graphs" => {
                    let mut graphs = self
                        .data
                        .graph_classes
                        .values()
                        .collect::<Vec<&GraphClass>>();
                    graphs.sort_by_key(|x| &x.name_core.name);
                    let mut table = Table::new(vec![
                        "Graph class",
                        &format!(
                            "<a href=\"{}\">*</a>Score",
                            base(&("docs/legend/#score").into())
                        ),
                    ]);
                    for set in &graphs {
                        let relstring = progress::bar(set.score, 9);
                        table.add(vec![self.linkto(&set.get_link()), relstring]);
                    }
                    content += self.make_table(table).as_str();
                }
                "properties" => {
                    let mut properties = self
                        .data
                        .graph_class_properties
                        .values()
                        .collect::<Vec<&GraphClassProperty>>();
                    properties.sort_by_key(|x| &x.name_core.name);
                    let mut table = Table::new(vec![
                        "Property",
                        &format!(
                            "<a href=\"{}\">*</a>Score",
                            base(&("docs/legend/#score").into())
                        ),
                    ]);
                    for set in &properties {
                        let relstring = progress::bar(set.score, 9);
                        table.add(vec![self.linkto(&set.get_link()), relstring]);
                    }
                    content += self.make_table(table).as_str();
                }
                "sources" => {
                    let mut table = Table::new(vec![
                        "#",
                        "Year",
                        &format!(
                            "<a href=\"{}\">*</a>Score",
                            base(&("docs/legend/#score").into())
                        ),
                        "Source",
                    ]);
                    let mut index = 0;
                    for source in self.data.sources.values() {
                        if let SourceKey::Bibtex {
                            entry_key,
                            name,
                            entry_content,
                        } = &source.sourcekey
                        {
                            let relstring = progress::bar(source.score, 9);
                            table.add(vec![
                                format!("{:0>3}", index),
                                source.time.to_string(),
                                relstring,
                                self.linkto(&source.get_link()),
                            ]);
                            index += 1;
                        }
                    }
                    content += self.make_table(table).as_str();
                }
                "tags" => {
                    for tag in self.data.tags.values() {
                        content += &format!("* {}\n", self.linkto(&tag.get_link()));
                    }
                    content += "\n\n";
                }
                unknown => {
                    return Err(MarkdownError::IdNotFound(unknown.into()));
                }
            }
            Ok(content)
        } else {
            Err(MarkdownError::MissingId)
        }
    }

    pub fn process_key(&self, input: &str, map: &HashMap<&str, Mappable>) -> Result<String> {
        let mut words: LinkedList<String> = LinkedList::new();
        for word in input.split(' ') {
            words.push_back(word.into());
        }
        if let Some(first_word) = words.pop_front() {
            match first_word.as_str() {
                "list" => self.process_list_key(&mut words),
                "dot" => self.embed_dot(&mut words),
                "zoomdot" => self.embed_zoomable_dot(&mut words),
                "pdf" => self.embed_pdf(&mut words),
                "color" => self.color(&mut words),
                unknown => {
                    if let Some(res) = map.get(unknown) {
                        match res {
                            Mappable::String(str) => Ok(str.clone()),
                            Mappable::Address(address) => Ok(self.linkto(address)),
                        }
                    } else {
                        words.push_front(unknown.into());
                        self.link_id(&mut words)
                    }
                }
            }
        } else {
            Err(MarkdownError::EmptyKey)
        }
    }

    pub fn linkto(&self, link: &Link) -> String {
        format!("[{}]({})", link.name, link.url)
    }

    pub fn make_row(&self, row: &Vec<String>) -> String {
        let mut content = String::new();
        content += "|";
        for cell in row {
            content += &format!(r" {cell} |");
        }
        content += "\n";
        content
    }

    pub fn make_table(&self, table: Table) -> String {
        let mut content = String::new();
        content += &self.make_row(&table.head);
        content += &self.make_row(&table.head.iter().map(|x| "---".to_string()).collect());
        for row in table.rows {
            content += &self.make_row(&row.iter().map(|x| x.into()).collect());
        }
        content
    }

    pub fn embed_dot(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let filename: String = keys.pop_front().unwrap();
        Ok(format!(
            "<p><div id=\"{}\" class=\"svg-diagram\"></div></p>\
            <script>\
            Viz.instance().then(function(viz) {{\
                fetch('{}')\
                    .then(response => response.text())\
                    .then((data) => {{\
                        var svg = viz.renderSVGElement(data);\
                        svg.setAttribute(\"width\", \"100%\");\
                        svg.setAttribute(\"height\", \"300pt\");\
                        document.getElementById(\"{}\").appendChild(svg);\
                    }})\
            }});\
            </script>\n\n",
            filename, filename, filename
        ))
    }

    pub fn embed_zoomable_dot(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let filename: String = keys.pop_front().unwrap();
        Ok(format!(
            "<p><div id=\"{}\" class=\"svg-diagram zoomable\"></div></p>\
            <script type=\"module\">\
            import {{ initializeSvgToolbelt }} from '{}';\
            Viz.instance().then(function(viz) {{\
                fetch('{}')\
                    .then(response => response.text())\
                    .then((data) => {{\
                        var svg = viz.renderSVGElement(data);\
                        document.getElementById(\"{}\").appendChild(svg);\
                        initializeSvgToolbelt('.zoomable', {{\
                            zoomStep: 0.3,\
                            minScale: 1,\
                            maxScale: 5,\
                        }});\
                    }})\
            }});\
            </script>\n\n",
            filename, "/parameters/svg-toolbelt.esm.js", filename, filename
        ))
    }

    pub fn embed_pdf(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let name: String = keys.pop_front().unwrap();
        let default = 480;
        let height: u32 = keys
            .pop_front()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap_or(default);
        Ok(format!(
            "\n<object data=\"{}\" type=\"application/pdf\" class=\"pdf-table-wrapper\" height=\"{}px\">\
            <embed src=\"{}\">\
            <p>This browser does not support PDFs. Please download the PDF to view it: <a href=\"{}\">Download PDF</a>.</p>\
            </embed>\
            </object>\n\n",
            name, height, name, name
        ))
    }

    pub fn color(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let colorname: String = keys.pop_front().unwrap();
        let color = Color::from_str(&colorname);
        Ok(format!("<span style=\"color:{}\">■</span>", color.hex()))
    }
}
