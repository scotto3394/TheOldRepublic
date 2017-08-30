//extern crates
extern crate httk;

//Inside Libraries
use httk::network::tokio::proto_server;

//External Libraries

fn main() {

	let address = "192.168.1.4:8080".parse().unwrap();
	proto_server(address);
}