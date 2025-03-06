//! Utilities for building the final pages

use std::collections::HashMap;
use std::path::PathBuf;

use crate::data::id::{AbstractId, HasId};

use super::markdown::GeneratedPage;

pub struct Substitute<'a> {
    pub target: PathBuf,
    pub object: Box<&'a dyn GeneratedPage>,
}

/// Values that may be inserted into [[]] custom markdown tags
pub enum Keys {
    Handmade,
    Id { id: String },
}

pub struct TargetPage<'a, 'b, 'c> {
    pub target: &'c PathBuf,
    pub substitute: Option<&'a Substitute<'a>>,
    pub source: Option<&'b PathBuf>,
}

pub fn add_content<'a, T>(
    collection: &'a Vec<T>,
    final_dir: &PathBuf,
    generated_pages: &mut HashMap<PathBuf, Substitute<'a>>,
) where
    T: GeneratedPage + HasId + 'static,
{
    for element in collection {
        let filename = format!("{}.md", element.id());
        let target_file = final_dir.join("html").join(filename);
        generated_pages.insert(
            target_file.clone(),
            Substitute {
                target: target_file,
                object: Box::new(element),
            },
        );
    }
}
