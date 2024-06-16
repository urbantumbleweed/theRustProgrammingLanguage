fn main() {
    let reference_to_nothing: String = dangle();
    
    println!("{}", reference_to_nothing)
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}