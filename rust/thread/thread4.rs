use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data=Arc::new(Mutex::new(vec![1u32, 2, 3]));

    let mut threads = Vec::new();
    for i in 0..3 {
        let data=data.clone();
        let handle = thread::spawn(move|| {
            let mut data=data.lock().unwrap();
            data[i] +=1;
        });
        threads.push(handle);
    }

    thread::sleep(std::time::Duration::new(1, 0));

    for h in threads {
        h.join().unwrap();
    }

    let r = data.lock().unwrap();
    println!("data={:?}, {:?}", data, r);

    let mut i = 0;
    for v in r.iter() {
        println!("index={}, {}, {}", i, v, r[i]);
        i+=1;
    }
}
