use std::io;

fn main() {
    println!("Enter a number to check if it is prime:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num = input.trim().parse::<u32>();
    if let Ok(number) = num {
        if prime(number) {
            println!("Prime number");
        } else {
            println!("Not Prime number");
        }
    } else {
        println!("Please enter a valid number.");
    }
}
fn prime(n : u32) -> bool {
    for i in 2..n/2{
        if n%i == 0{
            return false;
        }
    }
    return true;
}
