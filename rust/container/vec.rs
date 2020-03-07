fn main() {
    let mut numbers = vec![21];
//    let mut first = None;
    let first = numbers.pop();

    if let Some(first) = first {
        println!("first is {}", first);
    } else {
        println!("first is None");
    }
}
