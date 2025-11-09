//! Utilities for processing files and folders.

use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    fmt,
};

use anyhow::Result;
use log::{error, warn};


pub fn iterate_folder_recursively(path: &PathBuf) -> Vec<PathBuf> {
    let mut res = vec![];
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            res.push(path.clone());
                            res.extend(iterate_folder_recursively(&path));
                        }
                        if path.is_file() {
                            res.push(path);
                        }
                    }
                    Err(err) => {
                        error!("Error iterating directory entry: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            error!("Error reading directory: {}", err);
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

pub fn append_file_content(target_path: &PathBuf, content: &str) -> Result<()> {
    let mut target_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(target_path)?;
    target_file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn write_file_content(target_path: &PathBuf, content: &str) -> Result<()> {
    // assert!(!target_path.exists());
    try_create_parent_folder(target_path)?;
    let mut target_file = File::create(target_path)?;
    target_file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn reparent(target: &Path, old_ancestor: &Path, new_ancestor: &Path) -> Result<PathBuf> {
    let rel_target = target.strip_prefix(old_ancestor)?;
    let mut new_target = new_ancestor.to_path_buf();
    new_target.extend(rel_target);
    Ok(new_target)
}

pub fn copy_folder(source_path: &PathBuf, target_path: &Path) -> Result<()> {
    if !source_path.exists() {
        error!("tried to copy a non-existent folder {:?}", source_path);
        return Ok(()); // todo
    }
    assert!(source_path.is_dir());
    // todo check that target is not a descendant of source
    try_create_parent_folder(target_path)?;
    let to_copy_files = iterate_folder_recursively(source_path);
    for source_file in to_copy_files {
        let target_file = reparent(&source_file, source_path, target_path)?;
        try_create_parent_folder(&target_file)?;
        fs::copy(source_file, target_file)?;
    }
    Ok(())
}

pub fn copy_file(source_path: &PathBuf, target_path: &PathBuf) -> Result<()> {
    if !source_path.exists() {
        error!("tried to copy a non-existent file {:?}", source_path);
        return Ok(()); // todo
    }
    assert!(source_path.is_file());
    try_create_parent_folder(target_path)?;
    fs::copy(source_path, target_path)?;
    Ok(())
}

pub fn remove_file(target_path: &PathBuf) -> Result<()> {
    if target_path.exists() {
        fs::remove_file(target_path)?;
    } else {
        warn!("tried to remove non-existent file {:?}", target_path);
    }
    Ok(())
}

pub fn clear_folder(target_path: &PathBuf) -> Result<()> {
    if target_path.exists() {
        fs::remove_dir_all(target_path);
        fs::create_dir(target_path);
    } else {
        warn!("tried to remove non-existent folder {:?}", target_path);
    }
    Ok(())
}

pub fn try_create_parent_folder(target_path: &Path) -> Result<()> {
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}
