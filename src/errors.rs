use std::error::Error;
use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub struct InvalidHost;

impl Error for InvalidHost {}
impl Display for InvalidHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid host")
    }
}

#[derive(Debug)]
pub struct NotOk {
    pub message: String,
}

impl Error for NotOk {}
impl Display for NotOk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "proxy server rejected request and responded with: {}",
            self.message
        )
    }
}
