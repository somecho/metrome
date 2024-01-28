use core::fmt;

#[derive(Debug, PartialEq, Clone)]
/// Errors that occur when parsing the tokens and creating score
pub enum ParseError {
    /// Occurs when specifying a tempo without the numbers per minute.
    /// For example: `q =`
    MissingTempoSpecifier,
    /// Occurs when a number is used outside of the context of ratios or tempo specifications
    Number,
    /// Occurs when a dot is found outside of the context of extending ratio duration
    Dot,
    /// Occurs when an equal character is found outside the context of tempo specification
    Equal,
}

#[derive(Debug, PartialEq, Clone)]
/// Errors that occur when converting between units
pub enum ConversionError {
    /// Occurs when trying to convert a token to a duration that isn't a ratio
    NonRatioToDuration,
}

#[derive(Debug, PartialEq, Clone)]
/// Errors that occur when tokenizing the score
pub enum TokenError {
    /// Occurs when trying to use 0 as a number or in a ratio
    Zero,
    /// Occurs when an invalid character is found in the score
    InvalidCharacter(char),
    /// Occurs when the number of repeats is not specified after a repetition character
    /// Example: `qx` or `| q q q |%`
    MissingRepetition(char),
    /// Occurs when the bottom part of a ratio is missing
    IncompleteRatio,
    /// Occurs when a '/' is found before a number is found
    LeadingSlash,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MetrumError {
    ParseError(ParseError),
    ConversionError(ConversionError),
    TokenError(TokenError),
}
impl fmt::Display for MetrumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetrumError::ParseError(e) => match e {
                ParseError::MissingTempoSpecifier => {
                    write!(f, "A number must come after '=' when specifying tempo")
                }
                ParseError::Number => {
                    write!(f, "A number can only be used in the context of ratios or tempo specifications")
                }
                ParseError::Dot => {
                    write!(f, "A dot can only be used after a ratio or duration")
                }
                ParseError::Equal => {
                    write!(
                        f,
                        "An equal character can only be used when specifying a tempo"
                    )
                }
            },
            MetrumError::ConversionError(e) => match e {
                ConversionError::NonRatioToDuration => {
                    write!(f, "Cannot convert a non ratio to a duration")
                }
            },
            MetrumError::TokenError(e) => match e {
                TokenError::Zero => {
                    write!(f, "Use of 0 as a number of a ratio is not allowed")
                }
                TokenError::InvalidCharacter(c) => {
                    write!(f, "Use of Invalid Character: {c}")
                }
                TokenError::MissingRepetition(c) => {
                    write!(f, "The number of repetitions must be specified directly after the {c} character")
                }
                TokenError::IncompleteRatio => {
                    write!(f, "A number must come directly after a '/' in ratios")
                }
                TokenError::LeadingSlash => {
                    write!(
                        f,
                        "A '/' cannot come on its own without a number preceding it"
                    )
                }
            },
        }
    }
}
