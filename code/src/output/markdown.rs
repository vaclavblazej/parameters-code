//! Given the processed data generate markdown pages.


use std::collections::{LinkedList, HashMap, HashSet, VecDeque};
use std::{env, fmt};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use biblatex::Bibliography;
use log::error;
use regex::Regex;

use crate::data::data::{Data, Linkable, ProviderLink, Relation, Set, Showed, ShowedFact, Source, Tag};
use crate::data::preview::{PreviewRelation, PreviewSet, PreviewSource, PreviewSourceKey, PreviewTag, PreviewType};
use crate::general::enums::{CreatedBy, Page, SourceKey};
use crate::file;
use crate::general::progress;

use super::color::{relation_color, Color};
use super::diagram::{make_focus_drawing, make_subset_drawing};
use super::to_markdown::ToMarkdown;

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

fn html_base(id: &String) -> String {
    format!("{{{{< base >}}}}html/{}", id)
}

fn base(id: &String) -> String {
    format!("{{{{< base >}}}}{}", id)
}

impl Linkable for ProviderLink {
    fn get_url(&self) -> String {
        self.url.clone()
    }
    fn get_name(&self) -> String {
        self.provider.name.clone()
    }
}

impl Linkable for PreviewRelation {
    fn get_url(&self) -> String {
        html_base(&self.id)
    }
    fn get_name(&self) -> String {
        format!("{} → {}", self.subset.name, self.superset.name)
    }
}

