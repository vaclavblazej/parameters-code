//! Given raw data this module enriches and interconnects it.

use std::{collections::{HashMap, HashSet, VecDeque}, path::PathBuf};
use biblatex::{Bibliography, Chunk, DateValue, Entry, PermissiveType, Person, Spanned};
use log::{debug, error, trace, warn};
use serde::{Serialize, Deserialize};

use crate::{data::{data::{Data, Date, Linkable, PartialResult, PartialResultsBuilder, Provider, Relation, Set, Showed, ShowedFact, Source, SourceSubset}, simple_index::SimpleIndex}, general::{enums::{CpxTime, CreatedBy, Drawing, RawDrawing, SourcedCpxInfo, TransferGroup}, progress::ProgressDisplay}, input::build::{ASSUMED_SOURCE_ID, UNKNOWN_SOURCE_ID}, work::combine::combine_serial};
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
pub fn process_set(set: &RawSet, help: &SimpleIndex, data: &RawData, sources: &HashMap<String, Source>) -> Set {
    let mut timeline_map: HashMap<String, Vec<RawShowed>> = HashMap::new();
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
                &format!("A raw source {} does not have a processed source. Use create.source() to add new sources.", raw)
                );
            SourceSubset {
                preview: source.preview.clone(),
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

pub fn process_source(source: &RawSource,
                      rawdata: &RawData,
                      bibliography: &Option<Bibliography>,
                      preview_set_map: &HashMap<String, PreviewSet>,
                     ) -> Source {
    trace!("processing set {:?}", source.rawsourcekey);
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
            sourcekey = SourceKey::Bibtex {
                key: key.clone(),
                name,
                entry,
                relevance: source.relevance,
            };
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
        if fact_source == &source.id {
            showed.push(raw_showed.clone().preprocess(&sourcekey));
        }
    }
    Source {
        preview: source.clone().preprocess(&sourcekey),
        id: source.id.clone(),
        sourcekey,
        showed,
        time,
        drawings: source.drawings.iter().map(|drawing|drawing.preprocess(preview_set_map)).collect(),
    }
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
    result: PartialResult,
    relation_map: &mut HashMap<WorkRelation, PartialResult>,
    updated_relations: &mut VecDeque<WorkRelation>,
    partial_result_builder: &mut PartialResultsBuilder,
    ) {
    let res = if let Some(x) = relation_map.get_mut(&result.relation) {
        if let Some(res) = x.combine_parallel(&result, partial_result_builder) {
            trace!("updated relation (replace) {} {}", x.relation.subset.name, x.relation.superset.name);
            res
        } else {
            return;
        }
    } else {
        trace!("updated relation (insert) {} {}", result.relation.subset.name, result.relation.superset.name);
        result
    };
    updated_relations.push_back(res.relation.clone());
    relation_map.insert(res.relation.clone(), res);
}

