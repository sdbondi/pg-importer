use crate::dump_reader::reader;
use crate::{
    dump_reader::DumpReader,
    types::{AppError, CommandResult},
};
use clap::ArgMatches;
use std::{fs::File, path::Path};

pub fn handler(matches: &ArgMatches) -> CommandResult<()> {
    let dump_file = matches.value_of("dump-file").unwrap();
    let in_file = File::open(Path::new(dump_file)).map_err(|_| AppError::CannotOpenDumpFile)?;

    println!("Parsing dump file {}...", dump_file);
    let statements = DumpReader::new(&in_file, reader::Config::default())
        .read()
        .unwrap();

    println!("Collected {} statements", statements.len());

    Ok(())
}
