use crate::score::Score;
use crate::units;

const WEAK: &[u8; 65536] = include_bytes!("../assets/digital/weak.wav");
const STRONG: &[u8; 65536] = include_bytes!("../assets/digital/strong.wav");

pub struct Metronome {
    pub weak: Vec<i16>,
    pub strong: Vec<i16>,
}

impl Metronome {
    pub fn new() -> Self {
        let weak = hound::WavReader::new(std::io::BufReader::new(&WEAK[..]))
            .unwrap()
            .samples::<i16>()
            .into_iter()
            .map(|sample| sample.unwrap())
            .collect::<Vec<i16>>();
        let strong = hound::WavReader::new(std::io::BufReader::new(&STRONG[..]))
            .unwrap()
            .samples::<i16>()
            .into_iter()
            .map(|sample| sample.unwrap())
            .collect::<Vec<i16>>();
        Metronome { strong, weak }
    }
}

impl Score {
    /// writes the rhythmic score out as a click track wav file
    pub fn write_click_track(&self, path: &str) -> Result<(), hound::Error> {
        let metronome = Metronome::new();
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(path, spec)?;
        let num_samples = units::ms_to_samples(self.total_duration(), spec.sample_rate) as usize;
        let mut buf: Vec<i16> = vec![0; num_samples];
        let mut position = 0;
        for bar in self.bars.iter() {
            for dur in bar.durations.iter() {
                let beat = match dur.strong {
                    true => &metronome.strong,
                    false => &metronome.weak,
                };
                for (index, sample) in beat.iter().enumerate() {
                    buf[index + position] = *sample;
                }
                position += units::ms_to_samples(dur.ms, spec.sample_rate) as usize;
            }
        }
        for sample in buf.iter() {
            writer.write_sample(*sample)?
        }
        Ok(())
    }
}
