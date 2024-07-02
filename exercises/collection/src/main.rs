fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i = *i + 50;
    }

    for i in &v {
        println!("The first line is {}", i)
    }

;}
