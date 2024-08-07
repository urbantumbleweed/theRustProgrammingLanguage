use std::collections::HashMap;
use std::io;

pub fn main() {

    //given a list of integers
    // use a vector and return the mean, median, and mode
    // assume initially that the list is hardcoded
    // later, refactor to code to accept command line arguments
    const DONE: &str = "done";
    let mut input = String::new();
    let mut list: Vec<i32> = Vec::new();
    let prompt = format!("Enter integers separated by the return key. When complete type '{}' and press return:", DONE);
    println!("{}", prompt); 

    
    while input != DONE {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == DONE {
            break;
        }
        let number: i32 = input.trim()
            .parse()
            .expect("Please enter a valid integer");
        list.push(number);
    }



    // let mut list: Vec<i32> = input.to_vec();
    let sum: &i32 = &list.iter().fold(0, |sum, i| sum + i);
    let average: f64 = *sum as f64/ list.len() as f64;
    println!("The average is: {}. Sum is: {}", &average, &sum);

    // calculate the median

    list.sort_unstable();
    let middle_index = get_middle_index(&list);
    let median: &i32 = &list[*&middle_index];
    println!("The median of the set is: {}.", &median);

    // calculate the mode  
    let mode = get_mode(&list);
    println!("The mode of the set is: {}.", &mode);
}

fn get_middle_index<T>(list: &Vec<T>) -> usize {
    const HALF: i32 = 2;
    const ZERO_BASED_INDEX_ADJUSTMENT: i32 = 1;
    const EVEN_NUMBER: i32 = 0;
    let length: i32 = list.len() as i32;

    if &length % HALF == EVEN_NUMBER {
        ((&length / HALF) - ZERO_BASED_INDEX_ADJUSTMENT) as usize
    } else {
        (&length / HALF) as usize
    }
}

fn get_mode(list: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    let mut current_mode: i32 = *list.get(0).unwrap();
    let mut highest_count: i32 = 0;
    for &value in list.iter() {
        let count = counts.entry(value).or_insert(0);
        *count += 1;
        if *count > highest_count {
            highest_count = *count;
            current_mode = value;
        }
    }
    current_mode
}