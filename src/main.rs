#![feature(phase, slicing_syntax, globs)]

#[phase(plugin, link)] extern crate log;

extern crate regex;
#[phase(plugin)] extern crate regex_macros;
extern crate serialize;

extern crate glob;
extern crate typemap;
extern crate phantom;
extern crate sequence_trie;

extern crate docopt;
#[phase(plugin)] extern crate docopt_macros;

use std::io::fs::File;
use std::str::from_utf8;

use config::{Config, SourceDir, DestDir};
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
        Err(e) => panic!("Error: {}", e)
    };

    let src_dir = config.get::<SourceDir, Path>().clone();
    let dest_dir = config.get::<DestDir, Path>().clone();
    println!("Source is: {}, Dest is: {}", src_dir.display(), dest_dir.display());

    let include_file = File::open(&Path::new("include.ska")).read_to_end().unwrap();

    let matcher = match parse_include_file(from_utf8(include_file.as_slice()).unwrap()) {
        Ok(x) => x,
        Err(e) => panic!("Error: {}", e)
    };

    debug!("Include Tree:");
    debug!("{}", matcher.include_trie);
    debug!("Exclude Tree:");
    debug!("{}", matcher.exclude_trie);

    let (copy_paths, delete_paths) = match sync(&src_dir, &dest_dir, &matcher, &mut config) {
        Ok(x) => x,
        Err(e) => panic!(format!("Error: {}", e))
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
