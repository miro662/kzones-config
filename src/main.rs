use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::{self, File},
    io,
    path::PathBuf,
};

use arena::Arena;
use clap::Parser;
use parser::parse;
use serde::{Deserialize, Serialize};

mod arena;
mod instruction;
mod parser;

#[derive(Debug, Deserialize)]
struct Input {
    padding: u64,
    layouts: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct OutputLayout {
    name: String,
    padding: u64,
    zones: HashSet<Arena>,
}

#[derive(Serialize)]
struct Output(Vec<OutputLayout>);

impl TryInto<Output> for Input {
    type Error = ();

    fn try_into(self) -> Result<Output, Self::Error> {
        Ok(Output(
            self.layouts
                .into_iter()
                .map(|(name, layout_desc)| OutputLayout {
                    name,
                    padding: self.padding,
                    zones: parse(&layout_desc).slice(Arena::full()).into(),
                })
                .collect(),
        ))
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`struct Args {
struct Args {
    #[arg(long)]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input_file = fs::read_to_string(args.input).unwrap();
    let input: Input = toml::from_str(&input_file)?;
    let output: Output = input.try_into().unwrap();
    serde_json::to_writer(io::stdout(), &output)?;
    Ok(())
}
