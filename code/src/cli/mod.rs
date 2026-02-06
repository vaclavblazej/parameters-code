pub mod computation;
pub mod interactive;
pub mod page_build;
pub mod paths;

use std::collections::HashSet;
use std::env;

use crate::general::logger;

#[derive(Hash, PartialEq, Eq)]
pub(crate) enum Args {
    Preprocess,
    Dots,
    Pages,
    Table,
    Api,
    Clear,
    Interactive,
    Debug,
    Trace,
}

pub(crate) fn parse_args_and_init_logger() -> HashSet<Args> {
    let rawargs: Vec<String> = env::args().collect();
    let mut args = HashSet::new();
    for (i, arg) in rawargs.iter().enumerate() {
        if i == 0 {
            continue;
        }
        match arg.as_str() {
            "preprocess" => {
                args.insert(Args::Preprocess);
            }
            "dots" => {
                args.insert(Args::Dots);
            }
            "pages" => {
                args.insert(Args::Pages);
            }
            "table" => {
                args.insert(Args::Table);
            }
            "clear" => {
                args.insert(Args::Clear);
            }
            "trace" => {
                args.insert(Args::Trace);
            }
            "debug" => {
                args.insert(Args::Debug);
            }
            "fast" => {
                args.insert(Args::Clear);
                args.insert(Args::Preprocess);
                args.insert(Args::Dots);
                args.insert(Args::Pages);
            }
            "api" => {
                args.insert(Args::Api);
            }
            "all" => {
                args.insert(Args::Clear);
                args.insert(Args::Preprocess);
                args.insert(Args::Dots);
                args.insert(Args::Pages);
                args.insert(Args::Api);
                args.insert(Args::Table);
            }
            "interactive" | "i" => {
                args.insert(Args::Interactive);
            }
            other => panic!("unknown parameter: '{}'", other),
        }
    }
    logger::init(if args.contains(&Args::Trace) {
        log::LevelFilter::Trace
    } else if args.contains(&Args::Debug) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    });
    args
}
