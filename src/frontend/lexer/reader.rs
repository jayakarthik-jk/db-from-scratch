use std::io::{BufReader, Read};

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Character {
    pub(crate) value: char,
    pub(crate) position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Position {
    pub row: usize,
    pub col: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { row: 1, col: 0 }
    }
}

// iterates over a source by char. asumes source is valid utf8

pub(crate) struct CharacterIterator<T: Read> {
    reader: BufReader<T>,
    buffer: Vec<u8>,
    cursor: usize,
    pub(crate) position: Position,
}

impl<T: Read> CharacterIterator<T> {
    pub(crate) fn new(input: T) -> Self {
        Self {
            reader: BufReader::new(input),
            buffer: Vec::new(),
            cursor: 0,
            position: Position::default(),
        }
    }

    fn refill_buffer(&mut self) -> bool {
        self.buffer.clear();
        self.cursor = 0;

        let mut temp = [0u8; 1024];
        let n = self.reader.read(&mut temp).expect("Read failed");

        if n == 0 {
            return false;
        }

        self.buffer.extend_from_slice(&temp[..n]);
        true
    }
}

impl<T: Read> Iterator for CharacterIterator<T> {
    type Item = Result<Character, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.buffer.len() && !self.refill_buffer() {
            return None;
        }

        let slice = &self.buffer[self.cursor..];
        match std::str::from_utf8(slice) {
            Ok(valid_str) => {
                let ch = valid_str.chars().next()?;
                let len = ch.len_utf8();

                self.cursor += len;

                match ch {
                    '\n' => {
                        self.position.row += 1;
                        self.position.col = 0;
                    }
                    _ => {
                        self.position.col += 1;
                    }
                }

                Some(Ok(Character {
                    value: ch,
                    position: self.position,
                }))
            }
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                if valid_up_to == 0 {
                    panic!("Invalid UTF-8 encountered in input");
                }

                // Safe slice of valid UTF-8
                let ch = std::str::from_utf8(&slice[..valid_up_to])
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();

                let len = ch.len_utf8();
                self.cursor += len;

                match ch {
                    '\n' => {
                        self.position.row += 1;
                        self.position.col = 0;
                    }
                    _ => {
                        self.position.col += 1;
                    }
                }

                Some(Ok(Character {
                    value: ch,
                    position: self.position,
                }))
            }
        }
    }
}
