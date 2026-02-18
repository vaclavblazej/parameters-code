use super::*;

fn make_index() -> SimpleIndex<&'static str> {
    let relations = vec![
        ("A", "B", InclusionStatus::Inclusion),
        ("B", "A", InclusionStatus::Inclusion),
        ("B", "C", InclusionStatus::Inclusion),
        ("D", "C", InclusionStatus::Exclusion),
    ];
    SimpleIndex::new(&relations)
}

#[test]
fn eqsets_mutual_inclusion() {
    let idx = make_index();
    let mut eq = idx.get_eqsets(&"A");
    eq.sort();
    assert_eq!(eq, vec!["A", "B"]);
}

#[test]
fn subsets_excludes_equivalences() {
    let idx = make_index();
    let subs = idx.get_subsets(&"B");
    assert!(!subs.contains(&"A"));
}

#[test]
fn supersets_excludes_equivalences() {
    let idx = make_index();
    let sups = idx.get_supersets(&"A");
    assert!(!sups.contains(&"B"));
}

#[test]
fn supersets_includes_strict() {
    let idx = make_index();
    let sups = idx.get_supersets(&"B");
    assert!(sups.contains(&"C"));
}

#[test]
fn antisubsets() {
    let idx = make_index();
    let anti = idx.get_antisubsets(&"C");
    assert!(anti.contains(&"D"));
}

#[test]
fn antisupersets() {
    let idx = make_index();
    let anti = idx.get_antisupersets(&"D");
    assert!(anti.contains(&"C"));
}

#[test]
fn first_subset_of_second_query() {
    let idx = make_index();
    assert!(idx.first_subset_of_second(&"A", &"B"));
    assert!(idx.first_subset_of_second(&"B", &"C"));
    assert!(!idx.first_subset_of_second(&"C", &"A"));
}
