use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(short = 'i', long = "input")]
    pub input_dir: std::path::PathBuf,
    #[arg(short = 'o', long = "output")]
    pub output_dir: std::path::PathBuf,
}
