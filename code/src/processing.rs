//! Gets raw data and prepares interconnected data.

use std::{collections::{HashMap, HashSet, VecDeque}, path::PathBuf};
use biblatex::Bibliography;

use crate::{raw::{RawSet, RawSource, RawSourceKey}, simpleindex::SimpleIndex, data::{Data, Source, Set, RawData, Date, ShowedFact, SourceSubset, Showed, SourceKey}, file};

pub fn bfs<F>(start: &RawSet, get_neighbors: F) -> Vec<RawSet>
where
    F: Fn(&RawSet) -> Vec<RawSet>,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    queue.push_back(start.clone());
    visited.insert(start.clone());
    while let Some(current) = queue.pop_front() {
        let neighbors = get_neighbors(&current);
        result.push(current);
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                queue.push_back(neighbor);
            }
        }
    }
    result
}

pub fn anti_bfs<F>(sets: &Vec<RawSet>, get_antineighbors: F) -> Vec<RawSet>
where
    F: Fn(&RawSet) -> Vec<RawSet>,
{
    let unique: HashSet<_> = sets.into_iter().flat_map(|s| get_antineighbors(s)).collect();
    unique.into_iter().collect()
}

pub fn process_set(set: &RawSet, help: &SimpleIndex, data: &RawData, sources: &HashMap<RawSource, Source>) -> Set {
    let mut timeline_map: HashMap<RawSource, Vec<Showed>> = HashMap::new();
    for (source, showed) in &data.factoids {
        let should_save = match &showed.fact {
            ShowedFact::Relation(relation) if &relation.superset == set || &relation.subset == set => true,
            ShowedFact::Definition(defined_set) if defined_set == set => true,
            _ => false,
        };
        if should_save {
            let arr = timeline_map.entry(source.clone()).or_insert(vec![]);
            arr.push(showed.clone());
        }
    }
    let mut timeline: Vec<SourceSubset> = timeline_map.into_iter()
        .map(|(raw, showed_vec)| {
            let source = sources.get(&raw).unwrap();
            SourceSubset {
                raw,
                id: source.id.clone(),
                sourcekey: source.sourcekey.clone(),
                showed: showed_vec,
                time: source.time.clone(),
            }
        })
    .collect();
    timeline.sort_by_key(|subset| subset.time.clone());
    let supersets = bfs(&set, |x| help.get_supersets(x));
    let subsets = bfs(&set, |x| help.get_subsets(x));
    let super_exclusions = anti_bfs(&subsets, |x| help.get_antisupersets(&x));
    let sub_exclusions = anti_bfs(&supersets, |x| help.get_antisubsets(&x));
    let unknown = vec![]; // todo
    // let mut providers = vec![]; // todo
    // for (rawset, num) in &data.isgci {
        // if *rawset == *set {
            // providers.push(Provider::Isgci(*num));
        // }
    // }
    // let transfers = HashMap::new(); // todo
    Set{
        raw: set.clone(),
        id: set.id.clone(),
        name: set.name.clone(),
        kind: set.kind.clone(),
        // providers,
        timeline,
        // transfers,
        supersets: prepare_extremes(supersets, help),
        subsets: prepare_extremes(subsets, help),
        super_exclusions: prepare_extremes(super_exclusions, help),
        sub_exclusions: prepare_extremes(sub_exclusions, help),
        unknown: prepare_extremes(unknown, help),
    }
}

/// Enhance the source key with additional information
pub fn process_sourcekey(sourcekey: &RawSourceKey, bibtex: &Option<Bibliography>) -> SourceKey {
    match sourcekey {
        RawSourceKey::Bibtex { key } => {
            let formatted_citation = match bibtex {
                Some(bib) => {
                    let entry = bib.get(&key).unwrap();
                    format!("{:?}", entry) // fixme
                },
                None => {
                    "missing".into()
                },
            };
            SourceKey::Bibtex { key: key.clone(), formatted_citation }
        },
        RawSourceKey::Online { url } => { SourceKey::Online { url: url.clone() } },
        RawSourceKey::Unknown => { SourceKey::Unknown }
    }
}

pub fn process_source(source: &RawSource, help: &SimpleIndex, bibliography: &Option<Bibliography>) -> Source {
    let mut showed = vec![]; // todo
    let mut res = Source {
        raw: source.clone(),
        id: source.id.clone(),
        sourcekey: process_sourcekey(&source.rawsourcekey, bibliography),
        showed,
        time: Date{}, // todo
    };
    res
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Sets {
    pub minimal: Vec<RawSet>,
    pub maximal: Vec<RawSet>,
    pub all: Vec<RawSet>,
}

pub fn prepare_extremes(raw_set: Vec<RawSet>, data: &SimpleIndex) -> Sets {
    let mut minimal = Vec::new();
    let mut maximal = Vec::new();
    let mut all = Vec::new();
    for i in 0..raw_set.len() {
        let current_set = &raw_set[i];
        let mut is_maximal = true;
        let mut is_minimal = true;
        for j in 0..raw_set.len() {
            if i != j {
                let other_set = &raw_set[j];
                if data.is_below(current_set, other_set) {
                    is_maximal = false;
                }
                if data.is_below(other_set, current_set) {
                    is_minimal = false;
                }
            }
        }
        if is_maximal {
            maximal.push(current_set.clone());
        }
        if is_minimal {
            minimal.push(current_set.clone());
        }
        all.push(current_set.clone());
    }
    Sets { minimal, maximal, all }
}

pub fn process_raw_data(rawdata: &RawData, handcrafted_dir: &PathBuf) -> Data {
    let bibtex_path = handcrafted_dir.join("main.tex");
    let bibliography_res = file::read_file_content(&bibtex_path);
    let bibliography = match bibliography_res {
        Ok(bibliography_str) => {
            Some(Bibliography::parse(&bibliography_str).unwrap())
        },
        Err(_) => {
            println!("cannot load bibliography from {:?}", handcrafted_dir);
            None
        }
    };
    let mut links = HashMap::new();
    let simpleindex = SimpleIndex::new(rawdata);
    let mut sources = vec![];
    let mut source_keys: HashMap<RawSource, Source> = HashMap::new();
    for rawsource in &rawdata.sources {
        let source = process_source(&rawsource, &simpleindex, &bibliography);
        source_keys.insert(rawsource.clone(), source.clone());
        sources.push(source);
    }
    let mut parameters = vec![];
    for set in &rawdata.parameters {
        parameters.push(process_set(&set, &simpleindex, &rawdata, &source_keys));
    }
    parameters.sort_by_key(|x|x.name.clone());
    let mut graph_classes = vec![];
    for set in &rawdata.graph_classes {
        graph_classes.push(process_set(&set, &simpleindex, &rawdata, &source_keys));
    }
    graph_classes.sort_by_key(|x|x.name.clone());
    Data {
        links,
        parameters,
        graph_classes,
        sources,
    }
}
