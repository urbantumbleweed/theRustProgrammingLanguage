fn main() {
   let a: [&str;5] = [
    "apple", 
    "bananna", 
    "strawberry", 
    "blackberry", 
    "orange"
   ];

   for (i, &item) in a.iter().enumerate() {
        println!("{} is at index {}", item, i);
   }
}
