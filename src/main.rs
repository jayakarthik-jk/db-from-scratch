pub(crate) mod common;
pub(crate) mod error;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod source;

use std::io::{self, Cursor, Write};

use crate::source::SplitRawStatements;

use {lexer::Lexer, parser::Parser};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut command = String::new();

        print!("> ");
        stdout.flush().expect("Unable to flush to stdout");

        while !command.ends_with(";\n") {
            let read = stdin
                .read_line(&mut command)
                .expect("Unable to read command");
            if read == 0 {
                return;
            }
        }

        for raw_statement in Cursor::new(command).split_raw_statements() {
            let lexer = Lexer::new(raw_statement.iter());
            let parser = Parser::new(lexer);

            parser.for_each(|st| match st {
                Err(err) => {
                    err.print(&raw_statement);
                }
                Ok(s) => {
                    println!("{:?}", s);
                }
            });
        }
    }
}
