use std::net::TcpListener;
use std::thread;
use webserver;

fn main() {
    let listener = TcpListener::bind("localhost:80").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| {
            webserver::handle_connection(stream.unwrap());
        });
    }
}
