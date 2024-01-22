//! Utilities for building the final pages

use std::collections::HashMap;
use std::path::PathBuf;

use crate::{markdown::GeneratedPage, data::HasId};

// pub enum PageAction {
    // CreateDirectory { path: PathBuf },
    // Copy { source: PathBuf, target: PathBuf },
// }

pub struct Substitute {
    pub target: PathBuf,
    pub object: Box<dyn GeneratedPage>,
}

/// Values that may be inserted into [[]] custom markdown tags 
pub enum Keys {
    Handmade,
    Id { id: String },
}

// impl ToString for Keys {
    // fn to_string(&self) -> String {
        // match self {
            // Keys::Handmade => "content".into(),
            // Keys::Id { id } => format!("id {}", id),
        // }
    // }
// }

pub struct TargetPage<'a, 'b, 'c> {
    pub target: &'c PathBuf,
    pub substitute: Option<&'a Substitute>,
    pub source: Option<&'b PathBuf>,
}

pub fn add_content<T>(collection: &Vec<T>, final_dir: &PathBuf, generated_pages: &mut HashMap<PathBuf, Substitute>)
    where T: GeneratedPage + HasId + Clone + 'static,
{
    for element in collection {
        let filename = format!("{}.md", &element.id());
        let target_file = final_dir.join("html").join(filename);
        generated_pages.insert(target_file.clone(), Substitute{ target: target_file, object: Box::new(element.clone()) });
    }
}
