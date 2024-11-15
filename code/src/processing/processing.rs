//! Given raw data this module enriches and interconnects it.

use std::{collections::{HashMap, HashSet, VecDeque}, path::PathBuf};
use biblatex::{Bibliography, Entry};

use crate::{data::{data::{Data, Date, Linkable, Relation, Set, Showed, ShowedFact, Source, SourceSubset}, simpleindex::SimpleIndex}, general::{enums::TransferGroup, hide::filter_hidden}};
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
        for bigset in [&set.equivsets, &set.subsets.minimal, &set.supersets.maximal] {
            for sset in bigset {
                if !visited.contains_key(sset) {
                    let new_distance = current_distance + 1;
                    visited.insert(sset.clone(), new_distance);
                    queue.push_back((sset.clone(), new_distance));
                }
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
                preview: raw.preprocess(&source.sourcekey),
                id: source.id.clone(),
                sourcekey: source.sourcekey.clone(),
                showed: showed_vec.into_iter().map(|x|x.preprocess(&source.sourcekey)).collect(),
                time: source.time.clone(),
            }
        })
    .collect();
    timeline.sort_by_key(|subset| subset.time.clone());
    timeline.reverse();
    let subsets = help.get_subsets(&set);
    let supersets = help.get_supersets(&set);
    let sub_exclusions = help.get_antisubsets(&set);
    let super_exclusions = help.get_antisupersets(&set);
    let mut unknown_map: HashSet<PreviewSet> = HashSet::new();
    for par in &data.sets {
        unknown_map.insert(par.clone().into());
    }
    for s in &subsets {
        unknown_map.remove(&s);
    }
    for s in &supersets {
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
        equivsets: help.get_equiv(&set),
        subsets: prepare_extremes(subsets, help),
        supersets: prepare_extremes(supersets, help),
        sub_exclusions: prepare_extremes(sub_exclusions, help),
        super_exclusions: prepare_extremes(super_exclusions, help),
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
        RawSourceKey::Other { name, description } => {
            sourcekey = SourceKey::Other {
                name: name.clone(),
                description: description.clone(),
            };
        },
    }
    let mut showed = vec![];
    for (fact_source, raw_showed) in &rawdata.factoids {
        if fact_source == source {
            showed.push(raw_showed.clone().preprocess(&sourcekey));
        }
    }
    let mut res = Source {
        preview: source.clone().preprocess(&sourcekey),
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
    for current_set in &preview_set {
        let mut is_maximal = true;
        let mut is_minimal = true;
        for other_set in &preview_set {
            if current_set != other_set {
                let ab = data.first_subset_of_second(current_set, other_set);
                let ba = data.first_subset_of_second(other_set, current_set);
                if ab && !ba {
                    is_minimal = false;
                }
                if ba && !ab {
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
fn combine_relations(sets: &Vec<PreviewSet>, first_relations: Vec<Relation>, transfers: &HashMap<TransferGroup, HashMap<PreviewSet, Vec<PreviewSet>>>) -> Vec<Relation> {
    let mut map: HashMap<(PreviewSet, PreviewSet), Relation> = first_relations.into_iter().map(|x|((x.subset.clone(), x.superset.clone()), x)).collect();
    for i in 1..=4 { // todo remove this and fix the process to make the connections correct
        println!("combining relations iteration {}", i);
        let mut current_relations: Vec<Relation> = Vec::new();
        for (k, v) in &map {
            current_relations.push(v.clone());
        }
        let new_relations = apply_transfers(transfers, &current_relations);
        for rel in new_relations {
            let key = (rel.subset.clone(), rel.superset.clone());
            map.entry(key).and_modify(|x|{
                x.combine_parallel(&rel);
            }).or_insert(rel);
        }
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
                let ar_key = (ar_preview.subset.clone(), ar_preview.superset.clone());
                for br_preview in &outrel {
                    let br_key = (br_preview.subset.clone(), br_preview.superset.clone());
                    let ar = map.get(&ar_key).unwrap();
                    let br = map.get(&br_key).unwrap();
                    if ar.subset == br.superset {
                        continue;
                    }
                    let res_key = (ar.subset.clone(), br.superset.clone());
                    let ser = ar.combine_serial(br);
                    map.entry(res_key).and_modify(|x|{
                        x.combine_parallel(&ser);
                    }).or_insert(ser);
                }
            }
            // outgoing inclusion and exclusion implies exclusion between endpoints
            for ar_preview in &outrel {
                let ar_key = (ar_preview.subset.clone(), ar_preview.superset.clone());
                for br_preview in &outrel {
                    if ar_preview == br_preview {
                        continue;
                    }
                    match (&ar_preview.cpx, &br_preview.cpx) {
                        (CpxInfo::Inclusion { .. } | CpxInfo::Equivalence, CpxInfo::Exclusion) => {},
                        _ => continue,
                    }
                    let br_key = (br_preview.subset.clone(), br_preview.superset.clone());
                    let ar = map.get(&ar_key).unwrap();
                    let br = map.get(&br_key).unwrap();
                    let res_key = (ar.superset.clone(), br.superset.clone());
                    // println!("==========\n{:?}\n{:?}\n", ar, br);
                    let preview = PreviewRelation {
                        id: "".into(),
                        cpx: CpxInfo::Exclusion,
                        subset: ar.superset.clone(),
                        superset: br.superset.clone(),
                    };
                    let ser = Relation {
                        id: "".into(),
                        cpx: preview.cpx.clone(),
                        subset: preview.subset.clone(),
                        superset: preview.superset.clone(),
                        preview,
                        combined_from: Some((ar_preview.clone(), br_preview.clone())),
                        essential: true,
                    };
                    map.entry(res_key.clone()).and_modify(|x|{
                        x.combine_parallel(&ser);
                    }).or_insert(ser);
                }
            }
            // incoming inclusion and exclusion implies exclusion between endpoints
            // todo almost copy of the above
            for ar_preview in &inrel {
                let ar_key = (ar_preview.subset.clone(), ar_preview.superset.clone());
                for br_preview in &inrel {
                    if ar_preview == br_preview {
                        continue;
                    }
                    match (&ar_preview.cpx, &br_preview.cpx) {
                        (CpxInfo::Exclusion, CpxInfo::Inclusion { .. } | CpxInfo::Equivalence) => {},
                        _ => continue,
                    }
                    let br_key = (br_preview.subset.clone(), br_preview.superset.clone());
                    let ar = map.get(&ar_key).unwrap();
                    let br = map.get(&br_key).unwrap();
                    let res_key = (ar.subset.clone(), br.subset.clone());
                    let preview = PreviewRelation {
                        id: "".into(),
                        cpx: CpxInfo::Exclusion,
                        subset: ar.subset.clone(),
                        superset: br.subset.clone(),
                    };
                    let ser = Relation {
                        id: "".into(),
                        cpx: preview.cpx.clone(),
                        subset: preview.subset.clone(),
                        superset: preview.superset.clone(),
                        preview,
                        combined_from: Some((ar_preview.clone(), br_preview.clone())),
                        essential: true,
                    };
                    map.entry(res_key).and_modify(|x|{
                        x.combine_parallel(&ser);
                    }).or_insert(ser);
                }
            }
        }
    }
    map.into_values().collect()
}

impl Relation {
    // todo this should be changed to find the simplest way to find the resulting complexity
    pub fn combine_parallel(&mut self, other: &Relation) {
        assert_eq!(self.superset, other.superset);
        assert_eq!(self.subset, other.subset);
        // todo merging entries just via complexity is not good enough, they combine in a more nuanced way
        match self.cpx.combine_parallel(&other.cpx){
            Ok(res) => {
                self.preview.cpx = res.clone();
                self.cpx = res;
            },
            Err(err) => eprintln!("{}\n{:?}\n{:?}", err, self.preview, other.preview),
        }
    }
    pub fn combine_serial(&self, other: &Relation) -> Relation {
        assert_eq!(self.superset, other.subset);
        let cpx = self.cpx.combine_serial(&other.cpx);
        let preview = PreviewRelation {
            id: "".into(),
            subset: self.subset.clone(),
            superset: other.superset.clone(),
            cpx: cpx.clone(),
        };
        Relation {
            id: preview.id.clone(),
            subset: preview.subset.clone(),
            superset: preview.superset.clone(),
            preview,
            cpx,
            combined_from: Some((self.preview.clone(), other.preview.clone())),
            essential: true,
        }
    }
}

fn apply_transfers(transfers: &HashMap<TransferGroup, HashMap<PreviewSet, Vec<PreviewSet>>>, relations: &Vec<Relation>) -> Vec<Relation> {
    let mut transferred_relations = Vec::new();
    for relation in relations {
        let top = relation.subset.clone();
        let bot = relation.superset.clone();
        for (transfer_group, map) in transfers.iter() {
            if let (Some(top_res), Some(bot_res)) = (map.get(&top), map.get(&bot)) {
                let mut res_cpx = relation.cpx.clone();
                if let CpxInfo::Inclusion { mn, mx } = &res_cpx {
                    if let crate::general::enums::CpxTime::Constant = mx {
                        res_cpx = CpxInfo::Inclusion {
                            mn: mn.clone(),
                            mx: crate::general::enums::CpxTime::Linear
                        };
                    }
                    for tr in top_res {
                        for br in bot_res {
                            let prev = PreviewRelation {
                                id: "".into(),
                                cpx: res_cpx.clone(),
                                subset: tr.clone(),
                                superset: br.clone(),
                            };
                            let rel = Relation {
                                id: prev.id.clone(),
                                cpx: prev.cpx.clone(),
                                subset: prev.subset.clone(),
                                superset: prev.superset.clone(),
                                preview: prev,
                                essential: false,
                                combined_from: None, // todo signify that this relation was transferred
                            };
                            transferred_relations.push(rel);
                        }
                    }
                }
            }
        }
    }
    transferred_relations
}

pub fn process_raw_data(rawdata: &RawData, bibliography_file: &PathBuf) -> Data {
    let bibliography = load_bibliography(&bibliography_file);
    let mut sources = vec![];
    let mut source_keys: HashMap<RawSource, Source> = HashMap::new();
    for rawsource in &rawdata.sources {
        let source = process_source(&rawsource, &rawdata, &bibliography);
        source_keys.insert(rawsource.clone(), source.clone());
        sources.push(source);
    }
    sources.reverse();
    let mut preview_sets: Vec<PreviewSet> = vec![];
    for set in &rawdata.sets {
        preview_sets.push(set.clone().into());
    }
    let mut raw_relations = Vec::new();
    for (raw_source, showed) in &rawdata.factoids {
        match &showed.fact {
            RawShowedFact::Relation(rel) => raw_relations.push(rel.clone()),
            RawShowedFact::Citation(_) => (),
            RawShowedFact::Definition(_) => (),
        }
    }
    let mut transfers: HashMap::<TransferGroup, HashMap<PreviewSet, Vec<PreviewSet>>> = HashMap::new();
    for (key, raw_pairs) in &rawdata.transfer {
        let mut res: HashMap<PreviewSet, Vec<PreviewSet>> = HashMap::new();
        for raw_pair in raw_pairs {
            let (from, to) = raw_pair.clone();
            let res_from: PreviewSet = from.into();
            let res_to: PreviewSet = to.into();
            res.entry(res_from).or_insert_with(|| vec![]).push(res_to.clone());

        }
        transfers.insert(key.clone(), res);
    }
    let first_relations = process_relations(raw_relations);
    let mut relations = combine_relations(&preview_sets, first_relations, &transfers);
    let preview_relations = relations.iter().map(|x|x.preview.clone()).collect();
    let essential_relations_vec = filter_hidden(preview_relations, &preview_sets);
    let essential_relations_set: HashSet<&PreviewRelation> = essential_relations_vec.iter().collect();
    for rel in &mut relations {
        rel.essential = essential_relations_set.contains(&rel.preview);
    }
    let simpleindex = SimpleIndex::new(&relations);
    let mut sets = vec![];
    for set in &rawdata.sets {
        sets.push(process_set(set.clone().into(), &simpleindex, &rawdata, &source_keys));
    }
    let mut linkable: HashMap<String, Box<dyn Linkable>> = HashMap::new();
    for set in &sets {
        linkable.insert(set.id.clone(), Box::new(set.preview.clone()));
    }
    for rel in &relations {
        linkable.insert(rel.id.clone(), Box::new(rel.preview.clone()));
    }
    for source in &sources {
        linkable.insert(source.id.clone(), Box::new(source.preview.clone()));
    }
    Data::new(sets, relations, linkable, sources)
}
