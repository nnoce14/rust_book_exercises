use std::fs::{self, File};
use std::io::{self, Read};
use std::net::IpAddr;

fn main() {
    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    
    if let Ok(username) = read_username_from_file_shortcut() {
        println!("Username: {}", username);
    } else {
        println!("Error with file 'hello.txt'...");
    }

    /*
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    */

    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
