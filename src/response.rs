//! Module that defines the response object.

use chrono::{DateTime, Utc};
use std::{array::IntoIter, collections::HashMap};

use crate::request;

#[derive(Debug)]
pub enum StatusCode {
    Success,
    NotImplemented,
    NotFound,
}

impl StatusCode {
    pub fn code(&self) -> u16 {
        match self {
            Self::Success => 200,
            Self::NotImplemented => 501,
            Self::NotFound => 404,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::NotImplemented => "Not Implemented",
            Self::NotFound => "Not Found",
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    protocol: String,
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpResponse {
    fn default_headers() -> HashMap<String, String> {
        let now: DateTime<Utc> = Utc::now();
        IntoIter::new([
            ("Server".to_owned(), "jhttp/0.1".to_owned()), // TODO: versioning!
            ("Date".to_owned(), now.to_rfc2822()),         // TODO: proper date
            ("Content-Type".to_owned(), "text/html".to_owned()),
            ("Content-Length".to_owned(), "0".to_owned()),
        ])
        .collect()
    }

    pub fn new(
        status: StatusCode,
        mut headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        if let Some(body) = &body {
            headers.insert(
                "Content-Length".to_owned(),
                body.as_bytes().len().to_string(),
            );
        }
        Self {
            protocol: "HTTP/1.1".to_owned(),
            status,
            headers,
            body,
        }
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self::new(StatusCode::Success, Self::default_headers(), None)
    }
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\n");
        let body = match &self.body {
            None => "",
            Some(v) => v,
        };

        format!(
            "{} {} {}\n\
            {}\n\
            \n\
            {}",
            self.protocol,
            self.status.code(),
            self.status.message(),
            headers,
            body
        )
    }
}
