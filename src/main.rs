use std::error::Error;
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

fn handle_request(request: &HttpRequest) -> String {
    format!("{:?}", request)
}

fn parse_request(request_data: &[u8]) -> Result<HttpRequest, Box<dyn Error>> {
    let data_result = std::str::from_utf8(request_data)?;
    Ok(HttpRequest::from_str(data_result)?)
}

fn serve(mut stream: TcpStream, conf: std::sync::Arc<StreamConf>) {
    println!("{:?}", *conf);
    let mut data = [0 as u8; 1024 * 1024 * 1];
    let thread_id = std::thread::current().id();

    while match stream.read(&mut data) {
        Ok(size) => {
            println!("[{:?}] received", thread_id);

            let response = match parse_request(&data) {
                Err(e) => {
                    eprintln!("[{:?}] error processing request {}", thread_id, e);
                    // TODO: empty response
                    "".to_owned()
                }
                Ok(request) => handle_request(&request),
            };

            stream.write(response.as_bytes());

            size > 0
        }
        Err(m) => {
            println!(
                "Error occurred, terminating connection with {:?}",
                stream.peer_addr()
            );
            println!("{}", m);
            false
        }
    } {}

    match stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    };
}

fn main() {
    let ip = "127.0.0.1:8001";

    let s = StreamConf::new("testing".to_owned());

    match TcpListener::bind(ip) {
        Ok(l) => listen(l, s),
        Err(m) => println!("{:?}", m),
    }
}
