use std::io;
use std::path::Path;
use std::fs::{read_dir, walk_dir, PathExt};

use sequence_trie::SequenceTrie;

use pattern::Pattern;
use path::StringComponents;

use self::Class::*;

pub struct Matcher {
    pub include_trie: PatternTrie,
    pub exclude_trie: PatternTrie
}

#[derive(Clone, Copy)]
pub enum Class {
    Included,
    Excluded,
    Both
}

pub type PatternTrie = SequenceTrie<Pattern, ()>;
pub type PathTrie = SequenceTrie<String, ()>;

impl Matcher {
    /// Determine if a given path is included or excluded by the pair of matching tries.
    ///
    /// Paths are included if and only if they match a longer trail of patterns in the
    /// include trie than they do in the exclude trie.
    ///
    /// Paths which match trails of equal length in both tries are classified as `Both`.
    pub fn classify(&self, path: &Path) -> Class {
        // Split the path into its components.
        let path_components: Vec<String> = path.string_components();

        // Explore down the tree in layers, as there could be multiple matches at each level.
        let mut matching_include_nodes = vec![&self.include_trie];
        let mut matching_exclude_nodes = vec![&self.exclude_trie];

        for component in path_components {
            // Expand the layers of include and exclude nodes.
            matching_include_nodes = new_matching_nodes(&component, matching_include_nodes);
            matching_exclude_nodes = new_matching_nodes(&component, matching_exclude_nodes);

            match (matching_include_nodes.is_empty(), matching_exclude_nodes.is_empty()) {
                // If both pattern paths are exhausted, it's a tie.
                (true, true) => return Both,
                // If only the inclusion path is exhausted, the path is excluded.
                (true, false) => return Excluded,
                // If only the exclusion pattern path is exhausted, the path is included.
                (false, true) => return Included,
                // If neither layer is empty, continue.
                (false, false) => ()
            }
        }

        // If the path runs out before both pattern tries, it is both included and excluded!
        Both
    }

    /// Recursively classify every file under a given directory.
    ///
    /// For files that are unclassifiable, the `include_by_default` parameter determines
    /// whether the files should be included or excluded.
    ///
    /// Returns two tries of paths, for included and excluded files respectively.
    /// The paths in both tries are relative to the root.
    pub fn classify_recursive(&self, root: &Path, include_by_default: bool)
    -> io::Result<(PathTrie, PathTrie)>
    {
        let mut include_trie: PathTrie = SequenceTrie::new();
        let mut exclude_trie: PathTrie = SequenceTrie::new();

        let mut stack = vec![root.to_path_buf()];

        while let Some(path) = stack.pop() {
            let relative_path = path.relative_from(root).unwrap();

            let trie = match self.classify(&relative_path) {
                Included => &mut include_trie,
                Excluded => &mut exclude_trie,
                Both => {
                    // Directories classified as both need further exploration.
                    if path.is_dir() {
                        let children = try!(read_dir(&path));
                        for entry in children.into_iter() {
                            let child = try!(entry).path();
                            stack.push(child);
                        }
                        continue;
                    }

                    // Files need to be discriminated by the tie-breaker.
                    if include_by_default {
                        &mut include_trie
                    } else {
                        &mut exclude_trie
                    }
                }
            };

            // If the path is a directory with an unambiguous match, add all files recursively.
            // Files beneath the directory cannot be classified different from the directory.
            if path.is_dir() {
                let path_dir_walk = try!(walk_dir(&path));

                for entry in path_dir_walk {
                    let child_path = try!(entry).path();
                    // Avoid unneccessary insert operations for child directories.
                    if child_path.is_dir() {
                        continue;
                    }

                    let path_key = child_path.relative_from(root).unwrap().string_components();
                    trie.insert(&path_key[..], ());
                }
            }
            // If the path corresponds to a regular file, add it to its respective trie.
            else {
                let path_key = relative_path.string_components();
                trie.insert(&path_key[..], ());
            }
        }

        Ok((include_trie, exclude_trie))
    }
}

fn new_matching_nodes<'a>(component: &str, matching_nodes: Vec<&'a PatternTrie>)
-> Vec<&'a PatternTrie>
{
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
