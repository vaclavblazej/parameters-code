//! Given raw data this module enriches and interconnects it.

use std::{collections::{HashMap, HashSet, VecDeque}, path::PathBuf};
use biblatex::{Bibliography, Entry};
use serde::{Serialize, Deserialize};

use crate::{data::{data::{CreatedBy, Data, Date, Linkable, Provider, Relation, Set, Showed, ShowedFact, Source, SourceSubset}, simpleindex::SimpleIndex}, general::{enums::TransferGroup, hide::filter_hidden}};
use crate::general::{enums::SourceKey, enums::CpxInfo, file};
use crate::input::raw::*;
use crate::data::preview::*;
use crate::general::enums::CpxTime::Constant;


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
    let mut providers = vec![];
    for (rawprovider, links) in &data.provider_links {
        let provider : Provider = rawprovider.clone().into();
        for link in links {
            if *set.id == *link.set.id {
                providers.push(link.clone().preprocess(&provider));
            }
        }
    }
    // let transfers = HashMap::new(); // todo
    Set{
        preview: set.clone().into(),
        id: set.id.clone(),
        name: set.name.clone(),
        kind: set.kind.clone().into(),
        providers,
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
                    if let Some(e) = bib.get(&key) { // todo fixme
                        Some(format!("{}", e.to_biblatex_string()))
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
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Sets {
    pub minimal: Vec<PreviewSet>,
    pub maximal: Vec<PreviewSet>,
    pub all: Vec<PreviewSet>,
}

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

fn process_relations(sets: &Vec<PreviewSet>,
                     composed_sets: &Vec<(PreviewSet, Vec<PreviewSet>)>,
                     raw_relations: Vec<RawRelation>,
                     transfers: &HashMap<TransferGroup, HashMap<PreviewSet, Vec<PreviewSet>>>,
                     ) -> Vec<Relation> {
    let relations: Vec<Relation> = raw_relations.into_iter().map(|x|x.into()).collect();
    let mut res: HashMap<(PreviewSet, PreviewSet), Relation> = HashMap::new();
    for relation in relations {
        let pair = (relation.subset.clone(), relation.superset.clone());
        let mut updated_relations: VecDeque<PreviewRelation> = VecDeque::new();
        // todo add progress in history when the collection is more complete
        let combined = if let Some(mut value) = res.get_mut(&pair) {
            if !value.combine_parallel(&relation) {
                continue;
            }
            updated_relations.push_back(relation.preview.clone());
            true
        } else {
            res.insert(pair, relation.clone());
            updated_relations.push_back(relation.preview.clone());
            false
        };
        // let intersection_parameters = sets.iter().filter_map(|x|{
            // match x.crea
        // }).collect();
        let mut improved_relations = 0;
        while let Some(relation) = updated_relations.pop_front() {
            improved_relations += 1;
            // apply the new or improved relation
            for set in sets {
                if *set == relation.subset || *set == relation.superset {
                    continue;
                }
                // equivalence ab copies the new relation cd into ef
                for (a,b,c,d,e,f) in vec![
                    (relation.subset.clone(), relation.superset.clone(), set.clone(), relation.superset.clone(), set.clone(), relation.subset.clone()),
                    (relation.subset.clone(), relation.superset.clone(), set.clone(), relation.subset.clone(), set.clone(), relation.superset.clone()),
                    (relation.subset.clone(), relation.superset.clone(), relation.superset.clone(), set.clone(), relation.subset.clone(), set.clone()),
                    (relation.subset.clone(), relation.superset.clone(), relation.subset.clone(), set.clone(), relation.superset.clone(), set.clone()),
                ] {
                    let Some(ab) = res.get(&(a.clone(), b.clone())) else { continue };
                    let Some(cd) = res.get(&(c.clone(), d.clone())) else { continue };
                    match &ab.cpx {
                        CpxInfo::Equivalence => {},
                        _ => continue,
                    }
                    let ac = (a.clone(), c.clone());
                    let mut result = Relation::new(&e, &f, cd.cpx.clone(), CreatedBy::Todo);
                    res.entry((e, f)).and_modify(|x|{
                        x.combine_parallel(&result);
                    }).or_insert(result);
                }
                // inclusion ab and inclusion bc imply inclusion ac
                for (a,b,c) in vec![
                    (set.clone(), relation.subset.clone(), relation.superset.clone()),
                    (relation.subset.clone(), relation.superset.clone(), set.clone()),
                ] {
                    let Some(ab) = res.get(&(a.clone(), b.clone())) else { continue };
                    let Some(bc) = res.get(&(b.clone(), c.clone())) else { continue };
                    let ac = (a.clone(), c.clone());
                    let result = ab.combine_serial(bc);
                    if let Some(mut x) = res.get_mut(&ac) {
                        if x.combine_parallel(&result) {
                            updated_relations.push_back(result.preview.clone());
                        }
                    } else {
                        updated_relations.push_back(result.preview.clone());
                        res.insert(ac, result);
                    }
                }
                // inclusion ab and exclusion cd implies exclusion ef
                for (a,b,c,d,e,f) in vec![
                    (relation.subset.clone(), relation.superset.clone(), set.clone(), relation.superset.clone(), set.clone(), relation.subset.clone()),
                    (relation.subset.clone(), relation.superset.clone(), relation.subset.clone(), set.clone(), relation.superset.clone(), set.clone()),
                    (set.clone(), relation.superset.clone(), relation.subset.clone(), relation.superset.clone(), relation.subset.clone(), set.clone()),
                    (relation.subset.clone(), set.clone(), relation.subset.clone(), relation.superset.clone(), set.clone(), relation.superset.clone()),
                ] {
                    let Some(ab) = res.get(&(a.clone(), b.clone())) else { continue };
                    let Some(cd) = res.get(&(c.clone(), d.clone())) else { continue };
                    match (&ab.cpx, &cd.cpx) {
                        (CpxInfo::Inclusion { .. } | CpxInfo::Equivalence, CpxInfo::Exclusion) => {},
                        _ => continue,
                    }
                    let mut result = Relation::new(&e, &f, CpxInfo::Exclusion, CreatedBy::Todo);
                    res.entry((e, f)).and_modify(|x|{
                        x.combine_parallel(&result);
                    }).or_insert(result);
                }
            }
            // inclusion ab implies inclusion f(a)f(b) for a transfer f
            let new_relations = apply_transfers(transfers, &relation);
            for result in new_relations {
                let key = (result.subset.clone(), result.superset.clone());
                if let Some(mut x) = res.get_mut(&key) {
                    if x.combine_parallel(&result) {
                        updated_relations.push_back(result.preview.clone());
                    }
                } else {
                    updated_relations.push_back(result.preview.clone());
                    res.insert(key, result);
                }
            }
            // inclusion ab and ac imply inclusion a(b+c)
            for (composed_set, elements) in composed_sets {
                if &relation.subset == composed_set {
                    continue;
                }
                if elements.contains(&relation.superset) {
                    let mut result = Relation::new(&relation.subset, &composed_set, CpxInfo::Inclusion { mn: Constant, mx: Constant }, CreatedBy::Todo);
                    let mut okay = true;
                    for element in elements {
                        let a = relation.subset.clone();
                        let Some(ab) = res.get(&(a, element.clone())) else { okay = false; break };
                        match &ab.cpx {
                            CpxInfo::Inclusion { .. } | CpxInfo::Equivalence => {},
                            _ => { okay = false; break },
                        }
                        result = result.combine_plus(ab);
                    }
                    if !okay { continue }
                    res.entry((result.subset.clone(), result.superset.clone())).and_modify(|x|{
                        x.combine_parallel(&result);
                    }).or_insert(result);
                }
            }
        }
        println!("improved {} relations", improved_relations);
    }
    res.into_values().collect()
}

impl Relation {
    pub fn new(subset: &PreviewSet, superset: &PreviewSet, cpx: CpxInfo, created_by: CreatedBy) -> Self{
        let preview = PreviewRelation {
            id: format!("{}_{}", subset.id, superset.id),
            cpx,
            subset: subset.clone(),
            superset: superset.clone(),
        };
        Self {
            id: preview.id.clone(),
            cpx: preview.cpx.clone(),
            subset: preview.subset.clone(),
            superset: preview.superset.clone(),
            preview,
            created_by: CreatedBy::Directly, // todo
            essential: true,
        }
    }
    // todo this should be changed to find the simplest way to find the resulting complexity
    pub fn combine_parallel(&mut self, other: &Relation) -> bool {
        assert_eq!(self.superset, other.superset);
        assert_eq!(self.subset, other.subset);
        // todo merging entries just via complexity is not good enough, they combine in a more nuanced way
        let original = self.cpx.clone();
        match self.cpx.combine_parallel(&other.cpx){
            Ok(res) => {
                if original != res {
                    self.preview.cpx = res.clone();
                    self.cpx = res;
                    true
                } else {
                    false
                }
            },
            Err(err) => {
                eprintln!("{}\n{:?}\n{:?}", err, self.preview, other.preview);
                false
            }
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
            created_by: CreatedBy::TransitiveInclusion(self.preview.clone(), other.preview.clone()),
            essential: true,
        }
    }
    pub fn combine_plus(&self, other: &Relation) -> Relation {
        assert_eq!(self.subset, other.subset); // expected to be used for combined parameters only
        let cpx = self.cpx.combine_plus(&other.cpx);
        let preview = PreviewRelation {
            id: "".into(),
            subset: self.subset.clone(),
            superset: self.superset.clone(),
            cpx: cpx.clone(),
        };
        Relation {
            id: preview.id.clone(),
            subset: preview.subset.clone(),
            superset: preview.superset.clone(),
            preview,
            cpx,
            created_by: CreatedBy::TransitiveInclusion(self.preview.clone(), other.preview.clone()),
            essential: true,
        }
    }
}

fn apply_transfers(transfers: &HashMap<TransferGroup, HashMap<PreviewSet, Vec<PreviewSet>>>, relation: &PreviewRelation) -> Vec<Relation> {
    let mut transferred_relations = Vec::new();
    let top = relation.subset.clone();
    let bot = relation.superset.clone();
    for (transfer_group, map) in transfers.iter() {
        if let (Some(top_res), Some(bot_res)) = (map.get(&top), map.get(&bot)) {
            let mut res_cpx = relation.cpx.clone();
            if let CpxInfo::Inclusion { mn, mx } = &res_cpx {
                if let Constant = mx {
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
                            created_by: CreatedBy::TransferredFrom(transfer_group.clone(), relation.clone()),
                        };
                        transferred_relations.push(rel);
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
    let mut composed_sets: Vec<(PreviewSet, Vec<PreviewSet>)> = vec![];
    for set in &rawdata.sets {
        if let Some(Composition::Intersection(ref vec)) = set.composed {
            let comp_preview: Vec<PreviewSet> = vec.iter().map(|x|x.clone().into()).collect();
            composed_sets.push((set.clone().into(), comp_preview));
        }
    }
    let mut providers = vec![];
    for (raw_provider, _) in &rawdata.provider_links {
        providers.push(raw_provider.clone().into());
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
    let mut raw_relations = Vec::new();
    for (raw_source, showed) in &rawdata.factoids {
        match &showed.fact {
            RawShowedFact::Relation(rel) => raw_relations.push(rel.clone()),
            RawShowedFact::Citation(_) => (),
            RawShowedFact::Definition(_) => (),
        }
    }
    let mut relations = process_relations(&preview_sets, &composed_sets, raw_relations, &transfers);
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
    Data::new(sets, relations, sources, providers)
}
