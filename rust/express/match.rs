fn main() {
	for i in -1 .. 10 {
		match i {
			0 => println!("first=0"),
			//1...3 => println!("second={}", i),
			1 ..= 3 => println!("second={}", i),
			5|7|8 => println!("third={}", i),
			_ => println!("other={}", i),
		}
	}

	let b = true;
	let mut x = 0;
	if let true = b {
		x = 1;
	}
	println!("x={}", x);

	let mut v = vec![1, 2, 3, 4, 5];
	loop {
		match v.pop() {
			Some(x) => println!("{}", x),
			None => break,
		}
	}

	println!("");
	let mut v = vec![1, 2, 3, 4, 5];
	while let Some(x) = v.pop() {
		println!("{}", x);
	}
}
