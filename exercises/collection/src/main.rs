fn main() {

    //given a list of integers
    // use a vector and return the mean, median, and mode
    // assume initially that the list is hardcoded
    // later, refactor to code to accept command line arguments

    let input: [i32; 10] = [9, 23, -2, 87, 12, -1, 34, 25, 9, 15];

    let mut list: Vec<i32> = Vec::new();
    fn 
    for i in &input {
        list.push(*i);
    }
    let iterator = list.iter();
    let mut sum: i32 = 0;
    for v in iterator {
      sum += *v;
    }
    let average: f64 = sum as f64/ list.len() as f64;
    println!("The average is: {}. Sum is: {}", &average, &sum);


}
