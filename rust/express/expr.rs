fn main() {
	let n = 4;
	let check = if n < 10 && n > 0 {
		n * 10
	} else {
		n / 2
	};

	println!("n={}", check);

	for i in 1 .. 10 {
		if i % 2 == 0 {
			println!("i={}", i);
		}
	}
}
