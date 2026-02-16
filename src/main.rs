use colored::*;
use std::io;
use std::io::Write;

// internal modules are being imported
mod palindrome;

fn main() {
    let mut input = String::new();

    loop {
        print!("{}", "Please enter a string: ".bold().blue());
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Somesome went wrong");

        if !input.trim().is_empty() {
            break;
        }
    }
    
    // currently it is what it is. i'd like to return both the bool and the str and then use it without making a copy.
    let res_palindrome: bool = palindrome::palindrome(input.clone());
    
    if res_palindrome {
        println!("{}", format!("\"{}\" IS a palindrome", input.trim()).green());
    } else {
        println!("{}", format!("\"{}\" ISN'T a palindrome", input.trim()).red());
    }
}
