//! Given raw data this module enriches and interconnects it.

use std::{collections::{HashMap, HashSet, VecDeque}, path::PathBuf};
use biblatex::{Bibliography, Chunk, DateValue, Entry, PermissiveType, Person, Spanned};
use serde::{Serialize, Deserialize};

use crate::{data::{data::{Data, Date, Linkable, PartialResult, PartialResultsBuilder, Provider, Relation, Set, Showed, ShowedFact, Source, SourceSubset}, simpleindex::SimpleIndex}, general::{enums::{CreatedBy, SourcedCpxInfo, TransferGroup}, hide::filter_hidden, progress::ProgressDisplay}};
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
pub fn process_set(set: &RawSet, help: &SimpleIndex, data: &RawData, sources: &HashMap<RawSource, Source>) -> Set {
    let mut timeline_map: HashMap<RawSource, Vec<RawShowed>> = HashMap::new();
    for (raw_source, showed) in &data.factoids {
        let should_save = match &showed.fact {
            RawShowedFact::Relation(relation) if relation.superset.id == set.id || relation.subset.id == set.id => true,
            RawShowedFact::Definition(defined_set) if defined_set.id == set.id => true,
            RawShowedFact::Citation( .. ) => false,
            RawShowedFact::Relation( .. ) => false,
            RawShowedFact::Definition( .. ) => false,
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
    let preview = set.clone().into();
    let subsets = help.get_subsets(&preview);
    let supersets = help.get_supersets(&preview);
    let sub_exclusions = help.get_antisubsets(&preview);
    let super_exclusions = help.get_antisupersets(&preview);
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
            if *preview.id == *link.set.id {
                providers.push(link.clone().preprocess(&provider));
            }
        }
    }
    // let transfers = HashMap::new(); // todo
    Set{
        preview: preview.clone().into(),
        id: preview.id.clone(),
        name: preview.name.clone(),
        typ: preview.typ.clone().into(),
        providers,
        timeline,
        aka: set.aka.clone(),
        abbr: set.abbr.clone(),
        tags: set.tags.iter().map(|x|x.clone().into()).collect(),
        // transfers,
        equivsets: help.get_eqsets(&preview),
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
            let mut name: Option<String> = None;
            let entry = match bibliography {
                Some(bib) => {
                    if let Some(e) = bib.get(&key) {
                        if let Ok(title) = e.title() {
                            let title_str: String = title.iter().map(|Spanned { v: chunk, span: _ }|{
                                match chunk {
                                    Chunk::Normal(value) => value.clone(),
                                    Chunk::Verbatim(value) => format!("`{}`", value),
                                    Chunk::Math(value) => format!("${}$", value),
                                }
                            }).fold("".into(), |mut a, b| { a.push_str(&b); a });
                            name = Some(match name {
                                Some(mut q) => { q.push_str(&title_str); q },
                                None => title_str
                            });
                        }
                        if let Ok(fauthors) = e.author() {
                            let sauthors: Vec<String> = fauthors.iter().map(|x|x.name.clone()).collect();
                            let authors = sauthors.join(", ");
                            name = Some(match name {
                                Some(mut q) => { q.push_str(&format!(" by {}", authors)); q },
                                None => authors
                            });
                        }
                        if let Ok(dt) = e.date() {
                            match dt{
                                PermissiveType::Typed(t) => {
                                    match t.value {
                                        DateValue::At(d) => {time = Date {
                                            year: Some(d.year),
                                            month: None,
                                            day: None,
                                        }},
                                        DateValue::After(d) => {panic!("unknown date type")},
                                        DateValue::Before(d) => {panic!("unknown date type")},
                                        DateValue::Between(s, e) => {panic!("unknown date type")},
                                    }
                                },
                                PermissiveType::Chunks(chunks) => {
                                    panic!("unknown date type")
                                },
                            }
                        }
                        Some(format!("{}", e.to_biblatex_string()))
                    } else {
                        None
                    }
                },
                None => {
                    None
                },
            };
            sourcekey = SourceKey::Bibtex { key: key.clone(), name, entry };
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

fn add_and_update(
    relation_map: &mut HashMap<(PreviewSet, PreviewSet), Relation>,
    changed_relation: (PreviewSet, PreviewSet),
    result: Relation,
    updated_relations: &mut VecDeque<PreviewRelation>,
    ) {
    if let Some(mut x) = relation_map.get_mut(&changed_relation) {
        if x.combine_parallel(&result) {
            updated_relations.push_back(result.preview.clone());
        }
    } else {
        updated_relations.push_back(result.preview.clone());
        relation_map.insert(changed_relation, result);
    }
}

fn process_relations(sets: &Vec<PreviewSet>,
                     composed_sets: &Vec<(PreviewSet, Vec<PreviewSet>)>,
                     factoids: &Vec<(RawSource, RawShowed)>,
                     transfers: &HashMap<TransferGroup, HashMap<PreviewSet, Vec<PreviewSet>>>,
                     sources: &HashMap<RawSource, Source>,
                     ) -> (Vec<Relation>, Vec<PartialResult>) {
    let mut relations: Vec<Relation> = vec![];
    let mut partial_results_builder = PartialResultsBuilder::new();
    for (raw_source, showed) in factoids {
        match &showed.fact {
            RawShowedFact::Relation(rel) => {
                let preview_source = sources.get(raw_source).unwrap().preview.clone();
                let partial_result = partial_results_builder.partial_result(CreatedBy::Directly(preview_source));
                let handle = partial_result.handle;
                let sourced_cpx = rel.cpx.clone().to_sourced(partial_result);
                relations.push(Relation::new(&rel.subset.clone().into(), &rel.superset.clone().into(), sourced_cpx, handle));
            },
            RawShowedFact::Citation(_) => (),
            RawShowedFact::Definition(_) => (),
        }
    }
    let mut res: HashMap<(PreviewSet, PreviewSet), Relation> = HashMap::new();
    let mut progress = ProgressDisplay::new("processing", 21329);
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
                let xx = relation.subset.clone();
                let yy = relation.superset.clone();
                let zz = set.clone();
                for (x,y,z) in vec![
                    (xx.clone(), yy.clone(), zz.clone()),
                    (xx.clone(), zz.clone(), yy.clone()),
                    (yy.clone(), xx.clone(), zz.clone()),
                    (yy.clone(), zz.clone(), xx.clone()),
                    (zz.clone(), xx.clone(), yy.clone()),
                    (zz.clone(), yy.clone(), xx.clone()),
                ] {
                    let Some(ab) = res.get(&(x.clone(), y.clone())) else { continue };
                    match ab.cpx.clone() {
                        SourcedCpxInfo::Equal { source } => {
                            for (c,d,e,f) in vec![
                                (z.clone(), x.clone(), z.clone(), y.clone()),
                                (z.clone(), y.clone(), z.clone(), x.clone()),
                                (x.clone(), z.clone(), y.clone(), z.clone()),
                                (y.clone(), z.clone(), x.clone(), z.clone()),
                            ] {
                                let Some(cd) = res.get(&(c.clone(), d.clone())) else { continue };
                                let created_by = CreatedBy::SameThroughEquivalence(cd.handle, source.handle);
                                let partial_result = partial_results_builder.partial_result(created_by);
                                let result = Relation::new(&e, &f, cd.cpx.clone(), partial_result.handle);
                                add_and_update(&mut res, (e, f), result, &mut updated_relations);
                            }
                        },
                        _ => continue,
                    }
                }
                // inclusion ab and inclusion bc imply inclusion ac
                for (a,b,c) in vec![
                    (set.clone(), relation.subset.clone(), relation.superset.clone()),
                    (relation.subset.clone(), relation.superset.clone(), set.clone()),
                ] {
                    let Some(ab) = res.get(&(a.clone(), b.clone())) else { continue };
                    let Some(bc) = res.get(&(b.clone(), c.clone())) else { continue };
                    let ac = (a.clone(), c.clone());
                    let result = ab.combine_serial(bc, &mut partial_results_builder);
                    add_and_update(&mut res, ac, result, &mut updated_relations);
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
                        (SourcedCpxInfo::Inclusion { mn: _, mx: (_, smx) }, SourcedCpxInfo::Exclusion { source }) => {
                            let partial_result = partial_results_builder.partial_result(CreatedBy::TransitiveExclusion(smx.handle, source.handle));
                            let src = SourcedCpxInfo::Exclusion {
                                source: partial_result.clone()
                            };
                            let result = Relation::new(&e, &f, src, partial_result.handle);
                            add_and_update(&mut res, (e, f), result, &mut updated_relations);
                        },
                        _ => continue,
                    }
                }
            }
            // inclusion ab implies inclusion f(a)f(b) for a transfer f
            if let Some(ab) = res.get(&(relation.subset.clone(), relation.superset.clone())) {
                let new_relations = apply_transfers(transfers, &ab, &mut partial_results_builder);
                for result in new_relations {
                    let key = (result.subset.clone(), result.superset.clone());
                    add_and_update(&mut res, key, result, &mut updated_relations);
                }
            }
            // inclusion ab and ac imply inclusion a(b+c)
            for (composed_set, elements) in composed_sets {
                if &relation.subset == composed_set {
                    continue;
                }
                if elements.contains(&relation.superset) {
                    let mut opt_result: Option<Relation> = None;
                    let mut okay = true;
                    for element in elements {
                        let a = relation.subset.clone();
                        let Some(ab) = res.get(&(a, element.clone())) else { okay = false; break };
                        match &ab.cpx {
                            SourcedCpxInfo::Inclusion { .. } => {},
                            _ => { okay = false; break },
                        }
                        match opt_result {
                            Some(res) => opt_result = Some(res.combine_plus(ab)),
                            None => opt_result = Some(ab.clone()),
                        }
                    }
                    if !okay { continue }
                    if let Some(result) = opt_result {
                        add_and_update(&mut res, (result.subset.clone(), result.superset.clone()), result, &mut updated_relations);
                    }
                }
            }
        }
        progress.increase(improved_relations);
    }
    progress.done();
    (res.into_values().collect(), partial_results_builder.done())
}

