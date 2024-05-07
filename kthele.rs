use std::io;

fn kth_smallest(mut arr: Vec<i32>, k: usize) -> Option<i32> {
    if arr.len() < k || k == 0 {
        None
    } else {
        arr.sort();
        Some(arr[k - 1])
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter the size of the array:");
    if io::stdin().read_line(&mut input).is_err() {
        println!("Error reading size of array.");
        return;
    }
    let x = str_to_int(&input);

    let mut array = Vec::new();

    for _ in 0..x {
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading an array element.");
            return;
        }
        let num = input.trim().parse();
        match num {
            Ok(num) => array.push(num),
            Err(_) => {
                println!("Please enter a valid number.");
                return;
            }
        }
    }

    println!("Enter k value:");
    input.clear();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Error reading k value.");
        return;
    }
    let k = str_to_int(&input);

    match kth_smallest(array, k) {
        Some(value) => println!("The {}-th smallest element is {}", k, value),
        None => println!("Invalid input"),
    }
}

fn str_to_int(input: &str) -> usize {
    input.trim().parse().expect("Please enter a valid number")
}
