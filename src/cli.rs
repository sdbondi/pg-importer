use crate::commands;
use clap::{App, Arg, SubCommand};
#[allow(unreachable_code)]
use std::process;

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
                )
                .arg(
                    Arg::with_name("exclude-schema")
                        .short("x")
                        .long("exclude-schema")
                        .takes_value(true)
                        .help("Exclude a schema"),
                )
                .arg(
                    Arg::with_name("exclude-extension")
                        .long("exclude-extension")
                        .takes_value(true)
                        .help("Exclude an extension"),
                )
                .arg(
                    Arg::with_name("exclude-tabledata")
                        .long("exclude-tabledata")
                        .takes_value(true)
                        .help("Exclude an tabledata"),
                ),
        )
        .subcommand(
            SubCommand::with_name("read").arg(
                Arg::with_name("dump-file")
                    .short("f")
                    .takes_value(true)
                    .required(true)
                    .help("SQL DB dump file"),
            ),
        )
        .get_matches();

    let result = match matches.subcommand() {
        ("import", Some(matches)) => commands::import::handler(matches),
        ("read", Some(matches)) => commands::read::handler(matches),
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
