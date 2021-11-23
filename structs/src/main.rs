struct User {       
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("nnoce12@gmail.com"),
        username: String::from("nnoce"),
        active: true,
        sign_in_count: 1,
    };

    print_user(&user1);

    let mut user1 = User {
        email: String::from("nnoce12@gmail.com"),
        username: String::from("nnoce"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("nrn26@drexel.edu");
    
    print_user(&user1);

    let user1 = build_user(String::from("nnoce14@gmail.com"), String::from("nnoce14"));

    print_user(&user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    print_user(&user2);

}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("\nHello, world!\nMy username is {}, and my email is {}.\nI am active is {}, and my sign in count is {}.", user.username, user.email, user.active, user.sign_in_count);
}
    
    
