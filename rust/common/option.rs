
enum Option<T=i32> {
    None, Some(T)
}

impl <T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => panic!("None"),
//            Option::None => default,
        }
    }

    fn hello(&self) {
        println!("hello!");
    }
}

/*
fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    match option {
        Option::None => default,
        Option::Some(val) => val,
    }
}
*/

fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Option::Some(offset);
        }
    }

    Option::None
}

/*
fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        Option::None => None,
            Option::Some(val) => Some(f(val)),
    }
}

fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i + 1..])
}
*/

/*
enum Oo<T> {
    Some(T), None
}

impl Oo<T> {
    fn unwrap(self) -> T {
        match self {
            Oo::Some(val) => val,
            Oo::None => panic!("None")
        }
    }

    fn Debug(self) -> T {
        match self {
            Oo::Some(val) => val,
            Oo::None => panic!("None")
        }
    }
}
*/

struct S<T> {
    data: T
}

struct Num<T> {
    data: Option<T>
}

fn main() {
    let s: &str = "file.ext";
    let c: char = '.';
    match find(s, c) {
        Option::None => println!("None"),
        Option::Some(i) => println!("i={}, {}, s={}", i, i.to_string(), &s[i + 1..])
    }

    let t = find(s, c);
    t.hello();
    let i = t.unwrap();
    println!("t={}, s={}", i, &s[i..]);

    let a: Option<i32> = Option::Some(64);
    println!("a={}", a.unwrap());
    let b: Option<f32> = Option::Some(1.1);
    println!("b={}", b.unwrap());
    let _c: Option<i32> = Option::None;
//    println!("c={}", c.unwrap());

    let d = S { data: 100 };
    println!("d={}", d.data);

    let e: S<i32> = S { data: 1000 };
    println!("e={}", e.data);

    let f: S<bool> = S { data: false };
    println!("f={}", f.data);

    let _g: Num<i32> = Num { data: Option::Some(100) };
//    println!("g={:#?}", g.data);
}
