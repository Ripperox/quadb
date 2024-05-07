use std::io;

fn main() {
    println!("Enter a string to check if palindrome");
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let trim_str =str.trim();
    let char_vector: Vec<char> = trim_str.chars().collect();

    if palindrome(&char_vector) {
        println!("Palindrome");
    } else {
        println!("Not a Palindrome");
    }
}
fn palindrome(char_vector : &[char]) -> bool {
    let length = char_vector.len();
    for i in 0..(length / 2) + 1 {
        if char_vector[i] != char_vector[length - i - 1] {
            return false;
        }
    }
    return true;
}
