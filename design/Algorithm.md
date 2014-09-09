Skarn Selective File Sync Algorithm
====

## Features

* Hash, modification time or content-based checking.
* Optional deletion of extraneous files.
    - Delete files in the destination missing from the source.
    - Delete files excluded by the algorithm.
    - Both of the above (default).

## Input
* `src_dir`, a `Path` describing the source directory.
* `dest_dir`, a `Path` describing the destination directory.
* `include_tree`, a `PatternNode` describing the files to include.
* `exclude_tree`, a `PatternNode` describing the files to exclude.
* `options`, an object describing the comparison mechanism and deletion behaviour.

## Output:
* (`copy_paths`, `delete_paths`)
* `copy_paths`, a vector of paths relative to the source directory in need of copying.
* `delete_paths`, a vector of paths relative to the destination directory in need of deletion.

## Algorithm

```python
# 1. Construct a tree of included files from the source directory.
src_tree = empty tree
for each path in a recursive traversal of src_dir:
    # Find the most specific match from either the include tree or the exclude tree.
    matching_include_nodes = list containing only the include tree root
    matching_exclude_nodes = list containing only the exclude tree root

    include_path = false

    for each component in the path:
        # Expand the current matching include tree nodes.
        new_matching_include_nodes = empty list
        for each node in matching_include_nodes:
            for each child of the node:
                if the current component matches the child's pattern:
                    add the child to the new_matching_include_nodes list

        # Expand the current matching exclude tree nodes.
        new_matching_exclude_nodes = empty list
        for each node in matching_exclude_nodes:
            for each child of the node:
                if the current component matches the child's pattern:
                    add the child to the new_matching_exclude_nodes list

        # If both trees lack more specific patterns, allow
        # the excluding rule to dominate.
        if new_matching_include_nodes.length == 0 and
           new_matching_exclude_nodes.length == 0:
            include_path = false
            break

        # If the include tree has a more specific pattern and the exclude tree
        # is exhausted, include the path!
        if new_matching_exclude_nodes.length == 0:
            include_path = true
            break

        # Likewise if the exclude tree has a more specific pattern.
        if new_matching_include_nodes.length == 0:
            include_path = false
            break

        matching_include_nodes = new_matching_include_nodes
        matching_exclude_nodes = new_matching_exclude_nodes

    if include_path:
        if path corresponds to a directory:
            recursively add all paths below that directory to src_tree
        else:
            add the path to src_tree
```
