use std::path::PathBuf;

use anyhow::Result;
use biblatex::Bibliography;
use log::error;

use crate::general::file;

pub fn load_bibliography(bibliography_file: &PathBuf) -> Result<Bibliography> {
    let bibliography_res = file::read_file_content(bibliography_file);
    match bibliography_res {
        Ok(bibliography_str) => Ok(Bibliography::parse(&bibliography_str).unwrap()),
        Err(error) => {
            error!("cannot load bibliography from {:?}", bibliography_file);
            Err(error)
        }
    }
}
