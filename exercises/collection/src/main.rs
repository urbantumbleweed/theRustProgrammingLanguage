fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

   let third: &i32 = &v[2];
   print!("The third element is {}", third);

   match v.get(10) {
    Some(i) => println!("the option is {}", i),
    None => println!("there is nothing here"),
   }
}
