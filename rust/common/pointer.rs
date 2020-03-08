fn main() {
	let mut x = 10;
	let ptr_x = &mut x as *mut i32;
	unsafe {
		*ptr_x = 100;
	}
	println!("x={}, ptr_x={:p}", x, ptr_x);
}
