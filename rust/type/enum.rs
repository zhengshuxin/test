enum Number { One, Two, Three }
enum Option { Some(&'static str), None }
fn main() {
	let a = Number::One;
	match a {
		Number::One => println!("one"),
		Number::Two => println!("two"),
		Number::Three => ()
	}

	let s = Some("hello");
	println!("unwrap {}", s.unwrap());

	match s {
		Some(x) => println!("str is: {}", x),
		_ => ()
	};
}
