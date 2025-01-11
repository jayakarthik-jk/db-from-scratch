pub(crate) mod keyword;
pub(crate) mod literal;
pub(crate) mod reader;
pub(crate) mod symbol;
pub(crate) mod token;

use keyword::Keyword;
use literal::Literal;
use reader::{CharacterIterator, Position};
use symbol::Symbol;
pub(crate) use token::Token;
use token::TokenKind;

use std::{collections::VecDeque, io::Read};

#[derive(Debug, Clone, Copy)]
pub(crate) enum LexerError {
    UnTerminatedStringLiteral(Position),
    NumberExceededSize(Position),
    IllegalCharacter(char, Position),
    EOF,
}

pub(crate) struct Lexer<T>
where
    T: Read,
{
    cursor: CharacterIterator<T>,
    buffer: VecDeque<char>,
}

impl<T> Lexer<T>
where
    T: Read,
{
    pub(crate) fn new(input: T) -> Self {
        Self {
            cursor: CharacterIterator::new(input),
            buffer: VecDeque::with_capacity(2),
        }
    }

    fn get_next(&mut self) -> Option<char> {
        self.buffer.pop_back().or_else(|| self.cursor.next())
    }

    fn rewind(&mut self, ch: char) {
        self.buffer.push_back(ch);
    }

    fn expect(&mut self, expected: char) -> bool {
        if let Some(actual) = self.get_next() {
            if expected == actual {
                return true;
            }
            self.rewind(actual);
        }
        false
    }
}

