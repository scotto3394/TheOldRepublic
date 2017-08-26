use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
	/* Get request from connection*/
	let mut buffer = [0; 512];
	stream.read(&mut buffer).unwrap();
	//println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

	let get = b"GET / HTTP/1.1\r\n";
	
	/* Choose appropriate response*/
	let (status_line, filename) = if buffer.starts_with(get) {
		("HTTP/1.1 200 OK\r\n\r\n", "tests/hello.html")
	} else {
		("HTTP/1.1 404 NOT FOUND\r\n\r\n", "tests/404.html")
	}; 
	let mut file = File::open(filename).unwrap();
	let mut contents = String::new();
					
	file.read_to_string(&mut contents).unwrap();
	
	let response = format!("{}{}", status_line, contents);

	/* Send response*/
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}