use std::io;

fn main() {
    println!("Enter a word to be converted to pig latin:");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    guess = guess.trim().to_string();
    
    println!("\nThe word {} translated to pig latin is: {}", 
        guess, convert_to_pig_latin(&guess)
    );
}

fn convert_to_pig_latin(string: &String) -> String {
   let mut chars = string.chars().peekable();
   let mut new_s = String::new();
   while let Some(c) = chars.next() {
       let suffix = match c {
           'a' | 'e' | 'i' | 'o' | 'u' => {
               new_s.push(c);
               String::from("-hay")
           }
           'a'..='z' | 'A'..='Z' => {
               format!("-{}ay", c)
           }
           _ => {
               new_s.push(c);
               continue;
           }
       };

       while let Some(&c) = chars.peek() {
           match c {
               'a'..='z' | 'A'..='Z' => {
                   chars.next();
                   new_s.push(c);
               }
               _ => break,
           }
       }

       new_s += &suffix;
   }
   new_s
}
