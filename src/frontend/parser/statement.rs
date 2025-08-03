use std::fmt::{write, Display};

use crate::frontend::lexer::token::Identifier;

use super::{alter_type::AlterType, datatype::Datatype, expression::Expression};

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
    Drop,
    // DML
    Insert,
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
            Statement::Drop => write!(f, "DROP"),
            Statement::Insert => write!(f, "INSERT"),
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
