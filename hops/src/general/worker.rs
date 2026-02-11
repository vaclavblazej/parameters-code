//! Basic parallel processing utility to spawn a worker that can be given tasks.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use anyhow::Result;
use log::info;

pub trait Task {
    fn process(&self) -> Result<()>;
}

pub struct Worker {
    sender: mpsc::Sender<Option<Box<dyn Task + Send>>>,
    join_handle: thread::JoinHandle<()>,
}

type MspcChannel = (
    mpsc::Sender<Option<Box<dyn Task + Send>>>,
    mpsc::Receiver<Option<Box<dyn Task + Send>>>,
);

impl Worker {
    pub fn new() -> Self {
        let (sender, receiver): MspcChannel = mpsc::channel();
        let join_handle = thread::spawn(move || {
            info!("spawned a worker thread");
            while let Ok(message) = receiver.recv() {
                match message {
                    Some(content) => {
                        // Handle potential errors in processing
                        if let Err(e) = content.process() {
                            eprintln!("Error processing task: {:?}", e);
                        }
                    }
                    None => {
                        info!("Received termination signal");
                        return;
                    }
                };
            }
        });
        Self {
            sender,
            join_handle,
        }
    }

    pub fn send(&self, task: Box<dyn Task + Send>) {
        if let Err(e) = self.sender.send(Some(task)) {
            eprintln!("Failed to send task: {:?}", e);
        }
    }

    pub fn join(self) {
        let _ = self.sender.send(None);
        let _ = self.join_handle.join();
    }
}
