//! sync.rs, part of Skarn.
//!
//! This file contains the selective file sync algorithm described in `design/Algorithm.md`.

use std::io::fs::walk_dir;

use trie::Trie;

use pattern::{Pattern, PatternTrie};
use config::Config;

pub fn sync(src_dir: &Path,
            dest_dir: &Path,
            include_tree: &PatternTrie,
            exclude_tree, &PatternTrie,
            options: &Config) -> IoResult<(Vec<Path>, Vec<Path>)> {
    // Trie of included paths in the source directory.
    let mut src_tree: Trie<String, bool> = Trie::new();

    let src_dir_walk = match walk_dir(src_dir) {
        Ok(walk) => walk,
        Err(e) => return Err(e)
    };

    for path in src_dir_walk {
        let mut matching_include_nodes: Vec<&PatternTrie> = vec![include_tree];
        let mut matching_exclude_nodes: Vec<&PatternTrie> = vec![exlude_tree];

        let mut is_included_path = false;

        let path_components: Vec<&str> = path.str_components.map(|c| c.unwrap()).collect();

        for &component in path_components.iter() {
            // Expand the layers of include and exclude nodes.
            matching_include_nodes = new_matching_nodes(path, matching_include_nodes);
            matching_exclude_nodes = new_matching_nodes(path, matching_exclude_nodes);

            // If both pattern paths are exhausted, allow the exclusion rule to dominate.
            if matching_include_nodes.is_empty() &&
                matching_exclude_nodes.is_empty() {
                is_included_path = false;
                break;
            }

            // If only the exclusion pattern path is exhausted, include the path.
            if matching_exclude_nodes.is_empty() {
                is_included_path = true;
                break;
            }

            // If only the inclusion pattern path is exhausted, exclude the path.
            if matching_include_nodes.is_empty() {
                is_included_path = false;
                break;
            }
        }

        if is_included_path {
            // Add all files in the directory.
            if path.is_dir() {
                for child_path in walk_dir(path).unwrap() {
                    let path_key: Vec<String> = child_path.str_components()
                                            .map(|s| s.unwrap().to_string())
                                            .collect();
                    src_tree.insert(path_key.as_slice(), true);
                }
            }
            // Add the file itself.
            else {
                let path_key: Vec<String> = path_components.iter().map(|s| s.to_string()).collect();
                src_tree.insert(path_key.as_slice(), true);
            }
        }


    }
}

fn new_matching_nodes(component: &str, matching_nodes: Vec<&PatternTrie>) -> Vec<&PatternTrie> {
    let new_matching_nodes = vec![];

    for &node in matching_nodes.iter() {
        for child in node.children.keys() {
            if child.pattern.matches(component) {
                new_matching_nodes.push(child);
            }
        }
    }
}
