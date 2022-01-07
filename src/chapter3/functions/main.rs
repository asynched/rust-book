pub fn run() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    function_body();

    let x = plus_one(5);
    println!("5 + 1 = {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn function_body() {
    let y = {
        let x = 0;
        x + 10
    };

    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
