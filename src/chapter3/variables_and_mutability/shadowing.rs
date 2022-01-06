pub fn run() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in this scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    spaces_example();
}

fn spaces_example() {
    let spaces = "    ";
    let spaces = spaces.len();

    println!("Amount of spaces: {}", spaces);
}
