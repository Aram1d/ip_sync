use std::error::Error;
pub fn map_prefixed_err(prefix: &str) -> impl Fn(Box<dyn Error>) -> Box<dyn Error> {
    let prefix = prefix.to_string();
    move |err: Box<dyn Error>| -> Box<dyn Error> {
        Box::<dyn Error>::from(format!("{} {}", prefix, err))
    }
}
