#![feature(phase, slicing_syntax, globs)]

#[phase(plugin, link)] extern crate log;

extern crate regex;
#[phase(plugin)] extern crate regex_macros;
extern crate "rustc-serialize" as rustc_serialize;

extern crate glob;
extern crate sequence_trie;

extern crate docopt;
#[phase(plugin)] extern crate docopt_macros;

use std::io::fs::File;
use std::str::from_utf8;

use config::PatternSource::IncludeFile;
use parser::parse_include_file;
use sync::sync;

pub mod compare;
pub mod config;
pub mod parser;
pub mod path;
pub mod pattern;
pub mod matcher;
pub mod sync;
pub mod arg_parser;

fn main() {
    let mut config = match arg_parser::parse_args() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let include_file = match config.pattern_type {
        IncludeFile(ref file) => File::open(file).read_to_end().unwrap(),
        _ => unimplemented!()
    };

    let matcher = match parse_include_file(from_utf8(include_file.as_slice()).unwrap()) {
        Ok(x) => x,
        Err(e) => panic!("Error: {}", e)
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

    println!("Would Copy:");
    for path in copy_paths.keys() {
        println!("{}", path);
    }

    println!("Would Delete:");
    for path in delete_paths.keys() {
        println!("{}", path);
    }
}
