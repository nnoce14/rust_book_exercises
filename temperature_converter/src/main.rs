use std::io;

fn main() {
    println!("Welcome to the Fahrenheit to Celisus Converter!");
    println!("Please enter a temperature in Fahrenheit below:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f32 = temperature.trim().parse().expect("invalid input");

    println!("\nThe temperature in Fahrenheit is {:.2} degrees F.", temperature);

    let celsius_temperature = fahrenheit_to_celsius(temperature);

    println!("The temperature in Celsius is {:.2} degrees C.", celsius_temperature);
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) * (5.0/9.0)
}

