// use std::collections::{HashMap, HashSet};

// use crate::raw::RawSet;

// use super::build::Data;

// fn postprocess(data: &mut Data) -> &Data {
    // propagate_equivalences(data);
    // propagate_parent_to_notes(data);
    // derive_bound_properties_from_notes(data);
    // set_edge_properties_to_nodes(data);
    // propagate_graph_classes(data);
    // derive_unboundeness_from_graph_classes(data);
    // propagate_bounds(data);
    // derive_connection_properties(data);
    // data
// }

// // fn generate_distance_parameters(data: &mut Data) {
    // // for graph_class in &data.graph_classes {
        // // let par = distance_to(&graph_class);
        // // has_bounded(
            // // &mut data,
            // // &format!("{}_itself", graph_class.id),
            // // graph_class,
            // // par,
            // // vec![Note {
                // // id: format!("{}_itself_note", graph_class.id),
                // // text: "Distance to a graph class contains itself even when no vertices were removed.".to_string(),
            // // }],
        // // );
    // // }
    // // let inclusion_note = Note {
        // // id: "!6nm4uH".to_string(),
        // // text: "graph inclusion".to_string(),
        // // range: Range(LINEAR),
    // // };
    // // for graph_inclusion in &data.graph_inclusions {
        // // let upperpar = distance_to(&graph_inclusion.subclass);
        // // let lowerpar = distance_to(&graph_inclusion.superclass);
        // // bound(
            // // &mut data,
            // // upperpar,
            // // lowerpar,
            // // vec![inclusion_note.clone()],
        // // );
    // // }
// // }

// fn propagate_equivalences(data: &mut Data) {
    // let mut eq_remove_map: HashMap<&RawSet, &Equivalency> = HashMap::new();
    // for eq in &data.equivalencies {
        // eq_remove_map.insert(&eq.removed, eq);
        // eq.persist.notes.push(Note {
            // id: format!("{}_note", eq.id),
            // text: format!("is equivalent to [{}](#{})", eq.removed.name, eq.removed.id),
        // });
        // eq.removed.notes.push(Note {
            // id: format!("{}_note", eq.id),
            // text: format!("is equivalent to [{}](#{}) (see more info there)", eq.persist.name, eq.persist.id),
        // });
        // let eq_note = Note {
            // id: format!("{}_note", eq.id),
            // text: format!("is equivalent to [{}](#{})", eq.removed.name, eq.removed.id),
            // range: Range(LINEAR, LINEAR),
        // };
        // let bound = data.get_bound(eq.persist, eq.removed);
        // bound.notes.push(eq_note);
        // let bound2 = data.get_bound(eq.removed, eq.persist);
        // bound2.notes.push(eq_note);
    // }
    // // Implement the remaining logic
// }

// fn propagate_parent_to_notes(data: &mut Data) {
    // for graph_class in &mut data.graph_classes {
        // for note in &mut graph_class.notes {
            // if let Some(parent) = note.parent.as_mut() {
                // *parent = graph_class.clone();
            // }
        // }
    // }
    // for parameter in &mut data.parameters {
        // for note in &mut parameter.notes {
            // if let Some(parent) = note.parent.as_mut() {
                // *parent = parameter.clone();
            // }
        // }
    // }
// }

// fn derive_bound_properties_from_notes(data: &mut Data) {
    // for bound in data.iter_bounds() {
        // for note in &bound.notes {
            // if let Some(range) = &note.range {
                // bound.range.intersect(range);
            // }
        // }
    // }
// }

// fn set_edge_properties_to_nodes(data: &mut Data) {
    // for bound in data.iter_bounds() {
        // if bound.range.is_bounded() {
            // bound.to.above.push(bound.fr.clone());
            // bound.fr.below.push(bound.to.clone());
        // }
    // }
    // for equivalency in &data.equivalencies {
        // equivalency.persist.above.push(equivalency.removed.clone());
        // equivalency.persist.below.push(equivalency.removed.clone());
        // equivalency.removed.above.push(equivalency.persist.clone());
        // equivalency.removed.below.push(equivalency.persist.clone());
    // }
    // for parameter in &mut data.parameters {
        // parameter.above_no_eq.extend(parameter.above.difference(&parameter.below));
        // parameter.below_no_eq.extend(parameter.below.difference(&parameter.above));
    // }
// }

// fn propagate_graph_classes(data: &mut Data) {
    // let universe: Vec<(&GraphClass, &Parameter)> = cross(&data.graph_classes, &data.parameters).collect();

    // let mut down_seed: Vec<(&GraphClass, &Parameter)> = vec![];
    // for graph_class in &data.graph_classes {
        // for parameter in &graph_class.contained_in {
            // down_seed.push((graph_class, parameter));
        // }
    // }
    // let res_down = iterate_next(&universe, &down_seed, down, true);

    // for parameter in &mut data.parameters {
        // parameter.bounded_for.clear();
    // }

    // for graph_class in &mut data.graph_classes {
        // graph_class.has_bounded.clear();
    // }

    // for (_, (gc, par)) in res_down {
        // par.bounded_for.push(gc.clone());
        // gc.contained_in.push(par.clone());
    // }

    // let mut up_seed: Vec<(&GraphClass, &Parameter)> = vec![];
    // for graph_class in &data.graph_classes {
        // for parameter in &graph_class.not_contained_in {
            // up_seed.push((graph_class, parameter));
        // }
    // }
    // let res_up = iterate_next(&universe, &up_seed, up, false);

    // for (_, (gc, par)) in res_up {
        // par.unbounded_for.push(gc.clone());
        // gc.not_contained_in.push(par.clone());
    // }
