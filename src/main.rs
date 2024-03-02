use clap::Parser;
use metrome::{scanner, score::Score};

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), hound::Error> {
    let args = Args::parse();
    let file = std::fs::read_to_string(&args.path).unwrap();
    let separator = match cfg!(target_os = "windows") {
        true => '\\',
        _ => '/',
    };
    let output_path = match &args.output {
        Some(path) => path.clone(),
        None => format!("{}.wav", &args.path.split(separator).last().unwrap()),
    };
    let tokens = scanner::scan(file).unwrap();
    let score = Score::new(tokens).unwrap();
    score.write_click_track(&output_path)
}
