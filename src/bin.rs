//extern crates
extern crate httk;

//Inside Libraries
//use httk::network::tokio::proto_server;
use httk::network::tokio_client::start_connection;

//External Libraries
use std::net::ToSocketAddrs;

fn main() {

	let address = "coruscant.smanifold.com:80";
	// proto_server(address);
	start_connection(address);
}