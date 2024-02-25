//! Given raw data this module enriches and interconnects it.

use std::{collections::{HashMap, HashSet, VecDeque}, path::PathBuf};
use biblatex::Bibliography;

use crate::{input::raw::{RawData, RawRelation, RawSet, RawSource, RawSourceKey}, data::simpleindex::SimpleIndex, data::data::{Data, Date, Relation, Set, Showed, ShowedFact, Source, SourceKey, SourceSubset}, file};

pub fn bfs<F>(start: &RawSet, get_neighbors: F, include_start: bool) -> Vec<RawSet>
where
    F: Fn(&RawSet) -> Vec<RawSet>,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    queue.push_back(start.clone());
    visited.insert(start.clone());
    if include_start {
        result.push(start.clone());
    }
    while let Some(current) = queue.pop_front() {
        let neighbors = get_neighbors(&current);
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                queue.push_back(neighbor.clone());
                result.push(neighbor.clone());
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

pub fn bfs_limit_distance(set: &Set, data: &Data, distance: usize) -> HashSet<RawSet> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(set.raw.clone());
    queue.push_back((set.raw.clone(), 0));
    while let Some((raw_set, current_distance)) = queue.pop_front() {
        let set = data.get_set(&raw_set);
        if current_distance >= distance {
            continue;
        }
        for subset in &set.subsets.maximal {
            if visited.insert(subset.clone()) {
                queue.push_back((subset.clone(), current_distance + 1));
            }
        }
        for superset in &set.supersets.minimal {
            if visited.insert(superset.clone()) {
                queue.push_back((superset.clone(), current_distance + 1));
            }
        }
    }
    visited
}

/// Given a RawSet create a full Set with all the information.
pub fn process_set(set: &RawSet, help: &SimpleIndex, data: &RawData, sources: &HashMap<RawSource, Source>) -> Set {
    let mut timeline_map: HashMap<RawSource, Vec<Showed>> = HashMap::new();
    for (raw_source, showed) in &data.factoids {
        let should_save = match &showed.fact {
            ShowedFact::Relation(relation) if &relation.superset == set || &relation.subset == set => true,
            ShowedFact::Definition(defined_set) if defined_set == set => true,
            _ => false,
        };
        if should_save {
            let arr = timeline_map.entry(raw_source.clone()).or_insert(vec![]);
            arr.push(showed.clone());
        }
    }
    let mut timeline: Vec<SourceSubset> = timeline_map.into_iter()
        .map(|(raw, showed_vec)| {
            let source = sources.get(&raw).expect(
                &format!("A raw source {} does not have a processed source. Use create.source() to add new sources.", raw.id)
                );
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
    let supersets = help.get_supersets(&set);
    let subsets = help.get_subsets(&set);
    let super_exclusions = help.get_antisupersets(&set);
    let sub_exclusions = help.get_antisubsets(&set);
    let mut all_parameters = HashSet::new();
    for par in &data.sets {
        all_parameters.insert(par.clone());
    }
    for s in &supersets {
        all_parameters.remove(&s);
    }
    for s in &subsets {
        all_parameters.remove(&s);
    }
    let unknown: Vec<RawSet> = all_parameters.iter().cloned().collect();
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

/// Enrich the source key with additional information
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
    let sourcekey = process_sourcekey(&source.rawsourcekey, bibliography);
    let mut res = Source {
        raw: source.clone(),
        id: source.id.clone(),
        sourcekey,
        showed,
        time: Date{year: None, month: None, day: None}, // todo
    };
    res
}

/// Minimal and maximal refer to inclusion-wise extremes. An isolated element
/// would be included in all three sets.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
                if data.first_subset_of_second(current_set, other_set) {
                    is_minimal = false;
                }
                if data.first_subset_of_second(other_set, current_set) {
                    is_maximal = false;
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

fn load_bibliography(bibliography_file: &PathBuf) -> Option<Bibliography> {
    let bibliography_res = file::read_file_content(&bibliography_file);
    match bibliography_res {
        Ok(bibliography_str) => {
            Some(Bibliography::parse(&bibliography_str).unwrap())
        },
        Err(error) => {
            println!("cannot load bibliography from {:?}", bibliography_file);
            println!("{:?}", error);
            None
        }
    }
}

/// Get list of things we know about pairwise relations. Go through it and make
/// sure each relation is represented only once and that that representation
/// contains all the information one may need.
fn process_relations(raw_relations: Vec<RawRelation>) -> Vec<Relation> {
    let mut relations = Vec::new();
    let mut relation_idx: HashMap<(RawSet, RawSet), Relation> = HashMap::new();
    for (idx, raw_relation) in raw_relations.iter().enumerate() {
        let key = (raw_relation.subset.clone(), raw_relation.superset.clone());
        if let Some(value) = relation_idx.get(&key) {
            // todo merge relations
        } else {
            relation_idx.insert(key, Relation::new(raw_relation));
        }
    }
    relations
}

pub fn process_raw_data(rawdata: &RawData, bibliography_file: &PathBuf) -> Data {
    // todo, create urls that markdowns can use; these are maps from id to address of the entity
    let simpleindex = SimpleIndex::new(rawdata);
    let bibliography = load_bibliography(&bibliography_file);
    let mut sources = vec![];
    let mut source_keys: HashMap<RawSource, Source> = HashMap::new();
    for rawsource in &rawdata.sources {
        let source = process_source(&rawsource, &simpleindex, &bibliography);
        source_keys.insert(rawsource.clone(), source.clone());
        sources.push(source);
    }
    sources.reverse();
    let mut sets = vec![];
    for set in &rawdata.sets {
        sets.push(process_set(&set, &simpleindex, &rawdata, &source_keys));
    }
    let mut raw_relations = Vec::new();
    for (raw_source, showed) in &rawdata.factoids {
        match &showed.fact {
            ShowedFact::Relation(rel) => raw_relations.push(rel.clone()),
            ShowedFact::Citation(_) => (),
            ShowedFact::Definition(_) => (),
        }
    }
    let relations = process_relations(raw_relations);
    Data::new(sets, relations, HashMap::new(), sources)
}