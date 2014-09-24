//! sync.rs, part of Skarn.
//!
//! This file contains the selective file sync algorithm described in `design/Algorithm.md`.

use std::io::IoResult;
use std::io::fs::{PathExtensions, walk_dir};

use trie::Trie;

use pattern::{Pattern, PatternTrie};
use config::{Config, ComparisonMethod};
use compare::ComparisonMethodTrait;

pub type StringTrie = Trie<String, ()>;

pub fn generate_match_tree( src_dir: &Path,
                            include_tree: &PatternTrie,
                            exclude_tree: &PatternTrie,
                            options: &Config) -> IoResult<StringTrie> {
    debug!("Generating Match Tree.");
    let mut match_tree: StringTrie = Trie::new();

    let mut src_dir_walk = try!(walk_dir(src_dir));

    for path_abs in src_dir_walk {
        let path = path_abs.path_relative_from(src_dir).unwrap();

        let mut matching_include_nodes: Vec<&PatternTrie> = vec![include_tree];
        let mut matching_exclude_nodes: Vec<&PatternTrie> = vec![exclude_tree];

        let mut is_included_path = false;

        let path_components: Vec<&str> = path.str_components().map(|c| c.unwrap()).collect();

        for &component in path_components.iter() {
            // Expand the layers of include and exclude nodes.
            matching_include_nodes = new_matching_nodes(component, matching_include_nodes);
            matching_exclude_nodes = new_matching_nodes(component, matching_exclude_nodes);

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
            debug!(" Match: `{}`", path.display());
            // If the path is a directory, add all files contained in it.
            if path.is_dir() {
                let mut path_dir_walk = try!(walk_dir(&path));
                for child_path in path_dir_walk {
                    let path_key: Vec<String> = child_path.str_components()
                                            .map(|s| s.unwrap().to_string())
                                            .collect();
                    match_tree.insert(path_key.as_slice(), ());
                }
            }
            // If the path is a file, add it directly.
            else {
                let path_key: Vec<String> = path_components.iter().map(|s| s.to_string()).collect();
                match_tree.insert(path_key.as_slice(), ());
            }
        } else {
            debug!(" No Match: `{}`", path.display());
        }
    }

    Ok(match_tree)
}

fn new_matching_nodes<'a>(component: &str, matching_nodes: Vec<&'a PatternTrie>) -> Vec<&'a PatternTrie> {
    let mut new_matching_nodes = vec![];

    for &node in matching_nodes.iter() {
        for (child_pattern, child) in node.children.iter() {
            if child_pattern.matches(component) {
                new_matching_nodes.push(child);
            }
        }
    }
    new_matching_nodes
}

pub fn sync(src_dir: &Path,
            dest_dir: &Path,
            include_tree: &PatternTrie,
            exclude_tree: &PatternTrie,
            options: &mut Config) -> IoResult<(StringTrie, StringTrie)> {
    let mut copy_paths: StringTrie = try!(generate_match_tree(src_dir, include_tree, exclude_tree, options));
    let mut delete_paths = Trie::new();

    let comparison_method = options.get::<ComparisonMethod, ComparisonMethod>();

    let mut dest_dir_walk = try!(walk_dir(dest_dir));

    debug!("Exploring destination directory.");

    for path_abs in dest_dir_walk {
        let path = path_abs.path_relative_from(dest_dir).unwrap();

        let path_key: Vec<String> = path.str_components().map(|s| s.unwrap().to_string()).collect();

        if copy_paths.find(path_key.as_slice()).is_some() {
            let src_abs = src_dir.join(path.clone());
            let dest_abs = dest_dir.join(path.clone());

            let same_file = try!(comparison_method.same_file(&src_abs, &dest_abs));

            if same_file {
                debug!(" Files Match: `{}`", path.display());
                copy_paths.remove(path_key.as_slice());
            } else {
                debug!(" Files Differ: `{}`", path.display());
                // TODO: Deletion logic.
            }
        }
    }

    Ok((copy_paths, delete_paths))
}

