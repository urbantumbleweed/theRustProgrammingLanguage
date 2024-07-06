fn main() {

    //given a list of integers
    // use a vector and return the mean, median, and mode
    // assume initially that the list is hardcoded
    // later, refactor to code to accept command line arguments

    let input: [i32; 11] = [9, 23, -2, 87, 12, -1, 34, 25, 9, 15, 17];

    let mut list: Vec<i32> = input.to_vec();
    let sum: &i32 = &list.iter().fold(0, |sum, i| sum + i);
    let average: f64 = *sum as f64/ list.len() as f64;
    println!("The average is: {}. Sum is: {}", &average, &sum);

    // calculate the median

    list.sort_unstable();
    println!("Sorted list: {:#?} with length: {}", list, list.len());
    let middle_index: usize = if list.len() % 2 == 0 {
        (list.len() / 2) - 1
    } else {
        list.len() / 2
    };
    let median: &i32 = &list[*&middle_index];
    println!("The median of the set is: {}. Middle index: {}", &median, &middle_index);
}
