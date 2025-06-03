use std::{collections::HashMap, path::PathBuf, sync::mpsc::{self, Sender}, thread::{self, JoinHandle}};

use anyhow::Result;
use log::info;

use crate::{data::preview::PreviewSet, general::file, output::table::render_table, work::processing::RelatedSets, Paths};


pub struct Worker {
    sender: Sender<Option<Task>>,
    join_handle: JoinHandle<()>
}

impl Worker {

    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let join_handle = thread::spawn(move || {
            info!("spawned a worker thread");
            while let Ok(message) = receiver.recv() {
                if let Some(content) = message {
                    match content {
                        Task::CreateTable { related_sets_map, ordered_draw_sets, paths, name } => {
                            build_table(&related_sets_map, &ordered_draw_sets, &paths, name);
                        }
                    }
                } else {
                    return;
                }
            }
        });
        Self { sender, join_handle }
    }

    pub fn send(&self, task: Task) {
        self.sender.send(Some(task));
    }

    pub fn join(self) {
        self.sender.send(None);
        self.join_handle.join();
    }

}

fn build_table(
    related_sets_map: &HashMap<PreviewSet, RelatedSets>,
    ordered_draw_sets: &[PreviewSet],
    paths: &Paths,
    name: String,
) -> Result<()> {
    let tmp_folder = &paths.table_tikz_folder
        .parent().unwrap_or_else(||panic!("the used path is not expected to be the root"))
        .join(format!("thread_tmp_{}", name));
    file::copy_folder(&paths.table_tikz_folder, tmp_folder);
    let done_pdf = render_table(related_sets_map, ordered_draw_sets, tmp_folder)?;
    let final_pdf = paths.html_dir.join(format!("{}.pdf", name));
    info!("copy the pdf to {:?}", &final_pdf);
    file::copy_file(&done_pdf, &final_pdf);
    Ok(())
}

pub enum Task {
    CreateTable{
        related_sets_map: HashMap<PreviewSet, RelatedSets>,
        ordered_draw_sets: Vec<PreviewSet>,
        paths: Box<Paths>,
        name: String,
    },
}
