use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use clap::ArgMatches;

use crate::types::{AppError, HandlerResult};
use std::collections::HashMap;

pub fn handler(matches: &ArgMatches) -> HandlerResult {
    let connection_string = matches.value_of("connection").unwrap();
    let dump_file = matches.value_of("dump-file").unwrap();
    let out_file = matches.value_of("out-file");

    let file = File::open(Path::new(dump_file)).map_err(|_| AppError::CannotOpenDumpFile)?;

    let statements = DumpReader::from_file(&file).read();

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

    pub fn read(&mut self) -> io::Result<Vec<Statements>> {
        let mut statements = Vec::new();
        loop {
            if self.read_until_comment().is_none() {
                break Ok(statements);
            }

            let comment = self.read_comment_block();
            if comment.is_none() {
                continue;
            }

            println!("comment block = {:?}", comment);
            return Ok(statements);

            let mut buf = String::new();
            let len = self.buf_reader.read_line(&mut buf)?;

            if len > 0 {
                println!("{}", buf);
            }
            break Ok(statements);
        }
    }

    fn read_comment_block(&mut self) -> Option<CommentBlock> {
        let mut buf = String::new();
        let mut comment_block = String::new();

        loop {
            match self.buf_reader.read_line(&mut buf) {
                Ok(len) => {
                    if len == 0 {
                        return CommentBlock::from_string(comment_block.clone());
                    }

                    if !buf.starts_with("--") {
                        return CommentBlock::from_string(comment_block.clone());
                    } else {
                        comment_block.push_str(&buf.clone());
                        buf.clear();
                    }
                }
                Err(_) => return None,
            }
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

#[derive(Debug)]
struct CommentBlock {
    entry_id: Option<u32>,
    meta: Option<Meta>,
}

impl CommentBlock {
    pub fn from_string(s: String) -> Option<Self> {
        Self::parse_toc(s)
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

struct Statements {}
