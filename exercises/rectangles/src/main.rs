fn main() {
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels", area(width1, height1));
}

fn area(rect: (u32, u32)) -> u32 {
    let (w, h) = rect;
    w * h
}
