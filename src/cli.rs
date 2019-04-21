#[allow(unreachable_code)]
use std::process;

use clap::{App, Arg, SubCommand};

use crate::commands;
use crate::types::HandlerResult;

pub fn run_cli() {
    let matches = App::new("DB importer")
        .about("Reliable Postgres DB importer")
        .subcommand(
            SubCommand::with_name("import")
                .about("Import the database from the given dump")
                .arg(
                    Arg::with_name("connection")
                        .required(false)
                        .short("c")
                        .takes_value(true)
                        .help("Destination postgres connection string"),
                )
                .arg(
                    Arg::with_name("dump-file")
                        .short("f")
                        .takes_value(true)
                        .required(true)
                        .help("SQL DB dump file"),
                )
                .arg(
                    Arg::with_name("out-file")
                        .short("o")
                        .takes_value(true)
                        .required(true)
                        .help("Output dump file"),
                ),
        )
        .get_matches();

    let result: HandlerResult = match matches.subcommand() {
        ("import", Some(matches)) => commands::import::handler(matches),
        ("", _) => {
            eprintln!("Missing subcommand");
            process::exit(1);
        }
        _ => {
            eprintln!("Invalid subcommand '{}'", matches.subcommand().0);
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("{:#?}", e);
        process::exit(1);
    }
}
