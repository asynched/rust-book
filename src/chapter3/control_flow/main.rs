pub fn run() {
    if_else();
    loops();
    returning_from_loop();
    while_loops();
    looping_through_collection();
    looping_through_collection_reversed();
}

fn looping_through_collection() {
    let source: Vec<i32> = (0..5).collect();

    for item in source {
        println!("Item: {}", item);
    }
}

fn looping_through_collection_reversed() {
    println!("Reverse counter...");

    for i in (0..=3).rev() {
        println!("{}", i);
    }
}

fn loops() {
    let mut counter = 0;

    loop {
        println!("Again...");
        counter += 1;

        if counter > 10 {
            break;
        }
    }
}

fn while_loops() {
    let mut counter = 3;

    while counter != 0 {
        println!("Decreasing... Current value is: {}", counter);
        counter -= 1;
    }
}

fn returning_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of the loop is: {}", result);
}

fn if_else() {
    let condition = true;
    let number = if condition { 5 } else { 10 };

    println!("The value of number is: {}", number);
}
