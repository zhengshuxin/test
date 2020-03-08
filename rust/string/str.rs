fn main() {
    let s = "hello world";
    println!("s={}, {}, len={}", s, s.contains("hello"), s.len());
    println!("starts_with={}, ends_with={}",
            s.starts_with("he"), s.ends_with("ld"));

    let t = s.replace("l", "xx");
    println!("{}, {}", t, s);

    let s = " hello world ";
    let t = s.trim();
    println!("t=[{}]", t);

    let s = " hello world ";
    let t = s.trim_start().trim_end();
    println!("t=[{}]", t);

    let n = 100_000_000;
    let f = 100.11;
    println!("n={}, {}, f={}, {}", n, n.to_string(), f, f.to_string());

    let s = "4";
    let s2 = "100.11";
    let n: i32 = s.parse().unwrap();
    let n2: f32 = s2.parse().unwrap();
    println!("n={}, n2={}", n, n2);


    let s = "中国，hello world!";
    println!("{:?}", s.get(..).unwrap());
    println!("{:?}", s.get(0..6).unwrap());

    let mut ss = String::from(s);
    ss += " hello world";
    println!("ss={}", ss);

    println!("ss={:p}, {:p}", ss.as_ptr(), &ss);
}
