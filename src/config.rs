//! config.rs, part of Skarn.

use std::collections::HashSet;

use compare::ComparisonMethod;

pub enum PatternSource {
    IncludeFile(Path),
    Git
}

#[deriving(PartialEq, Eq, Hash, Show, Copy, Clone)]
pub enum DeleteBehaviour {
    IncludedNoEquiv,
    ExcludedEquiv,
    ExcludedNoEquiv
}

pub struct Config {
    pub source_dir: Path,
    pub dest_dir: Path,
    pub pattern_type: PatternSource,
    pub comparison_method: Box<ComparisonMethod + 'static>,
    pub delete_behaviour: HashSet<DeleteBehaviour>,
    pub include_by_default: bool,
}
