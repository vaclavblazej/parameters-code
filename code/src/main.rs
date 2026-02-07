#![allow(unused)]
// #![deny(clippy::unwrap_used)]
// #![deny(clippy::expect_used)]
// #![deny(clippy::panic)]
// #![deny(unused_must_use)]

mod general {
    pub mod cache;
    pub mod file;
    pub mod logger;
    pub mod progress;
    pub mod strings;
    pub mod timer;
    pub mod worker;
}
mod input {
    pub mod build;
    pub mod builder;
    pub mod concretizable;
    pub mod distance_to;
    pub mod intersectable;
    pub mod provider;
    pub mod raw;
    pub mod raw_enums;
    pub mod source;
}
pub mod data {
    pub mod bibliography;
    pub mod data;
    pub mod date;
    pub mod digraph;
    pub mod enums;
    pub mod id;
    pub mod link;
    pub mod preview;
    pub mod score;
    pub mod simple_index;
}
mod work {
    pub mod combine;
    pub mod compare;
    pub mod convert;
    pub mod hide;
    pub mod hierarchy;
    pub mod preview_collection;
    pub mod processing;
}
mod output {
    pub mod api;
    pub mod color;
    pub mod diagram;
    pub mod dot;
    pub mod markdown;
    pub mod pages;
    pub mod table;
    pub mod to_markdown;
}
mod cli;
mod collection;

fn main() {
    let mut computation = cli::computation::Computation::new();
    computation.clear();
    computation.retrieve_and_process_data();
    computation.make_dots();
    computation.make_relation_table();
    computation.make_api();
    computation.make_pages();
    computation.interactive();
    computation.worker.join();
}
