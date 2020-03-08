fn main() {
	let mut v = vec![];
	for i in 0 .. 10 {
		v.push(i);
	}

	while let Some(x) = v.pop() {
		println!("x={}", x);
	}

	let mut v = [0; 10];
	for i in 0 .. 10 {
		v[i] = i;
	}
	println!("{:?}, 10={}, 5..7={:?}, 5..={:?}, ..5={:?}, ..=5={:?}",
		v, v[9], &v[5..7], &v[5..], &v[..5], &v[..=5]);
}
