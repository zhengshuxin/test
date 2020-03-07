
fn call_with_one<F>(some_closure: F) -> i32 where F: Fn(i32) -> i32 {
    some_closure(1)
}

fn call_with_one2(some_closure: &Fn(i32)->i32) {
    some_closure(1)
}

fn main() {
    let n = 10;
    let plus  = |x| {
        if x > 0 {
            x + n
        } else {
            x - n
        }
    };

    println!("afster plus={}",  plus(10));
    println!("afster plus={}",  plus(-10));

    let nums = vec![1, 2, 3];
    println!("nums={:?}", nums);

    //let takes_nums = || nums;
    let takes_nums = || &nums;
    println!("takes_nums={:?}, first={}", takes_nums(), takes_nums()[0]);
    println!("nums={:?}", nums);

    let mut num = 5;
    let mut owns_num = move || { num += 5; num };
    let n = owns_num();
    println!("owns_num={}", n);
    println!("first->num={}", num);

    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    println!("second->num={}", num);

    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }
    println!("third->num={}", num);

    let res = call_with_one(|x| x + 2);
    println!("res={}", res);

    let res = call_with_one2(&|x| x + 2);
    println!("res={}", res);
}
