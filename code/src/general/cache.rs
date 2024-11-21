use std::path::PathBuf;
use std::marker::PhantomData;

use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};

use crate::general::file;

pub struct Cache<T> {
    file: PathBuf,
    _marker: PhantomData<T>,
}

impl<T> Cache<T> {
    pub fn new(file: &PathBuf) -> Self {
        Self {
            file: file.clone(),
            _marker: PhantomData,
        }
    }

    pub fn save(&self, object: &T) -> Result<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string_pretty(object)?;
        println!("writing content");
        file::write_file_content(&self.file, serialized.as_str())?;
        println!("done content");
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
                println!("{:?}", err);
                return None
            },
        };
        serde_json::from_str(&serialized).unwrap_or_else(|err|{
            println!("{:?}", err);
            return None;
        })
    }
}
