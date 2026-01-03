pub(crate) mod analyzer;
pub(crate) mod common;
pub(crate) mod error;
pub(crate) mod lexer;
pub(crate) mod parser;

use std::io::{self, Cursor, Write};

use crate::lexer::source::SplitRawStatements;
use analyzer::Analyzer;

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
            let analyzer = Analyzer::new(parser);

            analyzer.for_each(|statement| match statement {
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
                Ok(s) => {
                    println!("{:?}", s);
                }
            });
        }
    }
}
