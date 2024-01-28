use core::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingTempoSpecifier,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MissingTempoSpecifier => {
                write!(f, "A number must come after '=' when specifying tempo")
            }
        }
    }
}
