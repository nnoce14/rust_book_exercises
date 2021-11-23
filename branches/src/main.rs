use std::io;

fn main() {
    println!("Enter a number below:");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => do_something(),
    };

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    let number = if number < 10 { 10 } else { 5 };

    println!("The value of the number is: {}", number);
}

fn do_something() -> i32 {
    0
}
