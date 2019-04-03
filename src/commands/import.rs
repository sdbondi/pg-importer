use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
    str::FromStr,
    io::Seek,
};

use clap::ArgMatches;

use crate::types::{AppError, HandlerResult};
use std::collections::HashMap;
use crate::commands::import::LineType::Comment;

pub fn handler(matches: &ArgMatches) -> HandlerResult {
    let connection_string = matches.value_of("connection").unwrap();
    let dump_file = matches.value_of("dump-file").unwrap();
    let out_file = matches.value_of("out-file");

    let file = File::open(Path::new(dump_file)).map_err(|_| AppError::CannotOpenDumpFile)?;

    let statements = DumpReader::from_file(&file).read();

    println!("{:#?}", statements);

    //    DumpWriter::from_sections(sections).write_to_file()

//    let read_buf = BufReader::new(file);

    // Digest the contents and make a dump
//    let parsed = contents.unwrap().parse::<Dump>();

    Ok(())

    // Drop Dump
}

pub struct DumpReader<'a> {
    buf_reader: io::BufReader<&'a File>
}

impl<'a> DumpReader<'a> {
    pub fn from_file(file: &'a File) -> Self {
        Self { buf_reader: io::BufReader::new(file) }
    }

    pub fn read(&mut self) -> io::Result<Vec<Statement>> {
        let mut statements = Vec::new();
        let mut last_toc_comment = None;
        let mut blank_count = 0;
        loop {
            if let Ok(line) = self.next_line_type() {
                if last_toc_comment.is_some() {
                    let mut statement = self.read_statement()?;

                    statement.set_toc_from_comment(last_toc_comment.unwrap());
                    println!("statement {:#?}", statement);
                    statements.push(statement.clone());
                    last_toc_comment = None;
                    continue;
                }

                match line {
                    LineType::Blank => {
                        blank_count += 1;
                        if blank_count > 10 {
                            break Ok(statements);
                        }
                        let mut dummy_buf = String::new();
                        self.buf_reader.read_line(&mut dummy_buf);
                        continue
                    },
                    LineType::Comment => {
                        blank_count = 0;
                        match self.read_comment_block() {
                            Ok(c) => {
                                if c.is_toc() {
                                    last_toc_comment = Some(c.clone());
                                } else {
                                    last_toc_comment = None;
                                }

                                println!("comment {:#?}", c);
                            }
                            Err(e) => {
                                if e.kind() == io::ErrorKind::UnexpectedEof {
                                    break Ok(statements);
                                } else {
                                    break Err(e);
                                }
                            }
                        }
                    }
                    LineType::Statement => {
                        blank_count = 0;
                        let statement = self.read_statement()?;
                        println!("statement {:#?}", statement);
                        statements.push(statement.clone());
                        last_toc_comment = None;
                    }
                }
            } else {
                break Ok(statements);
            }
        }
    }

    fn read_statement(&mut self) -> io::Result<Statement> {
        let mut buf = String::new();
        let mut statements = Vec::new();
        let mut empty_line_count = 0;
        let mut loop_count = 0;

        loop {
            loop_count += 1;
            let len = self.buf_reader.read_line(&mut buf)?;

            let buf_trimmed = {
                let tmp = buf.to_string().clone();
                tmp.trim().to_string()
            };
            println!("sbuf {}", buf);
            if buf_trimmed.len() == 0 {
                if !statements.is_empty() {
                    empty_line_count += 1;
                }
                if empty_line_count == 2 || loop_count > 5 {
                    break Ok(Statement::from_sql(statements.join(" ")));
                }
                buf.clear();
                continue;
            }

            if !buf_trimmed.starts_with("--") {
                statements.push(buf_trimmed.clone());
            }

            buf.clear();
            empty_line_count = 0;
        }
    }

    fn next_line_type(&mut self) -> io::Result<LineType> {
        let mut buf = String::new();
        self.buf_reader.read_line(&mut buf)?;
        self.buf_reader.seek(io::SeekFrom::Current(-(buf.len() as i64)));

        let buf = buf.trim();

        if buf.len() == 0 {
            return Ok(LineType::Blank);
        }

        if buf.starts_with("--") {
            Ok(LineType::Comment)
        } else {
            Ok(LineType::Statement)
        }
    }

    fn read_comment_block(&mut self) -> io::Result<CommentBlock> {
        let mut buf = String::new();
        let mut comment_block = String::new();

        let comment = loop {
            let len = self.buf_reader.read_line(&mut buf)?;
//            let buf_trimmed = buf;
//                {
//                let tmp = buf.to_string().clone();
//                tmp.trim().to_string()
//            };
            if buf.len() == 0 {
                break CommentBlock::from_string(comment_block.clone());
            }

            if buf.starts_with("--") {
                comment_block.push_str(&buf.clone());
            } else {
                break CommentBlock::from_string(comment_block.clone());
            }

            buf.clear();
        };

        if let Some(c) = comment {
            Ok(c)
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidData))
        }
    }

    fn read_until_comment(&mut self) -> Option<String> {
        let mut buf = String::new();
        loop {
            match self.buf_reader.read_line(&mut buf) {
                Ok(len) => {
                    if len > 0 {
                        if buf.starts_with("--") {
                            return Some(buf.clone());
                        } else {
                            buf.clear();
                        }
                    }
                }
                Err(_) => return None,
            }
        }
    }
}

