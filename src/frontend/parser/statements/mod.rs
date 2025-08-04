pub(crate) mod alter;
pub(crate) mod create;
pub(crate) mod drop;
pub(crate) mod insert;
pub(crate) mod select;

use super::{datatype::Datatype, expression::Expression};
use crate::frontend::lexer::token::Identifier;
use alter::AlterType;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Column {
    pub name: Identifier,
    pub data_type: Datatype,
}

#[derive(Debug)]
pub(crate) enum Statement {
    // DDL
    Create {
        table_name: Identifier,
        columns: Vec<Column>,
    },
    Alter {
        table_name: Identifier,
        alter_types: Vec<AlterType>,
    },
    Drop {
        table_name: Identifier,
    },
    // DML
    Insert {
        table_name: Identifier,
        columns: Option<Vec<Identifier>>,
        values: Vec<Expression>,
    },
    Update,
    Delete,
    // DQL
    Select {
        select_expressions: Vec<Expression>,
        from: Option<Identifier>,
    },
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Create {
                table_name,
                columns,
            } => {
                write!(f, "CREATE TABLE {} (", table_name)?;
                for (i, column) in columns.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", column.name, column.data_type)?;
                }
                write!(f, ")")
            }
            Statement::Alter {
                table_name,
                alter_types,
            } => {
                write!(f, "ALTER TABLE {} ", table_name)?;
                for (i, alter_type) in alter_types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    match alter_type {
                        AlterType::Add(column) => write!(f, "ADD {}", column.name)?,
                        AlterType::Drop(name) => write!(f, "DROP COLUMN {}", name)?,
                        AlterType::Modify(column) => write!(f, "MODIFY {}", column.name)?,
                        AlterType::Rename { old, new } => {
                            write!(f, "RENAME COLUMN {} TO {}", old, new)?
                        }
                    }
                }
                Ok(())
            }
            Statement::Drop { table_name } => write!(f, "DROP TABLE {}", table_name),
            Statement::Insert {
                table_name,
                columns,
                values,
            } => {
                write!(f, "INSERT INTO {}", table_name)?;
                if let Some(cols) = columns {
                    write!(f, " ({})", cols.join(", "))?;
                }
                write!(f, " VALUES (")?;
                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, ")")
            }
            Statement::Update => write!(f, "UPDATE"),
            Statement::Delete => write!(f, "DELETE"),
            Statement::Select {
                select_expressions,
                from,
            } => {
                write!(f, "SELECT ")?;
                for (i, expr) in select_expressions.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", expr)?;
                }
                if let Some(table) = from {
                    write!(f, " FROM {}", table)?;
                }
                Ok(())
            }
        }
    }
}
