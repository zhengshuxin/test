fn main() {
	let s: &'static str = "hello Rust!";
	let ptr = s.as_ptr();
	let len = s.len();
	println!("s={}, ptr={:p}, len={}", s, ptr, len);

	let x = "100";
	let i: i32 = x.parse().unwrap();
	println!("i={}, {}", i, x.parse::<i32>().unwrap());
}
