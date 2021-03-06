//! This is the root of the "Hall of the Tauntaun King" library or more
//! succintly known as `httk`. This library will hold all the essential
//! functions, structures, and components to run the client and server
//! applications for the future game.

// Networking crates
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

// Graphics crates
extern crate cursive;

// Database crates
extern crate rusqlite;

//To Do: Clean up the `pub` keywords everywhere.
pub mod force; // Core module
pub mod holocron; // Database module
pub mod holonet; // Network module
pub mod holoterminal; // Graphics module
mod jedi_code; // Combat module
