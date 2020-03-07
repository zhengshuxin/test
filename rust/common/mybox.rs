use std::ops::Deref;

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("name is {}", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    println!("x={}, y={}", x, *y);

    let s: MyBox<String> = MyBox::new(String::from("hello world"));
    println!("MyBox s={}", *s);
    hello(&s);
}
