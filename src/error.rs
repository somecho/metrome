use core::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingTempoSpecifier,
}

#[derive(Debug, PartialEq)]
pub enum ConversionError {
    NonRatioToDuration,
}

#[derive(Debug, PartialEq)]
pub enum MetrumError {
    ParseError(ParseError),
    ConversionError(ConversionError),
}
impl fmt::Display for MetrumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetrumError::ParseError(e) => match e {
                ParseError::MissingTempoSpecifier => {
                    write!(f, "A number must come after '=' when specifying tempo")
                }
            },
            MetrumError::ConversionError(e) => match e {
                ConversionError::NonRatioToDuration => {
                    write!(f, "Cannot convert a non ratio to a duration")
                }
            },
        }
    }
}
