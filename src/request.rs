//! Module that defines that HTTP Request object
//!
//!

use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use crate::error::ServerError;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
}

impl FromStr for Method {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_uppercase()[..] {
            "GET" => Ok(Self::GET),
            _ => Err(ServerError::NotImplemented.into()),
        }
    }
}

/// HttpRequest struct that contains the raw request
/// headers and body of an incoming request
#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) protocol: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: String,
}

impl FromStr for HttpRequest {
    type Err = Box<dyn Error>;

    /// Produces an HttpRequest Result from a string.
    ///
    /// Clones the string data into the object and retains ownership
    fn from_str(request_body: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = request_body.lines().collect();
        let (method_line, rest) = match lines.is_empty() {
            true => Err("Request does not contain any content"),
            false => Ok((&lines[0], &lines[1..])),
        }?;

        let method_parts: Vec<&str> = method_line.split(" ").collect();
        let method_result = match method_parts.len() == 3 {
            true => Ok((
                Method::from_str(method_parts[0])?,
                method_parts[1].to_owned(),
                method_parts[2].to_owned(),
            )),
            false => Err(format!("Invalid method request line {}", method_line)),
        }?;

        let body_location = rest.iter().position(|&v| v.is_empty());

        let (header_lines, body_lines) = match body_location {
            Some(v) => (&rest[..v], Some(&rest[(v + 1)..])),
            None => (rest, None),
        };

        // claims a copy of the values
        let (method, path, protocol) = method_result;
        let headers: HashMap<String, String> = header_lines
            .iter()
            .map(|v| {
                v.split_once(":")
                    .map(|(k, v)| (k.to_owned(), v.trim_start().to_owned()))
            })
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect();

        let body = match body_lines {
            Some(b) => b.join("\n"),
            None => "".to_owned(),
        };

        Ok(HttpRequest {
            method,
            path,
            protocol,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::array::IntoIter;

    fn test_headers() -> HashMap<String, String> {
        IntoIter::new([
            ("User-Agent".to_owned(), "curl/7.76.1".to_owned()),
            ("Host".to_owned(), "127.0.0.1:8001".to_owned()),
        ])
        .collect()
    }

    #[test]
    fn test_from_str_simple() {
        let http_form_data = HttpRequest::from_str(
            "POST /somewhere HTTP/1.1\n\
             Host: 127.0.0.1:8001\n\
             User-Agent: curl/7.76.1\n\
             \n\
             tempting=isn't it",
        );

        assert_eq!(
            HttpRequest {
                method: "POST".to_owned(),
                path: "/somewhere".to_owned(),
                protocol: "HTTP/1.1".to_owned(),
                headers: test_headers(),
                body: "tempting=isn't it".to_owned(),
            },
            http_form_data.expect("http data should be properly translated")
        );
    }

    #[test]
    fn test_from_str_no_body() {
        let http_no_body = HttpRequest::from_str(
            "GET /v1/test HTTP/1.1\n\
             Host: 127.0.0.1:8001\n\
             User-Agent: curl/7.76.1\n",
        );

        assert_eq!(
            HttpRequest {
                method: "GET".to_owned(),
                path: "/v1/test".to_owned(),
                protocol: "HTTP/1.1".to_owned(),
                headers: test_headers(),
                body: "".to_owned(),
            },
            http_no_body.expect("http data should be properly translated")
        );
    }

    #[test]
    fn test_from_str_new_lines_start_body() {
        let http_new_lines_start_body = HttpRequest::from_str(
            "GET /v1/test HTTP/1.1\n\
             Host: 127.0.0.1:8001\n\
             User-Agent: curl/7.76.1\n\
             \n\
             \n\
             \n\
             yeah=yeah\n",
        );

        assert_eq!(
            HttpRequest {
                method: "GET".to_owned(),
                path: "/v1/test".to_owned(),
                protocol: "HTTP/1.1".to_owned(),
                headers: test_headers(),
                body: "\n\nyeah=yeah".to_owned(),
            },
            http_new_lines_start_body.expect("http data should be properly translated")
        );
    }

    #[test]
    fn test_from_str_space_after_header_value() {
        let http_header_space = HttpRequest::from_str(
            "GET /v1/test HTTP/1.1\n\
             User-Agent:       curl/7.76.1    \n",
        );
        let mut headers = HashMap::<String, String>::new();
        headers.insert("User-Agent".to_owned(), "curl/7.76.1    ".to_owned());

        assert_eq!(
            HttpRequest {
                method: "GET".to_owned(),
                path: "/v1/test".to_owned(),
                protocol: "HTTP/1.1".to_owned(),
                headers: headers,
                body: "".to_owned(),
            },
            http_header_space.expect("http data should be properly translated")
        );
    }

    #[test]
    fn test_from_str_no_content() {
        let http_no_content = HttpRequest::from_str("");

        assert!(http_no_content.is_err());
    }

    #[test]
    fn test_from_str_invalid_method() {
        let http_invalid_method = HttpRequest::from_str(
            "GET /v1/test\n\
            Test: test\n",
        );

        assert!(http_invalid_method.is_err());
    }
}
