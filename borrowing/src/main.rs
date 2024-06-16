fn main() {
    let mut s: String = String::from("hello");
    change(&mut s);

    let s = s;
    let t = &s;
    let t1 = &s;

    println!("{}, {}", t, t1);

}

fn change(s: &mut String) {
    for _ in 0..10 {
        s.push_str("-o");
    }
}