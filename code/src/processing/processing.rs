//! Given raw data this module enriches and interconnects it.

use std::{collections::{HashMap, HashSet, VecDeque}, path::PathBuf};
use biblatex::{Bibliography, Entry};

use crate::data::{data::{Data, Date, Relation, Set, Showed, ShowedFact, Source, SourceSubset}, simpleindex::SimpleIndex};
use crate::general::{enums::SourceKey, enums::CpxInfo, file};
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

pub fn bfs_limit_distance(set: &Set, data: &Data, distance: usize) -> HashMap<PreviewSet, usize> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    visited.insert(set.preview.clone(), 0);
    queue.push_back((set.preview.clone(), 0));
    while let Some((raw_set, current_distance)) = queue.pop_front() {
        let set = data.get_set(&raw_set);
        if current_distance >= distance {
            continue;
        }
        for subset in &set.subsets.all {
            if !visited.contains_key(subset) {
                let new_distance = current_distance + 1;
                visited.insert(subset.clone(), new_distance);
                queue.push_back((subset.clone(), new_distance));
            }
        }
        for superset in &set.supersets.all {
            let new_distance = current_distance + 1;
            if !visited.contains_key(superset) {
                visited.insert(superset.clone(), new_distance);
                queue.push_back((superset.clone(), new_distance));
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
    timeline.reverse();
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

pub fn process_source(source: &RawSource, rawdata: &RawData, bibliography: &Option<Bibliography>) -> Source {
    let mut sourcekey: SourceKey;
    let mut time = Date::empty();
    match &source.rawsourcekey {
        RawSourceKey::Bibtex { key } => {
            let entry = match bibliography {
                Some(bib) => {
                    if let Some(e) = bib.get(&key) {
                        time = e.into();
                        Some(e.clone())
                    } else {
                        None
                    }
                },
                None => {
                    None
                },
            };
            sourcekey = SourceKey::Bibtex { key: key.clone(), entry };
        },
        RawSourceKey::Online { url } => {
            sourcekey = SourceKey::Online { url: url.clone() };
        },
        RawSourceKey::Unknown => {
            sourcekey = SourceKey::Unknown;
        },
    }
    let mut showed = vec![];
    for (fact_source, raw_showed) in &rawdata.factoids {
        if fact_source == source {
            showed.push(raw_showed.clone().into());
        }
    }
    let mut res = Source {
        preview: source.clone().preprocess(),
        id: source.id.clone(),
        sourcekey,
        showed,
        time,
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
    let add_to_relation_idx = |relation_idx: &mut HashMap<(RawSet, RawSet), Relation>, key, raw_relation: &RawRelation| {
        if let Some(mut value) = relation_idx.get_mut(&key) {
            value.combine_parallel(&raw_relation.clone().into());
        } else {
            relation_idx.insert(key, raw_relation.clone().into());
        }
    };
    let mut relation_idx: HashMap<(RawSet, RawSet), Relation> = HashMap::new();
    for raw_relation in raw_relations {
        let key = (raw_relation.subset.clone(), raw_relation.superset.clone());
        add_to_relation_idx(&mut relation_idx, key.clone(), &raw_relation);
        if raw_relation.cpx == CpxInfo::Equivalence {
            let (a, b) = key;
            let flipped = RawRelation {
                subset: raw_relation.superset,
                superset: raw_relation.subset,
                cpx: raw_relation.cpx,
            };
            add_to_relation_idx(&mut relation_idx, (b, a), &flipped);
        }
    }
    relation_idx.into_values().collect()
}

/// Take a set of relations and attempt to combine in every possible way to create
/// novel relations and overriding superseded relations.
fn combine_relations(sets: &Vec<Set>, first_relations: Vec<Relation>) -> Vec<Relation> {
    let mut map: HashMap<(PreviewSet, PreviewSet), Relation> = first_relations.into_iter().map(|x|((x.superset.clone(), x.subset.clone()), x)).collect();
    let mut relations = Vec::new();
    for i in 1..5 { // todo remove this but to make the connections correct
        for set in sets {
            let mut inrel = Vec::new();
            let mut outrel = Vec::new();
            for ((from, to), rel) in &map {
                if from.id == set.id {
                    outrel.push(rel.preview.clone());
                }
                if to.id == set.id {
                    inrel.push(rel.preview.clone());
                }
            }
            // two inclusions compose serially
            for ar_preview in &inrel {
                let ar_key = (ar_preview.superset.clone(), ar_preview.subset.clone());
                for br_preview in &outrel {
                    let br_key = (br_preview.superset.clone(), br_preview.subset.clone());
                    let ar = map.get(&ar_key).unwrap();
                    let br = map.get(&br_key).unwrap();
                    let res_key = (ar.superset.clone(), br.subset.clone());
                    let ser = ar.combine_serial(br);
                    map.entry(res_key).and_modify(|x|{
                        x.combine_parallel(&ser);
                    }).or_insert(ser);
                }
            }
            // outgoing inclusion and exclusion implies exclusion between endpoints
            for ar_preview in &outrel {
                let ar_key = (ar_preview.superset.clone(), ar_preview.subset.clone());
                for br_preview in &outrel {
                    if ar_preview == br_preview {
                        continue;
                    }
                    match (&ar_preview.cpx, &br_preview.cpx) {
                        (CpxInfo::Inclusion { .. }, CpxInfo::Exclusion) => {},
                        _ => continue,
                    }
                    let br_key = (br_preview.superset.clone(), br_preview.subset.clone());
                    let ar = map.get(&ar_key).unwrap();
                    let br = map.get(&br_key).unwrap();
                    let res_key = (ar.subset.clone(), br.subset.clone());
                    // println!("==========\n{:?}\n{:?}\n", ar, br);
                    let preview = PreviewRelation {
                        id: "".into(),
                        cpx: CpxInfo::Exclusion,
                        superset: ar.subset.clone(),
                        subset: br.subset.clone(),
                    };
                    let ser = Relation {
                        id: "".into(),
                        cpx: preview.cpx.clone(),
                        superset: preview.superset.clone(),
                        subset: preview.subset.clone(),
                        preview,
                        combined_from: Some((ar_preview.clone(), br_preview.clone())),
                    };
                    map.entry(res_key).and_modify(|x|{
                        x.combine_parallel(&ser);
                    }).or_insert(ser);
                }
            }
            // incoming inclusion and exclusion implies exclusion between endpoints
            // todo almost copy of the above
            for ar_preview in &inrel {
                let ar_key = (ar_preview.superset.clone(), ar_preview.subset.clone());
                for br_preview in &inrel {
                    if ar_preview == br_preview {
                        continue;
                    }
                    match (&ar_preview.cpx, &br_preview.cpx) {
                        (CpxInfo::Exclusion, CpxInfo::Inclusion { .. }) => {},
                        _ => continue,
                    }
                    let br_key = (br_preview.superset.clone(), br_preview.subset.clone());
                    let ar = map.get(&ar_key).unwrap();
                    let br = map.get(&br_key).unwrap();
                    let res_key = (ar.superset.clone(), br.superset.clone());
                    let preview = PreviewRelation {
                        id: "".into(),
                        cpx: CpxInfo::Exclusion,
                        superset: ar.superset.clone(),
                        subset: br.superset.clone(),
                    };
                    let ser = Relation {
                        id: "".into(),
                        cpx: preview.cpx.clone(),
                        superset: preview.superset.clone(),
                        subset: preview.subset.clone(),
                        preview,
                        combined_from: Some((ar_preview.clone(), br_preview.clone())),
                    };
                    map.entry(res_key).and_modify(|x|{
                        x.combine_parallel(&ser);
                    }).or_insert(ser);
                }
            }
        }
    }
    for ((a,b), rel) in map {
        relations.push(rel);
    }
    relations
}

impl Relation {
    // todo insert backtrace of inferences
    pub fn combine_parallel(&mut self, other: &Relation) {
        assert_eq!(self.superset, other.superset);
        assert_eq!(self.subset, other.subset);
        match self.cpx.combine_parallel(&other.cpx){
            Ok(res) => self.cpx = res,
            Err(err) => eprintln!("{}\n{:?}\n{:?}", err, self.preview, other.preview),
        }
    }
    pub fn combine_serial(&self, other: &Relation) -> Relation{
        assert_eq!(self.subset, other.superset);
        let cpx = self.cpx.combine_serial(&other.cpx);
        let preview = PreviewRelation {
            id: "".into(),
            superset: self.superset.clone(),
            subset: other.subset.clone(),
            cpx: cpx.clone(),
        };
        Relation {
            id: preview.id.clone(),
            superset: preview.superset.clone(),
            subset: preview.subset.clone(),
            preview,
            cpx,
            combined_from: Some((self.preview.clone(), other.preview.clone())),
        }
    }
}

pub fn process_raw_data(rawdata: &RawData, bibliography_file: &PathBuf) -> Data {
    // todo, create urls that markdowns can use; these are maps from id to address of the entity
    let simpleindex = SimpleIndex::new(rawdata);
    let bibliography = load_bibliography(&bibliography_file);
    let mut sources = vec![];
    let mut source_keys: HashMap<RawSource, Source> = HashMap::new();
    for rawsource in &rawdata.sources {
        let source = process_source(&rawsource, &rawdata, &bibliography);
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
    let mut transfered_relations = Vec::new();
    for relation in &raw_relations {
        for (transfer_group, map) in rawdata.transfer.iter() {
            let some_top = map.get(&relation.subset);
            let some_bot = map.get(&relation.superset);
            if let Some(top) = some_top {
                if let Some(bot) = some_bot {
                    let rel = RawRelation {
                        cpx: relation.cpx.clone(),
                        subset: top.clone(),
                        superset: bot.clone(),
                    };
                    transfered_relations.push(rel);
                }
            }
        }
    }
    for tr in transfered_relations {
        raw_relations.push(tr);
    }
    let first_relations = process_relations(raw_relations);
    let relations = combine_relations(&sets, first_relations);
    Data::new(sets, relations, HashMap::new(), sources)
}
