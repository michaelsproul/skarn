//! compare.rs, part of Skarn.
//!
//! This module contains functions that compare two files for equality.

use std::io::{IoResult, EndOfFile};
use std::io::fs::File;

pub trait ComparisonMethodTrait {
    fn same_file(&self, &Path, &Path) -> IoResult<bool>;
}

#[deriving(Copy)]
pub struct Content;

impl ComparisonMethodTrait for Content {
    fn same_file(&self, x: &Path, y: &Path) -> IoResult<bool> {
        let mut x_file = try!(File::open(x));
        let mut y_file = try!(File::open(y));

        loop {
            match (x_file.read_byte(), y_file.read_byte()) {
                (Ok(x_byte), Ok(y_byte)) => {
                    if x_byte != y_byte {
                        return Ok(false);
                    }
                },
                (Err(xe), Err(ye)) => {
                    if xe.kind == ye.kind && xe.kind == EndOfFile {
                        return Ok(true);
                    }
                    return Err(xe);
                },
                (Err(e), _) | (_, Err(e)) => {
                    if e.kind == EndOfFile {
                        return Ok(false);
                    }
                    return Err(e);
                }
            }
        }
    }
}
