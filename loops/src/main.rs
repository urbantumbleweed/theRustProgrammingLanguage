fn main() {
    let mut count = 1;
    let result = while count < 99 {
        println!("Hello, world! Iteration #{}", count);
        count += 1
    };

    println!("{:#?}", result);
}
