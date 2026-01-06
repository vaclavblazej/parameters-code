//! A tiny utility to print elsapsed time along with messages.

use std::time::Instant;

use log::info;

pub struct Timer {
    instant: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
        }
    }

    pub fn print(&self, message: &str) {
        info!("{:?} {}", self.instant.elapsed(), message);
    }
}
