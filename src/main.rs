#![feature(box_syntax)]
#![feature(plugin)]
#![feature(path_relative_from)]
#![feature(path_ext)]
#![feature(fs_walk)]

#![plugin(docopt_macros, regex_macros)]

// Rust-lang libraries.
extern crate regex;
extern crate rustc_serialize;
extern crate glob;
#[macro_use] extern crate log;
extern crate env_logger;

// Third-party libraries.
extern crate sequence_trie;
extern crate docopt;

use std::fs::File;
use std::io::Read;
use std::error::Error as StdError;

use config::PatternSource::IncludeFile;
use parser::parse_include_file;
use sync::sync;

// Configuration and argument parsing.
pub mod arg_parser;
pub mod config;

// File system manipulation.
pub mod compare;
pub mod path;

// Include file parsing.
pub mod parser;
pub mod pattern;

// Selection algorithm logic.
pub mod matcher;
pub mod sync;

pub mod error;
//pub mod debug;

fn main() {
    // Set up logging.
    env_logger::init().unwrap();

    info!("Skarn starting up");

    // Parse the command-line arguments to create a config file.
    let mut config = match arg_parser::parse_args() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e.description());
            return;
        }
    };

    // Read the include file.
    let mut include_file = String::new();
    match config.pattern_type {
        IncludeFile(ref filename) => {
            File::open(filename).unwrap().read_to_string(&mut include_file).ok();
        },
        _ => unimplemented!()
    };

    // Parse the include file.
    let matcher = match parse_include_file(&include_file) {
        Ok(x) => x,
        Err(e) => {
            println!("Syntax error in include file.");
            println!("{:?}", e);
            return;
        }
    };

    debug!("Include Tree:");
    debug!("{:?}", matcher.include_trie);
    debug!("Exclude Tree:");
    debug!("{:?}", matcher.exclude_trie);

    let (copy_paths, delete_paths) = match sync(&matcher, &mut config) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    debug!("Paths to copy:");
    for path in copy_paths.keys() {
        debug!("{:?}", path);
    }

    debug!("Would Delete:");
    for path in delete_paths.keys() {
        debug!("{:?}", path);
    }
}
