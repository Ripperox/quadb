use std::io;

fn main() {

    println!("Enter a string to check if palindrome");
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let Str = str.trim();
    if palindrome(Str){
         println!("Palindrome");
    }
    else{
         println!("Not a Palindrome");
   }
    
}
fn palindrome(str : &str) -> bool{
   let char_vector: Vec<char> = s.chars().collect();
   let length= str.chars().count();
   for i in 0..(length/2)+1{
      if char_vector[i]!= char_vector[length- i +1]{
         return false;
      }
   }
   return true;
}
