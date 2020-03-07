use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;

fn handle_conn(mut stream: TcpStream) ->Result<(), Error> {
    println!("incoming from {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}

fn listen(addr: &str) -> TcpListener {
    let listener = TcpListener::bind(addr).expect("coudn't bind");
    return listener;
}

fn main() {
    let addr = "0.0.0.0:8288";
    let listener = listen(addr);

    println!("listener fd is {}", listener.as_raw_fd());

    let listen_fd = listener.as_raw_fd();
    let listener2;
    unsafe {
        listener2 = TcpListener::from_raw_fd(listen_fd);
    }

    for stream in listener2.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e); }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_conn(stream)
                    .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
