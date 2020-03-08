fn main() {
	let s: &'static str = "hello Rust!";
	let ptr = s.as_ptr();
	let len = s.len();
	println!("s={}, ptr={:p}, len={}", s, ptr, len);
}
