//! The source file for the `server` binary. Intended for the GM
//! of the game, in conjunction with a player operated `client` binary.

//extern crates
extern crate httk;

//Inside Libraries
use httk::holonet::server::proto_server;

//External Libraries

fn main() {

	let address = "192.168.1.4:8080".parse().unwrap();
	proto_server(address);
}