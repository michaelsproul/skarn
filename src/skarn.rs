#![feature(phase)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

mod pattern;
mod parser;

fn main() {
    println!("Skarn.");
}
