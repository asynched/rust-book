use std::io;

pub fn run() {
    integers();
    tuples();
    arrays();
    accessing_array_items();
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];

    println!("Array: {:?}", a);
}

fn accessing_array_items() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter a number: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let index = input
        .trim()
        .parse::<usize>()
        .expect("Failed to convert input to usize");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn tuples() {
    let red: (u8, u8, u8) = (255, 0, 0);

    println!("Red: {:?}", red);

    let (r, g, b) = red;

    println!("RGB = {}, {}, {}", r, g, b);

    let r = red.0;

    println!("Amount of red: {}", r);
}

fn integers() {
    println!("Max value of an unsigned 32 bit integer: {}", u32::MAX);
    println!("Max value of an unsigned 64 bit integer: {}", u64::MAX);
    println!("Max value of a signed 32 bit integer: {}", i32::MAX);
    println!("Max value of a signed 64 bit integer: {}", i64::MAX);
}