// }

// fn nonisomorphism_witnesses(upper: &GraphClass, lower: &GraphClass) -> Vec<GraphClass> {
    // let upper_unbounded_for: HashSet<_> = upper.unbounded_for.iter().collect();
    // let lower_bounded_for: HashSet<_> = lower.bounded_for.iter().collect();
    // let intersection: HashSet<_> = upper_unbounded_for.intersection(&lower_bounded_for).collect();
    // intersection.into_iter().cloned().collect()
// }

// fn derive_connection_properties(data: &mut Data) {
    // for connection in data.iter_connections() {
        // let witnesses = nonisomorphism_witnesses(&connection.fr, &connection.to);
        // if !witnesses.is_empty() {
            // connection.reverse_bound.range.mn_ub = DOES_NOT_EXIST;
            // if let Some(graph_class) = witnesses.first() {
                // let strict_fw_note = Note {
                    // id: format!("{}_strict", connection.id),
                    // text: format!(
                        // "This inclusion is proper because [{}](../{}) graph class has bounded [{}](../{}) but unbounded [{}](../{}).",
                        // graph_class.name, graph_class.id, connection.to.name, connection.to.id, connection.fr.name, connection.fr.id,
                    // ),
                // };
                // connection.forward_bound.notes.push(strict_fw_note);

                // let strict_rw_note = Note {
                    // id: format!("{}_strict", connection.id),
                    // text: format!(
                        // "[{}](../{}) cannot bound [{}](../{}) because we know that [{}](../{}) graph class has bounded {} but unbounded {}.",
                        // connection.to.name, connection.to.id, connection.fr.name, connection.fr.id, graph_class.name, graph_class.id, connection.to.name, connection.fr.name,
                    // ),
                // };
                // connection.reverse_bound.notes.push(strict_rw_note);
            // }

            // let mut can_move_head_up = true;
            // for anc in &connection.to.above_no_eq {
                // if anc == &connection.fr {
                    // continue;
                // }
                // let anc_witnesses = nonisomorphism_witnesses(&connection.fr, anc);
                // if anc_witnesses.is_empty() {
                    // can_move_head_up = false;
                    // break;
                // }
            // }
            // if !can_move_head_up {
                // connection.known_optimal_head = true;
            // }

            // let mut can_move_tail_down = true;
            // for des in &connection.fr.below_no_eq {
                // if des == &connection.to {
                    // continue;
                // }
                // let des_witnesses = nonisomorphism_witnesses(des, &connection.to);
                // if des_witnesses.is_empty() {
                    // can_move_tail_down = false;
                    // break;
                // }
            // }
            // if !can_move_tail_down {
                // connection.known_optimal_tail = true;
            // }
        // }
    // }
// }


// fn derive_unboundeness_from_graph_classes(data: &mut Data) {
    // for graph_class in &data.graph_classes {
        // for (fr, to) in cross(&graph_class.contained_in, &graph_class.not_contained_in) {
            // let bound = data.get_bound(fr, to);
            // bound.range.mn_ub = DOES_NOT_EXIST;
        // }
    // }
// }

// fn cross<'a, T, U>(iter_a: &'a Vec<T>, iter_b: &'a Vec<U>) -> impl Iterator<Item = (&'a T, &'a U)> {
    // iter_a.iter().flat_map(move |a| iter_b.iter().map(move |b| (a, b)))
// }

// fn iterate_next<'a, T, F>(
    // universe: &'a Vec<T>,
    // seeds: Vec<(&'a T, &'a T)>,
    // next_function: F,
    // include_seeds: bool,
// ) -> Vec<(&'a T, &'a T)>
// where
    // F: Fn(&'a T) -> Vec<&'a T>,
// {
    // let mut unvisited: HashSet<&'a T> = universe.iter().collect();
    // for seed in &seeds {
        // assert!(unvisited.contains(seed.0) && unvisited.contains(seed.1));
        // unvisited.remove(seed.0);
        // unvisited.remove(seed.1);
    // }

    // let mut open: Vec<(&'a T, &'a T)> = seeds.clone();
    // let mut result: Vec<(&'a T, &'a T)> = Vec::new();

    // if include_seeds {
        // result.extend(seeds);
    // }

    // let mut i = 0;
    // while i < open.len() {
        // let (par, _) = open[i];
        // for bpar in next_function(par) {
            // if unvisited.contains(bpar) {
                // unvisited.remove(bpar);
                // open.push((par, bpar));
                // result.push((par, bpar));
            // }
        // }
        // i += 1;
    // }

    // result
// }

// fn propagate_bounds(data: &mut Data) {
    // for param in &data.parameters {
        // for (parent, desc_par) in iterate_next(
            // &data.parameters,
            // vec![(&param, &param)],
            // |p| p.below.clone(),
            // true,
        // ) {
            // let a = data.get_bound(param, parent).clone();
            // let b = data.get_bound(parent, desc_par).clone();
            // let c = data.get_bound(param, desc_par);
            // c.enrich(a.concatenate(b));
        // }
    // }
// }

// fn main() {
    // // Create and manipulate Data instance here
    // let mut data = Data {
        // graph_classes: vec![],
        // parameters: vec![],
        // // Initialize other fields as needed
    // };

    // // Call postprocess to process the data
    // postprocess(&mut data);

    // // Access the processed data
    // println!("{:?}", data);
// }
