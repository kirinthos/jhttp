use std::error::Error;

extern crate jhttp;

use crate::jhttp::conf::HttpServerConf;
use crate::jhttp::server::HttpServer;

fn main() -> Result<(), Box<dyn Error>> {
    let s = HttpServerConf::new("127.0.0.1".to_owned(), 8001, "./html".into());
    let server = HttpServer::new(s);
    server.listen()?;
    Ok(())
}
