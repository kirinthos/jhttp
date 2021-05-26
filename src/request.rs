//! Module that defines that HTTP Request object
//!
//!

use std::collections::HashMap;
use std::error::Error;

use super::util;

/// HttpRequest struct that contains the raw request
/// headers and body of an incoming request
pub struct HttpRequest {
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    fn from_str(request_body: &str) -> Result<HttpRequest, Box<dyn Error>> {
        let lines: Vec<&str> = request_body.split('\n').collect();
        let (method, rest) = match lines.is_empty() {
            true => Err("Request does not contain any content"),
            false => Ok((&lines[0], &lines[1..])),
        }?;

        let body_location = rest.iter().position(|&v| v.is_empty());

        let (header_lines, body_lines) = match body_location {
            Some(v) => (&rest[..v], Some(&rest[(v + 1)..])),
            None => (rest, None),
        };

        println!("{:?}\n{:?}\n{:?}", method, header_lines, body_lines);
        let headers: HashMap<String, String> = header_lines
            .iter()
            .map(|v| {
                v.split_once(":")
                    .map(|(k, v)| (k.to_owned(), v.trim().to_owned()))
            })
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect();

        let body = String::from("body");
        Ok(HttpRequest { headers, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_new() {
        let http =
            HttpRequest::from_str("POST /somewhere HTTP/1.1\nHello: There\n\ntempting: isn't it");
        assert!(false);
    }
}
