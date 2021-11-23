fn main() {
    // string literal &str
    let s = "hello";

    println!("{}", s);

    // String 
    let mut s = String::from("hello");

    s.push_str(", world!"); // appends a literal to a String

    println!("{}", s);

    {
        let mut name = String::from("Nicholas"); // name is valid from this point forward

        name.push_str(" Ryan Noce");

        println!("My name is {}.", name);
    }                                           // this scope is now over, and name is no
                                                // longer valid
    { 
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}\n", s1, s2);

        let s = String::from("hello");
    
        takes_ownership(s);

        let x = 5;

        makes_copy(x);
    }

    {
        let s1 = gives_ownership();

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);
    }

    {
        let s1 = String::from("hello");

        //let (s2, len) = calculate_length(s1);
        
        let len = calculate_length(&s1);

        println!("\nThe length of {} is {} characters long.", s1, len);
    }

    {
        let mut s = String::from("hello");

        change(&mut s);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string

}

fn takes_and_gives_back(a_string: String) -> String {

    a_string

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
