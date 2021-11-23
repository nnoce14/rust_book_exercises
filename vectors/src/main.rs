#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {

    let v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // this will crash the program at runtime, the compiler doesnt pick this up
    // let does_not_exist = &v[100];

    // this will return a None instead of crashing unexpectedly
    let does_not_exist = v.get(100);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("\nAdding 50 to all the elements...\n");

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut line_number = 1;
    println!("\nSpreadsheet Elements");
    for elem in &row {
        match elem {
            SpreadsheetCell::Int(value) => println!("{}. {}", line_number, value),
            SpreadsheetCell::Float(value) => println!("{}. {}", line_number, value),
            SpreadsheetCell::Text(value) => println!("{}. {}", line_number, value),
        }
        line_number += 1;
    }
}
