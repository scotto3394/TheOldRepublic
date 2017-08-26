//extern crates
extern crate httk;

//Outside Libraries
use std::net::TcpListener;

//Inside Libraries
use httk::network::handle_connection;
use httk::network::ThreadPool;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	//let listener = TcpListener::bind("192.168.1.2:8080").unwrap();
	let pool = ThreadPool::new(4);

	let mut counter = 0;

	for stream in listener.incoming() {
		if counter == 15 {
			println!("Shutting down.");
			break;
		}

		counter += 1;


		let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});
	}
}