use clap::Parser as ClapParser;

use ::orgncf_generator::process;

#[derive(ClapParser, Debug)]
struct Args {
  /// Input file
  #[clap(short, long)]
  input: String,
  #[clap(short, long, default_value = ".")]
  output_dir: String,
}

fn main() {
  let args = Args::parse();
  // read the input file into a string
  let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");
  if let Err(e) = process(&input, &args.input, &args.output_dir) {
    eprintln!("Error parsing file {}: {}", args.input, e);
  }
}

