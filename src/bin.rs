//extern crates
extern crate httk;

//Inside Libraries
use httk::network::tokio::proto_server;

fn main() {
	let address = "127.0.0.1:8080".parse().unwrap();
	proto_server(address);
}