type Meta = HashMap<String, String>;

enum LineType {
    Blank,
    Comment,
    Statement,
}

#[derive(Debug, Clone, Default)]
struct CommentBlock {
    entry_id: Option<u32>,
    meta: Option<Meta>,
}

impl CommentBlock {
    pub fn from_string(s: String) -> Option<Self> {
        match Self::parse_toc(s) {
            Some(c) => Some(c),
            None => Some(Default::default()),
        }
    }

    pub fn is_toc(&self) -> bool {
        self.entry_id.is_some()
    }

    fn parse_toc(lines: String) -> Option<Self> {
        let mut toc_str = None;
        let mut meta_str = None;
        for s in lines.split('\n') {
            if s.starts_with("-- TOC") {
                toc_str = Some(s.to_owned());
            }
            if s.starts_with("-- Name:") {
                meta_str = Some(s.to_owned());
            }
        }

        if toc_str.is_none() || meta_str.is_none() {
            None
        } else {
            Some(CommentBlock {
                entry_id: Self::parse_toc_entry_id(toc_str.unwrap()),
                meta: Self::parse_toc_meta_str(meta_str.unwrap()),
            })
        }
    }

    fn parse_toc_meta_str(meta_str: String) -> Option<Meta> {
        let meta_str = &meta_str[2..meta_str.len()];
        let mut meta = HashMap::new();
        for part in meta_str.split(';') {
            let v = part.splitn(2, ':').collect::<Vec<&str>>();
            if v.len() != 2 {
                return None;
            }
            meta.insert(v[0].trim().to_string(), v[1].trim().to_string());
        }
        Some(meta)
    }

    fn parse_toc_entry_id(toc_str: String) -> Option<u32> {
        let mut p = Parser::new(toc_str.as_str());
        p.read_token("--");
        p.read_token("TOC");
        p.read_token("entry");
        p.read_number().map(|n| n as u32)
    }
}

struct Parser<'a> {
    pos: usize,
    data: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a str) -> Self {
        Parser { data: data.as_bytes(), pos: 0 }
    }

    fn read_token(&mut self, tok: &str) -> Option<String> {
        let start = self.pos;
        let end = self.pos + tok.len();
        if self.data[start..end] == *tok.as_bytes() {
            self.pos += tok.len();
            self.read_whitespace();
            Some(tok.to_string())
        } else {
            None
        }
    }

    fn read_whitespace(&mut self) {
        while self.data[self.pos..self.pos + 1] == *b" " {
            self.pos += 1
        }
    }

    fn read_number(&mut self) -> Option<u64> {
        let mut pos = self.pos;
        let mut number = 0u64;
        while pos < self.data.len() {
            let ch = self.data[pos];

            if ch < b'0' || ch > b'9' {
                break;
            }
            number = number * 10u64 + (ch - b'0') as u64;
            pos += 1;
        }

        if pos == self.pos {
            None
        } else {
            self.pos = pos;
            Some(number)
        }
    }
}

#[derive(Debug, Clone)]
enum StatementType {
    Acl,
    Comment,
    Constraint,
    Extension,
    FkConstraint,
    Function,
    Index,
    Table,
    TableData,
    Unknown,
}

impl FromStr for StatementType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACL" => Ok(StatementType::Acl),
            "COMMENT" => Ok(StatementType::Comment),
            "CONSTRAINT" => Ok(StatementType::Constraint),
            "EXTENSION" => Ok(StatementType::Extension),
            "FK CONSTRAINT" => Ok(StatementType::FkConstraint),
            "FUNCTION" => Ok(StatementType::Function),
            "INDEX" => Ok(StatementType::Index),
            "TABLE" => Ok(StatementType::Table),
            "TABLE DATA" => Ok(StatementType::TableData),
            _ => Ok(StatementType::Acl),
        }
    }
}

#[derive(Debug, Clone)]
struct Toc {
    entry_id: u32,
    name: String,
    ty: StatementType,
}

#[derive(Debug, Clone)]
struct Statement {
    sql: String,
    toc: Option<Toc>,
}

impl Statement {
    pub fn from_sql(sql: String) -> Self {
        Self { sql, toc: None }
    }

    fn set_toc_from_comment(&mut self, comment: CommentBlock) {
        if !comment.is_toc() {
            panic!("Cannot set TOC from non-TOC comment");
        }
        let meta = comment.meta.unwrap();
        let name = &meta["Name"];
        let ty = meta["Type"].parse::<StatementType>();
        if let Err(e) = ty {
            panic!("Unable to parse StatementType: {}", e);
        }

        self.toc = Some(Toc {
            entry_id: comment.entry_id.unwrap(),
            name: name.clone(),
            ty: ty.unwrap(),
        });
    }
}
