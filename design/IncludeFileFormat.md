Include File Format
===================

Skarn performs selective file copying by reading patterns from a file with the following syntax.

## Simple Paths

A simple path is a path relative to the source directory which may include the `*` wildcard. All other characters are interpreted as part of the path.

Examples:

```
John Scofield/A Go Go/
Aaron Goldberg/* [live].flac
*/Jazz
üñiçødé/yes/
```
Trailing slashes are ignored.

The `*` wildcard can be escaped with a backslash:

```
Path with a \*
```

Literal backslashes can be obtained using `\\`. Forward slashes cannot be escaped.

Note that because the paths are relative, they cannot begin with a `/` character.

## Preludes

A prelude is a set of characters surrounded by forward slashes placed at the beginning of a line. A prelude specifies how the rest of the line should be interpreted.

```
/<prelude characters>/ <line body>
```

All lines with preludes must include a single space after the prelude.

## Glob Paths

In addition to simple paths, Skarn also supports paths specified using shell glob syntax.

Glob paths are introduced using the `/*/` prelude.

```
/*/ <pattern>
```

The exact glob syntax has not yet been finalised (I will probably use Rust's `glob` crate).

## Excluded Paths

Both simple and glob paths may be specified as exclude patterns using preludes.

Excluded simple paths use the `/!/` prelude.

Examples:

```
/!/ Some Large File.bin
/!/ Artist/Overplayed Album
/!/ *.pdf
```

Excluded glob paths use either the `/!*/` or `/*!/` prelude.

Examples:

```
/!*/ [ck]atz
/*!/ [ck]atz
```

## Comments

Lines beginning with the `/#/` prelude are taken as comments.

## Summary

Simple paths: Only `*`, `/` and `\` have special meaning.

Glob paths: `/*/`

Excluded paths: `/!/`

Excluded glob paths: `/!*/` (or `/*!/`)

Comments: `/#/`
