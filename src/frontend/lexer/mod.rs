pub(crate) mod keyword;
pub(crate) mod literal;
pub(crate) mod reader;
pub(crate) mod symbol;
pub(crate) mod token;

use crate::util::layer::Layer;
use keyword::Keyword;
use literal::Literal;
use reader::{Character, Position};
use symbol::Symbol;
pub(crate) use token::Token;
use token::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LexerError {
    UnTerminatedStringLiteral(Position),
    NumberExceededSize(Position),
    IllegalCharacter(char, Position),
}

pub(crate) struct Lexer<CharacterLayer>
where
    CharacterLayer: Layer<Character, ()>,
{
    characters: CharacterLayer,
}

impl<CharacterLayer> Lexer<CharacterLayer>
where
    CharacterLayer: Layer<Character, ()>,
{
    pub(crate) fn new(characters: CharacterLayer) -> Self {
        Self { characters }
    }

    fn expect(&mut self, expected: char) -> bool {
        if let Some(actual) = self.next_character() {
            if expected == actual.value {
                return true;
            }
            self.characters.rewind(actual);
        }
        false
    }

    fn next_character(&mut self) -> Option<Character> {
        self.characters.next()?.ok()
    }
}

impl<CharacterLayer> Iterator for Lexer<CharacterLayer>
where
    CharacterLayer: Layer<Character, ()>,
{
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.next_character() {
            let token = match ch.value {
                ' ' => {
                    continue;
                }
                '\n' => {
                    continue;
                }
                '\r' => {
                    let Some(ch) = self.next_character() else {
                        continue;
                    };
                    if ch.value == '\n' {
                        continue;
                    }
                    self.characters.rewind(ch);
                    continue;
                }

                // TODO: handle multi line strings
                '\"' => {
                    let mut word = String::new();
                    while let Some(next_ch) = self.next_character() {
                        if next_ch.value == '\"' {
                            return Some(Ok(Token::new(
                                TokenKind::Literal(word.into()),
                                ch.position,
                            )));
                        } else if next_ch.value == '\n' || next_ch.value == '\r' {
                            return Some(Err(LexerError::UnTerminatedStringLiteral(ch.position)));
                        }
                        word.push(next_ch.value);
                    }
                    return Some(Err(LexerError::UnTerminatedStringLiteral(ch.position)));
                }
                '\'' => {
                    let mut word = String::new();
                    while let Some(next_ch) = self.next_character() {
                        if next_ch.value == '\'' {
                            return Some(Ok(Token::new(
                                TokenKind::Literal(word.into()),
                                ch.position,
                            )));
                        } else if next_ch.value == '\n' || next_ch.value == '\r' {
                            return Some(Err(LexerError::UnTerminatedStringLiteral(ch.position)));
                        }
                        word.push(next_ch.value);
                    }
                    return Some(Err(LexerError::UnTerminatedStringLiteral(ch.position)));
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

                '+' if self.expect('=') => Token::from_symbol(Symbol::PlusEquals, ch.position),
                '+' => Token::from_symbol(Symbol::Plus, ch.position),
                '-' if self.expect('=') => Token::from_symbol(Symbol::MinusEquals, ch.position),
                '-' => Token::from_symbol(Symbol::Minus, ch.position),
                '*' if self.expect('=') => Token::from_symbol(Symbol::StarEquals, ch.position),
                '*' => Token::from_symbol(Symbol::Star, ch.position),
                '/' if self.expect('=') => Token::from_symbol(Symbol::DivideEquals, ch.position),
                '/' => Token::from_symbol(Symbol::Divide, ch.position),
                '%' if self.expect('=') => Token::from_symbol(Symbol::PercentEquals, ch.position),
                '%' => Token::from_symbol(Symbol::Percent, ch.position),
                '=' if self.expect('=') => Token::from_symbol(Symbol::Equals, ch.position),
                '=' => Token::from_symbol(Symbol::Equal, ch.position),
                '!' if self.expect('=') => Token::from_symbol(Symbol::NotEquals, ch.position),
                '!' => Token::from_symbol(Symbol::Not, ch.position),
                '<' if self.expect('=') => {
                    Token::from_symbol(Symbol::LessThanOrEquals, ch.position)
                }
                '<' => Token::from_symbol(Symbol::LessThan, ch.position),
                '>' if self.expect('=') => {
                    Token::from_symbol(Symbol::GreaterThanOrEquals, ch.position)
                }
                '>' => Token::from_symbol(Symbol::GreaterThan, ch.position),
                '&' if self.expect('&') => Token::from_symbol(Symbol::And, ch.position),
                '&' => Token::from_symbol(Symbol::BitAnd, ch.position),
                '|' if self.expect('|') => Token::from_symbol(Symbol::Or, ch.position),
                '|' => Token::from_symbol(Symbol::BitOr, ch.position),
                '^' => Token::from_symbol(Symbol::BitXor, ch.position),
                '~' => Token::from_symbol(Symbol::BitNot, ch.position),

                // skip comments
                '#' => {
                    while let Some(ch) = self.next_character() {
                        if ch.value == '\n' {
                            break;
                        }
                    }
                    continue;
                }
                // Identifiers and keywords
                ch_val if ch_val.is_ascii_alphabetic() || ch_val == '_' => {
                    let mut word = String::from(ch_val);
                    while let Some(next_ch) = self.next_character() {
                        if next_ch.value == ' ' {
                            break;
                        }
                        if !next_ch.value.is_ascii_alphanumeric() && next_ch.value != '_' {
                            self.characters.rewind(next_ch);
                            break;
                        }
                        word.push(next_ch.value);
                    }
                    if let Some(keyword) = Keyword::get_keyword_kind(&word) {
                        Token::new(TokenKind::Keyword(keyword), ch.position)
                    } else if let Some(literal) = Literal::get_literal(&word) {
                        Token::new(TokenKind::Literal(literal), ch.position)
                    } else {
                        Token::new(TokenKind::Ident(word), ch.position)
                    }
                }

                // Numeric Literals
                ch_val if ch_val.is_ascii_digit() => {
                    let mut number_as_string = String::from(ch_val);

                    // Collect the integer part of the number.
                    while let Some(next_ch) = self.next_character() {
                        if next_ch.value.is_ascii_digit() {
                            number_as_string.push(next_ch.value);
                        } else if next_ch.value == '.' {
                            // Handle fractional part
                            number_as_string.push('.');
                            while let Some(fraction_ch) = self.next_character() {
                                if fraction_ch.value.is_ascii_digit() {
                                    number_as_string.push(fraction_ch.value);
                                } else {
                                    self.characters.rewind(fraction_ch);
                                    break;
                                }
                            }
                            // Parse as a floating-point number.
                            return Some(match number_as_string.parse::<f64>() {
                                Ok(number) => {
                                    Ok(Token::new(TokenKind::Literal(number.into()), ch.position))
                                }
                                Err(_) => Err(LexerError::NumberExceededSize(ch.position)),
                            });
                        } else {
                            self.characters.rewind(next_ch);
                            break;
                        }
                    }

                    // Parse as an integer if no fractional part.
                    match number_as_string.parse::<i32>() {
                        Ok(number) => Token::new(TokenKind::Literal(number.into()), ch.position),
                        Err(_) => return Some(Err(LexerError::NumberExceededSize(ch.position))),
                    }
                }
                value => {
                    return Some(Err(LexerError::IllegalCharacter(value, ch.position)));
                }
            };
            return Some(Ok(token));
        }
        None
    }
}
