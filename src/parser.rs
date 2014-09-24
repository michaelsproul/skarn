/// parser.rs, part of Skarn.
/// This module contains functions for processing an include file into a tree of PatternNode objects.

use trie::Trie;

use pattern::{Pattern, PatternTrie, Plain, Glob};
use regex::Regex;

static comment_line_regex: Regex = regex!("^/#/ .*");
static line_regex: Regex = regex!(r"^(?P<prelude>/(?P<inner_prelude>[!\*]{1,2})/ )?(?P<path>[^/].*)$");

pub type PatternTriePair = (PatternTrie, PatternTrie);

pub enum Prelude {
    SimpleInclude,
    SimpleExclude,
    GlobInclude,
    GlobExclude,
}

#[deriving(Show)]
pub enum ParseError {
    InvalidLine,
    InvalidPrelude,
    TrivialInput
}

pub fn parse_include_file(include_file: &str) -> Result<PatternTriePair, ParseError> {
    let mut include_tree: PatternTrie = Trie::new();
    let mut exclude_tree: PatternTrie = Trie::new();

    let mut is_trivial_tree = true;
    for line in include_file.lines() {
        if comment_line_regex.is_match(line) {
            continue;
        }

        let (path_components, prelude) = match parse_single_line(line) {
            Ok(result) => result,
            Err(e) => return Err(e)
        };

        is_trivial_tree = false;

        match prelude {
            SimpleInclude | GlobInclude => include_tree.insert(path_components.as_slice(), ()),
            SimpleExclude | GlobExclude => exclude_tree.insert(path_components.as_slice(), ())
        };
    }

    if is_trivial_tree {
        return Err(TrivialInput);
    }

    Ok((include_tree, exclude_tree))
}

pub fn parse_single_line(line: &str) -> Result<(Vec<Pattern>, Prelude), ParseError> {
    // Parse the line into a prelude and path.
    let captures = match line_regex.captures(line) {
        Some(captures) => captures,
        None => return Err(InvalidLine)
    };

    // Extract the prelude.
    let prelude = match captures.name("prelude") {
        "" => SimpleInclude,
        _ => match captures.name("inner_prelude") {
            "*" => GlobInclude,
            "!" => SimpleExclude,
            "!*" | "*!" => GlobExclude,
            other => return Err(InvalidPrelude)
        }
    };

    // Extract the path, which is guaranteed to be non-empty by the regex.
    let path = captures.name("path");

    // Split the path into its components, and make each component a path.
    let simple_map = |comp| Pattern::simple_pattern(comp);
    let glob_map = |comp| Pattern::glob_pattern(comp);

    let map_fn = match prelude {
        SimpleInclude | SimpleExclude => simple_map,
        GlobInclude | GlobExclude => glob_map
    };

    let components: Vec<Pattern> = path.split('/').map(map_fn).collect();

    Ok((components, prelude))
}
