use std::{
    io::{self, Write},
    process::exit,
};

fn print_prefix(stdout: &mut io::Stdout) {
    print!("> ");
    stdout.flush().expect("Unable to flush to stdout");
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut app = App::default();

    loop {
        let mut command = String::new();

        print_prefix(&mut stdout);

        while !command.ends_with(";\n") {
            let mut buffer = String::new();
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read command");
            command.push_str(&buffer);
        }

        let statement = match App::prepare_statement(&command[..command.len() - 2].trim()) {
            Ok(statement) => statement,
            Err(err) => {
                use StatementPreparationError::*;
                match err {
                    UnrecognizedCommand => eprintln!("Unrecognized command"),
                    MissingField => eprintln!("Insert statement missing fields"),
                    InvalidValue => eprintln!("Invalid value for field"),
                }
                continue;
            }
        };

        if let Err(err) = app.process_statement(statement) {
            match err {
                StatementProcessingError::UnknownMetaCommand => {
                    eprintln!("Unrecognized Meta command")
                }
            }
        }
    }
}

enum Statement {
    Meta(String),
    Insert(Row),
}

enum StatementPreparationError {
    UnrecognizedCommand,
    MissingField,
    InvalidValue,
}

enum StatementProcessingError {
    UnknownMetaCommand,
}

#[derive(Default)]
struct App {
    table: Table,
}

impl App {
    fn prepare_statement(command: &str) -> Result<Statement, StatementPreparationError> {
        if command.starts_with('.') {
            return App::prepare_meta_statement(&command[1..]);
        }

        const INSERT: &'static str = "insert ";
        if command.starts_with(INSERT) {
            return App::prepare_insert_statement(&command[INSERT.len()..]);
        }

        return Err(StatementPreparationError::UnrecognizedCommand);
    }

    fn prepare_meta_statement(meta_command: &str) -> Result<Statement, StatementPreparationError> {
        return Ok(Statement::Meta(meta_command.to_owned()));
    }

    fn prepare_insert_statement(data: &str) -> Result<Statement, StatementPreparationError> {
        let splitted: Vec<&str> = data.splitn(3, ' ').collect();
        let id = splitted
            .get(0)
            .ok_or(StatementPreparationError::MissingField)?
            .parse::<u32>()
            .map_err(|_| StatementPreparationError::InvalidValue)?;
        let username = splitted
            .get(1)
            .ok_or(StatementPreparationError::MissingField)?
            .to_string();
        let email = splitted
            .get(2)
            .ok_or(StatementPreparationError::MissingField)?
            .to_string();
        return Ok(Statement::Insert(Row {
            id,
            username,
            email,
        }));
    }

    fn process_statement(&mut self, statement: Statement) -> Result<(), StatementProcessingError> {
        match statement {
            Statement::Meta(command) => match command.as_str() {
                "exit" => {
                    println!("Bye!");
                    exit(0);
                }
                "show" => {
                    for row in self.table.iter() {
                        println!("( {}, {}, {} )", row.id, row.username, row.email);
                    }
                }
                _ => return Err(StatementProcessingError::UnknownMetaCommand),
            },
            Statement::Insert(row) => {
                self.table.insert(row);
            }
        }
        Ok(())
    }
}

// Sizes
const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

// Offsets
const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;
const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

struct Row {
    id: u32,
    username: String,
    email: String,
}

struct Page {
    data: [u8; PAGE_SIZE],
}

impl Page {
    pub fn new() -> Self {
        Self {
            data: [0u8; PAGE_SIZE],
        }
    }
}

#[derive(Default)]
struct Table {
    row_count: usize,
    pages: Vec<Page>,
}

struct TableIterator<'table> {
    inner: &'table Table,
    read_count: usize,
}

impl<'table> TableIterator<'table> {
    fn new(inner: &'table Table) -> Self {
        Self {
            inner,
            read_count: 0,
        }
    }
}

impl<'table> Iterator for TableIterator<'table> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read_count >= self.inner.row_count {
            return None;
        }
        let page_idx = self.read_count / ROWS_PER_PAGE;
        let page = self.inner.pages.get(page_idx)?;
        // Remaining rows in current page
        let row_offset = self.read_count % ROWS_PER_PAGE;
        // How much bytes we what to skip
        let byte_offset = row_offset * ROW_SIZE;
        let row: &[u8; ROW_SIZE] = &page.data[byte_offset..byte_offset + ROW_SIZE]
            .try_into()
            .expect("Unable to get row from page");

        self.read_count += 1;

        return Some(Row::deserialize(row));
    }
}

impl Table {
    fn insert(&mut self, row: Row) {
        // index of the page we what to insert
        let page_idx = self.row_count / ROWS_PER_PAGE;
        // get the page if the page does not exist create one.
        let page = if let Some(page) = self.pages.get_mut(page_idx) {
            page
        } else {
            self.pages.push(Page::new());
            self.pages
                .get_mut(page_idx)
                .expect("Page index in miscalculated.")
        };
        // Remaining rows in current page
        let row_offset = self.row_count % ROWS_PER_PAGE;
        // How much bytes we what to skip
        let byte_offset = row_offset * ROW_SIZE;
        let row = row.serialize();
        page.data[byte_offset..byte_offset + ROW_SIZE].copy_from_slice(&row);
        self.row_count += 1;
    }

    fn iter(&self) -> TableIterator {
        return TableIterator::new(self);
    }
}

trait NBytes {
    fn get_n_bytes<const N: usize>(&self) -> [u8; N];
}

impl NBytes for &[u8] {
    fn get_n_bytes<const N: usize>(&self) -> [u8; N] {
        let mut result = [0u8; N]; // Create a zero-initialized array
        let len = self.len().min(N); // Determine the number of elements to copy
        result[..len].copy_from_slice(&self[..len]); // Copy the elements
        result
    }
}

impl Row {
    fn serialize(self) -> [u8; ROW_SIZE] {
        let id: [u8; ID_SIZE] = self.id.to_le_bytes();
        let username: [u8; USERNAME_SIZE] = self.username.as_bytes().get_n_bytes();
        let email: [u8; EMAIL_SIZE] = self.email.as_bytes().get_n_bytes();

        let mut row = [0u8; ROW_SIZE];
        row[ID_OFFSET..ID_OFFSET + ID_SIZE].copy_from_slice(&id);
        row[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE].copy_from_slice(&username);
        row[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE].copy_from_slice(&email);
        return row;
    }

    fn deserialize(source: &[u8; ROW_SIZE]) -> Row {
        let id: &[u8; ID_SIZE] = &source[ID_OFFSET..ID_OFFSET + ID_SIZE]
            .try_into()
            .expect("Unable to deserialize id from source");
        let username: &[u8; USERNAME_SIZE] = &source
            [USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE]
            .try_into()
            .expect("Unable to deserialize email from source");
        let email: &[u8; EMAIL_SIZE] = &source[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE]
            .try_into()
            .expect("Unable to deserialize email from source");
        // TODO: need to remove 0 padding
        Row {
            id: u32::from_le_bytes(id.clone()),
            username: String::from_utf8_lossy(username).to_string(),
            email: String::from_utf8_lossy(email).to_string(),
        }
    }
}
