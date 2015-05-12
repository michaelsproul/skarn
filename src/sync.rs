//! sync.rs, part of Skarn.
//!
//! This file contains the selective file sync algorithm described in `design/Algorithm.md`.

use std::io;
use std::fs::{PathExt, walk_dir};

use sequence_trie::SequenceTrie;

use matcher::{Matcher, PathTrie};
use matcher::Class::Included;
use config::Config;
use config::DeleteBehaviour::*;
use compare::ComparisonMethod;
use path::StringComponents;

pub fn sync(matcher: &Matcher, config: &Config) -> io::Result<(PathTrie, PathTrie)> {
    let source_dir = &config.source_dir;
    let dest_dir = &config.dest_dir;

    // Classify every file in the source directory.
    // Included files are initially marked for copying, and filtered upon traversal of the dest dir.
    let (mut copy_paths, _) = try!(matcher.classify_recursive(source_dir, config.include_by_default));

    let mut delete_paths = SequenceTrie::new();

    let delete_behaviour = &config.delete_behaviour;
    let comparison_method = &config.comparison_method;

    // Walk the destination directory.
    let dest_dir_walk = try!(walk_dir(dest_dir));
    for entry in dest_dir_walk {
        let path = try!(entry).path();

        // Create a relative path, and a path relative to the source directory.
        let relative_path = path.relative_from(dest_dir).unwrap();
        let source_equiv = source_dir.join(relative_path.clone());

        let path_key: Vec<String> = relative_path.string_components();

        // Case 1: Included, Equiv.
        // If the files match, remove the file from the list of files in need of copying.
        if copy_paths.get(&path_key[..]).is_some() {
            let same_file = try!(comparison_method.same_file(&path, &source_equiv));

            if same_file {
                debug!(" Files Match: {}", relative_path.display());
                copy_paths.remove(&path_key[..]);
            } else {
                debug!(" Files Differ: {}", relative_path.display());
            }
        }

        // Opportunistic deletion.
        // If configured to delete ALL types of extraneous files, there is no need for further checks.
        else if delete_behaviour.len() == 3 {
            delete_paths.insert(&path_key[..], ());
        }

        // Case 2: Excluded, Equiv.
        else if source_equiv.exists() {
            if delete_behaviour.contains(&ExcludedEquiv) {
                delete_paths.insert(&path_key[..], ());
            }
        }

        // Opportunistic deletion.
        // If configured to delete all files with no equivalent, delete away!
        else if delete_behaviour.contains(&IncludedNoEquiv) &&
                delete_behaviour.contains(&ExcludedNoEquiv) {
            delete_paths.insert(&path_key[..], ());
        }

        // Case 3: Included, No Equiv.
        else if let Included = matcher.classify(&relative_path) {
            if delete_behaviour.contains(&IncludedNoEquiv) {
                delete_paths.insert(&path_key[..], ());
            }
        }

        // Case 4: Excluded, No Equiv.
        else if delete_behaviour.contains(&ExcludedNoEquiv) {
            delete_paths.insert(&path_key[..], ());
        }
    }

    Ok((copy_paths, delete_paths))
}
