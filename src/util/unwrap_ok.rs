#[macro_export]
macro_rules! unwrap_ok {
    ($expr:expr) => {
        match $expr {
            None => return None,
            Some(Err(e)) => return Some(Err(e)),
            Some(Ok(val)) => val,
        }
    };
}

