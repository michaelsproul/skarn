Skarn Selective File Sync Algorithm
====

## Features

* Flexible "sameness" checking. By modification time, hash or file content.
* Optional deletion of extraneous and excluded files.
    - Delete files in the destination missing from the source.
    - Delete files excluded by the pattern trees.
    - Both of the above (default).

## Input
* `src_dir`, a `Path` describing the source directory.
* `dest_dir`, a `Path` describing the destination directory.
* `include_tree`, a `PatternNode` describing the files to include.
* `exclude_tree`, a `PatternNode` describing the files to exclude.
* `options`, an object describing the comparison mechanism and deletion behaviour.

The inlcude-tree and exclude-tree will normally be created from files in the [Skarn Include File Format](IncludeFileFormat.md).

## Output:
* (`copy_paths`, `delete_paths`)
* `copy_paths`, a vector of paths relative to the source directory in need of copying.
* `delete_paths`, a vector of paths relative to the destination directory in need of deletion.

## Algorithm

```python
# Major Step 1:
# Construct a tree (or trie) of paths from the source directory which
# are marked for inclusion by the pattern trees.
src_tree = empty tree

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
        # is exhausted, exclude the path.;
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
copy_paths = empty list
delete_paths = empty list

for each path in a recursive traversal of dest_dir:
    if the path is contained in src_tree:
        if the comparison function shows the files to be different:
            add the path to copy_paths
        else:
            continue

    # If the path isn't in the source tree it is either excluded or extraneous (extra).
    # FIXME: Using the current method more computation is required to determine if a path
    # is both extraneous *and* excluded. It is tempting to delete files like this only if
    # the "delete extraneous files" option is set.
    is_extraneous = false
    is_excluded = false
    if no equivalent file exists in the source directory:
        is_extraneous = true
    else:
        is_excluded = true

    if (is_extraneous and options.delete_extra) or
       (is_excluded and options.delete_excluded):
        add the path to copy_paths

return (copy_paths, delete_paths)
```

As an algorithm, this is public domain. Feel free to make your own implementations, although Skarn compatability and credit would be awesome.
