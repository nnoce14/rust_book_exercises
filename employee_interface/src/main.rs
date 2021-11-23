use std::{io, collections::HashMap};

fn main() {
    println!("\nWelcome to the Employee Interface!");
    print_menu();

    let mut choice = get_user_choice();
    let mut employees: HashMap<String, Vec<String>>  = HashMap::new();

    while choice != 4 {

        match choice {
            1 => add_employee_to_system(&mut employees),
            2 => print_employees_in_department(&employees),
            3 => print_all_employees(&employees),
            _ => break,
        }

        print_menu();
        choice = get_user_choice();
    }
}

fn add_employee_to_system(map: &mut HashMap<String, Vec<String>>) {
    println!("\nEnter the add employee command below: [i.e. 'Add <employee> to <department>']");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess = guess.trim().to_string();
    let tokens: Vec<String> = guess.split_whitespace().map(str::to_string).collect();
    
    let department = tokens.get(3).unwrap().to_string();
    let name = tokens.get(1).unwrap().to_string();

    if map.contains_key(&department) {
        let vector = map.get_mut(&department).unwrap();
        vector.push(name);
        println!("Vector: {:?}", vector);
    } else {
        map.insert(department, vec![name]);
    }

}

fn print_menu() {
    println!("\nPlease select an option below\n1) Add an employee to the system\n2) Print employees in a specific department\n3) Print all employees by department alphabetically\n4) Quit\n");
}

fn get_user_choice() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 4,
    }
}

fn print_employees_in_department(map: &HashMap<String, Vec<String>>) {
    println!("\nEnter the department you want to see below:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess = guess.trim().to_string();
    match map.get(&guess) {
        Some(x) => print_employees(&x),
        None => println!("Cant find shit"),
    }
}

fn print_employees(vec: &Vec<String>) {
    for employee in vec {
        println!("> {}", employee);
    }
    println!("");
}

fn print_all_employees(map: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<String> = map.keys().cloned().collect();
    departments.sort();

    for department in &departments {
        println!("\n{}", department);
        if let Some(names) = map.get(department) {
            print_employees(&names);
        }
    }
}
