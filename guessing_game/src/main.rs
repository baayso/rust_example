use std::io;

fn main() {
    println!("Guess the number!");

    print!("Please input your guess.");

    let mut guess = String::new();

    io::stdio().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    print!("You guessed: {}", guess);
}
