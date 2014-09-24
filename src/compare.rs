//! compare.rs, part of Skarn.
//!
//! This module contains functions that compare two files for equality.

use std::io::{IoResult, EndOfFile};
use std::io::fs::File;

pub trait ComparisonMethodTrait {
    fn same_file(&self, &Path, &Path) -> IoResult<bool>;
}

pub struct Content;

impl ComparisonMethodTrait for Content {
    fn same_file(&self, x: &Path, y: &Path) -> IoResult<bool> {
        let mut x_file = try!(File::open(x));
        let mut y_file = try!(File::open(y));

        for (x_byte, y_byte) in x_file.bytes().zip(y_file.bytes()) {
            let x_byte = match x_byte {
                Ok(byte) => byte,
                Err(ref e) if e.kind == EndOfFile => return Ok(false),
                Err(e) => return Err(e)
            };

            let y_byte = match y_byte {
                Ok(byte) => byte,
                Err(ref e) if e.kind == EndOfFile => return Ok(false),
                Err(e) => return Err(e)
            };

            if x_byte != y_byte {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
