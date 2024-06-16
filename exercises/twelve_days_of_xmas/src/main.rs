const DAYS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
const LOVER_LINE: &str = "my true love sent to me";
const SINGULAR_FIRST_LINE: &str = "A Partridge in a Pear Tree";
const LINES: [&str; 12] = [
    "And a Partridge in a Pear Tree",
    "Two Turtle Doves",
    "Three French Hens",
    "Four Calling Birds",
    "Five Gold Rings",
    "Six Geese a Laying",
    "Seven Swans a Swimming",
    "Eight Maids a Milking",
    "Nine Ladies Dancing",
    "Ten Lords a Leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers Drumming"
];

fn main() {
    println!("On the {} day of Christmas, \n{}", DAYS[0], LOVER_LINE);
    println!("{}.", SINGULAR_FIRST_LINE);
    println!("\n");


    for day in 1..DAYS.len() {
        println!("On the {} day of Christmas, \n{}", DAYS[day], LOVER_LINE);
        for i in (0..day).rev() {
            println!("{},", LINES[i]);
        }
        println!("\n");
    }
}
