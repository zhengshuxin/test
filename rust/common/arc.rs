use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;
const N: usize = 10;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();

    for _ in 0..N {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }

    rx.recv().unwrap();
    println!("data={:?}, data={}", data, data.lock().unwrap());
}
