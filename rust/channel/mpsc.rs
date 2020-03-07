use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for _ in 0..10 {
        let sender = mpsc::Sender::clone(&tx);
        let handle = thread::spawn(move || {
            let vals = vec![
                String::from("hello"),
                String::from("world"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                sender.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
            drop(sender);
        });

        handles.push(handle);
    }

    drop(tx);

    let mut n = 0;
    for received in rx {
        println!("Got {}", received);
        n += 1;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Got {} messages!", n);
}
