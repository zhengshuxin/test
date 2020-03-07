use std::sync::Mutex;
use std::thread;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    /*
    let mutexed_number = Rc::new(Mutex::new(5));
    let mutexed_number_clone_1 = mutexed_number.clone();

    let h = thread::spawn(move || {
        let number = mutexed_number.lock().unwrap();
        println!("1 Rc/Mutexed number plus one equals {}", *number + 1);
    });
    h.join().unwrap();
    */

    let n = Rc::new(10u32);
    let n1 = n.clone();
    let n2 = n.clone();
    println!("n={}, n1={}, n2={}", n, n1, n2);

    let n3 = Rc::downgrade(&n);
    println!("n3={:?}", n3);

    let n4: Option<Rc<_>> = n3.upgrade();
    println!("n4={}", n4.unwrap());

    let numbers: Vec<_> = (0..10u32).collect();
    println!("numbers={:?}", numbers);

    let shared_numbers = Arc::new(numbers);

    let mut threads: Vec<_> = Vec::new();
    for i in 0..10 {
        let child_numbers = shared_numbers.clone();
        let h = thread::spawn(move || {
            let local = &child_numbers[..];
            println!(">>{:?}, {}<<", local, local[i]);
        });

        threads.push(h);
    }

    for h in threads {
        h.join().unwrap();
    }
}
