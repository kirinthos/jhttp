//! Module that defines that HTTP Request object
//!
//!

use std::collections::HashMap;

pub use util;

/// HttpRequest struct that contains the raw request
/// headers and body of an incoming request
pub struct HttpRequest {
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    fn from_str(request_body: &str) -> HttpRequest {
        let segments = util::partition_by(|v| v == "", request_body.split("\n"));
        let headers: HashMap<String, String> = ;
        let body = String::from("body");
        HttpRequest { headers, body }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_new() {
        let http = HttpRequest::from_str("Hello: There");
    }
}
