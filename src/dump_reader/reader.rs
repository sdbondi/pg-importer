use std::{
    fs::{self, File},
    io::Seek,
    io::SeekFrom,
    io::{self, BufRead},
    path::Path,
};

use super::{comment::CommentBlock, statement::Statement};

enum LineType {
    Blank,
    Comment,
    Statement,
}

pub struct DumpReader<'a> {
    buf_reader: io::BufReader<&'a File>,
    pos: usize,
    line_count: usize,
}

impl<'a> DumpReader<'a> {
    pub fn from_file(file: &'a File) -> Self {
        Self {
            buf_reader: io::BufReader::new(file),
            pos: 0,
            line_count: 0,
        }
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
                        self.read_line(&mut dummy_buf);
                        continue;
                    }
                    LineType::Comment => {
                        blank_count = 0;
                        match self.read_comment_block() {
                            Ok(c) => {
                                if c.is_toc() {
                                    last_toc_comment = Some(c.clone());
                                } else {
                                    last_toc_comment = None;
                                }
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

        let initial_pos = self.pos;
        let initial_line = self.line_count;

        loop {
            loop_count += 1;
            let len = self.read_line(&mut buf)?;

            let buf_trimmed = {
                let tmp = buf.to_string().clone();
                tmp.trim().to_string()
            };
            if buf_trimmed.len() == 0 {
                if !statements.is_empty() {
                    empty_line_count += 1;
                }
                if empty_line_count == 2 || loop_count > 5 {
                    break Ok(Statement::new(
                        statements.join(" "),
                        initial_line,
                        initial_pos,
                        self.pos,
                    ));
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
        self.peek_line(&mut buf)?;

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
            let len = self.read_line(&mut buf)?;
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
            match self.read_line(&mut buf) {
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

    fn peek_line(&mut self, buf: &mut String) -> io::Result<usize> {
        let size = self.buf_reader.read_line(buf)?;
        self.buf_reader
            .seek(io::SeekFrom::Current(-(buf.len() as i64)));
        Ok(size)
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        let size = self.buf_reader.read_line(buf)?;
        self.pos += size;
        self.line_count += 1;
        Ok(size)
    }
}
