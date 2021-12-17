use std::fmt::*;

struct Point<T> { x: T, y: T }

impl<T> Point<T> {
	fn new(x: T, y: T) -> Self {
		Point{ x: x, y: y }
	}
}

impl Debug for Point<&str> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
	}
}

impl Debug for Point<i32> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
	}
}

#[derive(Debug, PartialEq)]
struct Point2<T> { x: T, y: T }

impl<T> Point2<T> {
	fn new(x: T, y: T) ->Self {
		Point2 { x: x, y: y }
	}

	fn set(&mut self, x: T, y: T) {
		self.x = x;
		self.y = y;
	}

	fn get(&self) -> (&T, &T) {
		(&self.x, &self.y)
	}

	fn get_x(&self) -> &T {
		&self.x
	}

	fn get_y(&self) -> &T {
		&self.y
	}
}

fn main() {
	let p1 = Point::new(1, 2);
	let p2 = Point::new("1", "2");
	println!("p1={:?}, p2={:?}", p1, p2);

	let p3 = Point2::new(3, 4);
	let p4 = Point2::new("3", "4");
	let p5: Point2<i32> = Point2::new(5, 6);
	let mut p6: Point2<&str> = Point2::new("5", "6");
	println!("p3={:?}, p4={:?}, p5={:?}, p6={:?}", p3, p4, p5, p6);

	p6.set("7", "8");
	println!("p6={:?}", p6);

	let (x, y) = p6.get();
	println!("x={}, {}, y={}, {}", x, p6.get_x(), y, p6.get_y());
}
