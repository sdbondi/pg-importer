#[allow(unreachable)]
use std::process;

use clap::{App, Arg, SubCommand};

use crate::types::HandlerResult;
use crate::import;

pub fn run_cli() {
    let matches = App::new("DB importer")
        .about("Reliable Postgres DB importer")
        .subcommand(
            SubCommand::with_name("import")
                .about("Import the database from the given dump")
                .arg(
                    Arg::with_name("connection")
                        .required(true)
                        .short("c")
                        .takes_value(true)
                        .help("Postgres connection string"),
                )
                .arg(
                    Arg::with_name("dump-file")
                        .long("dump-file")
                        .short("f")
                        .takes_value(true)
                        .required(true)
                        .help("SQL DB dump file"),
                ),
        )
        .get_matches();

    let result: HandlerResult = match matches.subcommand() {
        ("import", Some(matches)) => import::handler(matches),
        ("", _) => {
            eprintln!("Missing subcommand");
            process::exit(1);
            unreachable!();
        }
        _ => {
            eprintln!("Invalid subcommand '{}'", matches.subcommand().0);
            process::exit(1);
            unreachable!();
        }
    };

    if let Err(e) = result {
        eprintln!("{:#?}", e);
        process::exit(1);
    }
}
