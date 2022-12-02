use clap::{Arg, ArgMatches, Command};
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdventCodeError {
    #[error("Input File Error: {}", .0)]
    InputFileError(String),
}

pub struct AdventCode {
    args: ArgMatches,
}

impl AdventCode {
    pub fn new(name: &str) -> AdventCode {
        let args = Command::new(name)
            .arg(
                Arg::new("input_path")
                    .value_name("FILE")
                    .help("Input file path")
                    .required(true),
            )
            .get_matches();

        AdventCode { args }
    }

    pub fn get_input(&self) -> Result<Vec<String>, AdventCodeError> {
        let mut lines = Vec::new();

        let input_path = self
            .args
            .value_of("input_path")
            .expect("Argument should always be given");

        let reader = {
            let input = match File::open(input_path) {
                Ok(f) => f,
                Err(_) => {
                    return Err(AdventCodeError::InputFileError(format!(
                        "Unable to open file '{}'",
                        input_path
                    )))
                }
            };

            BufReader::new(input)
        };

        for line in reader.lines() {
            match line {
                Ok(line) => lines.push(line),
                Err(_) => {
                    return Err(AdventCodeError::InputFileError(format!(
                        "Unable to read file '{}'",
                        input_path
                    )))
                }
            }
        }

        Ok(lines)
    }
}
