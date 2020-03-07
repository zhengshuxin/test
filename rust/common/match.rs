fn main() {
    let opt = Some(10);
    if let Some(x) = opt {
        println!("x={}", x);
    } else {
        println!("x=None");
    }

    match opt {
        Some(x) => println!("x={}", x),
        None => println!("None"),
    }
}
