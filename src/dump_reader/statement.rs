use std::str::FromStr;

use super::comment::CommentBlock;

#[derive(Debug, Clone)]
pub struct Toc {}

#[derive(Debug, Clone)]
pub struct Statement {
    pub pos_start: usize,
    pub pos_end: usize,
    pub line: usize,
    pub sql: String,
    pub entry_id: Option<u32>,
    pub name: Option<String>,
    pub ty: StatementType,
}

impl Statement {
    pub fn new(sql: String, line: usize, pos_start: usize, pos_end: usize) -> Self {
        Self {
            sql,
            line,
            pos_start,
            pos_end,
            entry_id: None,
            name: None,
            ty: StatementType::Unknown,
        }
    }

    pub(super) fn set_toc_from_comment(&mut self, comment: CommentBlock) {
        if !comment.is_toc() {
            panic!("Cannot set TOC from non-TOC comment");
        }
        let meta = comment.meta.unwrap();
        let name = &meta["Name"];
        let ty = meta["Type"].parse::<StatementType>();
        if let Err(e) = ty {
            panic!("Unable to parse StatementType: {}", e);
        }

        self.entry_id = Some(comment.entry_id.unwrap());
        self.name = Some(name.clone());
        self.ty = ty.unwrap();
    }

    pub fn set_sql(&mut self, sql: String) {
        self.sql = sql;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
            _ => Ok(StatementType::Unknown),
        }
    }
}
