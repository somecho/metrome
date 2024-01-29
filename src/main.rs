use clap::Parser;
use metrum::{scanner, score::Score};

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> Result<(), hound::Error> {
    let args = Args::parse();
    let file = std::fs::read_to_string(&args.path).unwrap();
    let output_path = format!("{}.wav", &args.path.split('/').last().unwrap());
    let tokens = scanner::scan(file).unwrap();
    let score = Score::new(tokens).unwrap();
    score.write_click_track(&output_path)
}
