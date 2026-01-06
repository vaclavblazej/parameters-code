use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use anyhow::Result;
use log::info;

use crate::general::file;
use crate::output::table::render_table;
use crate::work::sets::RelatedSets;

pub struct Worker {
    sender: Sender<Option<Box<dyn Task>>>,
    join_handle: JoinHandle<()>,
}

impl Worker {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let join_handle = thread::spawn(move || {
            info!("spawned a worker thread");
            while let Ok(message) = receiver.recv() {
                match message {
                    Some(content) => content.process(),
                    None => return,
                }
                // if let Some(content) = message {
                // content.process();
                // } else {
                // return;
                // }
            }
        });
        Self {
            sender,
            join_handle,
        }
    }

    pub fn send(&self, task: Task) {
        self.sender.send(Some(task));
    }

    pub fn join(self) {
        self.sender.send(None);
        self.join_handle.join();
    }
}

pub trait Task {
    fn process(self) -> Result<()>;
}
