use crate::{
    error::{ConversionError, MetrumError},
    scanner::Token,
    score::Tempo,
};

impl Tempo {
    /// Calculates how many whole notes per minute given the current tempo
    pub fn wholes_per_min(&self) -> f32 {
        let tempo_ratio = self.beat.0 as f32 / self.beat.1 as f32;
        tempo_ratio * self.num_beats as f32
    }

    /// Calculates the length of a whole note in this tempo in ms
    pub fn duration_of_whole(&self) -> f32 {
        60.0 * 1000.0 / self.wholes_per_min()
    }

    pub fn relative_to(&self, ratio1: &Token, ratio2: &Token) -> Result<Self, MetrumError> {
        match ratio1 {
            Token::Ratio(t1, b1) => match ratio2 {
                Token::Ratio(t2, b2) => {
                    let nums = self.wholes_per_min() / (*t1 as f32 / *b1 as f32);
                    Ok(Tempo {
                        beat: (*t2, *b2),
                        num_beats: nums as u16,
                    })
                }
                _ => Err(MetrumError::ConversionError(ConversionError::NonRatio)),
            },
            _ => return Err(MetrumError::ConversionError(ConversionError::NonRatio)),
        }
    }
}

impl Token {
    /// Converts a ratio to a duration. If token is not a ratio, returns a conversion error.
    pub fn as_duration_ms(&self, tempo: &Tempo, num_dots: u16) -> Result<f32, ConversionError> {
        match self {
            Token::Ratio(top, bottom) => {
                let duration_of_whole = tempo.duration_of_whole();
                let duration = duration_of_whole * (*top as f32 / *bottom as f32);
                let mut multiplier = 1.0;
                for i in 0..num_dots {
                    multiplier += 0.5 / (i as f32 + 1.0);
                }
                Ok(duration * multiplier)
            }
            _ => Err(ConversionError::NonRatioToDuration),
        }
    }
}

#[cfg(test)]
mod tests {
    mod tempo {
        use crate::{scanner::Token, score::Tempo};

        #[test]
        fn whole_duration() {
            let data = vec![
                (Tempo::new((1, 4), 120), 2000.0),
                (Tempo::new((1, 1), 60), 1000.0),
                (Tempo::new((1, 2), 60), 2000.0),
                (Tempo::new((1, 4), 60), 4000.0),
                (Tempo::new((1, 8), 60), 8000.0),
            ];
            for (tempo, duration) in data.iter() {
                assert_eq!(tempo.duration_of_whole(), *duration);
            }
        }

        #[test]
        fn relative_tempo_change() {
            let data = vec![
                (
                    Tempo::new((1, 4), 120),
                    Token::Ratio(1, 4),
                    Token::Ratio(2, 4),
                    Tempo::new((2, 4), 120),
                ),
                (
                    Tempo::new((1, 2), 60),
                    Token::Ratio(1, 4),
                    Token::Ratio(3, 8),
                    Tempo::new((3, 8), 120),
                ),
            ];
            for (tempo1, ratio1, ratio2, tempo2) in data.iter() {
                assert_eq!(tempo1.relative_to(ratio1, ratio2).unwrap(), *tempo2);
            }
        }
    }

    mod ratio {
        use crate::{scanner::Token, score::Tempo};

        #[test]
        fn duration() {
            let data = vec![
                (Tempo::new((1, 4), 120), Token::Ratio(1, 4), 0, 500.0),
                (Tempo::new((1, 4), 120), Token::Ratio(1, 4), 1, 750.0),
                (Tempo::new((1, 4), 60), Token::Ratio(1, 4), 0, 1000.0),
                (Tempo::new((1, 4), 60), Token::Ratio(1, 4), 2, 1750.0),
                (Tempo::new((1, 4), 240), Token::Ratio(1, 4), 0, 250.0),
                (Tempo::new((1, 4), 120), Token::Ratio(5, 4), 0, 2500.0),
                (Tempo::new((1, 4), 120), Token::Ratio(10, 8), 0, 2500.0),
            ];
            for (tempo, ratio, num_dots, duration) in data.iter() {
                assert_eq!(ratio.as_duration_ms(tempo, *num_dots).unwrap(), *duration);
            }
        }
    }
}
