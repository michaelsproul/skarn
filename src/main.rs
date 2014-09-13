#![feature(phase)]

extern crate glob;
#[phase(plugin)] extern crate regex_macros;
extern crate regex;
extern crate trie;

use trie::Trie;

use pattern::Pattern;

mod pattern;
//mod parser;

fn main() {
    let trie: Trie<Pattern, bool> = Trie::new();
    println!("Skarn.");
}
