use std::{
    fs::File,
    io::Seek,
    io::{self, BufRead},
};

use super::{comment::CommentBlock, statement::Statement, statement::StatementType};
use std::str::FromStr;

#[derive(Debug)]
enum LineType {
    Blank,
    Comment,
    Statement,
    Command,
}

pub struct DumpReader<'a> {
    buf_reader: io::BufReader<&'a File>,
    pos: usize,
    line_count: usize,
    config: Config,
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub exclude_schema: Vec<String>,
    pub exclude_extension: Vec<String>,
    pub exclude_table_data: Vec<String>,
}

impl<'a> DumpReader<'a> {
    pub fn new(file: &'a File, config: Config) -> Self {
        Self {
            buf_reader: io::BufReader::new(file),
            pos: 0,
            line_count: 0,
            config,
        }
    }

    pub fn read(&mut self) -> io::Result<Vec<Statement>> {
        let mut statements = Vec::new();
        let mut last_toc_comment = None;
        let mut blank_count = 0;
        loop {
            if let Ok(line) = self.next_line_type() {
                match line {
                    LineType::Blank => {
                        blank_count += 1;
                        if blank_count > 10 {
                            self.line_count -= blank_count;
                            break Ok(statements);
                        }
                        let mut dummy_buf = String::new();
                        self.read_line(&mut dummy_buf)?;
                        continue;
                    }
                    LineType::Comment => {
                        blank_count = 0;
                        match self.read_comment_block() {
                            Ok(c) => {
                                last_toc_comment = Some(c);
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
                        if let Some(comment) = last_toc_comment {
                            let mut exclude = false;
                            // 🤏🍝
                            if self.config.exclude_schema.contains(&comment.meta.schema) {
                                println!("Skipping schema {}", comment);
                                exclude = true;
                            } else if comment.meta.ty == StatementType::Extension
                                && self.config.exclude_extension.contains(&comment.meta.name)
                            {
                                println!("Skipping extension {}", comment);
                                exclude = true;
                            } else if comment.meta.ty == StatementType::TableData
                                && self.config.exclude_table_data.contains(&comment.meta.name)
                            {
                                println!("Skipping table data {}", comment);
                                exclude = true;
                            }

                            let mut statement = self.read_statement(Some(comment.meta.ty))?;
                            if !exclude {
                                statement.set_from_comment_block(comment);
                                println!("- {}", statement);
                                statements.push(statement.clone());
                            }
                        } else {
                            let statement = self.read_statement(None)?;
                            statements.push(statement.clone());
                        }

                        blank_count = 0;
                        last_toc_comment = None;
                    }
                    LineType::Command => {
                        let mut buf = String::new();
                        self.read_line(&mut buf)?;
                        statements.push(Statement::from_command(buf));

                        blank_count = 0;
                        last_toc_comment = None;
                    }
                }
            } else {
                break Ok(statements);
            }
        }
    }

    fn read_statement(&mut self, statement_type: Option<StatementType>) -> io::Result<Statement> {
        let initial_pos = self.pos;
        let initial_line_count = self.line_count;
        let mut statement = None;
        match statement_type {
            Some(statement_type) => match statement_type {
                StatementType::TableData => {
                    if let Some(s) = self.read_until("\\.") {
                        statement =
                            Some(Statement::new(s, initial_line_count, initial_pos, self.pos));
                    }
                }
                StatementType::Function => {
                    if let Some(s) = self.read_until_eofunc() {
                        statement =
                            Some(Statement::new(s, initial_line_count, initial_pos, self.pos));
                    }
                }
                _ => {
                    if let Some(s) = self.read_until_n_empty_lines(1) {
                        statement =
                            Some(Statement::new(s, initial_line_count, initial_pos, self.pos));
                    }
                }
            },
            None => {
                // Read until ends with semi
                if let Some(s) = self.read_until_ends_semi() {
                    statement = Some(Statement::new(s, initial_line_count, initial_pos, self.pos));
                }
            }
        }

        statement.ok_or(io::Error::from(io::ErrorKind::UnexpectedEof))
    }

    fn next_line_type(&mut self) -> io::Result<LineType> {
        let mut buf = String::new();
        self.peek_line(&mut buf)?;

        let buf = buf.trim();

        if buf.len() == 0 {
            return Ok(LineType::Blank);
        }

        if buf.starts_with("--") {
            Ok(LineType::Comment)
        } else if buf.starts_with("\\") {
            Ok(LineType::Command)
        } else {
            Ok(LineType::Statement)
        }
    }

    fn read_comment_block(&mut self) -> io::Result<CommentBlock> {
        let mut buf = String::new();
        let mut comment_block = String::new();

        let comment_str = loop {
            self.read_line(&mut buf)?;
            if buf.len() == 0 {
                break comment_block;
            }

            if buf.starts_with("--") {
                comment_block.push_str(&buf.clone());
            } else {
                break comment_block;
            }

            buf.clear();
        };

        let mut comment_block = CommentBlock::from_str(&comment_str)?;
        comment_block.lineno = self.line_count;
        Ok(comment_block)
    }

    fn read_until_eofunc(&mut self) -> Option<String> {
        let mut buf = String::new();
        let mut contents = String::new();
        loop {
            match self.read_line(&mut buf) {
                Ok(_) => {
                    contents.push_str(&buf);
                    let buf_trimmed = {
                        let tmp = buf.to_string().clone();
                        tmp.trim().to_string()
                    };
                    if buf_trimmed == "END $_$;" || buf_trimmed == "$_$;" || buf_trimmed == "$$;" {
                        return Some(contents);
                    } else {
                        buf.clear();
                    }
                }
                Err(_) => return None,
            }
        }
    }

    fn read_until(&mut self, pat: &str) -> Option<String> {
        let mut buf = String::new();
        let mut contents = String::new();
        loop {
            match self.read_line(&mut buf) {
                Ok(_) => {
                    contents.push_str(&buf);
                    if buf.starts_with(pat) {
                        return Some(contents.clone());
                    } else {
                        buf.clear();
                    }
                }
                Err(_) => return None,
            }
        }
    }

    fn read_until_n_empty_lines(&mut self, n: usize) -> Option<String> {
        let mut buf = String::new();
        let mut empty_line_count = 0;
        let mut contents = String::new();
        loop {
            match self.read_line(&mut buf) {
                Ok(_) => {
                    contents.push_str(&buf);
                    let buf_trimmed = {
                        let tmp = buf.to_string().clone();
                        tmp.trim().to_string()
                    };

                    if buf_trimmed.len() == 0 {
                        empty_line_count += 1;
                    }

                    if empty_line_count == n {
                        return Some(contents.clone());
                    }
                    buf.clear();
                }
                Err(_) => return None,
            }
        }
    }

    fn read_until_ends_semi(&mut self) -> Option<String> {
        let mut buf = String::new();
        let mut contents = String::new();
        loop {
            match self.read_line(&mut buf) {
                Ok(_) => {
                    contents.push_str(&buf);
                    let buf_trimmed = {
                        let tmp = buf.to_string().clone();
                        tmp.trim().to_string()
                    };

                    if buf_trimmed.ends_with(';') {
                        return Some(contents.clone());
                    } else {
                        buf.clear();
                    }
                }
                Err(_) => return None,
            }
        }
    }

    fn peek_line(&mut self, buf: &mut String) -> io::Result<usize> {
        let size = self.buf_reader.read_line(buf)?;
        self.buf_reader
            .seek(io::SeekFrom::Current(-(buf.len() as i64)))?;
        Ok(size)
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        let size = self.buf_reader.read_line(buf)?;
        self.pos += size;
        self.line_count += 1;
        Ok(size)
    }
}
