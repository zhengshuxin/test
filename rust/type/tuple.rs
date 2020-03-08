struct Color(i32, i32, i32);

fn move_coords(x: (i32, i32)) -> (i32, i32) {
	(x.0 + 1, x.1 + 1)
}

fn main() {
	let coords = (0, 1);
	let (x, y) = move_coords(coords);
	println!("x={}, y={}", x, y);

	let tuple : (&'static str, i32, char) = ("hello", 1, 'c');
	println!("t1={}, t2={}, t3={}", tuple.0, tuple.1, tuple.2);

	let color = Color(100, 200, 100);
	println!("color={}, {}, {}", color.0, color.1, color.2);
}
