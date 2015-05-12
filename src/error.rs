use std::error;
use std::fmt;

use docopt;

#[derive(Debug)]
pub struct Error {
    description: &'static str,
    detail: Option<String>,
    cause: Option<Box<error::Error>>
}

impl Error {
    pub fn new(description: &'static str) -> Error {
        Error {
            description: description,
            detail: None,
            cause: None
        }
    }

    pub fn with_detail(mut self, detail: String) -> Error {
        self.detail = Some(detail);
        self
    }

    pub fn with_cause(mut self, cause: Box<error::Error>) -> Error {
        self.cause = Some(cause);
        self
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.description
    }

    fn cause<'a>(&'a self) -> Option<&'a error::Error> {
        // Option::map doesn't seem to work here at the moment.
        match self.cause {
            Some(ref cause) => Some(&**cause),
            None => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.detail {
            Some(ref detail) => detail.fmt(f),
            None => self.description.fmt(f)
        }
    }
}

impl From<docopt::Error> for Error {
    fn from(err: docopt::Error) -> Error {
        Error::new("docopt parsing error").with_cause(box err as Box<error::Error>)
    }
}
