pub(crate) mod keyword;
pub(crate) mod literal;
pub(crate) mod reader;
pub(crate) mod symbol;
pub(crate) mod token;

use crate::error::DBError;
use keyword::Keyword;
use literal::Literal;
use reader::{Character, Position};
use std::iter::Peekable;
use symbol::Symbol;
pub(crate) use token::Token;
use token::{Span, TokenKind};

pub(crate) struct Lexer<Characters>
where
    Characters: Iterator<Item = Character>,
{
    characters: Peekable<Characters>,
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = Character>,
{
    type Item = Result<Token, DBError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.next_character() {
            let token = match ch.value {
                ch if ch.is_whitespace() => {
                    continue;
                }
                // Symbols
                '(' => Token::from_symbol(Symbol::OpenParanthesis, ch.position),

                ')' => Token::from_symbol(Symbol::CloseParanthesis, ch.position),

                '[' => Token::from_symbol(Symbol::OpenSquareBracket, ch.position),

                ']' => Token::from_symbol(Symbol::CloseSquareBracket, ch.position),

                '{' => Token::from_symbol(Symbol::OpenCurlyBracket, ch.position),

                '}' => Token::from_symbol(Symbol::CloseCurlyBracket, ch.position),

                ',' => Token::from_symbol(Symbol::Comma, ch.position),

                ';' => Token::from_symbol(Symbol::Semicolon, ch.position),

                '+' if self.if_next('=') => Token::from_symbol(Symbol::PlusEquals, ch.position),

                '+' => Token::from_symbol(Symbol::Plus, ch.position),

                '-' if self.if_next('=') => Token::from_symbol(Symbol::MinusEquals, ch.position),

                '-' => Token::from_symbol(Symbol::Minus, ch.position),

                '*' if self.if_next('=') => Token::from_symbol(Symbol::StarEquals, ch.position),

                '*' => Token::from_symbol(Symbol::Star, ch.position),

                '/' if self.if_next('=') => Token::from_symbol(Symbol::DivideEquals, ch.position),

                '/' => Token::from_symbol(Symbol::Divide, ch.position),

                '%' if self.if_next('=') => Token::from_symbol(Symbol::PercentEquals, ch.position),

                '%' => Token::from_symbol(Symbol::Percent, ch.position),

                '=' if self.if_next('=') => Token::from_symbol(Symbol::Equals, ch.position),

                '=' => Token::from_symbol(Symbol::Equal, ch.position),

                '!' if self.if_next('=') => Token::from_symbol(Symbol::NotEquals, ch.position),

                '!' => Token::from_symbol(Symbol::Not, ch.position),

                '<' if self.if_next('=') => {
                    Token::from_symbol(Symbol::LessThanOrEquals, ch.position)
                }
                '<' => Token::from_symbol(Symbol::LessThan, ch.position),

                '>' if self.if_next('=') => {
                    Token::from_symbol(Symbol::GreaterThanOrEquals, ch.position)
                }
                '>' => Token::from_symbol(Symbol::GreaterThan, ch.position),

                '&' if self.if_next('&') => Token::from_symbol(Symbol::And, ch.position),

                '&' => Token::from_symbol(Symbol::BitAnd, ch.position),

                '|' if self.if_next('|') => Token::from_symbol(Symbol::Or, ch.position),

                '|' => Token::from_symbol(Symbol::BitOr, ch.position),

                '^' => Token::from_symbol(Symbol::BitXor, ch.position),

                '~' => Token::from_symbol(Symbol::BitNot, ch.position),

                '\"' | '\'' => return Some(self.collect_string_literal(ch)),

                // skip comments
                '#' => {
                    self.characters
                        .find(|ch| ch.value != '\n');
                    continue;
                }

                // Numeric Literals
                ch_val if ch_val.is_ascii_digit() => return Some(self.collect_numeric_literal(ch)),
                // Identifiers and keywords
                ch_val if Self::is_valid_ident(ch_val) => self.collect_ident(ch),

                value => {
                    return Some(Err(DBError::IllegalCharacter(value, ch.position)));
                }
            };
            return Some(Ok(token));
        }
        None
    }
}

