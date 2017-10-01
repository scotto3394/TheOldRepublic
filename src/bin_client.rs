//! The source file for the `client` binary. Intended for players
//! of the game, in conjunction with a GM run `server` binary.

//extern crates
extern crate httk;
extern crate cursive;

//Inside Libraries
// use httk::holonet::client::start_connection;
use httk::holoterminal::*;

//External Libraries
use cursive::Cursive;

fn main() {
    //Start the Client connection
    // let address = "coruscant.smanifold.com:80";
    // start_connection(address);

    //Start the TUI
    let mut tui = Cursive::new();
    tui.add_global_callback('q', shutdown);
    startup(&mut tui);
    tui.run();
}
