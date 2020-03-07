fn main() {
    let s = " hello world! ";
    println!("s={}, {}", s, s.contains("hello"));
    println!("starts_with={}, ends_with={}",
            s.starts_with("he"), s.ends_with("llo"));

    let t = s.replace("l", "xx");
    println!("{}, {}", t, s);

    let t = s.trim();
    println!("[{}]", t);

    let t = s.trim_start().trim_end();
    println!("[{}]", t);

    let s = "4";
    let n: i32 = s.parse().unwrap();
    println!("{}", n);

    let s = "中国，hello world!";
    println!("{:?}", s.get(..).unwrap());
    println!("{:?}", s.get(0..6).unwrap());

    let mut ss = String::from(s);
    ss += " hello world";
    println!("ss={}", ss);

    println!("ss={:p}, {:p}", ss.as_ptr(), &ss);
}
