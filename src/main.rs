use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

mod conf;
use conf::StreamConf;

mod request;
use request::*;

mod util;

/// Listens for incoming connections using TcpListener
///
/// # Arguments
///
/// * `listener` - a TcpListener instance that is already bound
///
fn listen(listener: TcpListener, stream_conf: StreamConf) {
    let stream_conf = Arc::new(stream_conf);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let s = Arc::clone(&stream_conf);
                std::thread::spawn(move || serve(stream, s));
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn serve(mut stream: TcpStream, conf: std::sync::Arc<StreamConf>) {
    println!("{:?}", *conf);
    let mut data = [0 as u8; 256];

    while match stream.read(&mut data) {
        Ok(size) => {
            println!(
                "[{:?}] received: {}",
                std::thread::current().id(),
                std::str::from_utf8(&data[..size]).unwrap()
            );
            stream.write(&data[..size]).unwrap();
            size > 0
        }
        Err(m) => {
            println!(
                "Error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            println!("{}", m);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let ip = "127.0.0.1:8001";

    let s = StreamConf::new("testing".to_owned());

    match TcpListener::bind(ip) {
        Ok(l) => listen(l, s),
        Err(m) => println!("{:?}", m),
    }
}