impl Relation {

    pub fn id(subset: &PreviewSet, superset: &PreviewSet) -> String {
        format!("{}_{}", subset.id, superset.id)
    }

    pub fn new(subset: &PreviewSet, superset: &PreviewSet, cpx: SourcedCpxInfo, handle: usize) -> Self{
        let preview = PreviewRelation {
            id: Self::id(subset, superset),
            cpx: cpx.clone().into(),
            subset: subset.clone(),
            superset: superset.clone(),
        };
        Self {
            id: preview.id.clone(),
            handle,
            cpx,
            subset: subset.clone(),
            superset: superset.clone(),
            preview,
            essential: true,
        }
    }

}

fn apply_transfers(transfers: &HashMap<TransferGroup, HashMap<PreviewSet, Vec<PreviewSet>>>, relation: &Relation, partial_results_builder: &mut PartialResultsBuilder) -> Vec<Relation> {
    let mut transferred_relations: Vec<Relation> = Vec::new();
    let top = relation.subset.clone();
    let bot = relation.superset.clone();
    for (transfer_group, map) in transfers.iter() {
        if let (Some(top_res), Some(bot_res)) = (map.get(&top), map.get(&bot)) {
            let mut res_cpx = relation.cpx.clone();
            if let SourcedCpxInfo::Inclusion { mn: (mn, smn), mx: (mx, smx) } = &res_cpx {
                let partial_result = partial_results_builder.partial_result(CreatedBy::TransferredFrom(transfer_group.clone(), relation.handle));
                let handle = partial_result.handle;
                if let Constant = mx {
                    res_cpx = SourcedCpxInfo::Inclusion {
                        mn: (mn.clone(), smn.clone()),
                        mx: (
                            crate::general::enums::CpxTime::Linear,
                            partial_result
                            ),
                    };
                }
                for tr in top_res {
                    for br in bot_res {
                        let rel = Relation::new(
                            &tr.clone(),
                            &br.clone(),
                            res_cpx.clone(),
                            handle,
                            );
                        transferred_relations.push(rel);
                    }
                }
            }
        }
    }
    transferred_relations
}

