fn main() {
    /*
    let s = String::from("hello world; who ;are ;you");
    let split = s.trim_matches(' ').split(";");
    for i in split {
        println!("s={}, {}", i, s.trim_matches(' '));
    }
    */

    let s = "hello;;; world who are you".to_string();
    let s = s.replace(" ", ";");
    let m = s.split(";");
    for i in m {
        if i.len() > 0 {
            println!("{}", i);
        }
    }
}
