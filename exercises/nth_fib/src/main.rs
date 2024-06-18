use std::io;

fn main() {
    let mut n: String = String::new();

    println!("Enter the nth fibonacci number: ");
    io::stdin().read_line(&mut n).unwrap();

    let n: u32 = n.trim().parse().unwrap();
    let nth_fib = fib_dp(n);
    println!("The {}th fibonacci number is {}", n, nth_fib);
}

fn fib(n: u32) -> u128 {
    let mut fib = vec![0, 1];
    for i in 2..=n {
        fib.push(fib[(i - 1) as usize] + fib[(i - 2) as usize]);
    }
    fib[n as usize]
}
