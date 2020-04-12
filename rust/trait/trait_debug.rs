use std::fmt::*;

struct Point {
	x: i32,
	y: i32,
	z: i32,
}

impl Debug for Point {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Point {{ x: {}, y: {}, x: {} }}",
			self.x, self.y, self.z)
	}
}

fn main() {
	let o = Point { x: 0, y: 0, z: 0 };
	println!("The point is: {:?}", o);
}
