use clap::ArgMatches;
use std::{
    fs,
    str::FromStr,
};

use crate::types::{AppError, HandlerResult};

pub fn handler(matches: &ArgMatches) -> HandlerResult {
    let connection_string = matches.value_of("connection").unwrap();
    let dump_file = matches.value_of("dump-file").unwrap();

    let contents = fs::read_to_string(dump_file);
    if let Err(err) = contents {
        return Err(AppError::DumpFileNotFound);
    }

    // Digest the contents and make a dump
    let parsed = contents.unwrap().parse::<Dump>();

    Ok(())

    // Drop Dump
}

pub struct Dump {
    statements: Vec<Statements>,
}

impl Dump {
    pub fn parse_str(contents: &str) -> Result<Self, AppError> {
        unimplemented!();
    }
}

impl FromStr for Dump {
    type Err = AppError;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        Dump::parse_str(contents)
    }
}


struct Statements {

}