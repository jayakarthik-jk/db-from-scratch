// pub(crate) mod backend;
pub(crate) mod common;
pub(crate) mod lexer;
pub(crate) mod parser;

use std::io::{self, Write};

use common::layer::BufferedLayer;
use {
    lexer::{reader::CharacterIterator, Lexer},
    parser::Parser,
};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    // let mut app = App::new();
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

        let reader = CharacterIterator::new(std::io::Cursor::new(command));
        let lexer = Lexer::new(BufferedLayer::new(reader));
        let parser = Parser::new(BufferedLayer::new(lexer));
        parser.for_each(|statement| {
            println!("{:?}", statement.expect("Unable to parse statement"));
        });

        // let statement = match App::prepare_statement(&command[..command.len() - 2].trim()) {
        //     Ok(statement) => statement,
        //     Err(err) => {
        //         use StatementPreparationError::*;
        //         match err {
        //             UnrecognizedCommand => eprintln!("Unrecognized command"),
        //             MissingField => eprintln!("Insert statement missing fields"),
        //             InvalidValue => eprintln!("Invalid value for field"),
        //         }
        //         continue;
        //     }
        // };
        //
        // match app.process_statement(statement) {
        //     Ok(true) => break,
        //     Err(err) => match err {
        //         StatementProcessingError::UnknownMetaCommand => {
        //             eprintln!("Unrecognized Meta command")
        //         }
        //         StatementProcessingError::MaxRowReached => eprintln!("Max row reached"),
        //     },
        //     _ => (),
        // }
    }
}

// enum Statement {
//     Meta(String),
//     Insert(Row),
// }
//
// enum StatementPreparationError {
//     UnrecognizedCommand,
//     MissingField,
//     InvalidValue,
// }
//
// enum StatementProcessingError {
//     UnknownMetaCommand,
//     MaxRowReached,
// }
//
// struct App {
//     table: Table,
// }
//
// impl App {
//     fn new() -> Self {
//         let path = "test.db";
//         Self {
//             table: Table::new(path).unwrap(),
//         }
//     }
// }
//
// impl App {
//     fn prepare_statement(command: &str) -> Result<Statement, StatementPreparationError> {
//         if command.starts_with('.') {
//             return App::prepare_meta_statement(&command[1..]);
//         }
//
//         const INSERT: &'static str = "insert ";
//         if command.starts_with(INSERT) {
//             return App::prepare_insert_statement(&command[INSERT.len()..]);
//         }
//
//         return Err(StatementPreparationError::UnrecognizedCommand);
//     }
//
//     fn prepare_meta_statement(meta_command: &str) -> Result<Statement, StatementPreparationError> {
//         return Ok(Statement::Meta(meta_command.to_owned()));
//     }
//
//     fn prepare_insert_statement(data: &str) -> Result<Statement, StatementPreparationError> {
//         let splitted: Vec<&str> = data.splitn(3, ' ').collect();
//         let id = splitted
//             .get(0)
//             .ok_or(StatementPreparationError::MissingField)?
//             .parse::<u32>()
//             .map_err(|_| StatementPreparationError::InvalidValue)?;
//         let username = splitted
//             .get(1)
//             .ok_or(StatementPreparationError::MissingField)?
//             .to_string();
//         let email = splitted
//             .get(2)
//             .ok_or(StatementPreparationError::MissingField)?
//             .to_string();
//         return Ok(Statement::Insert(Row {
//             id,
//             username,
//             email,
//         }));
//     }
//
//     fn process_statement(
//         &mut self,
//         statement: Statement,
//     ) -> Result<bool, StatementProcessingError> {
//         match statement {
//             Statement::Meta(command) => match command.as_str() {
//                 "exit" => {
//                     println!("Bye!");
//                     return Ok(true);
//                 }
//                 "show" => {
//                     let cursor = Cursor::from_start(&mut self.table);
//                     for row in cursor {
//                         println!("( {}, {}, {} )", row.id, row.username, row.email);
//                     }
//                 }
//                 _ => return Err(StatementProcessingError::UnknownMetaCommand),
//             },
//             Statement::Insert(row) => {
//                 self.table.insert(row).map_err(|err| match err {
//                     InsertError::MaxRowReached => StatementProcessingError::MaxRowReached,
//                 })?;
//             }
//         }
//         Ok(false)
//     }
// }
