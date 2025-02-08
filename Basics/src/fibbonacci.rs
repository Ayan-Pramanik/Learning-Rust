fn main() {
    let x:i32 = 4;
    println!("{}",fib(x));
}

fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return 0;
    }else if num == 1 {
        return 1;
    }

    for _i in 1..num - 1 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}