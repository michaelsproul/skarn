#![feature(phase, if_let, while_let, slicing_syntax)]

#[phase(plugin, link)] extern crate log;

extern crate glob;
#[phase(plugin)] extern crate regex_macros;
extern crate regex;
extern crate typemap;
extern crate phantom;
extern crate trie;

use std::io::fs::File;
use std::str::from_utf8;

use config::Config;
use parser::parse_include_file;
use sync::sync;

pub mod compare;
pub mod config;
pub mod parser;
pub mod path;
pub mod pattern;
pub mod matcher;
pub mod sync;

fn main() {
    let mut config = Config::new();
    let src_dir = Path::new("dir1");
    let dest_dir = Path::new("dir2");

    let include_file = File::open(&Path::new("include.ska")).read_to_end().unwrap();

    let matcher = match parse_include_file(from_utf8(include_file.as_slice()).unwrap()) {
        Ok(x) => x,
        Err(e) => panic!(format!("Error: {}", e))
    };

    debug!("Include Tree:");
    debug!("{}", matcher.include_trie);
    debug!("Exclude Tree:");
    debug!("{}", matcher.exclude_trie);

    let (copy_paths, delete_paths) = match sync(&src_dir, &dest_dir, &matcher, &mut config) {
        Ok(x) => x,
        Err(e) => panic!(format!("Error: {}", e))
    };

    println!("Would Copy:")
    for path in copy_paths.keys() {
        println!("{}", path);
    }
    println!("Would Delete:")
    for path in delete_paths.keys() {
        println!("{}", path);
    }
}
