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

    println!("Enter a number to search for");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let target = str_to_int(&input);

    match binary_search(&array, target) {
        Some(index) => println!("Found number at index {}", index),
        None => println!("Not found"),
    }
}

fn binary_search(arr: &[u32], target: u32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

fn str_to_int(input: &str) -> u32 {
    input.trim().parse().expect("Please enter a valid number")
}
