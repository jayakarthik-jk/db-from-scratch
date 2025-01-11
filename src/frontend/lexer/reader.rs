use std::io::{BufReader, Read};

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Position {
    row: usize,
    col: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { row: 1, col: 0 }
    }
}

/// iterates one character at a time
/// used in lexer
pub(crate) struct CharacterIterator<T>
where
    T: Read,
{
    reader: BufReader<T>,
    buffer: [u8; 1024],
    read: usize,
    pub(crate) position: Position,
}

impl<T> CharacterIterator<T>
where
    T: Read,
{
    pub(crate) fn new(input: T) -> Self {
        Self {
            reader: BufReader::new(input),
            buffer: [0; 1024],
            read: 1024,
            position: Position::default(),
        }
    }
}

impl<T> Iterator for CharacterIterator<T>
where
    T: Read,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read >= self.buffer.len() {
            self.read = 0;
            if self.reader.read(&mut self.buffer).ok()? == 0 {
                return None;
            }
        }

        let ch = self.buffer[self.read] as char;

        self.read += 1;

        match ch {
            '\0' => return None,
            '\n' => {
                self.position.row += 1;
                self.position.col = 0;
            }
            _ => {
                self.position.col += 1;
            }
        }
        return Some(ch);
    }
}
