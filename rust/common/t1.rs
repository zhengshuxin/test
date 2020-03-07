fn arithmetic(m: i32, n: i32) {
    // 加法运算,有溢出风险
    println!("{}", m + n);
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32
}

fn default() -> Point3 {
    Point3 { x: 0, y : 0, z : 0 }
}

fn test() {
    let p1 = Point3 { x: 5, ..default() };
    let p2 = Point3 { y: 2, x: 4, ..p1 };

    println!("x={}, y={}, z={}", p2.x, p2.y, p2.z);
}

struct Point4 (
    i32, i32, i32,
);

fn test1() {
    let p = Point4 (0, 1, 2);
    println!("p={}, {}, {}", p.0, p.1, p.2);

    let p = Point4 { 0 : 2, 1 : 3, 2 : 4 };
    println!("p={}, {}, {}", p.0, p.1, p.2 );
}

fn test2() {
    type I = i32;

    fn f1(v : I) { println!("v={}", v); }
    fn f2(v : i32) { println!("v={}", v); }

    let v : i32 = 0;
    f1(v);
    f2(v);
}

enum Number {
    Int(i32), Float(f32)
}

fn read_num(num: &Number) {
    match num {
        &Number::Int(v) => println!("file={}, line={}, Int v={}", file!(), line!(), v),
        &Number::Float(v) => println!("Float v={}", v),
    }
}

fn test3() {
    let n: Number = Number::Int(100);
    read_num(&n);
    match n {
        Number::Int(v) => println!("Int v={}", v),
        Number::Float(v) => println!("Float v={}", v),
    }

    let mut n1: i32 = Number::Int as i32;
    n1 = 100;
    println!("test3=>n={}", n1);

    let v = [1, 2, 3, 4, 5, 6];
    let v: Vec<Option<&i32>> = v.iter().map(Some).collect();
    println!("v=={:?}, {:?}", v, v[0]);
    match v[0] {
        Some(1) => println!(">>some 1 {}<<<", v[0].unwrap()),
        Some(2) => println!(">>some 2<<<"),
        Some(3) => println!(">>some 3<<<"),
        _ => println!("other"),
    }
}

fn main() {
    test();
    test1();
    test2();
    test3();

    let m : i8 = 120;
    let n : i8 = 120;
    arithmetic(m as i32, n as i32);

    let p = (1, 2);
    let (a, b) = p;
    println!("a={}, b={}, p={:?}, {}, {}", a, b, p, p.0, p.1);
    println!("size of i8 {}", std::mem::size_of::<i8>());
    println!("size of char {}", std::mem::size_of::<char>());
    println!("size of u8 {}", std::mem::size_of::<u8>());
    println!("size of () {}", std::mem::size_of::<()>());

    let p : Point = Point{x:1, y:2};

    println!("size of Point {}, p.x={}, p.y={}",
            std::mem::size_of::<Point>(), p.x, p.y);
    move_point(p);

    let x = 1;
    let y = 2;
    let mut p : Point = Point { x, y };
    p.x = 3;
    p.y = 4;

    println!("size of Point {}, p.x={}, p.y={}",
            std::mem::size_of::<Point>(), p.x, p.y);

    let Point{x : px, y : py } = p;
    println!("x={}, y={}", px, py);
}

fn move_point(p : Point) {
    println!("move_point->size of Point {}, p.x={}, p.y={}",
            std::mem::size_of::<Point>(), p.x, p.y);
}
