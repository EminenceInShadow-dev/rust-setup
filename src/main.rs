use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <list of integers>", args[0]);
        return;
    }

    let numbers: Vec<i32> = args[1..]
        .iter()
        .map(|arg| arg.parse().expect("Please provide a list of integers"))
        .collect();

    let avg = average(&numbers);
    println!("The average is: {}", avg);
}

// Create function that returns the average of the numbers
fn average(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}