use std::error::Error;

extern crate jhttp;

use crate::jhttp::conf::StreamConf;
use crate::jhttp::server::listen;

fn main() -> Result<(), Box<dyn Error>> {
    let s = StreamConf::new("127.0.0.1".to_owned(), 8001);
    listen(s)?;
    Ok(())
}
