/// parser.rs, part of Skarn.
/// This module contains functions for processing an include file into a tree of PatternNode objects.

use pattern::{Pattern, PatternNode, Plain, Glob};
use regex::Regex;

static comment_line_regex: Regex = regex!("^/#/ .*");
static line_regex: Regex = regex!(r"^(?P<prelude>/(?P<inner_prelude>[!\*]{1,2})/ )?(?P<path>[^/].*)$");

pub type PatternTreePair = (PatternNode, PatternNode);

pub enum Prelude {
    SimpleInclude,
    SimpleExclude,
    GlobInclude,
    GlobExclude,
}

pub enum ParseError {
    InvalidLine,
    InvalidPrelude,
    TrivialInput
}

pub fn parse_include_file(include_file: &str) -> Result<PatternTreePair, ParseError> {
    let mut include_tree = PatternNode::new(Pattern::simple_pattern("root"));
    let mut exclude_tree = PatternNode::new(Pattern::simple_pattern("root"));

    let is_trivial_tree = true;
    for line in include_file.lines() {
        if comment_line_regex.is_match(line) {
            continue;
        }

        let path_components, prelude = match parse_single_line(line) {
            Ok(path_components, prelude) => (path_components, prelude),
            Err(e) => return Err(e)
        };

        is_trivial_tree = false;

        match prelude {
            SimpleInclude | GlobInclude => include_tree.insert(path_components),
            SimpleExclude | GlobExclude => exclude_tree.insert(path_components)
        }
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
