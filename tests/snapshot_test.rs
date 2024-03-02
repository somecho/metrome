extern crate test_generator;

#[cfg(test)]
mod snapshot {
    use metrome::{scanner, score::Score};
    use test_generator::test_resources;
    #[test_resources("examples/valid/*")]
    fn wav(path: &str) {
        let file = std::fs::read_to_string(path).unwrap();
        let tokens = scanner::scan(file).unwrap();
        let score = Score::new(tokens).unwrap();
        let buffer = score.wav_buffer(44100);

        let separator = match cfg!(target_os = "windows") {
            true => '\\',
            _ => '/',
        };
        let snapshot_path = format!(
            "tests{separator}assets{separator}wav{separator}{}.wav",
            path.split(separator).last().unwrap()
        );
        let mut reader = hound::WavReader::open(snapshot_path).unwrap();
        let snapshot_buffer = reader
            .samples::<i16>()
            .map(|s| s.unwrap())
            .collect::<Vec<i16>>();
        assert_eq!(buffer.len(), snapshot_buffer.len());
        for i in 0..buffer.len() {
            assert_eq!(buffer[i], snapshot_buffer[i]);
        }
    }
}
