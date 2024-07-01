fn main() {
   let a: [&str;5] = [
    "apple", 
    "bananna", 
    "strawberry", 
    "blackberry", 
    "orange"
   ];

   for element in a.iter() {
        println!("{}", element);
   }
}
