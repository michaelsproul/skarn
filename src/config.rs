//! config.rs, part of Skarn.

use std::collections::HashSet;

use compare::ComparisonMethod;
use error::Error;
use self::DeleteBehaviour::*;

pub enum PatternSource {
    IncludeFile(Path),
    Git
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum DeleteBehaviour {
    IncludedNoEquiv,
    ExcludedEquiv,
    ExcludedNoEquiv
}

impl DeleteBehaviour {
    pub fn from_str(delete_string: &str) -> Result<HashSet<DeleteBehaviour>, Error> {
        let mut set = HashSet::new();
        for s in delete_string.split(',') {
            match s {
                "all" => try!(set.insert_all(&[IncludedNoEquiv, ExcludedEquiv, ExcludedNoEquiv])),
                "excluded" => try!(set.insert_all(&[ExcludedEquiv, ExcludedNoEquiv])),
                "included-no-equiv" => try!(set.insert_all(&[IncludedNoEquiv])),
                "excluded-equiv" => try!(set.insert_all(&[ExcludedEquiv])),
                "excluded-no-equiv" => try!(set.insert_all(&[ExcludedEquiv])),
                _ => return Err(
                    Error::new("error parsing delete behaviour string")
                    .with_detail(format!("invalid option: '{}'", s))
                )
            }
        }
        Ok(set)
    }
}

trait InsertAll {
    // Insert a list of delete behaviours into a HashSet, returning an error if any of the values
    // are already present in the map.
    fn insert_all(&mut self, values: &[DeleteBehaviour]) -> Result<(), Error>;
}

impl InsertAll for HashSet<DeleteBehaviour> {
    fn insert_all(&mut self, values: &[DeleteBehaviour]) -> Result<(), Error> {
        for v in values.iter() {
            let newly_inserted = self.insert(*v);
            if !newly_inserted {
                return Err(
                    Error::new("error parsing delete behaviour string")
                    .with_detail(format!("duplicate option (implied or explicit): '{}'", v))
                );
            }
        }
        Ok(())
    }
}

pub struct Config {
    pub source_dir: Path,
    pub dest_dir: Path,
    pub pattern_type: PatternSource,
    pub comparison_method: Box<ComparisonMethod + 'static>,
    pub delete_behaviour: HashSet<DeleteBehaviour>,
    pub include_by_default: bool,
}
