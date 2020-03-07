
fn compare_option<T>(first: Option<T>, second: Option<T>) -> bool {
    match(first, second) {
        (Some(..), Some(..)) => true,
        (None, None) => true,
        _ => false
    }
}

/*
impl<T, U> Into<U> for T where U: From<T> {
    fn into(self) -> U {
        U::From(self)
    }
}
*/

use std::cmp::PartialOrd;

//fn max<T: PartialOrd>(a: T, b: T) -> T {
fn max<T>(a: T, b: T) ->T where T: PartialOrd {
    if a > b {
        a
    } else {
        b
    }
}

struct MyT {
    value: i32
}

impl PartialOrd for MyT {
    fn partial_cmp(&self, other: &MyT) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for MyT {
    fn eq(&self, other: &MyT) -> bool {
        self.value == other.value
    }
}

fn main() {
    println!("{}", compare_option::<i32>(Some(100), Some(200)));
    println!("{}", compare_option(Some(100_i32), Some(200_i32)));
    println!("{}", compare_option(Some(100), Some(200)));

    let a: i32 = 1000;
    let b: i32 = 200;
    println!("max={}", max(a, b));

    let a = MyT { value: 100 };
    let b = MyT { value: 200 };
    println!("max={}", max(a, b).value);
}
