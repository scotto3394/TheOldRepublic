//! The source file for the `client` binary. Intended for players
//! of the game, in conjunction with a GM run `server` binary.

//extern crates
extern crate httk;

//Inside Libraries
// use httk::holonet::client::start_connection;
use httk::holoterminal::draw_rectangle;

//External Libraries

fn main() {
	// let address = "coruscant.smanifold.com:80";
	// start_connection(address);
	draw_rectangle();
}