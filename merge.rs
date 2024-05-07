use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter size of the first array:");
    io::stdin().read_line(&mut input).unwrap();
    let x = str_to_int(&input);
    let mut array1 = Vec::new();

    for _ in 0..x {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num = str_to_int(&input);
        array1.push(num as i32); 
    }

    input.clear();
    println!("Enter size of the second array:");
    io::stdin().read_line(&mut input).unwrap();
    let y = str_to_int(&input);
    let mut array2 = Vec::new();

    for _ in 0..y {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num2 = str_to_int(&input);
        array2.push(num2 as i32);
    }


    let merged = merge_sorted_arrays(&array1, &array2);
    println!("Merged Array: {:?}", merged);
}

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

fn str_to_int(input: &str) -> u32 {
    input.trim().parse().expect("Please enter a valid number")
}
