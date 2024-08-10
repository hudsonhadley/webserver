use std::net::TcpListener;
use webserver;

fn main() {
    let listener = TcpListener::bind("localhost:80").unwrap();

    for stream in listener.incoming() {
        webserver::handle_connection(stream.unwrap());
    }
}
