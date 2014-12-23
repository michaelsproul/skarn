use glob;

use std::fmt::{mod, Formatter, Show};
use self::Pattern::{Plain, Glob};

/// Enum for different pattern types.
#[deriving(PartialEq, Eq, Hash, Clone)]
pub enum Pattern {
    /// Just a string, no wildcards.
    Plain(String),
    /// Glob pattern, using any globbing constructs.
    Glob(glob::Pattern)
}

impl Show for Pattern {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Plain(ref s) => s.fmt(fmt),
            Glob(_) => "Glob".fmt(fmt)
        }
    }
}

impl Pattern {
    /// Create a Pattern for part of a simple path (only '*' wildcards).
    pub fn simple_pattern(pattern: &str) -> Pattern {
        let contains_wildcards = Pattern::scan_for_wildcards(pattern);

        // Remove backslash escapes and create a plain pattern if no wildcards were found
        if !contains_wildcards {
            Pattern::create_plain_pattern(pattern)
        }
        // Otherwise, create a glob pattern with non-star wildcards escaped
        else {
            let escaped_pattern = Pattern::escape_all_but_star(pattern);
            Glob(glob::Pattern::new(escaped_pattern.as_slice()))
        }
    }

    /// Create a Glob Pattern from a string.
    pub fn glob_pattern(pattern: &str) -> Pattern {
        Glob(glob::Pattern::new(pattern))
    }

    /// Scan a simple pattern for unescaped '*' characters.
    fn scan_for_wildcards(pattern: &str) -> bool {
        let mut escaped = false;

        for c in pattern.chars() {
            match c {
                '\\' => {
                    if escaped {
                        escaped = false;
                    } else {
                        escaped = true;
                    }
                },

                '*' => {
                    if !escaped {
                        return true;
                    }
                    escaped = false;
                },

                _ => { escaped = false; }
            }
        }
        false
    }

    /// Create a plain pattern from a wildcard-free string.
    fn create_plain_pattern(pattern: &str) -> Pattern {
        let mut result = String::new();
        let mut escaped = false;

        for c in pattern.chars() {
            match c {
                // Add escaped backslashes, and set 'escaped' to true for unescaped ones.
                '\\' => {
                    if escaped {
                        result.push(c);
                        escaped = false;
                    } else {
                        escaped = true;
                    }
                },

                // Add all other characters
                c => {
                    result.push(c);
                    escaped = false;
                }
            }
        }
        Plain(result)
    }

    /// Escape every glob wildcard in the given string apart from '*'.
    /// Also remove backslashes from escaped stars and backslashes.
    pub fn escape_all_but_star(pattern: &str) -> String {
        let mut result = String::new();
        let mut escaped = false;

        for c in pattern.chars() {
            match c {
                // Surround glob wildcards by a [] group
                '?' | '[' | ']' => {
                    result.push('[');
                    result.push(c);
                    result.push(']');
                    escaped = false;
                },

                // Push unescaped '*'s verbatim, but shield escaped '*'s using []
                '*' => {
                    if escaped {
                        result.push_str("[*]");
                    } else {
                        result.push('*');
                    }
                    escaped = false;
                },

                // Add escaped backslashes
                '\\' => {
                    if escaped {
                        result.push('\\');
                        escaped = false;
                    } else {
                        escaped = true;
                    }
                },

                // Add anything else as is
                _ => { result.push(c) }
            }
        }
        result
    }

    /// Check if a string matches the pattern.
    pub fn matches(&self, string: &str) -> bool {
        match *self {
            Plain(ref pattern) => {
                pattern.as_slice() == string
            },

            Glob(ref pattern) => {
                pattern.matches(string)
            }
        }
    }
}

// Tests

#[test]
fn test_plain_patterns() {
    assert!(Pattern::simple_pattern("Hello World!").matches("Hello World!"));
    assert!(!Pattern::simple_pattern("Hello World!").matches("Hello World"));
}

#[test]
fn test_simple_pattern_escaping() {
    assert!(Pattern::simple_pattern(r"Backslash \\Wow").matches(r"Backslash \Wow"));
    assert!(Pattern::simple_pattern(r"Star \* Escape").matches("Star * Escape"));
    assert!(!Pattern::simple_pattern(r"Star \* Escape").matches("Star X Escape"));
}

#[test]
fn test_simple_pattern_matching() {
    let js = Pattern::simple_pattern("J*S");
    let matches = vec![
        "JS",
        "J.S",
        "J*S",
        "JASS",
        "JAVA SCRIPTS"
    ];
    for m in matches.iter() {
        assert!(js.matches(*m));
    }

    let non_matches = vec![
        "AJS",
        "JavaScript"
    ];
    for n in non_matches.iter() {
        assert!(!js.matches(*n));
    }
}

#[test]
fn test_simple_pattern_wildcards() {
    assert!(Pattern::simple_pattern("App*e [cow]?").matches("Apple [cow]?"));
    assert!(!Pattern::simple_pattern("App*e [cow]?").matches("Apple cd"));
}

#[test]
fn test_glob_pattern_wildcards() {
    assert!(Pattern::glob_pattern("App*e [cow]?").matches("Apple cd"));
    assert!(!Pattern::glob_pattern("Apple [cow]?").matches("Apple [cow]?"));
}
