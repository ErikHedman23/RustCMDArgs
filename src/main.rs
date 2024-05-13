// Command line arguments:
// These can be File Paths or Configuration settings
// To read cmd arguments, we use std::env::args, which returns an iterator over arguments passed to
// the program
// First arg is traditionally the executable path
// -- are often used for configuration settings, usually called flags
// It is your responsibility to parse --flags so that your program can determine what they are
// meant for.
use std::env;

fn main() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    // if you want to get a specific arg from the iterator, you can use the nth():
    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}
