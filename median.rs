use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter size of array");
    io::stdin().read_line(&mut input).unwrap();
    let x = str_to_int(&input);
    let mut array = Vec::new();

    for _ in 0..x {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num = str_to_int(&input);
        
        array.push(num);
    }

    if let Some(med) = median(&array) {
        println!("The median is: {}", med);
    } else {
        println!("The array is empty.");
    }
}

fn median(arr: &[u32]) -> Option<f64> {

    let len= arr.len();
    if len % 2 == 1 {
        Some(arr[len / 2] as f64)
    } else {
        Some((arr[len / 2 - 1] as f64 + arr[len / 2] as f64) / 2.0)
    }
}

fn str_to_int(input: &str) -> u32 {
    input.trim().parse().expect("Please enter a valid number")
}
