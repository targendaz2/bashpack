use clap::Parser;

#[derive(Parser)]
struct Cli {
    input_file: String,
    output_file: String,
}

fn main() {
    let args = Cli::parse();
}
