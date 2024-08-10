use std::net::TcpListener;
use threadpool::ThreadPool;
use webserver;

fn main() {
    let listener = TcpListener::bind("localhost:80").unwrap();

    let workers = 5;
    let pool = ThreadPool::new(workers);

    for stream in listener.incoming() {
        pool.execute(|| {
            webserver::handle_connection(stream.unwrap());
        });
    }
}
