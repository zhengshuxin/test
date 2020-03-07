use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const MAX: i32 = 50000000;

fn main() {
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    let c = thread::spawn(move || {
            for i in 0..MAX {
                tx.send(i).unwrap();
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
