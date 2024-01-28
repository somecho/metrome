use crate::{
    error::{MetrumError, ParseError},
    scanner::Token,
};

#[derive(Debug, PartialEq)]
pub struct Tempo {
    pub beat: (u16, u16),
    pub num_beats: u16,
}

impl Tempo {
    pub fn new(beat: (u16, u16), num_beats: u16) -> Self {
        Tempo { beat, num_beats }
    }
}

#[derive(Clone, Debug)]
pub struct Duration {
    ms: f32,
    strong: bool,
}

#[derive(Debug)]
pub struct Bar {
    pub durations: Vec<Duration>,
}

impl Bar {
    pub fn new() -> Self {
        Bar {
            durations: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Score {
    pub bars: Vec<Bar>,
}

impl Score {
    pub fn new(tokens: Vec<Token>) -> Result<Self, MetrumError> {
        let mut tokens = tokens.iter().peekable();
        let mut bars: Vec<Bar> = Vec::new();
        let mut bar = Bar::new();
        let mut tempo = Tempo::new((1, 4), 120);
        while tokens.peek().is_some() {
            let curr = tokens.next().unwrap();
            match curr {
                Token::Barline => {
                    if !bar.durations.is_empty() {
                        bars.push(bar);
                        bar = Bar::new();
                    }
                }
                Token::Ratio(top, bottom) => {
                    let mut num_dots = 0;
                    while tokens.peek().is_some() && **tokens.peek().unwrap() == Token::Dot {
                        num_dots += 1;
                        tokens.next();
                    }
                    if tokens.peek().is_some() && **tokens.peek().unwrap() == Token::Equal {
                        tokens.next();
                        let number = tokens.next();
                        match number {
                            Some(number) => match number {
                                Token::Number(n) => {
                                    tempo = Tempo::new((*top, *bottom), *n);
                                }
                                Token::Ratio(top2, bottom2) => {}
                                _ => {
                                    return Err(MetrumError::ParseError(
                                        ParseError::MissingTempoSpecifier,
                                    ));
                                }
                            },
                            None => {
                                return Err(MetrumError::ParseError(
                                    ParseError::MissingTempoSpecifier,
                                ));
                            }
                        }
                    } else {
                        let duration = curr.as_duration_ms(&tempo, num_dots);
                        match duration {
                            Ok(d) => bar.durations.push(Duration {
                                ms: d,
                                strong: bar.durations.is_empty(),
                            }),
                            Err(e) => {
                                return Err(MetrumError::ConversionError(e));
                            }
                        }
                    }
                }
                Token::NoteRepeat(_) => todo!(),
                Token::BarRepeat(_) => todo!(),
                Token::Number(_) => return Err(MetrumError::ParseError(ParseError::Number)),
                Token::Equal => return Err(MetrumError::ParseError(ParseError::Equal)),
                Token::Dot => return Err(MetrumError::ParseError(ParseError::Dot)),
            }
        }

        Ok(Score { bars })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error, scanner::scan};
    #[test]
    fn missing_tempo() {
        let toks = scan("q =".to_string()).unwrap();
        let score = Score::new(toks);
        assert_eq!(
            score.unwrap_err(),
            error::MetrumError::ParseError(ParseError::MissingTempoSpecifier)
        );
    }

    #[test]
    fn tempo_changes() {
        let data = vec![("| q h=120 q|", 250.0), ("|q q=q. q.|", 500.0)];
        for (s, duration) in data.iter() {
            let toks = scan(s.to_string()).unwrap();
            let score = Score::new(toks).unwrap();
            let note = &score.bars[0].durations[1];
            assert_eq!(note.ms, *duration);
        }
    }

    #[test]
    fn setting_tempo() {
        let data = vec!["q = 140", "1/3=120", "q. = 80", "1/5.=200"];
        for d in data.iter() {
            let toks = scan(d.to_string()).unwrap();
            let score = Score::new(toks);
            assert!(score.is_ok());
        }
    }

    #[test]
    fn single_bar() {
        let toks = scan("| q q q q |".to_string()).unwrap();
        let score = Score::new(toks);
        assert!(score.is_ok());
        assert_eq!(score.as_ref().unwrap().bars.len(), 1);
        assert_eq!(
            score
                .as_ref()
                .unwrap()
                .bars
                .first()
                .unwrap()
                .durations
                .len(),
            4
        );
        assert!(
            score
                .as_ref()
                .unwrap()
                .bars
                .first()
                .unwrap()
                .durations
                .first()
                .unwrap()
                .strong
        );
        assert!(
            !score
                .as_ref()
                .unwrap()
                .bars
                .first()
                .unwrap()
                .durations
                .get(1)
                .unwrap()
                .strong
        )
    }
}
