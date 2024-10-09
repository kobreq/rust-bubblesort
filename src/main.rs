use std::io;

fn sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    println!("Type in the numbers you want ordenated, separated by spaces.");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please, type only numbers."))
        .collect();

    println!("Given numbers: {:?}", numbers);
    sort(&mut numbers);
    println!("Ordenated numbers: {:?}", numbers);
    println!("Thank you for testing my program! :)");
}
