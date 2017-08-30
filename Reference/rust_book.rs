use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::time::Duration;


//==========================================================
// Structs
//==========================================================
struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}
//==========================================================
// Impl
//==========================================================

//------------------------------------------------------
// Miscellaneous
//------------------------------------------------------
trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self:Box<F>) {
		(*self)()
	}
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
	NewJob(Job),
	Terminate,
}

//------------------------------------------------------
// Workers
//------------------------------------------------------

impl Worker {
	/// Create a new Worker
	///
	/// The id gives each worker a unique identity and a channel receiver 
	/// to retrieve Job queues.
	fn new(id: usize, receiver:Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
		let thread = thread::spawn(move || {
			loop {
				let message = receiver.lock().unwrap().recv().unwrap();

				match message {
					Message::NewJob(job) => {
						println!("Worker {} got a job; executing.", id);
						job.call_box();
					},
					Message::Terminate => {
						println!("Worker {} was told to terminate.", id);
						break;
					}
				}
			}
		});

		Worker {
			id,
			thread: Some(thread),
		}
	}
}

//------------------------------------------------------
// ThreadPool
//------------------------------------------------------

impl ThreadPool {
	/// Create a new ThreadPool.
	///
	/// The size is the number of threads in the pool.
	///
	/// # Panics
	///
	/// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, receiver.clone()));
		}

		ThreadPool {
			workers,
			sender,
		}
	}

	pub fn execute<F>(&self, f: F)
		where
			F: FnOnce() + Send + 'static,
		{
			let job = Box::new(f);

			self.sender.send(Message::NewJob(job)).unwrap();
		}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		println!("Sending terminate message to all workers.");
		for _ in &mut self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}

		
		println!("Shutting down all workers.");
		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}

//==========================================================
// Functions
//==========================================================

pub fn start_connection(address: String, port: String) {
	let url = format!("{}:{}", address, port);

	let listener = TcpListener::bind(url).unwrap();
	let pool = ThreadPool::new(4);
	let mut counter = 0;

	for stream in listener.incoming() {
		if counter == 15 {
			println!("Shutting down.");
			break;
		}

		counter += 1;

		let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});
	}
}

fn handle_connection(mut stream: TcpStream) {
	/* Get request from connection*/
	let mut buffer = [0; 512];
	stream.read(&mut buffer).unwrap();

	let get = b"GET / HTTP/1.1\r\n";
	let sleep = b"GET /sleep HTTP/1.1\r\n";
	
	/* Choose appropriate response*/
	let (status_line, filename) = if buffer.starts_with(get) {
		("HTTP/1.1 200 OK\r\n\r\n", "tests/hello.html")
	} else if buffer.starts_with(sleep) {
		thread::sleep(Duration::from_secs(5));
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