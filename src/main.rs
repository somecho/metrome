use metrum::{scanner, score::Score};

fn main() {
    // let file = std::fs::read_to_string("examples/valid/eine-kleine-nachtmusik").unwrap();
    let file = String::from(
        "
        h=120
        | q q q q |  1/3x3  | 1/5x5 | qx4|
                            ",
    );
    let tokens = scanner::scan(file).unwrap();
    let score = Score::new(tokens).unwrap();
    score.write_click_track("test.wav");
    println!("{}", score);
}
