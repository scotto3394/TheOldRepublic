use std::net::SocketAddr;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::str;

pub fn start_connection(addr: &str) {
	let mut stream = TcpStream::connect(addr).unwrap();

	loop{
		let mut input_buffer = String::new();
		io::stdin().read_line(&mut input_buffer).unwrap();
		stream.write(input_buffer.as_bytes());

		let mut response_buffer = [0; 512];
		stream.read(&mut response_buffer).unwrap();
		let response = str::from_utf8(&response_buffer).unwrap();
		println!("Response was: {}", response.to_string());
	}
}