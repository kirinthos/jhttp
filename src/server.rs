use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Instant;

use crate::conf::StreamConf;
use crate::request::HttpRequest;

/// Listens for incoming connections using TcpListener
///
/// # Arguments
///
/// * `listener` - a TcpListener instance that is already bound
///
pub fn listen(stream_conf: StreamConf) -> Result<(), Box<dyn Error>> {
    let ip = "127.0.0.1:8001";
    let listener = TcpListener::bind(ip)?;

    let stream_conf = Arc::new(stream_conf);
    for stream in listener.incoming() {
        let stream = stream?;
        let s = Arc::clone(&stream_conf);
        std::thread::spawn(move || serve(stream, s));
    }
    Ok(())
}

fn handle_request(request: &HttpRequest) -> Result<String, Box<dyn Error>> {
    let data = "Hello, mate!";
    let data_size = data.as_bytes().len();
    Ok(format!(
        "HTTP/1.1 200 Success\n\
    Server: jhttp/0.1\n\
    Content-type: text/html\n\
    Content-Length: {}\n\
    \n\
    {}",
        data_size, data
    )
    .to_owned())
}

fn read_and_handle_request(
    mut stream: TcpStream,
    request_data: &[u8],
) -> Result<usize, Box<dyn Error>> {
    let data_result = std::str::from_utf8(request_data)?;
    let request = HttpRequest::from_str(data_result)?;
    let response = handle_request(&request)?;
    let bytes = stream.write(response.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(bytes)
}

fn serve(mut stream: TcpStream, conf: std::sync::Arc<StreamConf>) {
    println!("{:?}", *conf);
    let mut data = [0 as u8; 1024 * 1024 * 1];
    let thread_id = std::thread::current().id();

    match stream.read(&mut data) {
        Ok(_) => {
            println!("[{:?}] received", thread_id);

            match read_and_handle_request(stream, &data) {
                Err(e) => {
                    eprintln!("[{:?}] error processing request {}", thread_id, e);
                }
                Ok(_) => {}
            };
        }
        Err(m) => {
            eprintln!(
                "Error occurred, terminating connection with {:?}",
                stream.peer_addr()
            );
            eprintln!("{}", m);
        }
    }
}
