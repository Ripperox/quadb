use std::io;

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_current = arr[0];
    let mut max_global = arr[0];

    for &value in arr.iter().skip(1) {
        max_current = i32::max(value, max_current + value);
        if max_current > max_global {
            max_global = max_current;
        }
    }

    max_global
}

fn main() {
    let mut input = String::new();
    println!("Enter the size of the array:");
    io::stdin().read_line(&mut input).unwrap();
    let x = str_to_int(&input);
    let mut array1 = Vec::new();

    for _ in 0..x {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num = str_to_int(&input);
        array1.push(num as i32); 
    }

    let max_sum = max_subarray_sum(&array1);
    println!("The maximum subarray sum is: {}", max_sum);
}

fn str_to_int(input: &str) -> u32 {
    input.trim().parse().expect("Please enter a valid number")
}
