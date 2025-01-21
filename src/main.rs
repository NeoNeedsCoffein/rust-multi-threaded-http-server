use std::{
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream},
};

use rust_multi_threaded_http_server::ThreadPool;

const OK_HEADER: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_HEADER: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const METHOD_NOT_ALLOWED_HEADER: &str = "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n";


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);


    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let parts: Vec<&str> = http_request[0].split_whitespace().collect();
    let (method, path, _version) = (parts[0], parts[1], parts[2]);

    match method {
        "GET" => {
            let response = match path {
                "/" => OK_HEADER,
                _ => NOT_FOUND_HEADER,
            };

            send_response(&mut stream, response);
        }
        _ => {
            let response = METHOD_NOT_ALLOWED_HEADER;
            
            send_response(&mut stream, response);
        }
    }
}

fn send_response(stream: &mut TcpStream, response: &str) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}