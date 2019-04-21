use std::collections::HashMap;

pub type Meta = HashMap<String, String>;

#[derive(Debug, Clone, Default)]
pub struct CommentBlock {
    pub entry_id: Option<u32>,
    pub meta: Option<Meta>,
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
            if s.starts_with("-- Name:") || s.starts_with("-- Data for Name:") {
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
        let mut meta_str = meta_str[2..meta_str.len()].trim();
        if meta_str.starts_with("Data for") {
            meta_str = &meta_str[9..meta_str.len()];
        }
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
