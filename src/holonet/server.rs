//! The `holonet::server` module
//!
//! This module will focus on the server-side network management,
//! including but not limited to, asynchronous I/O, authentification,
//! and updates to all players.




//===========================================================
// Codec
//===========================================================
use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};

pub struct LineCodec;

impl Decoder for LineCodec {
	type Item = String;
	type Error = io::Error;

	fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
		if let Some(i) = buf.iter().position(|&b| b == b'\n') {
			// remove the serialized frame from the buffer.
			let line = buf.split_to(i);

			// Also remove the '\n'
			buf.split_to(1);

			// Turn this data into a UTF string and return it in a Frame.
			match str::from_utf8(&line) {
				Ok(s) => Ok(Some(s.to_string())),
				Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
			}
		} else {
			Ok(None)
		}
	}
}


impl Encoder for LineCodec {
	type Item = String;
	type Error = io::Error;

	fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
		buf.extend(msg.as_bytes());
		buf.extend(b"\n");
		Ok(())
	}
}

//===========================================================
// Protocol
//===========================================================
use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use futures::{Stream, Sink};

struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
	// For this protocol style, `Request` matches the `Item` type of the codec `Decoder`
	type Request = String;

	// For this protocol style, `Response` matches the `Item` type of the codec `Encoder`
	type Response = String;

	// Boilerplate
	type Transport = Framed<T, LineCodec>;
	type BindTransport = Box<Future<Item = Self::Transport, Error = io::Error>>;
	fn bind_transport(&self, io: T) -> Self::BindTransport {
		// The basic protocol (line-based)
		let transport = io.framed(LineCodec);
		
		// Build the handshake protocol.
		// Confirm they're a tauntaun or error out the connection.
		Box::new(transport.into_future()
			.map_err(|(e,_)| e)
			.and_then(|(line,transport)| {
				//Message received, verify credentials
				match line {
					Some(ref msg) if msg == "Quaggle, Quaggle, Quack" => {
						println!("SERVER: received client handshake");
						// Send acknowledgement
						let ret = transport.send("Welcome to a world of pure imagination!".into());
						Box::new(ret) as Self::BindTransport
					}
					_ => {
						// An imposter has shown up!
						// Error out the connection
						println!("SERVER: An imposter appeared!");
						let err = io::Error::new(io::ErrorKind::Other,
									"invalid handshake");
						let ret = future::err(err);
						Box::new(ret) as Self::BindTransport

					}
				}
			})
		)
	}

}
// To Do: Debug the Handshake Protocol 
//===========================================================
// Service
//===========================================================
use tokio_service::Service;
use futures::{future, Future};

pub struct Echo;

impl Service for Echo {
	// Match with the protocol
	type Request = String;
	type Response = String;

	// For non-streaming protocols, service errors are always io::Error
	type Error = io::Error;

	// The future for computing the response; box it for simplicity.
	type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

	// Produce a future for computing a response from a request.
	fn call(&self, req: Self::Request) -> Self::Future {
		// In this case, the response is immediate.
		Box::new(future::ok(req))
	}
}

//===========================================================
// Other
//===========================================================
use std::net::SocketAddr;
use tokio_proto::TcpServer;

pub fn proto_server(addr: SocketAddr) {
	let server = TcpServer::new(LineProto, addr);
	
	//We provide a way to *instantiate* the service for each new 
	// connection; here, we just immmediately return a new instance
	server.serve(|| Ok(Echo));
}
