fn main() {
    let mut count = 1;
    let result = loop {
        if count > 100 {
            break count;
        }
        println!("Hello, world! Iteration #{}", count);
        count += 1;
    };

    println!("{}", result);
}
