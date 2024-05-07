use std::io;

fn main() {
    println!("Enter a string to find the shortest word:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();  // Trim to remove the newline character from input
    let shortest = find_shortest_word(input);
    println!("The shortest word is '{}'", shortest);
}

fn find_shortest_word(s: &str) -> String {
    let words = s.split_whitespace();
    let mut shortest_word = String::new();
    let mut shortest_length = usize::MAX;  // Use usize::MAX for maximum initial value

    for word in words {
        let word_length = word.len();
        if word_length < shortest_length {
            shortest_word = word.to_string();  // Convert word slice to String
            shortest_length = word_length;
        }
    }

    shortest_word  // Return the shortest word as a String
}
