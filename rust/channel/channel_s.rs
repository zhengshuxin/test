use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const MAX: i32 = 100000000;

fn main() {
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let c = thread::spawn(move || {
            for _i in 0..MAX {
                //let s = String::from("hello world");
                //let s = String::with_capacity(8192);
                let s = "hello world".to_string();
                tx.send(s).unwrap();
            }

            std::thread::sleep(Duration::new(1, 0));
    });

    for i in 0..MAX {
        let n = rx.recv();
        /*
        match n {
            Ok(v) => println!("ok={}", v),
            Err(e) => { println!("error={}", e); break; }
        }
        */

        if i < 10 {
            println!("receive: {}", n.unwrap());
        }
    }
    let r = c.join();
    println!("ret={:?}, max={}", r, MAX);
}
