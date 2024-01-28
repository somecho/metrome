use metrum::{scanner, score::Score};

fn main() {
    let file = std::fs::read_to_string("examples/valid/eine-kleine-nachtmusik").unwrap();
    let tokens = scanner::scan(file).unwrap();
    let score = Score::new(tokens).unwrap();
    println!("{}", score);
}
