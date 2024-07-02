#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.1412),
        SpreadsheetCell::Text(String::from("Hello World"))
    ];

    for r in &row {
        match r {
            SpreadsheetCell::Int(i) => println!("Int is {}", i),
            SpreadsheetCell::Float(n) => println!("Float is {}", n),
            SpreadsheetCell::Text(s) => println!("{}", s),
        }
    }
;}
