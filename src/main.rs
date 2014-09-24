#![feature(phase)]

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
pub mod pattern;
pub mod sync;

fn main() {
    let mut config = Config::new();
    let src_dir = Path::new("dir1");
    let dest_dir = Path::new("dir2");

    let include_file = File::open(&Path::new("include.ska")).read_to_end().unwrap();

    let (include_tree, exclude_tree) = match parse_include_file(from_utf8(include_file.as_slice()).unwrap()) {
        Ok(x) => x,
        Err(e) => fail!(format!("Error: `{}`", e))
    };

    debug!("Include Tree:");
    debug!("{}", include_tree);
    debug!("Exclude Tree:");
    debug!("{}", exclude_tree);

    let (copy_paths, delete_paths) = match sync(&src_dir, &dest_dir, &include_tree, &exclude_tree, &mut config) {
        Ok(x) => x,
        Err(e) => fail!(format!("Error: `{}`", e))
    };

    println!("Would Copy:")
    for path in copy_paths.keys() {
        println!("{}", path);
    }
}
