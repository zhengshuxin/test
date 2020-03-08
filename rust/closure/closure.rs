fn math<F: Fn() -> i32>(op: F) -> i32 {
	op()
}

fn two_times_impl() -> impl Fn(i32) -> i32 {
	let i = 2;
	move |j| j * i
}
fn main() {
	let a = 3;
	let b = 2;
	assert_eq!(math(|| a + b), 5);
	assert_eq!(math(|| a * b), 6);
	println!("ok");

	let f = two_times_impl();
	println!("result={:?}", f(a));
}
