use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Instant;

use crate::conf::HttpServerConf;
use crate::error::ServerError;
use crate::request::{HttpRequest, Method};
use crate::response::{HttpResponse, StatusCode};

pub struct HttpServer {
    server_conf: std::sync::Arc<HttpServerConf>,
}

impl HttpServer {
    /// Listens for incoming connections using TcpListener
    ///
    /// # Arguments
    ///
    /// * `listener` - a TcpListener instance that is already bound
    ///
    pub fn listen(&self) -> Result<(), Box<dyn Error>> {
        let bind_str = self.server_conf.bind_string();
        println!("Starting socket listener on {}", bind_str);
        let listener = TcpListener::bind(bind_str)?;

        for stream in listener.incoming() {
            let stream = stream?;
            let s = Arc::clone(&self.server_conf);
            std::thread::spawn(move || {
                Servlet {
                    server_conf: s,
                    stream,
                }
                .serve()
            });
        }
        Ok(())
    }

    pub fn new(server_conf: HttpServerConf) -> Self {
        Self {
            server_conf: std::sync::Arc::new(server_conf),
        }
    }
}

impl Default for HttpServer {
    fn default() -> Self {
        HttpServer {
            server_conf: std::sync::Arc::new(HttpServerConf::default()),
        }
    }
}

/// Servlets contain a thread of execution that handles a request
/// these little guys will process a given request and then return a
/// response and exit.
struct Servlet {
    server_conf: std::sync::Arc<HttpServerConf>,
    stream: TcpStream,
}

impl Servlet {
    fn serve_get(&self, path: &str) -> Result<HttpResponse, Box<dyn Error>> {
        let path = self.server_conf.server_dir().join(if path == "/" {
            "index.html" //TODO: configuration
        } else {
            &path[1..]
        });
        println!("serving file {:?}", path,);

        let file_result = OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(path);

        match file_result {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                Ok(HttpResponse::new(
                    StatusCode::Success,
                    HashMap::new(),
                    Some(contents),
                ))
            }
            Err(_) => Err(ServerError::NotFound.into()),
        }
    }

    fn handle_request(&self, request: &HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
        Ok(match request.method {
            Method::GET => self.serve_get(&request.path)?,
        })
    }

    fn write_error_response(&mut self, error: &ServerError) -> Result<usize, Box<dyn Error>> {
        let response = match error {
            ServerError::NotImplemented => {
                HttpResponse::new(StatusCode::NotImplemented, HashMap::new(), None)
            }
            ServerError::NotFound => HttpResponse::new(StatusCode::NotFound, HashMap::new(), None),
        };
        let bytes = self.stream.write(response.to_string().as_bytes())?;
        Ok(bytes)
    }

    fn read_and_handle_request(&mut self, request_data: &[u8]) -> Result<usize, Box<dyn Error>> {
        let data_result = std::str::from_utf8(request_data)?;
        let request = data_result.parse::<HttpRequest>()?;
        let response = self.handle_request(&request)?;
        let bytes = self.stream.write(response.to_string().as_bytes())?;
        Ok(bytes)
    }

    fn serve(&mut self) {
        println!("{:?}", *self.server_conf);
        let mut data = [0 as u8; 1024 * 1024 * 1];
        let thread_id = std::thread::current().id();

        match self.stream.read(&mut data) {
            Ok(_) => {
                println!("[{:?}] received", thread_id);
                let timer = Instant::now();

                match self.read_and_handle_request(&data) {
                    Err(e) if e.downcast_ref::<ServerError>().is_some() => {
                        // Note: yes, discard result.
                        let _ = self.write_error_response(
                            e.downcast_ref::<ServerError>()
                                .expect("impossible. we checked for Some prior to unwrap"),
                        );
                    }
                    Err(e) => eprintln!("[{:?}] error processing request {}", thread_id, e),
                    Ok(_) => {}
                };

                match self.stream.shutdown(std::net::Shutdown::Both) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Error shutting down socket: {}", e),
                };

                println!("[{:?}] processed in {:?}", thread_id, timer.elapsed());
            }
            Err(m) => {
                eprintln!(
                    "Error occurred, terminating connection with {:?}",
                    self.stream.peer_addr()
                );
                eprintln!("{}", m);
            }
        }
    }
}
