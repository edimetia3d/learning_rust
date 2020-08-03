use std::net::TcpStream;
use std::io::prelude::*;

fn echo(path: &str) -> String {
    path.to_string()
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let income_request = std::str::from_utf8(&buffer).unwrap();
    let end_pos = income_request.find(" HTTP").unwrap();
    let path_slice = &income_request[4..end_pos];

    let response = echo(path_slice);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}