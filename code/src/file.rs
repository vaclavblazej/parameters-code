//! Utilities for processing files and folders.


use std::{path::PathBuf, fs::{File, self}, io::{Read, Write}};

use anyhow::Result;

pub fn iterate_folder(path: &PathBuf) -> Vec<PathBuf> {
    let mut res = vec![];
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            res.push(path.clone());
                            res.extend(iterate_folder(&path));
                        }
                        if path.is_file() {
                            res.push(path);
                        }
                    }
                    Err(err) => {
                        eprintln!("Error iterating directory entry: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
        }
    }
    res
}

pub fn read_file_content(source_path: &PathBuf) -> Result<String> {
    let mut source_file = File::open(source_path)?;
    let mut content = String::new();
    source_file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn append_file_content(target_path: &PathBuf, content: &String) -> Result<()> {
    let mut target_file = fs::OpenOptions::new().write(true).append(true).create(true).open(target_path).unwrap();
    target_file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn write_file_content(target_path: &PathBuf, content: &String) -> Result<()> {
    let mut target_file = File::create(target_path)?;
    target_file.write_all(content.as_bytes())?;
    Ok(())
}