impl Linkable for PreviewTag {
    fn get_url(&self) -> String {
        html_base(&self.id)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Linkable for PreviewSet {
    fn get_url(&self) -> String {
        html_base(&self.id)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Linkable for PreviewSource {
    fn get_url(&self) -> String {
        match &self.sourcekey {
            SourceKey::Bibtex { key: _, name: _, entry: _ } => html_base(&self.id),
            SourceKey::Online { url } => url.clone(),
            SourceKey::Other { name: _, description: _ } => html_base(&self.id),
        }
    }
    fn get_name(&self) -> String {
        match &self.sourcekey {
            SourceKey::Bibtex { key, name, entry: _ } => name.clone().unwrap_or(key.clone()),
            SourceKey::Online { url } => url.clone(),
            SourceKey::Other { name, description: _ } => name.into(),
        }
    }
}

pub trait GeneratedPage {
    fn get_page(&self, builder: &Markdown, final_dir: &PathBuf, working_dir: &PathBuf) -> String;
}

fn copy_file_to_final_location(file: &PathBuf, to_directory: &PathBuf) {
    assert!(file.exists());
    assert!(file.is_file());
    let filename = file.file_name().expect("Result file has no name").to_owned();
    fs::create_dir_all(&to_directory);
    assert!(to_directory.is_dir());
    let final_path = to_directory.join(&filename);
    fs::copy(&file, &final_path).expect("Failed to copy result to final directory");
}

impl GeneratedPage for Set {
    fn get_page(&self, builder: &Markdown, final_dir: &PathBuf, working_dir: &PathBuf) -> String {
        let mut res = String::new();
        res += &format!("# {}\n\n", self.name);
        if let Some(abbr) = &self.abbr {
            res += &format!("abbr: {}\n\n", abbr);
        }
        if !self.aka.is_empty() {
            res += &format!("aka: {}\n\n", self.aka.join(", "));
        }
        if !self.tags.is_empty() {
            let tag_strings: Vec<String> = self.tags.iter().map(|x|builder.linkto(x)).collect();
            res += &format!("tags: {}\n\n", tag_strings.join(", "));
        }
        let equivalent_strings: Vec<String> = self.equivsets.iter().filter(|x|x.id != self.id).map(|x|builder.linkto(x)).collect();
        if !equivalent_strings.is_empty() {
            res += &format!("equivalent to: {}\n\n", equivalent_strings.join(", "));
        }
        if !self.providers.is_empty() {
            let provider_strings: Vec<String> = self.providers.iter().map(|x|builder.linkto(x)).collect();
            res += &format!("providers: {}\n\n", provider_strings.join(", "));
        }
        res += "[[handcrafted]]\n\n";
        res += &match make_focus_drawing(&builder.data, self, 2, working_dir) {
            Ok(result_dot_file) => {
                copy_file_to_final_location(&result_dot_file, &final_dir.join("html"));
                let filename = result_dot_file.file_name().unwrap().to_string_lossy();
                format!("[[dot ../{}]]", filename)
            },
            Err(e) => {
                error!("{:?}", e);
                format!("{:?}", e)
            },
        };
        for (name, drawn_sets) in [
            (&format!("dif_inclusions_{}", self.id), builder.data.sets.iter().filter(|x| x.typ != self.typ).collect()),
            (&format!("same_inclusions_{}", self.id), builder.data.sets.iter().filter(|x| x.typ == self.typ).collect()),
        ]{
            res += &match make_subset_drawing(name, &builder.data, self, drawn_sets, working_dir) {
                Ok(result_pdf_file) => {
                    copy_file_to_final_location(&result_pdf_file, &final_dir.join("html"));
                    let filename = result_pdf_file.file_name().unwrap().to_string_lossy();
                    format!("[[dot ../{}]]", filename)
                },
                Err(e) => {
                    error!("{:?}", e);
                    format!("{:?}", e)
                },
            };
        }
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
        res += &format!("---\n\n## Relations\n\n");
        let mut relation_table = Table::new(vec!["Other", "", "Relation from", "Relation to"]);
        for set in &builder.data.sets {
            let relation_fr_el: String = match builder.data.get_relation(&set.preview, &self.preview){
                None => "unknown to HOPS".into(),
                Some(rel) => {
                    let name = rel.preview.short_description(builder);
                    format!("[{}]({})", name, rel.preview.get_url())
                }
            };
            let relation_to_el: String = match builder.data.get_relation(&self.preview, &set.preview){
                None => "unknown to HOPS".into(),
                Some(rel) => {
                    let name = rel.preview.short_description(builder);
                    format!("[{}]({})", name, rel.preview.get_url())
                }
            };
            let colorname = relation_color(self, set).name();
            relation_table.add(
                vec![
                builder.linkto(&set.preview),
                // the hidden span makes the color column sortable
                format!("<span style=\"display:none\">{}</span>[[color {}]]", colorname, colorname),
                relation_fr_el,
                relation_to_el,
                ]);
        }
        res += builder.make_table(relation_table).as_str();
        res += "\n";
        // make_subset_drawing
        let sources_timeline = &self.timeline;
        if !sources_timeline.is_empty() {
            res += &format!("---\n\n## Results\n\n");
            for source in sources_timeline {
                if let Some(val) = source.to_markdown(builder){
                    res += &val;
                }
            }
            res += "\n";
        }
        res
    }
}

impl GeneratedPage for Source {
    fn get_page(&self, builder: &Markdown, final_dir: &PathBuf, working_dir: &PathBuf) -> String {
        let mut res = String::new();
        match &self.sourcekey {
            SourceKey::Bibtex { key, name, entry } => {
                res += &format!("# {}\n\n", self.preview.get_name());
                if let Some(somebib) = builder.bibliography {
                    if let Some(val) = somebib.get(key) {
                        if let Ok(doi) = val.doi() {
                            let doi_url = format!("https://www.doi.org/{}", doi);
                            res += &format!("[{}]({})\n\n", doi_url, doi_url);
                        } else if let Ok(url) = val.url() {
                            res += &format!("[{}]({})\n\n", url, url);
                        }
                        // todo print the original (unformatted) biblatex citation
                        res += &format!("```bibtex\n{}\n```\n", val.to_biblatex_string());
                    } else {
                        error!("unable to load {} from main.bib", key);
                        res += &format!("an error occured while loading the bibtex entry for `{}`", key);
                    }
                }
            },
            SourceKey::Other { name, description } => {
                res += &format!("# {}\n\n", name);
                res += &format!("{}\n\n", description);
            },
            SourceKey::Online { url } => {
                res += &format!("# Online source {}\n\n", self.id);
            },
        }
        // res += &format!("{:?} {}", self.sourcekey, self.time);
        for s in &self.showed {
            if let Some(val) = s.to_markdown(builder) {
                res += &format!("* {}\n", val);
            }
        }
        res
    }
}

// todo
// fn format_created_by(data: &Data, relation: &Relation, indent: usize, set: &mut HashSet<PreviewRelation>) -> String{
    // let (res, children) = if set.contains(&relation.preview) {
        // match &relation.cpx {
            // CreatedBy::Directly => ("was proved directly", "".into()),
            // CreatedBy::Todo => ("information missing in HOPS", "".into()),
            // CreatedBy::TransferredFrom(a, b) => {
                // let rel = data.get_relation(&b.subset, &b.superset).unwrap();
                // ("relation implied from another relation", format_created_by(data, &rel, indent+1, set))
            // },
            // CreatedBy::TransitiveInclusion(a, b) => {
                // let rel_a = data.get_relation(&a.subset, &a.superset).unwrap();
                // let rel_b = data.get_relation(&b.subset, &b.superset).unwrap();
                // let format_a = format_created_by(data, &rel_a, indent+1, set);
                // let format_b = format_created_by(data, &rel_b, indent+1, set);
                // ("relation implied from relations", format!("{}\n{}", format_a, format_b))
            // },
            // CreatedBy::TransitiveExclusion(a, b) => {
                // let rel_a = data.get_relation(&a.subset, &a.superset).unwrap();
                // let rel_b = data.get_relation(&b.subset, &b.superset).unwrap();
                // let format_a = format_created_by(data, &rel_a, indent+1, set);
                // let format_b = format_created_by(data, &rel_b, indent+1, set);
                // ("relation implied from relations", format!("{}\n{}", format_a, format_b))
            // },
            // // _ => ("", "".into()),
        // }
    // } else {
        // panic!("cyclic dependence for {:?}", relation);
        // // ("a cyclic dependence, that's not good", "".into())
    // };
    // format!("{}* {}\n{}", " ".repeat(4*indent), res, children)
// }

impl GeneratedPage for Relation {
    fn get_page(&self, builder: &Markdown, final_dir: &PathBuf, working_dir: &PathBuf) -> String {
        let mut res = String::new();
        let join_el =
            if let Some(reverse_relation) = builder.data.get_relation(&self.superset, &self.subset) {
                format!("[→]({})", reverse_relation.preview.get_url())
            } else {
                format!("→")
            };
        res += &format!("# {} {} {}\n\n", builder.linkto(&self.subset), join_el, builder.linkto(&self.superset));
        let sub = builder.data.get_set(&self.subset);
        let sup = builder.data.get_set(&self.superset);
        res += &format!("color: [[color {}]]\n\n", relation_color(&sub, &sup).name());
        // res += &format_created_by(&builder.data, &self, 0, &mut HashSet::new()); // todo
        // pub id: String,
        // pub preview: PreviewRelation,
        // /// If inclusion, then subset is the parameter above which is potentially bigger for the same graph.
        // pub subset: PreviewSet,
        // /// If inclusion, then superset is the parameter below which is potentially smaller for the same graph.
        // pub superset: PreviewSet,
        // pub cpx: CpxInfo,
        // pub created_by: CreatedBy,
        // pub essential: bool,
        res
    }
}

impl GeneratedPage for Tag {
    fn get_page(&self, builder: &Markdown, final_dir: &PathBuf, working_dir: &PathBuf) -> String {
        let mut res = String::new();
        res += &format!("# {}\n\n", self.name);
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
            head: head.iter().map(|x|(*x).into()).collect(),
            rows: vec![],
        }
    }
    pub fn add(&mut self, row: Vec<String>) {
        assert_eq!(row.len(), self.head.len());
        self.rows.push(row);
    }
}

#[derive(Clone, Debug)]
pub struct Address {
    pub name: String,
    pub url: String,
}
impl Linkable for Address {
    fn get_url(&self) -> String { self.url.clone() }
    fn get_name(&self) -> String { self.name.clone() }
}

pub struct Markdown<'a> {
    pub data: &'a Data,
    pub urls: HashMap<String, Box<dyn Linkable>>,
    pub bibliography: &'a Option<Bibliography>,
}

#[derive(Clone, Debug)]
pub enum Mappable {
    Address(Address),
    String(String),
}

impl<'a> Markdown<'a> {

    pub fn new(data: &'a Data, urls: HashMap<String, Box<dyn Linkable>>, bibliography: &'a Option<Bibliography>) -> Markdown<'a> {
        Markdown { data, urls, bibliography }
    }

    pub fn substitute_custom_markdown(&self, line: &String, map: &HashMap<&str, Mappable>) -> String {
        let pattern = Regex::new(r"\[\[(?P<capturegroup>[^\]]+)\]\]").unwrap();
        let result = pattern.replace_all(line, |caps: &regex::Captures| {
            let part = caps.name("capturegroup");
            match part {
                Some(raw_name) => {
                    let key = raw_name.as_str();
                    match self.process_key(&key.into(), map) {
                        Ok(res) => res,
                        Err(error) => {
                            error!("  {}", error.to_string());
                            "<< substitution error >>".into()
                        }
                    }
                },
                None => {
                    error!("substitution error");
                    "<< substitution error >>".into()
                },
            }
        });
        result.into()
    }

    pub fn link_id(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let some_id = keys.pop_front();
        if let Some(id) = some_id {
            if let Some(link) = self.urls.get(&id) {
                Ok(format!("[{}]({})", link.get_name(), link.get_url()))
            } else {
                Err(MarkdownError::ErrSubstitutingId(id.into()))
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
                    let mut pars = self.data.sets.iter().filter(|&s| s.typ == PreviewType::Parameter).collect::<Vec<&Set>>();
                    pars.sort_by_key(|x|x.name.to_lowercase());
                    let mut table = Table::new(vec!["Parameter", &format!("<a href=\"{}\">*</a>Relevance", base(&(*"docs/legend/#relevance").into()))]);
                    for set in &pars {
                        let relstring = progress::bar(set.preview.relevance, 9);
                        table.add(vec![self.linkto(&set.preview), relstring]);
                    }
                    content += self.make_table(table).as_str();
                }
                "graphs" => {
                    let mut graphs = self.data.sets.iter().filter(|&s| s.typ == PreviewType::GraphClass).collect::<Vec<&Set>>();
                    graphs.sort_by_key(|x|&x.name);
                    let mut table = Table::new(vec!["Graph class", &format!("<a href=\"{}\">*</a>Relevance", base(&(*"docs/legend/#relevance").into()))]);
                    for set in &graphs {
                        let relstring = progress::bar(set.preview.relevance, 9);
                        table.add(vec![self.linkto(&set.preview), relstring]);
                    }
                    content += self.make_table(table).as_str();
                },
                "sources" => {
                    let mut table = Table::new(vec!["Order", "Year", "Source"]);
                    for (index, source) in self.data.sources.iter().enumerate() {
                        if let SourceKey::Bibtex { key, name, entry } = &source.sourcekey {
                            table.add(vec![format!("{:0>3}", index), source.time.to_string(), self.linkto(&source.preview)]);
                        }
                    }
                    content += self.make_table(table).as_str();
                },
                "tags" => {
                    for tag in &self.data.tags {
                        content += &format!("* {}\n", self.linkto(&tag.preview));
                    }
                    content += "\n\n";
                },
                unknown => {
                    return Err(MarkdownError::IdNotFound(unknown.into()));
                },
            }
            Ok(content)
        } else {
            Err(MarkdownError::MissingId)
        }
    }

