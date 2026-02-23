use colored::*;
use std::io;
use std::io::Write;

// use crate::binary_search::binary_search;

// internal modules are being imported
mod binary_search;
mod fib;
mod palindrome;

fn take_input() -> String {
    let mut input: String = String::new();

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
    return input;
}

fn main() {
    println!("{}", "===PALINDROME===".bold().magenta());
    let input: String = take_input();

    // NOTE: currently it is what it is. i'd like to return both the bool and the str and then use it without making a copy.
    let res_palindrome: bool = palindrome::palindrome(input.clone());

    if res_palindrome {
        println!(
            "{}",
            format!("\"{}\" IS a palindrome", input.trim()).green()
        );
    } else {
        println!(
            "{}",
            format!("\"{}\" ISN'T a palindrome", input.trim()).red()
        );
    }

    println!("{}", "===FIBONACCI SEQ===".bold().magenta());

    let mut input: String = String::new();

    print!(
        "{}",
        "Please enter how many entries you want in the fibonacci sequence: "
            .bold()
            .blue()
    );
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    let fib_input: u64 = input.trim().parse().expect("Please enter a valid number");

    fib::fib(fib_input);

    println!("{}", "===BINARY SEARCH===".bold().magenta());

    // we are reusing the input variable so we must clear it before we do anything
    input.clear();
    
    print!("{}", "What would you like to search: ".bold().blue());
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Somesome went wrong");

    let _find: i32 = input.trim().parse().expect("Not a valid number");

    match binary_search::binary_search(_find) {
        Some(index) => println!("Found at index {}", index),
        None => println!("Not found"),
    }
}
