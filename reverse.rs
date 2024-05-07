use std::io;

fn main() {
    println!("Enter a string to reverse it");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed_str = input.trim();
    let char_vector: Vec<char> = trimmed_str.chars().collect();

    let rev = reverse(&char_vector);

    println!("{}", rev);
}

fn reverse(char_vector: &[char]) -> String {
    let mut rev = String::new();
    for i in (0..char_vector.len()).rev() {
        rev.push(char_vector[i]);
    }
    rev
}
