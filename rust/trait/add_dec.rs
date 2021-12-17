trait Add<RHS, Output> {
	fn my_add(self, rhs: RHS) -> Output;
}

trait Dec<RHS = Self> {
	type Out;
	fn my_dec(self, rhs: RHS) -> Self::Out;
}

impl Add<i32, i32> for i32 {
	fn my_add(self, rhs: i32) -> i32 {
		(self + rhs)
	}
}

impl Add<u32, i32> for u32 {
	fn my_add(self, rhs: u32) -> i32 {
		(self + rhs) as i32
	}
}

impl Dec<i32> for i32 {
	type Out = i32;
	fn my_dec(self, rhs: i32) -> i32 {
		(self - rhs)
	}
}

impl Dec<u32> for u32 {
	type Out = i32;
	fn my_dec(self, rhs: u32) -> i32 {
		(self - rhs) as i32
	}
}

fn main() {
	let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
	let x: i32 = a.my_add(b);
	let y: i32 = c.my_add(d);
	println!("x={}, y={}", x, y);

	let x: i32 = a.my_dec(b);
	let y: i32 = d.my_dec(c);
	println!("x={}, y={}", x, y);
}
