use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    let data = Arc::new(Mutex::new(0u32));
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move|| {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }

    println!("all threads are finished!");
    let d = data.lock().unwrap();
	println!("d is {:?} {}, value is {}", d, d, *d);

    let n = 100;
    let mut n1 = n.clone();
    n1 += 1;
    println!("n={}, n1={}", n, n1);
}
