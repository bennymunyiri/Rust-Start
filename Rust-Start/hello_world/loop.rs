use std::thread::sleep;
use std::time::Duration;

fn print_sequence(prefix: &str, current_char: char) {
    for c in 'a'..=current_char {
        println!("{}{}", prefix, c);
        sleep(Duration::from_millis(50)); // Introduce a delay of 400 milliseconds
    }
}

fn main() {
    let target = "hello world";
    let mut prefix = String::new();

    for ch in target.chars() {
        print_sequence(&prefix, ch);
        prefix.push(ch);
    }

    println!("{}", target);
}
