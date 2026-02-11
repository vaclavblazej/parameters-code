use std::env;
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct Paths {
    pub(crate) parent: PathBuf,
    pub(crate) table_tikz_folder: PathBuf,
    pub(crate) handcrafted_dir: PathBuf,
    pub(crate) bibliography_file: PathBuf,
    pub(crate) final_dir: PathBuf,
    pub(crate) hugo_public_dir: PathBuf,
    pub(crate) working_dir: PathBuf,
    pub(crate) html_dir: PathBuf,
    pub(crate) api_dir: PathBuf,
    pub(crate) tmp_dir: PathBuf,
}

impl Paths {
    pub(crate) fn new() -> Self {
        let current = env::current_dir().unwrap();
        let parent = current.parent().unwrap();
        let table_tikz_folder = parent.join("scripts").join("table_tikz");
        let handcrafted_dir = parent.join("handcrafted");
        let bibliography_file = handcrafted_dir.join("main.bib");
        let final_dir = parent.join("web").join("content");
        let hugo_public_dir = parent.join("web").join("public");
        let temp_dir = env::temp_dir();
        let working_dir = temp_dir.join("target");
        let html_dir = final_dir.join("html");
        let api_dir = final_dir.join("api");
        let tmp_dir = temp_dir.join("tmp");
        Self {
            parent: parent.to_path_buf(),
            table_tikz_folder,
            handcrafted_dir,
            bibliography_file,
            final_dir,
            hugo_public_dir,
            working_dir,
            html_dir,
            api_dir,
            tmp_dir,
        }
    }
}
