use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request_line = reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let mut request_line = request_line.split(' ');

    request_line.next().unwrap(); // Throw the GET away
    let path_requested = request_line.next().unwrap();

    let (status_line, message_body) = match path_requested {
        "/" | "/index.html" => ("HTTP/1.1 200 OK",
                               &fs::read_to_string("index.html").unwrap()[..]),

        _ => ("HTTP/1.1 404 NOT FOUND", ""),
    };


    stream.write(
        format!(
            "{status_line}\r\nContent-Length: {}\r\n\r\n{message_body}\r\n", message_body.len()
        ).as_ref()).unwrap();


}

fn main() {
    let listener = TcpListener::bind("localhost:80").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}
