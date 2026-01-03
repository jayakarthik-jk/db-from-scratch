pub(crate) mod keyword;
pub(crate) mod literal;
pub(crate) mod source;
pub(crate) mod symbol;
pub(crate) mod token;

use crate::{
    common::{
        peekable_ext::ConsumeIf,
        position::{Position, Span},
    },
    error::DBError,
};
use keyword::Keyword;
use literal::Literal;
use source::Atom;
use std::iter::Peekable;
use symbol::Symbol;
pub(crate) use token::Token;
use token::TokenKind;

pub(crate) struct Lexer<Characters>
where
    Characters: Iterator<Item = Atom>,
{
    atoms: Peekable<Characters>,
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = Atom>,
{
    type Item = Result<Token, DBError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.next_atom() {
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

                '\"' | '\'' => match self.collect_string_literal(ch) {
                    Ok(token) => token,
                    err => return Some(err),
                },

                // skip comments
                '#' => {
                    self.atoms.find(|ch| ch.value != '\n');
                    continue;
                }

                // Numeric Literals
                ch_val if ch_val.is_ascii_digit() => match self.collect_numeric_literal(ch) {
                    Ok(token) => token,
                    err => return Some(err),
                },
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
    T: Iterator<Item = Atom>,
{
    pub(crate) fn new(atoms: T) -> Self {
        Self {
            atoms: atoms.peekable(),
        }
    }

    fn if_next(&mut self, expected: char) -> bool {
        self.atoms.if_consume(|ch| ch.value == expected)
    }

    fn next_atom(&mut self) -> Option<Atom> {
        self.atoms.next()
    }

    fn collect_rest_of_number(&mut self, initial_char: Atom) -> (String, Position) {
        let mut number_as_string = String::from(initial_char.value);
        let mut last_position = initial_char.position;

        while let Some(next_ch) = self
            .atoms
            .consume_if(|next_ch| next_ch.value.is_ascii_digit())
        {
            number_as_string.push(next_ch.value);
            last_position = next_ch.position;
        }

        (number_as_string, last_position)
    }

    pub(crate) fn collect_string_literal(&mut self, enclosing: Atom) -> Result<Token, DBError> {
        let mut word = String::new();
        while let Some(next_ch) = self
            .atoms
            .consume_if(|next_ch| next_ch.value != enclosing.value && next_ch.value != '\n')
        {
            word.push(next_ch.value);
        }
        if let Some(last) = self.atoms.consume_if(|ch| ch.value == enclosing.value) {
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

    pub(crate) fn collect_numeric_literal(&mut self, initial_char: Atom) -> Result<Token, DBError> {
        let (number_as_string, last_position) = self.collect_rest_of_number(initial_char);

        if let Some(next_ch) = self.atoms.consume_if(|ch| ch.value == '.') {
            // Collect the fractional part of the number.
            let (fraction_ch, last_position) = self.collect_rest_of_number(next_ch);
            if fraction_ch.len() == 1 {
                // If the fractional part is just a single character, it's invalid.
                return Err(DBError::UnterminatedNumberLiteral(next_ch.position));
            }
            let full_number = format!("{}{}", number_as_string, fraction_ch);

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

    pub(crate) fn collect_ident(&mut self, ch: Atom) -> Token {
        let mut word = String::from(ch.value);
        let mut last = ch;

        while let Some(next_ch) = self
            .atoms
            .consume_if(|next_ch| Self::is_valid_ident(next_ch.value))
        {
            word.push(next_ch.value);
            last = next_ch;
        }

        let span = Span {
            start: ch.position,
            end: last.position,
        };

        if let Some(keyword) = Keyword::get_keyword_kind(&word) {
            Token::new(TokenKind::Keyword(keyword), span)
        } else if let Some(literal) = Literal::get_literal(&word) {
            Token::new(TokenKind::Literal(literal), span)
        } else {
            Token::new(TokenKind::Ident(word), span)
        }
    }

    fn is_valid_ident(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_' || ch.is_ascii_digit()
    }
}
