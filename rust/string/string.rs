use std::time::Duration;
use std::thread;

const MAX: u32 = 100;

fn add_strings(v: &mut Vec<Box<String>>) {
    for _i in 0..MAX {
        let s = String::from("hello");
        v.push(Box::new(s));
    }
}

fn test1() {
    let mut v: Vec<Box<String>> = Vec::new();
    add_strings(&mut v);
    println!("in test1, v's len={}", v.len());
    thread::sleep(Duration::from_secs(1));
    println!("wakeup!");
}

fn add_strings2(v: &mut Vec<String>) {
    for _i in 0..MAX {
        let s = String::from("hello");
        v.push(s);
    }
}

fn test2() {
    let mut v: Vec<String> = Vec::new();
    add_strings2(&mut v);
    println!("in test2, v's len={}", v.len());
    thread::sleep(Duration::from_secs(1));
    println!("wakeup!");
    drop(v);
}

fn main() {
    test1();
    test2();
    let s : String = "hello world".to_string() + "hello world";
    println!("s={}", s);
    thread::sleep(Duration::from_secs(1));
}
