use crate::dump_reader::StatementType;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{fmt, io};

#[derive(Debug, Clone, Default)]
pub struct Meta {
    pub name: String,
    pub ty: StatementType,
    pub schema: String,
    pub owner: String,
}

impl Display for Meta {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}; Type: {}; Schema: {}, Owner: {}",
            self.name, self.ty, self.schema, self.owner
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommentBlock {
    pub entry_id: Option<u32>,
    pub meta: Meta,
    pub lineno: usize,
}

impl FromStr for CommentBlock {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_toc(s).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Unable to parse comment block")
        })
    }
}

impl CommentBlock {
    fn parse_toc(lines: &str) -> Option<Self> {
        let mut comment = Self::default();
        for s in lines.split('\n') {
            if s.starts_with("-- TOC") {
                comment.entry_id = Some(Self::parse_toc_entry_id(s)?);
            }
            if s.starts_with("-- Name:") || s.starts_with("-- Data for Name:") {
                comment.meta = Self::parse_toc_meta_str(s)?;
            }
        }

        Some(comment)
    }

    fn parse_toc_meta_str(meta_str: &str) -> Option<Meta> {
        let mut meta_str = meta_str[2..meta_str.len()].trim();
        if meta_str.starts_with("Data for") {
            meta_str = &meta_str[9..meta_str.len()];
        }
        let mut meta = Meta::default();
        for part in meta_str.split(';') {
            let mut iter = part.splitn(2, ':');

            let k = iter.next()?.trim();
            let v = iter.next()?.trim();
            match k {
                "Name" => {
                    meta.name = v.to_string();
                }
                "Type" => {
                    meta.ty = v.parse().ok()?;
                }
                "schema" => {
                    meta.schema = v.to_string();
                }
                "owner" => {
                    meta.owner = v.to_string();
                }
                _ => {}
            }
        }
        Some(meta)
    }

    fn parse_toc_entry_id(toc_str: &str) -> Option<u32> {
        let mut p = Parser::new(toc_str);
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
        Parser {
            data: data.as_bytes(),
            pos: 0,
        }
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

impl Display for CommentBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line#: {}, ID: {}, Meta({})",
            self.lineno,
            self.entry_id
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| "--".to_string()),
            self.meta
        )
    }
}
