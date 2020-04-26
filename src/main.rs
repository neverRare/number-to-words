use number_to_words::wordify;
use std::io;

fn main() {
    println!("Enter a number to spell out (enter \"exit\" to exit)");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let input: u64 = match input.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let output = match wordify(input, 0) {
            Ok(words) => words,
            Err(_) => continue,
        };
        println!("= {}", output);
    }
}
