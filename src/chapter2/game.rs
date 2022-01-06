use rand;
use std::io;
use std::io::Write;

pub fn run() {
    println!("Guess the number!");

    let random_number = rand::random::<u8>() % 50;

    loop {
        let user_input = input("Type in a number: ")
            .parse::<u8>()
            .expect("Could not convert input to an integer");

        if user_input == random_number {
            break;
        }

        if user_input > random_number {
            println!("Too high! Try a lower one! ;)\n");
        } else {
            println!("Too low! Try a higher one! ;)\n");
        }
    }

    println!("You guessed it right! The number was {}. xD", random_number);
}

fn flush_stdout() {
    match io::stdout().flush() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };
}

fn input(message: &str) -> String {
    print!("{}", message);
    flush_stdout();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_owned()
}
