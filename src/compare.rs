//! compare.rs, part of Skarn.
//!
//! This module contains functions that compare two files for equality.

use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub trait ComparisonMethod {
    fn same_file(&self, &Path, &Path) -> io::Result<bool>;
}

#[derive(Clone, Copy)]
pub struct Content;

impl ComparisonMethod for Content {
    fn same_file(&self, x: &Path, y: &Path) -> io::Result<bool> {
        let x_file = try!(File::open(x));
        let y_file = try!(File::open(y));

        let x_size = try!(x_file.metadata()).len();
        let y_size = try!(y_file.metadata()).len();

        if x_size != y_size {
            return Ok(false);
        }

        for (xbr, ybr) in x_file.bytes().zip(y_file.bytes()) {
            let xb = try!(xbr);
            let yb = try!(ybr);
            if xb != yb {
                return Ok(false)
            }
        }

        Ok(true)
    }
}
