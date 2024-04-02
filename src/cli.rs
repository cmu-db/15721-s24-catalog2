use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
  /// Sets the root of database
  #[arg(short, long, value_name = "db_root", default_value = "./database")]
  pub db_root: Option<PathBuf>,
}

pub fn parse() -> Cli {
  Cli::parse()
}
