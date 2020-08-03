use std::net::TcpListener;
use mini_httpd::thread_pool::*;
use mini_httpd::connection_handler::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for new_con in listener.incoming() {
        let connection = new_con.unwrap();
        pool.execute(|| {
            handle_connection(connection);
        });
    }
}