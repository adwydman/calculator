use std::io;

struct Input {
    first_number: i32,
    second_number: i32
}

enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide,
    NoneOp
}

fn options() {
    println!("1: Add");
    println!("2: Multiply");
    println!("3: Subtract");
    println!("4: Divide");
}

fn menu() -> Operation {
    options();

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");

    // shadowing - reassingin a variable
    let choice = match choice.trim() {
        "1" => Operation::Add,
        "2" => Operation::Multiply,
        "3" => Operation::Subtract,
        "4" => Operation::Divide,
        _ => Operation::NoneOp
    };

    choice // the same as `return choice;`  - this is valid, as Rust supports expressive paradigm
}

fn choose_numbers() -> Input {
    let mut first = String::new();
    let mut second = String::new();

    io::stdin().read_line(&mut first)
        .expect("Failed to read line");
    io::stdin().read_line(&mut second)
        .expect("Failed to read line");

    let first: i32 = match first.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let second: i32 = match second.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let numbers = Input { first_number: first, second_number: second};

    numbers // check `choice`
}

// this is a function that takes a function as an argument
// we need to define what this argument is - what arguments it takes and what it returns
fn use_operation<F>(f: F)
where F: Fn(&i32, &i32) -> i32 {
    let numbers = choose_numbers();

    let result = f(&numbers.first_number, &numbers.second_number);
    println!("Result: {}", result);
}

fn run_program() {
    // an infitie loop
    loop {
        let choice = menu();

        // match is an alternative to if/else
        // the documentation says it's more expressive and provides more readibility
        match choice {
            Operation::Add => use_operation(|x, y| x + y),
            Operation::Multiply => use_operation(|x, y| x * y),
            Operation::Subtract => use_operation(|x, y| x - y),
            Operation::Divide => use_operation(|x, y| x / y),
            Operation::NoneOp => continue
        }
    }
}

fn main() {
    run_program();
}
