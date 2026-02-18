
use crate::input::{build::CollectionBuilder, raw::RawData};

use super::*;

#[test]
fn test_collection() {
    assert_eq!(true, true);
}

pub fn build_collection() -> RawData {
    let mut create = CollectionBuilder::new();
    let tag_topology = create.tag("lJJaYb", "test tag", "test tag description");
    create.build()
}
