fn main() {
    let a: i32 = 1;
    //let b: Option<i32> = None;//Some(5);
    let b: Option<i32> = Some(5);
    if b == None {
        println!("None");
    } else {
        println!("not None");
    }
    let c = a + b.unwrap();
    println!("{}", c);
}
