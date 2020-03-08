use std::collections::LinkedList;
fn main() {
	let mut list = LinkedList::new();
	list.push_back("hello");
	list.push_back("world");

	for x in list.iter() {
		println!("iter x is {}", x);
	}
	while let Some(x) = list.pop_front() {
		println!("x is {}", x);
	}
}
