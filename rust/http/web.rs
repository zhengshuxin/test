//use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use std::io::prelude::*;
//use std::fs::File;
use std::thread;

// --snip--
fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 512];
	stream.read(&mut buffer).unwrap();

	// --snip--
	let get = b"GET / HTTP/1.1\r\n";
	let sleep = b"GET /sleep HTTP/1.1\r\n";
	let (_status_line, _filename) = if buffer.starts_with(get) {
		("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
	} else if buffer.starts_with(sleep) {
		thread::sleep(Duration::from_secs(5));
		("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
	} else {
		("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
	};
	// --snip--
}

fn main() {
	let addr = "127.0.0.1:7878";
	let listener = TcpListener::bind(addr).unwrap();
	println!("listen on {}", addr);

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		thread::spawn(|| { handle_connection(stream); });
	}
}
