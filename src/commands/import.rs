use clap::ArgMatches;
use std::fs::{self, File};
use std::path::Path;

use crate::dump_reader::statement::{Statement, StatementType};
use crate::{
    dump_reader::DumpReader,
    types::{AppError, HandlerResult},
};

pub fn handler(matches: &ArgMatches) -> HandlerResult {
    let connection_string = matches.value_of("connection").unwrap();
    let dump_file = matches.value_of("dump-file").unwrap();
    let out_file = matches.value_of("out-file");

    let file = File::open(Path::new(dump_file)).map_err(|_| AppError::CannotOpenDumpFile)?;

    let statements = DumpReader::from_file(&file).read().unwrap();
    let mut statements = Statements::wrap(statements);

    let mut alter_statements = statements.extract_and_convert_constraints();
    statements.append(&mut alter_statements);
    println!(
        "Got {} statments",
        statements.filter_by_type_mut(StatementType::Table).len()
    );

    //    DumpWriter::from_sections(sections).write_to_file()

    //    let read_buf = BufReader::new(file);

    // Digest the contents and make a dump
    //    let parsed = contents.unwrap().parse::<Dump>();

    Ok(())

    // Drop Dump
}

use sqlparser::{sqlast::SQLStatement, dialect::PostgreSqlDialect, sqlparser::Parser};

pub struct Statements(Vec<Statement>);

impl Statements {
    pub fn wrap(statements: Vec<Statement>) -> Self {
        Self(statements)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn filter_by_type_mut(&mut self, ty: StatementType) -> Vec<&mut Statement> {
        self.0.iter_mut().filter(|s| s.ty == ty).collect()
    }

    pub fn extract_and_convert_constraints(&mut self) -> Vec<Statement> {
        let mut create_tables = self.filter_by_type_mut(StatementType::Table);
        for ct in create_tables.iter_mut() {
            let constraints = ct.sql.find("CONSTRAINT")
                .map(|idx| {
                    let end = ct.sql.rfind(";");
                    (idx, end.unwrap())
                });

            if let Some((start, end)) = constraints {
                let constraints =ct.sql[start..end].to_string();
                println!("CONSTRAINT: {:?}", constraints);
                let mut remaining = ct.sql[0..start].trim().to_string();
                remaining.pop();
                remaining.push(';');
                ct.set_sql(remaining);
                println!("new sql: {}", ct.sql);
            }
        }
        vec![]
    }

    pub fn append(&mut self, statements: &mut Vec<Statement>) {
        self.0.append(statements);
    }
}

fn parse_statement(statement: Statement) -> Vec<SQLStatement> {
    let dialect = PostgreSqlDialect {};
    Parser::parse_sql(&dialect, statement.sql.clone()).unwrap()
}
