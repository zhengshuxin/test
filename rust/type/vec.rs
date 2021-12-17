fn main() {
	let mut v: Vec<u32> = vec![];
	v.push(1);
	v.push(2);
	v.push(3);
	println!("v is {:?}", v);

	let mut v = Vec::new();
	v.push(4);
	v.push(5);
	v.push(6);
	println!("v is {:?}", v);

	while let Some(x) = v.pop() {
		println!("x is {}", x);
	}
}
