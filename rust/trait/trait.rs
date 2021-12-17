#[derive(Debug, PartialEq)]
struct Foo(i32);

#[derive(Debug, PartialEq)]
struct Bar(i32, i32);

trait Inst {
	fn new(i: i32) -> Self;
	fn info(&self);
}

impl Inst for Foo {
	fn new(i: i32) -> Foo {
		Foo(i)
	}

	fn info(&self) {
		println!("{:?}", self);
	}
}

impl Inst for Bar {
	fn new(i: i32) -> Bar {
		Bar(i, i + 10)
	}
	fn info(&self) {
		println!("{:?}", self);
	}
}

fn foobar<T: Inst>(i: i32) -> T {
	T::new(i)
}

fn main() {
	let f: Foo = foobar(10);
	println!("f={:?}", f);
	f.info();

	let b: Bar = foobar(20);
	println!("b={:?}", b);
}
