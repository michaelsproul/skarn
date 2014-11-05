Skarn Selective File Sync Algorithm
====

## Features

* Flexible "sameness" checking. By modification time, hash or file content.
* Optional deletion of extraneous files. Three types:
    * Included with no equiv: Files included by the match trees with no equivalent in the source directory.
    * Excluded with equiv: Files excluded by the match trees that exist in both the source and destination directories.
    * Excluded with no equiv: Files excluded by the match trees with no equivalent in the source directory.

The natural fourth type of file (included with equiv) is copied only if the destination's copy differs from the source's.

## Input
* `src_dir`, a filesystem path describing the source directory.
* `dest_dir`, a filesystem path describing the destination directory.
* `include_tree`, a trie of patterns describing the files to include.
* `exclude_tree`, a trie of patterns describing the files to exclude.
* `options`, an object describing the comparison mechanism and deletion behaviour.

The include-tree and exclude-tree will normally be created from files in the [Skarn Include File Format](IncludeFileFormat.md).

## Output:
* (`copy_paths`, `delete_paths`)
* `copy_paths`, a trie of paths relative to the source directory in need of copying.
* `delete_paths`, a trie of paths relative to the destination directory in need of deletion.

## Algorithm

```python
# Major Step 1:
# Construct a trie of paths from the source directory which
# are marked for inclusion by the pattern trees.
src_tree = empty trie

for each path in a recursive traversal of src_dir:
    # Find the most specific match from either the include tree or the exclude tree.
    matching_include_nodes = singleton list containing the include tree root
    matching_exclude_nodes = singleton list containing the exclude tree root

    is_included_path = false

    for each component in the path:
        # Expand the current layer of matching include-tree nodes.
        new_matching_include_nodes = empty list
        for each node in matching_include_nodes:
            for each child of node:
                if the current path component matches the child's pattern:
                    add the child to the new_matching_include_nodes list

        # Expand the current matching exclude-tree nodes (as above).
        new_matching_exclude_nodes = empty list
        for each node in matching_exclude_nodes:
            for each child of the node:
                if the current component matches the child's pattern:
                    add the child to the new_matching_exclude_nodes list

        # If both trees lack more specific patterns, allow
        # the excluding rule to dominate.
        if new_matching_include_nodes.length == 0 and
           new_matching_exclude_nodes.length == 0:
            is_included_path = false
            break

        # If the include-tree has a more specific pattern and the exclude-tree
        # is exhausted, include the path.
        if new_matching_exclude_nodes.length == 0:
            is_included_path = true
            break

        # Likewise if the exclude-tree has a more specific pattern and the include-tree
        # is exhausted, exclude the path.
        if new_matching_include_nodes.length == 0:
            is_included_path = false
            break

        matching_include_nodes = new_matching_include_nodes
        matching_exclude_nodes = new_matching_exclude_nodes

    if is_included_path:
        if path corresponds to a directory:
            recursively add all paths below that directory to src_tree
        else:
            add the path to src_tree

# Main Step 2:
# Work out which files need copying and deleting.
copy_paths = src_tree (clone or move)
delete_paths = empty trie

for each path in a recursive traversal of dest_dir:
    # Case 1: Included with equiv.
    if the path is contained in copy_paths:
        if the comparison function shows the files to be the same:
            remove the path from copy_paths

    # Opportunistic deletion.
    else if the options dictate that all extraneous files should be deleted:
        add the path to delete_paths

    # Case 2: Excluded with equiv.
    else if a file exists with the same path relative to src_dir:
        if the options dictate that excluded equiv files should be deleted:
            add the path to delete_paths

    # More opportunistic deletion.
    else if the options dictate that included and excluded equiv files should be deleted:
        add the path to delete_paths

    # Case 3: Included with no equiv.
    # FIXME: Refactor referenced code above to match implementation and make this clearer.
    else if the matching procedure from above shows the path to be included:
        if the options dictate that included-no-equiv paths should be deleted:
            add the path to delete_paths

    # Case 4: Excluded with no equiv.
    else if the options dictate that excluded-no-equiv paths should be deleted:
        add the path to delete_paths

return (copy_paths, delete_paths)
```

As an algorithm, this is public domain. Feel free to make your own implementations, although Skarn compatability and credit would be awesome.
