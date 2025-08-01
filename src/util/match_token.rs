#[macro_export]
macro_rules! match_token {
    ($expr:expr,$match:expr) => {
        let token = unwrap_ok!($expr);
        if $match != token.kind {
            return Some(Err(ParserErrorKind::UnexpectedToken {
                expected: $match,
                found: token,
            }
            .into()));
        }
    };
}
