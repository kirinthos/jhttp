use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Instant;

use crate::conf::StreamConf;
use crate::error::ServerError;
use crate::request::{HttpRequest, Method};
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

fn serve_get(path: &str) -> Result<HttpResponse, Box<dyn Error>> {
    println!("serving file {} from {:?}", path, std::env::current_dir());
    let path = if path == "/" {
        "index.html" //TODO: configuration
    } else {
        &path[1..]
    };

    let file_result = OpenOptions::new()
        .write(false)
        .read(true)
        .create(false)
        .open(path);

    match file_result {
        Ok(mut file) => {
            let mut contents = String::new();

            file.read_to_string(&mut contents)?;
            println!("contents: {}", contents);
            Ok(HttpResponse::new(
                StatusCode::Success,
                HashMap::new(),
                Some(contents),
            ))
        }
        Err(_) => Err(ServerError::NotFound.into()),
    }
}

fn handle_request(request: &HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    Ok(match request.method {
        Method::GET => serve_get(&request.path)?,
    })
}

fn write_error_response(
    stream: &mut TcpStream,
    error: &ServerError,
) -> Result<usize, Box<dyn Error>> {
    let response = match error {
        ServerError::NotImplemented => {
            HttpResponse::new(StatusCode::NotImplemented, HashMap::new(), None)
        }
        ServerError::NotFound => HttpResponse::new(StatusCode::NotFound, HashMap::new(), None),
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
    let bytes = stream.write(response.to_string().as_bytes())?;
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
                Err(e) if e.downcast_ref::<ServerError>().is_some() => {
                    // Note: yes, discard result.
                    let _ = write_error_response(
                        &mut stream,
                        e.downcast_ref::<ServerError>()
                            .expect("impossible. we checked for Some prior to unwrap"),
                    );
                }
                Err(e) => eprintln!("[{:?}] error processing request {}", thread_id, e),
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
