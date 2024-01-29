use crate::{
    error::{MetrumError, ParseError},
    scanner::Token,
};

#[derive(Debug, PartialEq)]
/// A tempo represents how fast music is going and is often represented in terms of *how many this
/// kind of beats per minute*. For example 140 quarter notes per minute.
pub struct Tempo {
    /// A tuple representing the kind of beat as a ratio. For example, the tuple `(1,4)` would
    /// represent a quarter (1/4) note.
    pub beat: (u16, u16),
    /// The number of [Tempo::beat] per minute
    pub num_beats: u16,
}

impl Tempo {
    pub fn new(beat: (u16, u16), num_beats: u16) -> Self {
        Tempo { beat, num_beats }
    }
}

#[derive(Clone, Debug)]
/// A [Duration] struct contains how long a note is in *milliseconds* and whether the beat is
/// strong or weak.
pub struct Duration {
    pub ms: f32,
    pub strong: bool,
}

impl Duration {
    /// creates a weak [Duration] from any duration
    pub fn to_weak(&self) -> Self {
        Duration {
            ms: self.ms,
            strong: false,
        }
    }
}

#[derive(Debug, Clone)]
/// A struct containing a vector of durations
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
/// A score contains a vector of bars, which in turn contains a vector of durations. A score
/// represents rhythm as a collection of durations, which will later be used to write the click
/// track.
pub struct Score {
    pub bars: Vec<Bar>,
}

impl Score {
    /// Takes in a list of [tokens][crate::scanner::Token] that has been parsed by
    /// the [scanner][crate::scanner] and returns a [Score] wrapped in a result.
    pub fn new(tokens: Vec<Token>) -> Result<Self, MetrumError> {
        let mut tokens = tokens.iter().peekable();
        let mut bars: Vec<Bar> = Vec::new();
        let mut bar = Bar::new();
        let mut tempo = Tempo::new((1, 4), 120);
        let mut history: Vec<Token> = Vec::new();
        while tokens.peek().is_some() {
            let curr = tokens.next().unwrap();
            history.push(*curr);
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
                                Token::Ratio(top2, bottom2) => {
                                    let mut dots = 0;
                                    while tokens.peek().is_some()
                                        && **tokens.peek().unwrap() == Token::Dot
                                    {
                                        dots += 1;
                                        tokens.next();
                                    }
                                    tempo = tempo.relative_to(
                                        &Token::Ratio(*top, *bottom).apply_dots(num_dots)?,
                                        &Token::Ratio(*top2, *bottom2).apply_dots(dots)?,
                                    )?;
                                }
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
                Token::NoteRepeat(n) => {
                    if bar.durations.len() == 0 {
                        return Err(MetrumError::ParseError(ParseError::NothingToRepeat));
                    }
                    for _ in 0..(n - 1) {
                        bar.durations.push(bar.durations.last().unwrap().to_weak());
                    }
                }
                Token::BarRepeat(n) => {
                    let prev = history.get(history.len() - 2);
                    match prev {
                        Some(tok) => match tok {
                            Token::Barline => {
                                if bars.len() == 0 {
                                    return Err(MetrumError::ParseError(
                                        ParseError::NothingToRepeat,
                                    ));
                                }
                                for _ in 0..(n - 1) {
                                    bars.push(bars.last().unwrap().clone())
                                }
                            }
                            _ => return Err(MetrumError::ParseError(ParseError::BarRepeat)),
                        },
                        None => return Err(MetrumError::ParseError(ParseError::NothingToRepeat)),
                    }
                }
                Token::Number(_) => return Err(MetrumError::ParseError(ParseError::Number)),
                Token::Equal => return Err(MetrumError::ParseError(ParseError::Equal)),
                Token::Dot => return Err(MetrumError::ParseError(ParseError::Dot)),
            }
        }

        if !bar.durations.is_empty() {
            return Err(MetrumError::ParseError(ParseError::UnterminatedBar));
        }

        Ok(Score { bars })
    }

    /// Calculates and returns the total duration of the score in *milliseconds*.
    pub fn total_duration(&self) -> f32 {
        self.bars.iter().fold(0.0, |acc, e| {
            acc + e.durations.iter().fold(0.0, |acc, e| acc + e.ms)
        })
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for bar in self.bars.iter() {
            out += "| ";
            for dur in bar.durations.iter() {
                out += format!(" {} ", dur.ms).as_str();
            }
        }
        out += " |";
        write!(f, "{}", out)
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
        let data = vec![
            ("| q q=h q|", 250.0),
            ("|q q=q. q.|", 500.0),
            ("|q q=1/4. q.|", 500.0),
            ("|q 1/4=1/4. q.|", 500.0),
            ("|h 2/4=2/4. h.|", 1000.0),
            ("|q q.=q q|", 750.0),
        ];
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
    fn total_durations() {
        let data = vec![
            ("| qx4 |", 2000.0),
            ("q=60 | qx4 |", 4000.0),
            ("q=240 | qx4 |", 1000.0),
            ("| qx3 |", 1500.0),
        ];
        for (d, dur) in data.iter() {
            let toks = scan(d.to_string()).unwrap();
            let score = Score::new(toks).unwrap();
            assert_eq!(score.total_duration(), *dur);
        }
    }

    #[test]
    fn bar_repeats() {
        let data = vec![("| q |%2", 2)];
        for (d, num_bars) in data.iter() {
            let toks = scan(d.to_string()).unwrap();
            let score = Score::new(toks).unwrap();
            assert_eq!(score.bars.len(), *num_bars);
        }
    }

    #[test]
    fn note_repeats() {
        let data = vec![("| qx4 |", 4), ("| qx4 hx2 |", 6), ("| qx100 |", 100)];
        for (d, num_notes) in data.iter() {
            let toks = scan(d.to_string()).unwrap();
            let score = Score::new(toks).unwrap();
            assert_eq!(score.bars[0].durations.len(), *num_notes);
            assert!(score.bars[0].durations[0].strong);
            for i in 1..score.bars[0].durations.len() {
                assert!(
                    !score.bars[0].durations[i].strong,
                    "The rest of the beats in a bar must be weak"
                );
            }
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

    extern crate test_generator;
    use test_generator::test_resources;

    #[test_resources("examples/valid/*")]
    fn valid_scores(path: &str) {
        let input = std::fs::read_to_string(path).unwrap();
        let output = Score::new(scan(input).unwrap());
        assert!(output.is_ok(), "{}", output.unwrap_err());
    }
}
