use std::fmt::*;
struct Point { x: i32, y: i32, z: i32 }

impl Point {
	fn info(&self) {
		println!("x={}, y={}, x={}", self.x, self.y, self.z);
	}

	fn set(&mut self, x: i32, y: i32, z: i32) {
		self.x = x;
		self.y = y;
		self.z = z;
	}
}

impl Debug for Point {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Point {{ x: {}, y: {}, x: {} }}",
			self.x, self.y, self.z)
	}
}

#[derive(Debug, PartialEq)]
struct Point2 { x: i32, y: i32, z: i32 }

fn main() {
	let mut o = Point { x: 0, y: 0, z: 0 };
	println!("The point is: {:?}", o);
	o.info();
	o.set(1, 2, 3);
	o.info();

	let o2 = Point2 { x: 1, y: 1, z: 1 };
	println!("The Point2 is : {:?}", o2);
}
