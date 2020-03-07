use std::thread;

fn main() {
	let handle=thread::spawn(|| {
			println!("Hello from a thread!");
            "Hello from a thread!"
	});

    let ret = handle.join();
	println!("wait: {:?}", ret);
	println!("wait: {}", ret.unwrap());
}
