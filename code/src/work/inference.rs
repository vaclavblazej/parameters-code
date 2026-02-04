#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkRelation {
    pub subset: PreviewParameterId,
    pub superset: PreviewParameterId,
}

// todo, remove clone?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartialResult {
    pub handle: usize,
    pub created_by: CreatedBy,
    pub relation: WorkRelation,
    pub cpx: CpxInfo,
}

fn add_and_update(
    result: PartialResult,
    relation_map: &mut HashMap<WorkRelation, PartialResult>,
    updated_relations: &mut VecDeque<WorkRelation>,
    partial_result_builder: &mut PartialResultsBuilder,
) {
    let res = if let Some(x) = relation_map.get_mut(&result.relation) {
        if let Some(res) = x.combine_parallel(&result, partial_result_builder) {
            trace!(
                "updated relation (replace) {} {}",
                x.relation.subset, x.relation.superset,
            );
            res
        } else {
            return;
        }
    } else {
        trace!(
            "updated relation (insert) {} {}",
            result.relation.subset, result.relation.superset,
        );
        result
    };
    updated_relations.push_back(res.relation.clone());
    relation_map.insert(res.relation.clone(), res);
}

fn process_relations(
    composed_sets: &Vec<(PreviewParameterId, Vec<PreviewParameter>)>,
    transfers: &HashMap<TransferGroup, HashMap<PreviewParameterId, Vec<PreviewParameter>>>,
    sources: &HashMap<PreviewSourceId, Source>,
    preview_collection: &PreviewCollection,
) -> (Vec<Relation>, Vec<PartialResult>) {
    trace!("processing relations");
    let mut partial_results: Vec<PartialResult> = vec![];
    let mut partial_results_builder = PartialResultsBuilder::new();
    for (raw_source_id, showed) in &preview_collection.factoids {
        match &showed.fact {
            ShowedFact::Relation(status, relation) => {
                if matches!(status, ShowedStatus::Conjectured) {
                    continue;
                }
                if let Some(source) = sources.get(raw_source_id) {
                    let work_relation =
                        WorkRelation::new(&relation.subset.id, &relation.superset.id);
                    let partial_result = partial_results_builder.partial_result(
                        CreatedBy::Directly(source.preview()),
                        relation.cpx.clone(),
                        work_relation.clone(),
                    );
                    partial_results.push(partial_result);
                } else {
                    panic!("source not found {:?}", raw_source_id);
                }
            }
            ShowedFact::Definition(_, _) => (),
        }
    }
    let mut res: HashMap<WorkRelation, PartialResult> = HashMap::new();
    let mut progress = ProgressDisplay::new("processing", 22113);
    for partial_result in partial_results {
        let pair = partial_result.relation.clone();
        debug!(
            "processing relation from {} to {}",
            pair.subset, pair.superset
        );
        let mut updated_relations: VecDeque<WorkRelation> = VecDeque::new();
        // todo add progress in history when the collection is more complete
        add_and_update(
            partial_result,
            &mut res,
            &mut updated_relations,
            &mut partial_results_builder,
        );
        let mut improved_relations = 0;
        while let Some(relation) = updated_relations.pop_front() {
            improved_relations += 1;
            if improved_relations >= 5000 {
                panic!("5k updates during processing probably means a bug");
            }
            // apply the new or improved relation
            for set in &preview_collection.preview_sets {
                if set.id == relation.subset || set.id == relation.superset {
                    continue;
                }
                // equivalence ab copies the new relation cd into ef
                let xx = &relation.subset.clone();
                let yy = &relation.superset.clone();
                let zz = &set.id.clone();
                for (x, y, z) in [
                    (xx, yy, zz),
                    (xx, zz, yy),
                    (yy, xx, zz),
                    (yy, zz, xx),
                    (zz, xx, yy),
                    (zz, yy, xx),
                ] {
                    let Some(ab) = res.get(&WorkRelation::new(x, y)) else {
                        continue;
                    };
                    match ab.to_sourced() {
                        SourcedCpxInfo::Equal { source } => {
                            for (c, d, e, f) in
                                [(z, x, z, y), (z, y, z, x), (x, z, y, z), (y, z, x, z)]
                            {
                                let Some(cd) = res.get(&WorkRelation::new(c, d)) else {
                                    continue;
                                };
                                let created_by =
                                    CreatedBy::SameThroughEquivalence(cd.handle, source.handle);
                                let partial_result = partial_results_builder.partial_result(
                                    created_by,
                                    cd.cpx.clone(),
                                    WorkRelation::new(e, f),
                                );
                                debug!("equivalence");
                                add_and_update(
                                    partial_result,
                                    &mut res,
                                    &mut updated_relations,
                                    &mut partial_results_builder,
                                );
                            }
                        }
                        _ => continue,
                    }
                }
                // inclusion ab and inclusion bc imply inclusion ac
                for (a, b, c) in [
                    (&set.id, &relation.subset, &relation.superset),
                    (&relation.subset, &relation.superset, &set.id),
                ] {
                    let Some(ab) = res.get(&WorkRelation::new(a, b)) else {
                        continue;
                    };
                    let Some(bc) = res.get(&WorkRelation::new(b, c)) else {
                        continue;
                    };
                    if let (
                        SourcedCpxInfo::Inclusion {
                            mn: _,
                            mx: Some((mxa, sra)),
                        },
                        SourcedCpxInfo::Inclusion {
                            mn: _,
                            mx: Some((mxb, srb)),
                        },
                    ) = (ab.to_sourced(), bc.to_sourced())
                    {
                        let rel = sra.relation.combine_serial(&srb.relation);
                        let (a, b, time) = combine_serial((mxa, sra), (mxb, srb));
                        let pr = partial_results_builder.partial_result(
                            CreatedBy::TransitiveInclusion(a.handle, b.handle),
                            CpxInfo::Inclusion {
                                mn: None,
                                mx: Some(time),
                            },
                            rel,
                        );
                        debug!(
                            "inclusions {} {} + {} = {}",
                            updated_relations.len(),
                            a.handle,
                            b.handle,
                            c
                        );
                        add_and_update(
                            pr,
                            &mut res,
                            &mut updated_relations,
                            &mut partial_results_builder,
                        );
                    };
                }
                // inclusion ab and exclusion cd implies exclusion ef
                for (a, b, c, d, e, f) in [
                    (
                        &relation.subset,
                        &relation.superset,
                        &set.id,
                        &relation.superset,
                        &set.id,
                        &relation.subset,
                    ),
                    (
                        &relation.subset,
                        &relation.superset,
                        &relation.subset,
                        &set.id,
                        &relation.superset,
                        &set.id,
                    ),
                    (
                        &set.id,
                        &relation.superset,
                        &relation.subset,
                        &relation.superset,
                        &relation.subset,
                        &set.id,
                    ),
                    (
                        &relation.subset,
                        &set.id,
                        &relation.subset,
                        &relation.superset,
                        &set.id,
                        &relation.superset,
                    ),
                ] {
                    let Some(ab) = res.get(&WorkRelation::new(a, b)) else {
                        continue;
                    };
                    let Some(cd) = res.get(&WorkRelation::new(c, d)) else {
                        continue;
                    };
                    let res_relation = WorkRelation::new(e, f);
                    match (&ab.to_sourced(), &cd.to_sourced()) {
                        (
                            SourcedCpxInfo::Inclusion {
                                mn: _,
                                mx: Some((_, smx)),
                            },
                            SourcedCpxInfo::Exclusion { source },
                        ) => {
                            let created_by =
                                CreatedBy::TransitiveExclusion(smx.handle, source.handle);
                            let partial_result = partial_results_builder.partial_result(
                                created_by,
                                CpxInfo::Exclusion,
                                res_relation,
                            );
                            debug!("exclusions");
                            add_and_update(
                                partial_result,
                                &mut res,
                                &mut updated_relations,
                                &mut partial_results_builder,
                            );
                        }
                        _ => continue,
                    }
                }
            }
            // inclusion ab implies inclusion f(a)f(b) for a transfer f
            if let Some(ab) = res.get(&relation) {
                let new_partial_results =
                    apply_transfers(transfers, ab, &mut partial_results_builder);
                for partial_result in new_partial_results {
                    debug!(
                        "transfer from ({},{}) to ({},{})",
                        relation.subset,
                        relation.superset,
                        partial_result.relation.subset,
                        partial_result.relation.superset,
                    );
                    add_and_update(
                        partial_result,
                        &mut res,
                        &mut updated_relations,
                        &mut partial_results_builder,
                    );
                }
            }
            // inclusion ab and ac imply inclusion a(b+c)
            for (composed_set, composed_elements) in composed_sets {
                if &relation.subset == composed_set {
                    continue;
                }
                let hash_components: HashSet<PreviewParameterId> =
                    composed_elements.iter().map(|x| x.id.clone()).collect();
                if hash_components.contains(&relation.superset) {
                    debug!(
                        "attempting composition {} {}",
                        relation.subset, composed_set
                    );
                    let mut okay = true;
                    let opt_cpxs: Vec<SourcedCpxInfo> = composed_elements
                        .iter()
                        .map(|x| res.get(&WorkRelation::new(&relation.subset, &x.id)))
                        .filter_map(|x| {
                            if let Some(a) = x {
                                Some(a.to_sourced())
                            } else {
                                okay = false;
                                None
                            }
                        })
                        .collect();
                    if okay && !opt_cpxs.is_empty() {
                        let mut cpx: SourcedCpxInfo = opt_cpxs.first().unwrap().clone();
                        for i in 1..opt_cpxs.len() {
                            cpx = cpx.combine_plus(opt_cpxs.get(i).unwrap())
                        }
                        debug!("result: {:?}", cpx);
                        let handles: Vec<usize> = composed_elements
                            .iter()
                            .map(|x| res.get(&WorkRelation::new(&relation.subset, &x.id)))
                            .filter_map(|x| x.map(|a| a.handle))
                            .collect();
                        debug!("sum");
                        let key = WorkRelation::new(&relation.subset, composed_set);
                        let partial_result = partial_results_builder.partial_result(
                            CreatedBy::SumInclusion(handles),
                            cpx.into(),
                            key,
                        ); // todo check
                        add_and_update(
                            partial_result,
                            &mut res,
                            &mut updated_relations,
                            &mut partial_results_builder,
                        );
                    }
                }
            }
        }
        progress.increase(improved_relations);
    }
    progress.done();
    let result: Vec<Relation> = res
        .values()
        .map(|x: &PartialResult| {
            let subset = preview_collection
                .preview_set_map
                .get(&x.relation.subset)
                .unwrap()
                .clone();
            let superset = preview_collection
                .preview_set_map
                .get(&x.relation.superset)
                .unwrap()
                .clone();
            Relation::new(subset, superset, x.to_sourced(), x.handle)
        })
        .collect();
    (result, partial_results_builder.done())
}
