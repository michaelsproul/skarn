#![feature(phase, slicing_syntax, globs)]

#[phase(plugin, link)] extern crate log;

extern crate regex;
#[phase(plugin)]
extern crate regex_macros;
extern crate "rustc-serialize" as rustc_serialize;

extern crate glob;
extern crate sequence_trie;

extern crate docopt;
#[phase(plugin)]
extern crate docopt_macros;

use std::io::fs::File;
use std::str;
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

fn main() {
    // Parse the command-line arguments to create a config file.
    let mut config = match arg_parser::parse_args() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e.description());
            return;
        }
    };

    // Read and parse the include file.
    let include_file_data = match config.pattern_type {
        IncludeFile(ref file) => match File::open(file).read_to_end() {
            Ok(data) => data,
            Err(e) => {
                println!("Error reading include file.");
                println!("{}", e);
                return;
            }
        },
        _ => unimplemented!()
    };

    let include_file = match str::from_utf8(include_file_data[]) {
        Ok(data) => data,
        Err(..) => panic!()
    };

    let matcher = match parse_include_file(include_file) {
        Ok(x) => x,
        Err(e) => {
            println!("Syntax error in include file.");
            println!("{}", e);
            return;
        }
    };

    debug!("Include Tree:");
    debug!("{}", matcher.include_trie);
    debug!("Exclude Tree:");
    debug!("{}", matcher.exclude_trie);

    let (copy_paths, delete_paths) = match sync(&matcher, &mut config) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    debug!("Paths to copy:");
    for path in copy_paths.keys() {
        debug!("{}", path);
    }

    debug!("Would Delete:");
    for path in delete_paths.keys() {
        debug!("{}", path);
    }
}
