extern crate jhttp;
use crate::jhttp::conf::StreamConf;
use crate::jhttp::server::listen;

fn main() {
    let s = StreamConf::new("testing".to_owned());
    listen(s);
}
