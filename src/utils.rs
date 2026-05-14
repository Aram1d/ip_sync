use std::error::Error;
pub fn map_prefixed_err(prefix: &str) -> impl Fn(Box<dyn Error>) -> Box<dyn Error> {
    let prefix = prefix.to_string();
    move |err: Box<dyn Error>| -> Box<dyn Error> {
        Box::<dyn Error>::from(format!("{} {}", prefix, err))
    }
}

/// Walks the `source()` chain of an error and joins each level with ` -> `.
///
/// Many libraries (notably the AWS SDK) have terse top-level `Display`
/// implementations and store the real diagnostic in the source chain.
pub fn format_error_chain(err: &(dyn Error + 'static)) -> String {
    let mut msg = err.to_string();
    let mut source = err.source();
    while let Some(s) = source {
        msg.push_str(" -> ");
        msg.push_str(&s.to_string());
        source = s.source();
    }
    msg
}
