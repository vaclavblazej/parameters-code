use std::path::PathBuf;

use biblatex::Bibliography;
use log::error;

use crate::general::file;

pub fn load_bibliography(bibliography_file: &PathBuf) -> Option<Bibliography> {
    let bibliography_res = file::read_file_content(&bibliography_file);
    match bibliography_res {
        Ok(bibliography_str) => Some(Bibliography::parse(&bibliography_str).unwrap()),
        Err(error) => {
            error!("cannot load bibliography from {:?}", bibliography_file);
            error!("{:?}", error);
            None
        }
    }
}
