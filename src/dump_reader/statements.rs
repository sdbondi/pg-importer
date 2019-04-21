use super::{Statement, StatementType};

pub struct Statements(Vec<Statement>);

impl Statements {
    pub fn wrap(statements: Vec<Statement>) -> Self {
        Self(statements)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn filter_by_type_mut(&mut self, ty: StatementType) -> Vec<&mut Statement> {
        self.0.iter_mut().filter(|s| s.ty == ty).collect()
    }

    pub fn extract_and_convert_constraints(&mut self) -> Vec<Statement> {
        let mut create_tables = self.filter_by_type_mut(StatementType::Table);
        let mut alter_statements = vec![];
        for ct in create_tables.iter_mut() {
            let constraints = ct.sql.find("CONSTRAINT").map(|idx| {
                let end = ct.sql.rfind(");");
                (idx, end.unwrap())
            });

            if let Some((start, end)) = constraints {
                let constraints = ct.sql[start..end]
                    .to_string()
                    .trim()
                    .replace("CONSTRAINT", "ADD CONSTRAINT");
                let mut remaining = ct.sql[0..start].trim().to_string();
                remaining.pop();
                remaining.push_str(");");
                ct.set_sql(remaining);

                let table_name = extract_table_name_from_create_table(&ct.sql);

                if table_name.is_none() {
                    panic!("Unable to get table name from statement: {}", ct.sql);
                }

                let mut alter_statement = Statement::from_sql(format!(
                    "ALTER TABLE {} {};",
                    table_name.unwrap().to_string(),
                    constraints
                ));
                alter_statement.ty = StatementType::Constraint;
                alter_statements.push(alter_statement);
            }
        }
        alter_statements
    }

    pub fn append(&mut self, statements: &mut Vec<Statement>) {
        self.0.append(statements);
    }

    pub fn replace_all(&mut self, filter: &str, replacement: &str) -> usize {
        let mut num_replaced = 0;
        for s in self.0.iter_mut() {
            if s.sql.contains(filter) {
                num_replaced += 1;
                s.set_sql(s.sql.replace(filter, replacement));
            }
        }
        num_replaced
    }

    pub fn iter(&self) -> std::slice::Iter<Statement> {
        self.0.iter()
    }
}

impl IntoIterator for Statements {
    type Item = Statement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn extract_table_name_from_create_table(sql: &str) -> Option<String> {
    // Ghetto parsing
    let parts = sql.split(' ').take(3);
    parts.last().map(|s| s.to_owned())
}
