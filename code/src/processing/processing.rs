//! Given raw data this module enriches and interconnects it.

use std::{collections::{HashMap, HashSet, VecDeque}, path::PathBuf};
use biblatex::{Bibliography, Entry};

use crate::{data::{data::{Data, Date, Relation, Set, Showed, ShowedFact, Source, SourceSubset}, simpleindex::SimpleIndex}, general::file};
use crate::general::enums::SourceKey;
use crate::input::raw::*;
use crate::data::preview::*;


pub fn bfs<F>(start: &PreviewSet, get_neighbors: F, include_start: bool) -> Vec<PreviewSet>
where
    F: Fn(&PreviewSet) -> Vec<PreviewSet>,
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

pub fn anti_bfs<F>(sets: &Vec<PreviewSet>, get_antineighbors: F) -> Vec<PreviewSet>
where
    F: Fn(&PreviewSet) -> Vec<PreviewSet>,
{
    let unique: HashSet<_> = sets.into_iter().flat_map(|s| get_antineighbors(s)).collect();
    unique.into_iter().collect()
}

pub fn bfs_limit_distance(set: &Set, data: &Data, distance: usize) -> HashSet<PreviewSet> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(set.preview.clone());
    queue.push_back((set.preview.clone(), 0));
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
pub fn process_set(set: PreviewSet, help: &SimpleIndex, data: &RawData, sources: &HashMap<RawSource, Source>) -> Set {
    let mut timeline_map: HashMap<RawSource, Vec<RawShowed>> = HashMap::new();
    for (raw_source, showed) in &data.factoids {
        let should_save = match &showed.fact {
            RawShowedFact::Relation(relation) if relation.superset.id == set.id || relation.subset.id == set.id => true,
            RawShowedFact::Definition(defined_set) if defined_set.id == set.id => true,
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
                preview: raw.preprocess(),
                id: source.id.clone(),
                sourcekey: source.sourcekey.clone(),
                showed: showed_vec.into_iter().map(|x|x.into()).collect(),
                time: source.time.clone(),
            }
        })
    .collect();
    timeline.sort_by_key(|subset| subset.time.clone());
    let supersets = help.get_supersets(&set);
    let subsets = help.get_subsets(&set);
    let super_exclusions = help.get_antisupersets(&set);
    let sub_exclusions = help.get_antisubsets(&set);
    let mut unknown_map: HashSet<PreviewSet> = HashSet::new();
    for par in &data.sets {
        unknown_map.insert(par.clone().into());
    }
    for s in &supersets {
        unknown_map.remove(&s);
    }
    for s in &subsets {
        unknown_map.remove(&s);
    }
    let unknown = unknown_map.iter().cloned().collect();
    // let mut providers = vec![]; // todo
    // for (rawset, num) in &data.isgci {
        // if *rawset == *set {
            // providers.push(Provider::Isgci(*num));
        // }
    // }
    // let transfers = HashMap::new(); // todo
    Set{
        preview: set.clone().into(),
        id: set.id.clone(),
        name: set.name.clone(),
        kind: set.kind.clone().into(),
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
pub fn process_sourcekey(sourcekey: &RawSourceKey, bibliography: &Option<Bibliography>) -> SourceKey {
    match sourcekey {
        RawSourceKey::Bibtex { key } => {
            let formatted_citation = match bibliography {
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
        preview: source.clone().preprocess(),
        id: source.id.clone(),
        sourcekey,
        showed,
        time: Date::empty(), // todo
    };
    res
}

/// Minimal and maximal refer to inclusion-wise extremes. An isolated element
/// would be included in all three sets.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Sets {
    pub minimal: Vec<PreviewSet>,
    pub maximal: Vec<PreviewSet>,
    pub all: Vec<PreviewSet>,
}

// impl Into<PreviewSource> for RawSource {
    // fn into(self) -> PreviewSource {
        // PreviewSource{
            // id: self.id,
            // sourcekey: self.sourcekey.into(),
            // showed: self.showed.into_iter().map(|x|x.into()).collect(),
            // time: self.time,
        // }
    // }
// }

impl Into<PreviewSourceKey> for RawSourceKey {
    fn into(self) -> PreviewSourceKey {
        match self {
            Self::Bibtex { key } => PreviewSourceKey::Bibtex { key },
            Self::Online { url } => PreviewSourceKey::Online { url },
            Self::Unknown => PreviewSourceKey::Unknown,
        }
    }
}

impl Into<PreviewKind> for RawKind {
    fn into(self) -> PreviewKind {
        match self {
            Self::Parameter => PreviewKind::Parameter,
            Self::GraphClass => PreviewKind::GraphClass,
            Self::Intersection(a) => PreviewKind::Intersection(a.into_iter().map(|x|x.into()).collect()),
        }
    }
}

impl Into<PreviewSet> for RawSet {
    fn into(self) -> PreviewSet {
        PreviewSet {
            id: self.id,
            name: self.name,
            kind: self.kind.into(),
        }
    }
}

impl Into<Showed> for RawShowed {
    fn into(self) -> Showed {
        Showed {
            id: self.id,
            text: self.text,
            fact: self.fact.into(),
            page: self.page,
        }
    }
}

impl Into<ShowedFact> for RawShowedFact {
    fn into(self) -> ShowedFact {
        match self {
            Self::Relation(x) => ShowedFact::Relation(x.into()),
            Self::Citation(x) => ShowedFact::Citation(x.preprocess()),
            Self::Definition(x) => ShowedFact::Definition(x.into()),
        }
    }
}

impl Into<PreviewRelation> for RawRelation {
    fn into(self) -> PreviewRelation {
        PreviewRelation {
            subset: self.subset.into(),
            superset: self.superset.into(),
            cpx: self.cpx,
        }
    }
}

impl Into<Date> for Option<Entry> {
    fn into(self) -> Date {
        match self {
            Some(x) => {
                Date::empty() // todo
            },
            None => Date::empty(),
        }
    }
}

impl RawSource {
    pub fn preprocess(self) -> PreviewSource {
        PreviewSource {
            id: self.id,
            sourcekey: self.rawsourcekey.into(),
            time: Date::empty(),
        }
    }
}

impl Relation {
    pub fn new(preview: &RawRelation) -> Self {
        Self {
            preview: preview.clone().into(),
            subset: preview.subset.clone().into(),
            superset: preview.superset.clone().into(),
            cpx: preview.cpx.clone(),
        }
    }
}

pub fn prepare_extremes(preview_set: Vec<PreviewSet>, data: &SimpleIndex) -> Sets {
    let mut minimal = Vec::new();
    let mut maximal = Vec::new();
    let mut all = Vec::new();
    for i in 0..preview_set.len() {
        let current_set = &preview_set[i];
        let mut is_maximal = true;
        let mut is_minimal = true;
        for j in 0..preview_set.len() {
            if i != j {
                let other_set = &preview_set[j];
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
            relation_idx.insert(key, Relation::new(&raw_relation));
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
        sets.push(process_set(set.clone().into(), &simpleindex, &rawdata, &source_keys));
    }
    let mut raw_relations = Vec::new();
    for (raw_source, showed) in &rawdata.factoids {
        match &showed.fact {
            RawShowedFact::Relation(rel) => raw_relations.push(rel.clone()),
            RawShowedFact::Citation(_) => (),
            RawShowedFact::Definition(_) => (),
        }
    }
    let relations = process_relations(raw_relations);
    Data::new(sets, relations, HashMap::new(), sources)
}