pub fn process_raw_data(rawdata: &RawData, bibliography: &Option<Bibliography>) -> Data {
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
        if let Composition::Intersection(ref vec) = set.composed {
            let comp_preview: Vec<PreviewSet> = vec.iter().map(|x|x.clone().into()).collect();
            composed_sets.push((set.clone().into(), comp_preview));
        }
    }
    let mut providers = vec![];
    for (raw_provider, _) in &rawdata.provider_links {
        providers.push(raw_provider.clone().into());
    }
    let mut tags = vec![];
    for raw_tag in &rawdata.tags {
        let sets = rawdata.sets.iter().filter(|x|x.tags.contains(raw_tag)).map(|x|x.clone().into()).collect();
        tags.push(raw_tag.clone().preprocess(sets));
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
    let (mut relations, partial_results) = process_relations(&preview_sets, &composed_sets, &rawdata.factoids, &transfers, &source_keys);
    let preview_relations = relations.iter().map(|x|x.preview.clone()).collect();
    let essential_relations_vec = filter_hidden(preview_relations, &preview_sets);
    let essential_relations_set: HashSet<&PreviewRelation> = essential_relations_vec.iter().collect();
    for rel in &mut relations {
        rel.essential = essential_relations_set.contains(&rel.preview);
    }
    let simpleindex = SimpleIndex::new(&relations);
    let mut sets = vec![];
    for set in &rawdata.sets {
        sets.push(process_set(set, &simpleindex, &rawdata, &source_keys));
    }
    Data::new(sets, relations, sources, providers, tags, partial_results)
}
