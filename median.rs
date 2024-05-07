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

    match median8i(&array, target) {
        Some(index) => println!("Found number at index {}", index),
        None => println!("Not found"),
    }
}

fn median(arr: &[u32], target: u32) -> Option<usize> {
    
}

fn str_to_int(input: &str) -> u32 {
    input.trim().parse().expect("Please enter a valid number")
}
