use metrum::scanner;

fn main() {
    let file = std::fs::read_to_string("examples/valid/simple-score").unwrap();
    let output = scanner::scan(file).unwrap();
    println!("{:?}", output);
}