fn process_relations(sets: &Vec<PreviewSet>,
                     composed_sets: &Vec<(PreviewSet, Vec<PreviewSet>)>,
                     factoids: &Vec<(String, RawShowed)>,
                     transfers: &HashMap<TransferGroup, HashMap<PreviewSet, Vec<PreviewSet>>>,
                     sources: &HashMap<String, Source>,
                     ) -> (Vec<Relation>, Vec<PartialResult>) {
    trace!("processing relations");
    let mut partial_results: Vec<PartialResult> = vec![];
    let mut partial_results_builder = PartialResultsBuilder::new();
    for (raw_source_id, showed) in factoids {
        match &showed.fact {
            RawShowedFact::Relation(rel) => {
                if let Some(source) = sources.get(raw_source_id) {
                    let work_relation = WorkRelation::new(&rel.subset.clone().into(), &rel.superset.clone().into());
                    let partial_result = partial_results_builder.partial_result(CreatedBy::Directly(source.preview.clone()), rel.cpx.clone(), work_relation.clone());
                    partial_results.push(partial_result);
                } else {
                    panic!("source not found {:?}", raw_source_id);
                }
            },
            RawShowedFact::Citation(_) => (),
            RawShowedFact::Definition(_) => (),
        }
    }
    let mut res: HashMap<WorkRelation, PartialResult> = HashMap::new();
    let mut progress = ProgressDisplay::new("processing", 12486);
    for partial_result in partial_results {
        let pair = partial_result.relation.clone();
        debug!("processing relation from {} to {}", pair.subset.name, pair.superset.name);
        let mut updated_relations: VecDeque<WorkRelation> = VecDeque::new();
        // todo add progress in history when the collection is more complete
        add_and_update(partial_result, &mut res, &mut updated_relations, &mut partial_results_builder);
        let mut improved_relations = 0;
        while let Some(relation) = updated_relations.pop_front() {
            improved_relations += 1;
            if improved_relations >= 5000 {
                panic!("5k updates during processing probably means a bug");
            }
            // apply the new or improved relation
            for set in sets {
                if *set.id == relation.subset.id || *set.id == relation.superset.id {
                    continue;
                }
                // equivalence ab copies the new relation cd into ef
                let xx = relation.subset.clone();
                let yy = relation.superset.clone();
                let zz = set.clone();
                for (x, y, z) in vec![
                    (xx.clone(), yy.clone(), zz.clone()),
                    (xx.clone(), zz.clone(), yy.clone()),
                    (yy.clone(), xx.clone(), zz.clone()),
                    (yy.clone(), zz.clone(), xx.clone()),
                    (zz.clone(), xx.clone(), yy.clone()),
                    (zz.clone(), yy.clone(), xx.clone()),
                ] {
                    let Some(ab) = res.get(&WorkRelation::new(&x, &y)) else { continue };
                    match ab.to_sourced() {
                        SourcedCpxInfo::Equal { source } => {
                            for (c,d,e,f) in vec![
                                (z.clone(), x.clone(), z.clone(), y.clone()),
                                (z.clone(), y.clone(), z.clone(), x.clone()),
                                (x.clone(), z.clone(), y.clone(), z.clone()),
                                (y.clone(), z.clone(), x.clone(), z.clone()),
                            ] {
                                let Some(cd) = res.get(&WorkRelation::new(&c, &d)) else { continue };
                                let created_by = CreatedBy::SameThroughEquivalence(cd.handle, source.handle);
                                let partial_result = partial_results_builder.partial_result(created_by, cd.cpx.clone(), WorkRelation::new(&e, &f));
                                debug!("equivalence");
                                add_and_update(partial_result, &mut res, &mut updated_relations, &mut partial_results_builder);
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
                    let Some(ab) = res.get(&WorkRelation::new(&a, &b)) else { continue };
                    let Some(bc) = res.get(&WorkRelation::new(&b, &c)) else { continue };
                    if let (SourcedCpxInfo::Inclusion { mn: _, mx: Some((mxa, sra)) },
                            SourcedCpxInfo::Inclusion { mn: _, mx: Some((mxb, srb)) })
                            = (ab.to_sourced(), bc.to_sourced()) {
                        let rel = sra.relation.combine_serial(&srb.relation);
                        let (a, b, time) = combine_serial((mxa, sra), (mxb, srb));
                        let pr = partial_results_builder.partial_result(CreatedBy::TransitiveInclusion(a.handle, b.handle), CpxInfo::Inclusion { mn: None, mx: Some(time) }, rel);
                        debug!("inclusions {} {} + {} = {}", updated_relations.len(), a.handle, b.handle, c.name);
                        add_and_update(pr, &mut res, &mut updated_relations, &mut partial_results_builder);
                    };

                }
                // inclusion ab and exclusion cd implies exclusion ef
                for (a,b,c,d,e,f) in vec![
                    (relation.subset.clone(), relation.superset.clone(), set.clone(), relation.superset.clone(), set.clone(), relation.subset.clone()),
                    (relation.subset.clone(), relation.superset.clone(), relation.subset.clone(), set.clone(), relation.superset.clone(), set.clone()),
                    (set.clone(), relation.superset.clone(), relation.subset.clone(), relation.superset.clone(), relation.subset.clone(), set.clone()),
                    (relation.subset.clone(), set.clone(), relation.subset.clone(), relation.superset.clone(), set.clone(), relation.superset.clone()),
                ] {
                    let Some(ab) = res.get(&WorkRelation::new(&a, &b)) else { continue };
                    let Some(cd) = res.get(&WorkRelation::new(&c, &d)) else { continue };
                    let res_relation = WorkRelation::new(&e, &f);
                    match (&ab.to_sourced(), &cd.to_sourced()) {
                        (SourcedCpxInfo::Inclusion { mn: _, mx: Some((_, smx)) }, SourcedCpxInfo::Exclusion { source }) => {
                            let created_by = CreatedBy::TransitiveExclusion(smx.handle, source.handle);
                            let partial_result = partial_results_builder.partial_result(created_by, CpxInfo::Exclusion, res_relation);
                            debug!("exclusions");
                            add_and_update(partial_result, &mut res, &mut updated_relations, &mut partial_results_builder);
                        },
                        _ => continue,
                    }
                }
            }
            // inclusion ab implies inclusion f(a)f(b) for a transfer f
            if let Some(ab) = res.get(&relation) {
                let new_partial_results = apply_transfers(transfers, &ab, &mut partial_results_builder);
                for partial_result in new_partial_results {
                    debug!("transfer from ({},{}) to ({},{})", relation.subset.name, relation.superset.name, partial_result.relation.subset.name, partial_result.relation.superset.name);
                    add_and_update(partial_result, &mut res, &mut updated_relations, &mut partial_results_builder);
                }
            }
            // inclusion ab and ac imply inclusion a(b+c)
            for (composed_set, composed_elements) in composed_sets {
                if &relation.subset.id == &composed_set.id {
                    continue;
                }
                if composed_elements.contains(&relation.superset) {
                    debug!("attempting composition {} {}", relation.subset.id, composed_set.id);
                    let mut okay = true;
                    let opt_cpxs: Vec<SourcedCpxInfo> = composed_elements.iter()
                        .map(|x|res.get(&&WorkRelation::new(&relation.subset, &x)))
                        .filter_map(|x|if let Some(a) = x { Some(a.to_sourced()) } else { okay = false; None })
                        .collect();
                    if okay && opt_cpxs.len() > 0 {
                        let mut cpx: SourcedCpxInfo = opt_cpxs.get(0).unwrap().clone();
                        for i in 1 .. opt_cpxs.len() {
                            cpx = cpx.combine_plus(opt_cpxs.get(i).unwrap())
                        }
                        debug!("result: {:?}", cpx);
                        let handles: Vec<usize> = composed_elements.iter()
                            .map(|x|res.get(&&WorkRelation::new(&relation.subset, &x)))
                            .filter_map(|x|if let Some(a) = x { Some(a.handle) } else { None }).collect();
                        debug!("sum");
                        let key = WorkRelation::new(&relation.subset, &composed_set);
                        let partial_result = partial_results_builder.partial_result(CreatedBy::SumInclusion(handles), cpx.into(), key); // todo check
                        add_and_update(partial_result, &mut res, &mut updated_relations, &mut partial_results_builder);
                    }
                }
            }
        }
        progress.increase(improved_relations);
    }
    progress.done();
    let result: Vec<Relation> = res.values().map(|x: &PartialResult|{
        Relation::new(x.relation.clone(), x.to_sourced(), x.handle)
    }).collect();
    (result, partial_results_builder.done())
}

impl Relation {

    pub fn id(subset: &PreviewSet, superset: &PreviewSet) -> String {
        format!("{}_{}", subset.id, superset.id)
    }

    pub fn new(work_relation: WorkRelation, cpx: SourcedCpxInfo, handle: usize) -> Self {
        let preview = PreviewRelation {
            id: Relation::id(&work_relation.subset, &work_relation.superset),
            subset: work_relation.subset,
            superset: work_relation.superset,
            cpx: cpx.clone().into(),
        };
        Self {
            id: preview.id.clone(),
            handle,
            cpx,
            subset: preview.subset.clone(),
            superset: preview.superset.clone(),
            preview,
        }
    }

}

fn apply_transfers(
    transfers: &HashMap<TransferGroup, HashMap<PreviewSet, Vec<PreviewSet>>>,
    partial_result: &PartialResult,
    partial_results_builder: &mut PartialResultsBuilder,
    ) -> Vec<PartialResult> {
    let mut transferred_relations: Vec<PartialResult> = Vec::new();
    let top = partial_result.relation.subset.clone();
    let bot = partial_result.relation.superset.clone();
    for (transfer_group, map) in transfers.iter() {
        if let (Some(top_res), Some(bot_res)) = (map.get(&top), map.get(&bot)) {
            let mut res_cpx: SourcedCpxInfo = partial_result.clone().to_sourced();
            let okay = match res_cpx.clone() {
                SourcedCpxInfo::Inclusion { mn, mx } => {
                    res_cpx = SourcedCpxInfo::Inclusion {
                        mn,
                        mx: match mx {
                            // todo get rid of these exceptions via lambda that takes the result and transforms it
                            Some((Constant, smx)) => Some((CpxTime::Linear, smx)),
                            x => x,
                        },
                    };
                    true
                },
                _ => false,
            };
            if okay {
                let created_by = CreatedBy::TransferredFrom(transfer_group.clone(), partial_result.handle);
                for tr in top_res {
                    for br in bot_res {
                        let key = WorkRelation::new(&tr, &br);
                        let res = partial_results_builder.partial_result(created_by.clone(), res_cpx.clone().into(), key);
                        transferred_relations.push(res);
                    }
                }
            }
        }
    }
    transferred_relations
}

pub fn process_raw_data(rawdata: &RawData, bibliography: &Option<Bibliography>) -> Data {
    let preview_sets: Vec<PreviewSet> = rawdata.sets.iter().map(|x|x.clone().into()).collect();
    let preview_set_map: HashMap<String, PreviewSet> = preview_sets.iter().map(|x|(x.id.clone(), x.clone())).collect();
    let mut sources = vec![];
    let mut source_keys: HashMap<String, Source> = HashMap::new();
    for rawsource in &rawdata.sources {
        let source = process_source(&rawsource, &rawdata, &bibliography, &preview_set_map);
        source_keys.insert(rawsource.id.clone(), source.clone());
        sources.push(source);
    }
    sources.reverse();
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
    let (relations, partial_results) = process_relations(&preview_sets, &composed_sets, &rawdata.factoids, &transfers, &source_keys);
    let simple_index = SimpleIndex::new(&relations);
    let mut sets = vec![];
    for set in &rawdata.sets {
        sets.push(process_set(set, &simple_index, &rawdata, &source_keys));
    }
    // todo
    // let mut unused_ids: Vec<String> = vec![];
    // if let Some(unknown_source) = source_keys.get(UNKNOWN_SOURCE_ID) {
        // unused_ids.append(&mut unknown_source.showed.iter().map(|x|x.id.clone()).collect());
    // }
    // if let Some(assumed_source) = source_keys.get(ASSUMED_SOURCE_ID) {
        // unused_ids.append(&mut assumed_source.showed.iter().map(|x|x.id.clone()).collect());
    // }
    // if unused_ids.len() != 0 {
        // warn!("unnecessary unknown or assumed results with ids: {}", unused_ids.join(", "));
    // }
    Data::new(sets, relations, sources, providers, tags, partial_results)
}
