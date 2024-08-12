use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;

pub fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request_line = reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let mut request_line = request_line.split(' ');

    request_line.next().unwrap(); // Throw the GET away
    let path_requested = request_line.next().unwrap();

    let ok_tuple = ("HTTP/1.1 200 OK",
                    &fs::read_to_string("index.html").unwrap()[..]);


    let (status_line, message_body) = match path_requested {
        "/" | "/index.html" => ok_tuple,

        "/sleep" => {
            thread::sleep(Duration::from_secs(10));
            ok_tuple
        }

        _ => ("HTTP/1.1 404 NOT FOUND", ""),
    };


    stream.write(
        format!(
            "{status_line}\r\nContent-Length: {}\r\n\r\n{message_body}\r\n", message_body.len()
        ).as_ref()).unwrap();


}