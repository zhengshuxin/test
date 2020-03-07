use std::thread;
static mut VAR: i32 = 5;

fn main() {
    let child = thread::spawn(move || {
            unsafe {
                VAR += 1;
                println!("var={}", VAR);
            }
    });

    child.join().unwrap();
    //let _ = child.join();

    unsafe {
        println!("VAR={}", VAR);
    }
}
