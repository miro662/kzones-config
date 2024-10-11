use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use parser::{parse, ParserError};
use serde::{Deserialize, Serialize};
use snafu::{prelude::*, ResultExt};
use zone::Zone;

mod instruction;
mod parser;
mod zone;

#[derive(Debug, Deserialize, Default)]
struct Input {
    padding: u64,
    layouts: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct OutputLayout {
    name: String,
    padding: u64,
    zones: HashSet<Zone>,
}

#[derive(Serialize)]
struct Output(Vec<OutputLayout>);

impl TryInto<Output> for Input {
    type Error = Error;

    fn try_into(self) -> Result<Output, Self::Error> {
        let zones_result: Result<Vec<_>, Error> = self
            .layouts
            .into_iter()
            .map(|(name, layout_desc)| {
                let instruction =
                    parse(&layout_desc).context(PatternSnafu { name: name.clone() })?;
                Ok(OutputLayout {
                    name,
                    padding: self.padding,
                    zones: instruction.slice(Zone::full()),
                })
            })
            .collect();
        Ok(Output(zones_result?))
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`struct Args {
struct Args {
    #[arg(long)]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = generate_config(&args.input, io::stdout()) {
        eprintln!("{}", e)
    }
}

fn generate_config(input_file_path: &Path, output: impl Write) -> Result<(), Error> {
    let input_file = fs::read_to_string(&input_file_path).context(InputReadSnafu {
        path: input_file_path.to_string_lossy(),
    })?;
    let input: Input = toml::from_str(&input_file).context(InputDesrializeErorSnafu)?;
    let output_data: Output = input.try_into()?;
    serde_json::to_writer(output, &output_data).context(OutputSerializeSnafu)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Cannot read input file {path}: {source}"))]
    InputReadError { path: String, source: io::Error },
    #[snafu(display("Invaild pattern {name}: {source}"))]
    PatternError { name: String, source: ParserError },
    #[snafu(display("Cannot deserialize input file: {source}"))]
    InputDesrializeEror { source: toml::de::Error },
    #[snafu(display("Cannot serialize output: {source}"))]
    OutputSerializeError { source: serde_json::Error },
}
