fn main() {
    let mut s1: String = String::from("hello");
    
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &mut s1;
    
    println!("{}, {}, {}", r1, r2, r3);
}