use std::{fmt, str::FromStr};

use super::comment::CommentBlock;

#[derive(Debug, Clone)]
pub struct Toc {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Statement {
    pub pos: Option<(usize, usize)>,
    pub line: Option<usize>,
    pub sql: String,
    pub entry_id: Option<u32>,
    pub name: Option<String>,
    pub ty: StatementType,
}

impl Statement {
    pub fn new(sql: String, line: usize, pos_start: usize, pos_end: usize) -> Self {
        Self {
            sql,
            line: Some(line),
            pos: Some((pos_start, pos_end)),
            entry_id: None,
            name: None,
            ty: StatementType::Unknown,
        }
    }

    pub fn from_command(s: String) -> Self {
        Self {
            sql: s,
            line: None,
            pos: None,
            entry_id: None,
            name: None,
            ty: StatementType::Command,
        }
    }

    pub fn from_sql(sql: String) -> Self {
        Self {
            sql,
            line: None,
            pos: None,
            entry_id: None,
            name: None,
            ty: StatementType::Unknown,
        }
    }

    pub(super) fn set_from_comment_block(&mut self, comment: CommentBlock) {
        self.entry_id = comment.entry_id;
        self.name = Some(comment.meta.name);
        self.ty = comment.meta.ty;
    }

    pub fn set_sql(&mut self, sql: String) {
        self.sql = sql;
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Line: {}, Entry: {}, Name: {}, Type: {}",
            self.line.unwrap_or_default(),
            self.entry_id.unwrap_or_default(),
            self.name.as_ref().map(|s| s.as_str()).unwrap_or("--"),
            self.ty
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StatementType {
    Acl,
    Comment,
    Constraint,
    Extension,
    FkConstraint,
    Function,
    Index,
    Table,
    TableData,
    Command,
    Unknown,
}

impl Default for StatementType {
    fn default() -> Self {
        StatementType::Unknown
    }
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
            _ => Ok(StatementType::Unknown),
        }
    }
}

impl fmt::Display for StatementType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            StatementType::Acl => "ACL",
            StatementType::Comment => "COMMENT",
            StatementType::Extension => "EXTENSION",
            StatementType::FkConstraint => "FK CONSTRAINT",
            StatementType::Function => "FUNCTION",
            StatementType::Index => "INDEX",
            StatementType::Table => "TABLE",
            StatementType::TableData => "TABLE DATA",
            _ => "Unknown",
        };
        write!(f, "{}", s)
    }
}
