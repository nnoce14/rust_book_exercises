use std::io;

fn main() {

    println!("Nth Fibonnaci Number Generator!");
    println!("Enter the number in the sequence that you want to see [i.e. 8 for 8th term]:");

    let mut n = String::new(); 

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("invalid input");

    println!("You asked for term number {}.", n);
    println!("Term {} is {}", n, fib(n));
        
}

fn fib(n: u32) -> u32 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}
        

