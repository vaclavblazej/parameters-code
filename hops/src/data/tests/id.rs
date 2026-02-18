use super::*;

type TestId = Id<TypeParameter>;
type TestPreviewId = PreviewId<TypeParameter>;

#[test]
fn id_new_and_display() {
    let id = TestId::new("abc");
    assert_eq!(id.to_string(), "abc");
}

#[test]
fn preview_id_from_str() {
    let pid = TestPreviewId::from("xyz");
    assert_eq!(pid.to_string(), "xyz");
}

#[test]
fn id_preview_round_trip() {
    let id = TestId::new("foo");
    let preview = id.preview();
    assert_eq!(preview.to_string(), "foo");
}

#[test]
fn cross_type_equality_id_eq_preview() {
    let id = TestId::new("same");
    let pid = TestPreviewId::from("same");
    assert_eq!(id, pid);
    assert_eq!(pid, id);
}

#[test]
fn cross_type_inequality() {
    let id = TestId::new("alpha");
    let pid = TestPreviewId::from("beta");
    assert_ne!(id, pid);
    assert_ne!(pid, id);
}

#[test]
fn relation_id_format() {
    let a = Id::<TypeParameter>::new("tw");
    let b = Id::<TypeParameter>::new("pw");
    let rid = RelationId::new(&a, &b);
    assert_eq!(rid.to_string(), "tw_pw");
}
