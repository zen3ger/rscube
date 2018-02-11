use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AlgParseError {
    pub kind: AlgParseErrorKind,
}

#[derive(Debug)]
pub enum AlgParseErrorKind {
    EmptyInput,
    UnknownTurn,
    UnknownModifier,
}

impl fmt::Display for AlgParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AlgParseError: parse error")
    }
}

impl Error for AlgParseError {
    fn description(&self) -> &str {
        "AlgParseError: unable to parse algorithm"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.kind)
    }
}

impl fmt::Display for AlgParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlgParseErrorKind::EmptyInput => write!(f, "AlgParseErrorKind: empty input"),
            AlgParseErrorKind::UnknownTurn => write!(f, "AlgParseErrorKind: unknown turn"),
            AlgParseErrorKind::UnknownModifier => write!(f, "AlgParseErrorKind: unknown modifier"),
        }
    }
}

impl Error for AlgParseErrorKind {
    fn description(&self) -> &str {
        match *self {
            AlgParseErrorKind::EmptyInput => "Failed to parse `Move`: empty input",
            AlgParseErrorKind::UnknownTurn => "Failed to parse `Move`: unknown turn",
            AlgParseErrorKind::UnknownModifier => "Failed to parse `Move`: unknown modifier",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
