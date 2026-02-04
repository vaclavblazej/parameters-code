//! Utilities for building the final pages

use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use crate::data::id::{AbstractId, HasId};

use super::markdown::GeneratedPage;

pub struct Substitute<'a> {
    pub target: PathBuf,
    pub object: &'a dyn GeneratedPage,
}

/// Values that may be inserted into [[]] custom markdown tags
pub enum Keys {
    Handmade,
    Id { id: String },
}

pub struct TargetPage<'a> {
    pub target: &'a PathBuf,
    pub substitute: Option<&'a Substitute<'a>>,
    pub source: Option<&'a PathBuf>,
}

pub fn add_content<'a, T, I>(
    collection: I,
    final_dir: &Path,
    generated_pages: &mut HashMap<PathBuf, Substitute<'a>>,
) where
    I: Iterator<Item = &'a T>,
    T: GeneratedPage + HasId + 'static,
{
    for element in collection {
        let filename = format!("{}.md", element.id());
        let target_file = final_dir.join("html").join(filename);
        generated_pages.insert(
            target_file.clone(),
            Substitute {
                target: target_file,
                object: element,
            },
        );
    }
}