impl<T> Lexer<T>
where
    T: Read,
{
    pub(crate) fn next(&mut self) -> Result<Token, LexerError> {
        while let Some(ch) = self.get_next() {
            let token = match ch {
                ' ' => {
                    continue;
                }
                '\n' => {
                    continue;
                }
                '\r' => {
                    let Some(ch) = self.get_next() else { continue };
                    if ch == '\n' {
                        continue;
                    }
                    self.rewind(ch);
                    continue;
                }

                // TODO: handle multi line strings
                '\"' => {
                    let position = self.cursor.position;
                    let mut word = String::new();
                    while let Some(next_ch) = self.get_next() {
                        if next_ch == '\"' {
                            return Ok(Token::new(TokenKind::Literal(word.into()), position));
                        } else if next_ch == '\n' || next_ch == '\r' {
                            return Err(LexerError::UnTerminatedStringLiteral(position));
                        }
                        word.push(next_ch);
                    }
                    return Err(LexerError::UnTerminatedStringLiteral(position));
                }
                '\'' => {
                    let position = self.cursor.position;
                    let mut word = String::new();
                    while let Some(next_ch) = self.get_next() {
                        if next_ch == '\'' {
                            return Ok(Token::new(TokenKind::Literal(word.into()), position));
                        } else if next_ch == '\n' || next_ch == '\r' {
                            return Err(LexerError::UnTerminatedStringLiteral(position));
                        }
                        word.push(next_ch);
                    }
                    return Err(LexerError::UnTerminatedStringLiteral(position));
                }

                // Symbols
                '(' => Token::from_symbol(Symbol::OpenParanthesis, self.cursor.position),
                ')' => Token::from_symbol(Symbol::CloseParanthesis, self.cursor.position),
                '[' => Token::from_symbol(Symbol::OpenSquareBracket, self.cursor.position),
                ']' => Token::from_symbol(Symbol::CloseSquareBracket, self.cursor.position),
                '{' => Token::from_symbol(Symbol::OpenCurlyBracket, self.cursor.position),
                '}' => Token::from_symbol(Symbol::CloseCurlyBracket, self.cursor.position),
                ',' => Token::from_symbol(Symbol::Comma, self.cursor.position),
                ';' => Token::from_symbol(Symbol::Semicolon, self.cursor.position),

                '+' if self.expect('=') => {
                    Token::from_symbol(Symbol::PlusEquals, self.cursor.position)
                }
                '+' => Token::from_symbol(Symbol::Plus, self.cursor.position),
                '-' if self.expect('=') => {
                    Token::from_symbol(Symbol::MinusEquals, self.cursor.position)
                }
                '-' => Token::from_symbol(Symbol::Minus, self.cursor.position),
                '*' if self.expect('=') => {
                    Token::from_symbol(Symbol::StarEquals, self.cursor.position)
                }
                '*' => Token::from_symbol(Symbol::Star, self.cursor.position),
                '/' if self.expect('=') => {
                    Token::from_symbol(Symbol::DivideEquals, self.cursor.position)
                }
                '/' => Token::from_symbol(Symbol::Divide, self.cursor.position),
                '%' if self.expect('=') => {
                    Token::from_symbol(Symbol::PercentEquals, self.cursor.position)
                }
                '%' => Token::from_symbol(Symbol::Percent, self.cursor.position),
                '=' if self.expect('=') => Token::from_symbol(Symbol::Equals, self.cursor.position),
                '=' => Token::from_symbol(Symbol::Equal, self.cursor.position),
                '!' if self.expect('=') => {
                    Token::from_symbol(Symbol::NotEquals, self.cursor.position)
                }
                '!' => Token::from_symbol(Symbol::Not, self.cursor.position),
                '<' if self.expect('=') => {
                    Token::from_symbol(Symbol::LessThanOrEquals, self.cursor.position)
                }
                '<' => Token::from_symbol(Symbol::LessThan, self.cursor.position),
                '>' if self.expect('=') => {
                    Token::from_symbol(Symbol::GreaterThanOrEquals, self.cursor.position)
                }
                '>' => Token::from_symbol(Symbol::GreaterThan, self.cursor.position),
                '&' if self.expect('&') => Token::from_symbol(Symbol::And, self.cursor.position),
                '&' => Token::from_symbol(Symbol::BitAnd, self.cursor.position),
                '|' if self.expect('|') => Token::from_symbol(Symbol::Or, self.cursor.position),
                '|' => Token::from_symbol(Symbol::BitOr, self.cursor.position),
                '^' => Token::from_symbol(Symbol::BitXor, self.cursor.position),
                '~' => Token::from_symbol(Symbol::BitNot, self.cursor.position),

                // skip comments
                '#' => {
                    while let Some(ch) = self.get_next() {
                        if ch == '\n' {
                            break;
                        }
                    }
                    continue;
                }
                // Identifiers and keywords
                ch if ch.is_ascii_alphabetic() || ch == '_' => {
                    let position = self.cursor.position;
                    let mut word = String::from(ch);
                    while let Some(next_ch) = self.get_next() {
                        if next_ch == ' ' {
                            break;
                        }
                        if !next_ch.is_ascii_alphanumeric() && next_ch != '_' {
                            self.rewind(next_ch);
                            break;
                        }
                        word.push(next_ch);
                    }
                    if let Some(keyword) = Keyword::get_keyword_kind(&word) {
                        Token::new(TokenKind::Keyword(keyword), position)
                    } else if let Some(literal) = Literal::get_literal(&word) {
                        Token::new(TokenKind::Literal(literal), position)
                    } else {
                        Token::new(TokenKind::Identifier(word), position)
                    }
                }

                // Numeric Literals
                ch if ch.is_ascii_digit() => {
                    let position = self.cursor.position;
                    let mut number_as_string = String::from(ch);

                    // Collect the integer part of the number.
                    while let Some(next_ch) = self.get_next() {
                        if next_ch.is_ascii_digit() {
                            number_as_string.push(next_ch);
                        } else if next_ch == '.' {
                            // Handle fractional part
                            number_as_string.push('.');
                            while let Some(fraction_ch) = self.get_next() {
                                if fraction_ch.is_ascii_digit() {
                                    number_as_string.push(fraction_ch);
                                } else {
                                    self.rewind(fraction_ch);
                                    break;
                                }
                            }
                            // Parse as a floating-point number.
                            return match number_as_string.parse::<f64>() {
                                Ok(number) => {
                                    Ok(Token::new(TokenKind::Literal(number.into()), position))
                                }
                                Err(_) => Err(LexerError::NumberExceededSize(position)),
                            };
                        } else {
                            self.rewind(next_ch);
                            break;
                        }
                    }

                    // Parse as an integer if no fractional part.
                    match number_as_string.parse::<i32>() {
                        Ok(number) => Token::new(TokenKind::Literal(number.into()), position),
                        Err(_) => return Err(LexerError::NumberExceededSize(position)),
                    }
                }
                value => {
                    return Err(LexerError::IllegalCharacter(value, self.cursor.position));
                }
            };
            return Ok(token);
        }
        Err(LexerError::EOF)
    }
}
