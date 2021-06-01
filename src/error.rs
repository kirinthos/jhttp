//! Module that defines the various errors the server emits

use std::error::Error;

#[derive(Debug)]
pub enum ServerError {
    NotFound,
    NotImplemented,
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}
