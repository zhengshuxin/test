use std::sync::mpsc;
use std::thread;

const MAX: i32 = 10_000_000;

fn main() {
    let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = mpsc::sync_channel(100000);
    let t = thread::spawn(move || {
            for _ in 0..MAX {
                tx.send(1).unwrap();
            }
    });

    for i in 0..MAX {
        let n = rx.recv();
        if i < 10 {
            println!("receive: {}", n.unwrap());
        }
    }
    let r = t.join();
    println!("ret={:?}, max={}", r, MAX);
}
