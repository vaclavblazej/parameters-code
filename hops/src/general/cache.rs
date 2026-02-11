//! Save objects into temporary files to cache results of processing.

use std::path::PathBuf;
use std::{marker::PhantomData, path::Path};

use anyhow::Result;
use log::{debug, error};
use serde::{Serialize, de::DeserializeOwned};

use crate::general::file;

pub struct Cache<T> {
    file: PathBuf,
    _marker: PhantomData<T>,
}

/// Serializable object
impl<T> Cache<T> {
    pub fn new(file: &Path) -> Self {
        Self {
            file: file.to_path_buf(),
            _marker: PhantomData,
        }
    }

    pub fn save(&self, object: &T) -> Result<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string_pretty(object)?;
        debug!("writing content");
        file::write_file_content(&self.file, serialized.as_str())?;
        debug!("done content");
        Ok(())
    }

    pub fn load(&self) -> Option<T>
    where
        T: DeserializeOwned,
    {
        if !self.file.exists() {
            return None;
        }
        let serialized = match file::read_file_content(&self.file) {
            Ok(res) => res,
            Err(err) => {
                error!("{:?}", err);
                return None;
            }
        };
        serde_json::from_str(&serialized).unwrap_or_else(|err| {
            error!("{:?}", err);
            None
        })
    }
}