    pub fn process_key(&self, input: &String, map: &HashMap<&str, Mappable>) -> Result<String> {
        let mut words: LinkedList<String> = LinkedList::new();
        for word in input.split(' ') {
            words.push_back(word.into());
        }
        if let Some(first_word) = words.pop_front() {
            match first_word.as_str() {
                "list" => self.process_list_key(&mut words),
                "dot" => self.embed_dot(&mut words),
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
                },
            }
        } else {
            Err(MarkdownError::EmptyKey)
        }
    }

    pub fn linkto(&self, linkable: &dyn Linkable) -> String {
        format!("[{}]({})", linkable.get_name(), linkable.get_url())
    }

    pub fn make_row(&self, row: &Vec<String>) -> String {
        let mut content = String::new();
        content += "|";
        for cell in row { content += &format!(r" {cell} |"); }
        content += "\n";
        content
    }

    pub fn make_table(&self, table: Table) -> String {
        let mut content = String::new();
        content += &self.make_row(&table.head);
        content += &self.make_row(&table.head.iter().map(|x|format!("---")).collect());
        for row in table.rows {
            content += &self.make_row(&row.iter().map(|x|x.into()).collect());
        }
        content
    }

    pub fn embed_dot(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let filename: String = keys.pop_front().unwrap().into();
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
                        svgPanZoom(svg, {{\
                            zoomEnabled: true,\
                            zoomScaleSensitivity: 0.3,\
                            minZoom: 0.9,\
                            maxZoom: 6,\
                        }});\
                    }})\
            }});\
            </script>\n\n",
            filename, filename, filename
        ))
    }

    pub fn embed_pdf(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let name: String = keys.pop_front().unwrap().into();
        let default = 480;
        let height: u32 = keys.pop_front().and_then(|x| x.parse::<u32>().ok()).unwrap_or(default);
        Ok(format!(
            "\n<object data=\"{}\" type=\"application/pdf\" width=\"100%\" height=\"{}px\">\
            <embed src=\"{}\">\
            <p>This browser does not support PDFs. Please download the PDF to view it: <a href=\"{}\">Download PDF</a>.</p>\
            </embed>\
            </object>\n\n",
            name, height, name, name
            ))
    }

    pub fn color(&self, keys: &mut LinkedList<String>) -> Result<String> {
        let colorname: String = keys.pop_front().unwrap().into();
        let color = Color::from_str(&colorname);
        Ok(format!("<span style=\"color:{}\">■</span>", color.hex()))
    }

    pub fn make_page(&self, pagename: &str, content: String) {
        let mut final_markdown = String::new();
        final_markdown += "---\n";
        final_markdown += "layout: \"single\"\n";
        final_markdown += "title: \"Hierarchy of Parameters\"\n";
        final_markdown += "---\n";
        final_markdown += "<!--this is a generated file-->\n\n";
        final_markdown += &content;
        let filename = format!("./build/{}", pagename);
        let mut file = fs::File::create(&filename).expect("Unable to create file");
        file.write_all(final_markdown.as_bytes()).expect("Unable to write data to file");
        // println!("Saved website into {}", filename);
        // builder.make_page("_index.md", builder.landing_page_keys(&data));
        // for entry in &data.parameters {
            // builder.make_page(&format!("{}.md", entry.id), builder.format_set(entry));
        // }
    }

}
