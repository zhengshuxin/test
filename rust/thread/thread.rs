use std::thread;
use std::time::Duration;

fn test1() {
    let child = thread::spawn(move || {
            let t = Duration::new(1, 0);
            thread::sleep(t);
            println!("hello world! tid={:?}", thread::current());
    });
    let res = child.join();
    println!("res={:?}", res);

    let child = thread::Builder::new().name("child".to_string())
        .stack_size(4 * 1024 * 1024)
        .spawn(move || {
                println!("begin park!");
                thread::park();
                println!("wakeup");
                thread::sleep(Duration::new(1, 0));

    }).unwrap();

    thread::sleep(Duration::new(2, 0));
    println!("unpark child");
    child.thread().unpark();
    let res = child.join();
    println!("res={:?}", res);
}

fn main() {
    test1();

    let mut health = 12;
    let mut v: Vec<i32> = vec![];
    let t = thread::spawn(move || {
            health *= 2;
            v.push(1);
            println!("health={}, v[0]={}", health, v[0]);
            });

    let res = t.join();
    //println!("{}, {:?}", health, v);
    println!("res={:?}", res);

    for i in 0..10 {
        println!("i={}", i);
        if i == 5 {
            println!("break!");
            break;
        }
    }
}
