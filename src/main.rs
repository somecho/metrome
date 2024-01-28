use metrum::{
    scanner::{self, scan},
    score::Score,
};

fn main() {
    // let file = std::fs::read_to_string("examples/valid/simple-score").unwrap();
    // let output = scanner::scan(file).unwrap();

    let toks = scan("q =".to_string()).unwrap();
    let score = Score::new(toks);
    score.unwrap(); 

    // println!("{:?}", output);
}
