//! sync.rs, part of Skarn.
//!
//! This file contains the selective file sync algorithm described in `design/Algorithm.md`.

use std::io::IoResult;
use std::io::fs::{PathExtensions, walk_dir};
use std::collections::HashSet;

use trie::Trie;

use pattern::Pattern;
use matcher::{Matcher, PathTrie};
use matcher::Class::Included;
use config::{Config, ComparisonMethod, IncludeByDefault};
use config::{DeleteBehaviour, IncludedNoEquiv, ExcludedEquiv, ExcludedNoEquiv};
use compare::ComparisonMethodTrait;
use path::StringComponents;

pub fn sync(src_dir: &Path,
            dest_dir: &Path,
            matcher: &Matcher,
            options: &mut Config) -> IoResult<(PathTrie, PathTrie)> {
    // Classify every file in the source directory.
    // Included files are initially marked for copying, and filtered upon traversal of the dest dir.
    let include_by_default = *options.get::<IncludeByDefault, bool>();
    let (mut copy_paths, _) = try!(matcher.classify_recursive(src_dir, include_by_default));

    let mut delete_paths = Trie::new();

    // FIXME: Remove clone somehow.
    let delete_behaviour = options.get::<DeleteBehaviour, HashSet<DeleteBehaviour>>().clone();
    let comparison_method = options.get::<ComparisonMethod, ComparisonMethod>();

    // Walk the destination directory.
    let mut dest_dir_walk = try!(walk_dir(dest_dir));
    for path in dest_dir_walk {
        // Create a relative path, and a path relative to the source directory.
        let relative_path = path.path_relative_from(dest_dir).unwrap();
        let src_equiv = src_dir.join(relative_path.clone());

        let path_key: Vec<String> = relative_path.string_components();

        // Case 1: Included, Equiv.
        // If the files match, remove the file from the list of files in need of copying.
        if copy_paths.find(path_key[]).is_some() {
            let same_file = try!(comparison_method.same_file(&path, &src_equiv));

            if same_file {
                debug!(" Files Match: {}", relative_path.display());
                copy_paths.remove(path_key[]);
            } else {
                debug!(" Files Differ: {}", relative_path.display());
            }
        }

        // Opportunistic deletion.
        // If configured to delete ALL types of extraneous files, there is no need for further checks.
        else if delete_behaviour.len() == 3 {
            delete_paths.insert(path_key[], ());
        }

        // Case 2: Excluded, Equiv.
        else if src_equiv.exists() {
            if delete_behaviour.contains(&ExcludedEquiv) {
                delete_paths.insert(path_key[], ());
            }
        }

        // Opportunistic deletion.
        // If configured to delete all files with no equivalent, delete away!
        else if delete_behaviour.contains(&IncludedNoEquiv) &&
                delete_behaviour.contains(&ExcludedNoEquiv) {
            delete_paths.insert(path_key[], ());
        }

        // Case 3: Included, No Equiv.
        else if let Included = matcher.classify(&relative_path) {
            if delete_behaviour.contains(&IncludedNoEquiv) {
                delete_paths.insert(path_key[], ());
            }
        }

        // Case 4: Excluded, No Equiv.
        else if delete_behaviour.contains(&ExcludedNoEquiv) {
            delete_paths.insert(path_key[], ());
        }
    }

    Ok((copy_paths, delete_paths))
}
