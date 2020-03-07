fn main() {
    let list = vec!["hello", "world"];
    for it in list.iter() {
        if *it == "hello" {
            println!("it={}", it);
        }
    }
}
