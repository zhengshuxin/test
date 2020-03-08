use std::collections::VecDeque;
fn main() {
	let mut buf = VecDeque::new();
	buf.push_front(1);
	buf.push_front(2);
	println!("buf is {:?}", buf);

	buf.push_back(3);
	buf.push_back(4);
	println!("buf is {:?}", buf);

	//while let Some(x) = buf.pop_back() {
	while let Some(x) = buf.pop_front() {
		println!("x is {}", x);
	}
}
