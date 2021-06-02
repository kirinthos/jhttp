//! Server configuration module.
//!

use std::net::IpAddr;
use std::path::Path;
use std::path::PathBuf;

/// StreamConf contains configuration properties of the server
/// library. This describes things like the server root, data
/// size limit, and other attributes.
///
/// provide any valid IP address to bind the server, defaults to
/// 127.0.0.1:8001
#[derive(Debug)]
pub struct HttpServerConf {
    ip: IpAddr,
    port: u32,

    server_dir: PathBuf,
    default_page: String,
}

impl HttpServerConf {
    pub fn new(ip: String, port: u32, server_dir: PathBuf, default_page: String) -> Self {
        let ip = ip
            .parse()
            .expect("valid `IP` string on which the server will listen");

        if !server_dir.exists() || !server_dir.is_dir() {
            panic!("Invalid server directory");
        }

        Self {
            ip,
            port,
            server_dir,
            default_page,
        }
    }

    pub fn bind_string(&self) -> String {
        format!("{}:{}", self.ip.to_string(), self.port)
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }

    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn server_dir(&self) -> &Path {
        &self.server_dir
    }

    pub fn default_page(&self) -> &str {
        &self.default_page
    }
}

impl Default for HttpServerConf {
    fn default() -> Self {
        let dir = std::env::current_dir().expect("Valid current directory path");
        Self::new("127.0.0.1".to_owned(), 8001, dir, "index.html".to_owned())
    }
}
