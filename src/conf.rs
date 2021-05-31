//! Server configuration module.
//!

use std::net::IpAddr;

/// StreamConf contains configuration properties of the server
/// library. This describes things like the server root, data
/// size limit, and other attributes.
///
/// provide any valid IP address to bind the server, defaults to
/// 127.0.0.1:8001
#[derive(Debug)]
pub struct StreamConf {
    pub ip: IpAddr,
    pub port: u32,
}

impl Default for StreamConf {
    fn default() -> Self {
        Self::new("127.0.0.1".to_owned(), 8001)
    }
}

impl StreamConf {
    pub fn new(ip: String, port: u32) -> Self {
        let ip = ip
            .parse()
            .expect("valid `IP:port` string on which the server will listen");
        Self { ip, port }
    }

    pub fn bind_string(&self) -> String {
        format!("{}:{}", self.ip.to_string(), self.port)
    }
}
