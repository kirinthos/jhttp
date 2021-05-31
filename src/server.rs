use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Instant;

use crate::conf::StreamConf;
use crate::error::ServerError;
use crate::request::HttpRequest;
use crate::response::{HttpResponse, StatusCode};

/// Listens for incoming connections using TcpListener
///
/// # Arguments
///
/// * `listener` - a TcpListener instance that is already bound
///
pub fn listen(stream_conf: StreamConf) -> Result<(), Box<dyn Error>> {
    let bind_str = stream_conf.bind_string();
    println!("Starting socket listener on {}", bind_str);
    let listener = TcpListener::bind(bind_str)?;

    let stream_conf = Arc::new(stream_conf);
    for stream in listener.incoming() {
        let stream = stream?;
        let s = Arc::clone(&stream_conf);
        std::thread::spawn(move || serve(stream, s));
    }
    Ok(())
}

fn handle_request(_request: &HttpRequest) -> Result<String, Box<dyn Error>> {
    Ok(HttpResponse::default().to_string())
}

fn write_error_response(
    stream: &mut TcpStream,
    error: &ServerError,
) -> Result<usize, Box<dyn Error>> {
    let response = match error {
        ServerError::NotImplementedError => {
            HttpResponse::new(StatusCode::NotImplemented, HashMap::new(), None)
        }
    };
    let bytes = stream.write(response.to_string().as_bytes())?;
    Ok(bytes)
}

fn read_and_handle_request(
    stream: &mut TcpStream,
    request_data: &[u8],
) -> Result<usize, Box<dyn Error>> {
    let data_result = std::str::from_utf8(request_data)?;
    let request = data_result.parse::<HttpRequest>()?;
    let response = handle_request(&request)?;
    let bytes = stream.write(response.as_bytes())?;
    Ok(bytes)
}

fn serve(mut stream: TcpStream, conf: std::sync::Arc<StreamConf>) {
    println!("{:?}", *conf);
    let mut data = [0 as u8; 1024 * 1024 * 1];
    let thread_id = std::thread::current().id();

    match stream.read(&mut data) {
        Ok(_) => {
            println!("[{:?}] received", thread_id);
            let timer = Instant::now();

            match read_and_handle_request(&mut stream, &data) {
                Err(e) => {
                    match e.downcast_ref::<ServerError>() {
                        Some(v) => {
                            write_error_response(&mut stream, v);
                        }
                        None => {
                            eprintln!("[{:?}] error processing request {}", thread_id, e);
                        }
                    };
                }
                Ok(_) => {}
            };

            match stream.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(e) => eprintln!("Error shutting down socket: {}", e),
            };

            println!("[{:?}] processed in {:?}", thread_id, timer.elapsed());
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
