#[macro_export]
macro_rules! unwrap_ident {
    ($expr:expr) => {{
        use crate::frontend::lexer::token::TokenKind;
        use crate::frontend::parser::error::ParserError;
        use crate::frontend::parser::error::ParserErrorKind;

        let token = unwrap_ok!($expr);
        let ident = match token.kind {
            TokenKind::Identifier(ident) => ident,
            _ => {
                return Some(Err(ParserError {
                    kind: ParserErrorKind::Unexpected(token),
                }))
            }
        };
        Some(Ok(ident))
    }};
}
