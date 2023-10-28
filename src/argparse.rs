use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'i', long = "input")]
    pub input_dir: std::path::PathBuf,
    #[arg(short = 'o', long = "output")]
    pub output_dir: std::path::PathBuf,
}
