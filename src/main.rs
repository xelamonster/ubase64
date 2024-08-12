use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result};
use ubase64::decode::{STD_DECODER, URL_DECODER};
use ubase64::encode::{STD_ENCODER, URL_ENCODER};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
  #[command(visible_alias = "e")]
  Encode {
    #[arg(short, long)]
    file: bool,
    #[arg(long)]
    url: bool,
    value: String,
  },
  #[command(visible_alias = "d")]
  Decode {
    #[arg(short, long)]
    file: bool,
    #[arg(long)]
    url: bool,
    value: String,
  },
}

pub fn main() -> Result<()> {
  let cli = Cli::parse();
  match cli.command {
    Command::Encode { file, url, value } => {
      let input = if !file { value } else { read_to_string(value).into_diagnostic()? };
      let encoder = if url { URL_ENCODER } else { STD_ENCODER };
      println!("{}", encoder.encode(&input.as_bytes())?);
      Ok(())
    }
    Command::Decode { file, url, value } => {
      let input = if !file { value } else { read_to_string(value).into_diagnostic()? };
      let decoder = if url { URL_DECODER } else { STD_DECODER };
      println!("{}", String::from_utf8(decoder.decode(&input)?).into_diagnostic()?);
      Ok(())
    }
  }
}
