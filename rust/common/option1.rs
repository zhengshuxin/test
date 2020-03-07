fn main() {
	let option = Some(2);
	let new: Option<i32> = option.map(multiple2);
	println!("{:?}", new);
	fn multiple2(val: i32) -> i32{ val*2 }
    println!("{}", multiple2(10));

    let n = multiple2(20);
    let c = || n + 20;
    println!("n={}", c());
}
