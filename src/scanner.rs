use crate::error::{MetrumError, TokenError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Barline,
    Ratio(u16, u16),
    NoteRepeat(u16),
    BarRepeat(u16),
    Number(u16),
    Equal,
    Dot,
}

pub fn scan(score: String) -> Result<Vec<Token>, MetrumError> {
    let mut score = score.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while score.peek().is_some() {
        let curr = score.next().unwrap();
        match curr {
            ' ' => {}
            '\n' => {}
            '\r' => {}
            '.' => tokens.push(Token::Dot),
            '=' => tokens.push(Token::Equal),
            '|' => tokens.push(Token::Barline),
            'w' => tokens.push(Token::Ratio(1, 1)),
            'h' => tokens.push(Token::Ratio(1, 2)),
            'q' => tokens.push(Token::Ratio(1, 4)),
            'e' => tokens.push(Token::Ratio(1, 8)),
            's' => tokens.push(Token::Ratio(1, 16)),
            't' => tokens.push(Token::Ratio(1, 32)),
            'x' => {
                let mut num = String::new();
                while score.peek().is_some() && score.peek().unwrap().is_ascii_digit() {
                    num.push(score.next().unwrap());
                }
                if num.is_empty() {
                    return Err(MetrumError::TokenError(TokenError::MissingRepetition('x')));
                }
                let parsed = num.parse::<u16>().unwrap();
                if parsed <= 1 {
                    return Err(MetrumError::TokenError(TokenError::NotEnoughRepeats));
                }
                tokens.push(Token::NoteRepeat(parsed));
            }
            '%' => {
                let mut num = String::new();
                while score.peek().is_some() && score.peek().unwrap().is_ascii_digit() {
                    num.push(score.next().unwrap());
                }
                if num.is_empty() {
                    return Err(MetrumError::TokenError(TokenError::MissingRepetition('%')));
                }
                let parsed = num.parse::<u16>().unwrap();
                if parsed <= 1 {
                    return Err(MetrumError::TokenError(TokenError::NotEnoughRepeats));
                }
                tokens.push(Token::BarRepeat(parsed));
            }
            '/' => return Err(MetrumError::TokenError(TokenError::LeadingSlash)),
            _ => {
                if curr.is_ascii_digit() {
                    let mut num = String::from(curr);
                    while score.peek().is_some() && score.peek().unwrap().is_ascii_digit() {
                        num.push(score.next().unwrap());
                    }
                    let parsed_num = num.parse::<u16>().unwrap();
                    if parsed_num == 0 {
                        return Err(MetrumError::TokenError(TokenError::Zero));
                    }

                    if score.peek().is_some() && *score.peek().unwrap() == '/' {
                        score.next();
                        if score.peek().is_some() && score.peek().unwrap().is_ascii_digit() {
                            let mut bottom = String::new();
                            while score.peek().is_some() && score.peek().unwrap().is_ascii_digit() {
                                bottom.push(score.next().unwrap());
                            }
                            let parsed_bottom = bottom.parse::<u16>().unwrap();
                            if parsed_bottom == 0 {
                                return Err(MetrumError::TokenError(TokenError::Zero));
                            }
                            tokens.push(Token::Ratio(parsed_num, parsed_bottom));
                        } else {
                            return Err(MetrumError::TokenError(TokenError::IncompleteRatio));
                        }
                    } else {
                        tokens.push(Token::Number(parsed_num));
                    }
                } else {
                    return Err(MetrumError::TokenError(TokenError::InvalidCharacter(curr)));
                }
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_tokens() {
        let data = vec![
            (".", Token::Dot),
            ("=", Token::Equal),
            ("|", Token::Barline),
            ("w", Token::Ratio(1, 1)),
            ("h", Token::Ratio(1, 2)),
            ("q", Token::Ratio(1, 4)),
            ("e", Token::Ratio(1, 8)),
            ("s", Token::Ratio(1, 16)),
            ("t", Token::Ratio(1, 32)),
        ];
        for (s, tok) in data.iter() {
            let output = scan(s.to_string());
            assert!(output.is_ok());
            assert_eq!(output.clone().unwrap().len(), 1);
            assert_eq!(output.clone().unwrap()[0], *tok);
        }
    }

    #[test]
    fn numbers() {
        let data = vec![
            ("123", 1, vec![Token::Number(123)]),
            (
                "1 2 3",
                3,
                vec![Token::Number(1), Token::Number(2), Token::Number(3)],
            ),
        ];
        for (s, l, tokens) in data.iter() {
            let output = scan(s.to_string());
            assert!(output.is_ok(), "{}", output.unwrap_err());
            assert_eq!(output.clone().unwrap().len(), *l);
            assert_eq!(output.clone().unwrap(), *tokens);
        }
    }

    #[test]
    fn ratios() {
        let data = vec![
            ("1/2", 1, vec![Token::Ratio(1, 2)]),
            ("1/2 1/2", 2, vec![Token::Ratio(1, 2), Token::Ratio(1, 2)]),
        ];
        for (s, l, tokens) in data.iter() {
            let output = scan(s.to_string());
            assert!(output.is_ok(), "{}", output.unwrap_err());
            assert_eq!(output.clone().unwrap().len(), *l);
            assert_eq!(output.clone().unwrap(), *tokens);
        }
    }

    #[test]
    fn repeats() {
        let data = vec![
            ("x2", 1, vec![Token::NoteRepeat(2)]),
            ("x2 x5", 2, vec![Token::NoteRepeat(2), Token::NoteRepeat(5)]),
            ("%2", 1, vec![Token::BarRepeat(2)]),
            ("x2 %5", 2, vec![Token::NoteRepeat(2), Token::BarRepeat(5)]),
        ];

        for (s, l, tokens) in data.iter() {
            let output = scan(s.to_string());
            assert!(output.is_ok(), "{}", output.unwrap_err());
            assert_eq!(output.clone().unwrap().len(), *l);
            assert_eq!(output.clone().unwrap(), *tokens);
        }
    }

    #[test]
    fn invalid_scores() {
        let data = vec![
            "i", "ul", "/", "/8", "1/2/4", "1/ 2", "1 /2", "x 1", "% 1", "|q|%1", "qx1", "qx0",
        ];
        for s in data.iter() {
            let output = scan(s.to_string());
            assert!(output.is_err());
        }
    }

    extern crate test_generator;
    use test_generator::test_resources;

    #[test_resources("examples/valid/*")]
    fn valid_scores(path: &str) {
        let input = std::fs::read_to_string(path).unwrap();
        let output = scan(input);
        assert!(output.is_ok(), "{}", output.unwrap_err());
    }
}
