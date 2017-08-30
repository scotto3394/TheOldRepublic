//extern crates
extern crate httk;

//Inside Libraries
use httk::network::client::start_connection;

//External Libraries

fn main() {

	let address = "coruscant.smanifold.com:80";
	start_connection(address);
}