extern crate httk;

use std::net::TcpListener;
use httk::network::handle_connection;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	//let listener = TcpListener::bind("192.168.1.2:8080").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		handle_connection(stream);
	}
}