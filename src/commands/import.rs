use clap::ArgMatches;
use postgres::{Connection, TlsMode};
use std::{cmp::min, fs::File, io::Write, path::Path};

use crate::dump_reader::StatementType;
use crate::{
    dump_reader::{DumpReader, Statements},
    types::{AppError, HandlerResult},
};

pub fn handler(matches: &ArgMatches) -> HandlerResult {
    let connection_string = matches.value_of("connection");
    let dump_file = matches.value_of("dump-file").unwrap();
    let out_file = matches.value_of("out-file").unwrap();

    let in_file = File::open(Path::new(dump_file)).map_err(|_| AppError::CannotOpenDumpFile)?;

    println!("Parsing dump file {}...", dump_file);
    let statements = DumpReader::from_file(&in_file).read().unwrap();
    let mut statements = Statements::wrap(statements);

    // Now to fudge things
    println!("Fixing constraints...");
    let mut alter_statements = statements.extract_and_convert_constraints();
    statements.append(&mut alter_statements);
    statements.replace_all(
        "SELECT pg_catalog.set_config('search_path', '', false);",
        "SELECT pg_catalog.set_config('search_path', 'public', false);",
    );

        println!("Writing output dump file {}...", out_file);
        let mut f = File::create(Path::new(out_file)).map_err(|_| AppError::CannotWriteToOutfile)?;

        for s in statements.iter() {
            f.write(format!("-- {}\n\n{}\n\n\n", s, s.sql).as_bytes())
                .unwrap();
        }

    if let Some(conn_str) = connection_string {
        let mut conn = Connection::connect(conn_str, TlsMode::None).unwrap();
        for s in statements.into_iter() {
            if s.ty == StatementType::Command {
                if s.sql.starts_with("\\connect") {
                    let db = s.sql.split(" ").last().unwrap();
                    let db = &db[1..db.len() - 2];
                    println!("Connecting to db {}", db);
                    conn =
                        Connection::connect(format!("{}/{}", conn_str, db), TlsMode::None)
                            .unwrap();
                } else {
                    println!("WARNING: skipping psql command {}", s.sql);
                }
                continue;
            }
            let nl_idx = s.sql.find('\n');
            println!(
                "Executing line {} [{}...]",
                s.line.unwrap_or(0),
                &s.sql //[0..min(nl_idx.unwrap_or(100), s.sql.len())]
            );

            match conn.batch_execute(&s.sql) {
                Ok(_) => {}
                Err(e) => {
                    println!("Failed for sql: {:#?}", s.sql);
                    panic!("Error (line {:?}): {:?}", s.line.unwrap_or(0), e);
                }
            }
        }
    }

    println!("Done.\n\nYou may import {} using psql.", out_file);
    Ok(())
}
