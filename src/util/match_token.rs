#[macro_export]
macro_rules! match_token {
    ($expr:expr, $pat:pat) => {{
        use crate::frontend::parser::error::ParserError;
        use crate::frontend::parser::error::ParserErrorKind;
        use crate::unwrap_ok;

        let token = unwrap_ok!($expr);
        match token.kind {
            $pat => Some(Ok::<_, ParserError>(())),
            _ => Some(Err(ParserErrorKind::Unexpected(token).into())),
        }
    }};

    ($expr:expr, $pat:pat, $ok:ident) => {{
        use crate::frontend::parser::error::ParserErrorKind;
        use crate::unwrap_ok;

        let token = unwrap_ok!($expr);
        match token.kind {
            $pat => Some(Ok($ok)),
            _ => Some(Err(ParserErrorKind::Unexpected(token).into())),
        }
    }};
    ($expr:expr, $pat:pat, $ok:expr, $err:expr) => {{
        use crate::unwrap_ok;

        let token = unwrap_ok!($expr);
        match token {
            $pat => Some(Ok($ok)),
            _ => Some(Err($err)),
        }
    }};
}
