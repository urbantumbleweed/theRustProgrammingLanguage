fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;

    println!("The area of the rectangle is {} square pixels", area(width1, height1));
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}
