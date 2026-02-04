// // transfers ///////////////////////////////////////////////////////////////
// let mut transfers: HashMap<TransferGroup, HashMap<PreviewParameterId, Vec<PreviewParameter>>> =
//     HashMap::new();
// for (key, raw_pairs) in &raw_transfer {
//     let mut res: HashMap<PreviewParameterId, Vec<PreviewParameter>> = HashMap::new();
//     for raw_pair in raw_pairs {
//         let (from, to) = raw_pair.clone();
//         let res_from: PreviewParameter = preview_collection
//             .preview_set_map
//             .get(&from)
//             .unwrap()
//             .clone();
//         let res_to: PreviewParameter = preview_collection.preview_set_map.get(&to).unwrap().clone();
//         res.entry(res_from.id).or_default().push(res_to);
//     }
//     transfers.insert(key.clone(), res);
// }

fn apply_transfers(
    transfers: &HashMap<TransferGroup, HashMap<PreviewParameterId, Vec<PreviewParameter>>>,
    partial_result: &PartialResult,
    partial_results_builder: &mut PartialResultsBuilder,
) -> Vec<PartialResult> {
    let mut transferred_relations: Vec<PartialResult> = Vec::new();
    let top = &partial_result.relation.subset;
    let bot = &partial_result.relation.superset;
    for (transfer_group, map) in transfers.iter() {
        if let (Some(top_res), Some(bot_res)) = (map.get(top), map.get(bot)) {
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
                }
                _ => false,
            };
            if okay {
                let created_by =
                    CreatedBy::TransferredFrom(transfer_group.clone(), partial_result.handle);
                for tr in top_res {
                    for br in bot_res {
                        let key = WorkRelation::new(&tr.id, &br.id);
                        let res = partial_results_builder.partial_result(
                            created_by.clone(),
                            res_cpx.clone().into(),
                            key,
                        );
                        transferred_relations.push(res);
                    }
                }
            }
        }
    }
    transferred_relations
}
