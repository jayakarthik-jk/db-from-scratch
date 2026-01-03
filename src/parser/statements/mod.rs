pub(crate) mod alter;
pub(crate) mod create;
pub(crate) mod delete;
pub(crate) mod drop;
pub(crate) mod insert;
pub(crate) mod select;
pub(crate) mod update;

use super::{datatype::Datatype, expression::Expression};
use crate::lexer::token::Ident;
use alter::AlterType;
use std::fmt::Display;
use update::UpdateSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Column {
    pub name: Ident,
    pub data_type: Datatype,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Statement {
    CreateDatabase {
        database_name: Ident,
    },
    // DDL
    CreateTable {
        table_name: Ident,
        columns: Vec<Column>,
    },
    AlterTable {
        table_name: Ident,
        alter_types: Vec<AlterType>,
    },
    DropTable {
        table_name: Ident,
    },
    // DML
    Insert {
        table_name: Ident,
        columns: Option<Vec<Ident>>,
        values: Vec<Expression>,
    },
    Update {
        table_name: Ident,
        set: Vec<UpdateSet>,
        predicate: Option<Expression>,
    },
    Delete {
        table_name: Ident,
        predicate: Option<Expression>,
    },
    // DQL
    Select {
        select_expressions: Vec<Expression>,
        from: Option<Ident>,
        predicate: Option<Expression>,
    },
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::CreateTable {
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
            Statement::AlterTable {
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
            Statement::DropTable { table_name } => write!(f, "DROP TABLE {}", table_name),
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
            Statement::Update {
                table_name,
                set,
                predicate,
            } => {
                write!(f, "UPDATE {} SET ", table_name)?;
                for (i, update_set) in set.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} = {}", update_set.column, update_set.value)?;
                }
                if let Some(pred) = predicate {
                    write!(f, " WHERE {}", pred)?;
                }
                Ok(())
            }
            Statement::Delete {
                table_name,
                predicate,
            } => {
                write!(f, "DELETE FROM {}", table_name)?;
                if let Some(pred) = predicate {
                    write!(f, " WHERE {}", pred)?;
                }
                Ok(())
            }
            Statement::Select {
                select_expressions,
                from,
                predicate,
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
                if let Some(pred) = predicate {
                    write!(f, " WHERE {}", pred)?;
                }
                Ok(())
            }
            Statement::CreateDatabase { database_name } => {
                write!(f, "CREATE DATABASE {}", database_name)
            }
        }
    }
}
