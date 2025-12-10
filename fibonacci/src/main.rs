fn fib(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }

    b
}

fn main() {
    let n = 50;
    let x = fib(n);

    println!("The value of the {n}-th fibonacci is: {x}");
}