impl<T> Lexer<T>
where
    T: Iterator<Item = Character>,
{
    pub(crate) fn new(characters: T) -> Self {
        Self {
            characters: characters.peekable(),
        }
    }

    fn if_next(&mut self, expected: char) -> bool {
        self.characters.next_if(|ch| ch.value == expected).is_some()
    }

    fn next_character(&mut self) -> Option<Character> {
        self.characters.next()
    }

    fn collect_rest_of_number(&mut self, initial_char: Character) -> (String, Position) {
        let mut number_as_string = String::from(initial_char.value);
        let mut last_position = initial_char.position;

        while let Some(next_ch) = self
            .characters
            .next_if(|next_ch| next_ch.value.is_ascii_digit())
        {
            number_as_string.push(next_ch.value);
            last_position = next_ch.position;
        }

        (number_as_string, last_position)
    }

    pub(crate) fn collect_string_literal(
        &mut self,
        enclosing: Character,
    ) -> Result<Token, DBError> {
        let mut word = String::new();
        println!("Collecting string literal starting with: {}", enclosing.value);
        while let Some(next_ch) = self
            .characters
            .next_if(|next_ch| next_ch.value != enclosing.value && next_ch.value != '\n')
        {
            println!("Collecting string literal: {}", next_ch.value);
            word.push(next_ch.value);
        }
        if let Some(last) = self.characters.next_if(|ch| ch.value == enclosing.value) {
            return Ok(Token::new(
                TokenKind::Literal(word.into()),
                Span {
                    start: enclosing.position,
                    end: last.position,
                },
            ));
        }
        // If we reach here, it means we didn't find a closing quote.
        Err(DBError::UnTerminatedStringLiteral(enclosing.position))
    }

    pub(crate) fn collect_numeric_literal(
        &mut self,
        initial_char: Character,
    ) -> Result<Token, DBError> {
        let (number_as_string, last_position) = self.collect_rest_of_number(initial_char);

        if let Some(next_ch) = self.characters.next_if(|ch| ch.value == '.') {
            // Collect the fractional part of the number.
            let (fraction_ch, last_position) = self.collect_rest_of_number(next_ch);
            let full_number = format!("{}.{}", number_as_string, fraction_ch);

            // Parse as a floating-point number.
            match full_number.parse::<f64>() {
                Ok(number) => Ok(Token::new(
                    TokenKind::Literal(number.into()),
                    Span {
                        start: initial_char.position,
                        end: last_position,
                    },
                )),
                Err(_) => Err(DBError::NumberExceededSize(initial_char.position)),
            }
        } else {
            // Parse as an integer if no fractional part.
            match number_as_string.parse::<i32>() {
                Ok(number) => Ok(Token::new(
                    TokenKind::Literal(number.into()),
                    Span {
                        start: initial_char.position,
                        end: last_position,
                    },
                )),
                Err(_) => Err(DBError::NumberExceededSize(initial_char.position)),
            }
        }
    }

    pub(crate) fn collect_ident(&mut self, ch: Character) -> Token {
        let mut word = String::from(ch.value);
        let mut last_position = ch.position;

        while let Some(next_ch) = self
            .characters
            .next_if(|next_ch| Self::is_valid_ident(next_ch.value))
        {
            word.push(next_ch.value);
            last_position = next_ch.position;
        }

        if let Some(keyword) = Keyword::get_keyword_kind(&word) {
            Token::from_keyword(keyword, ch.position)
        } else if let Some(literal) = Literal::get_literal(&word) {
            Token::new(
                TokenKind::Literal(literal),
                Span {
                    start: ch.position,
                    end: last_position,
                },
            )
        } else {
            Token::new(
                TokenKind::Ident(word),
                Span {
                    start: ch.position,
                    end: last_position,
                },
            )
        }
    }

    fn is_valid_ident(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_' || ch.is_ascii_digit()
    }
}
