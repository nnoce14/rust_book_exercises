use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_,_> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Green");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, transferred ownership to map
    // println!("Name: {}, Value: {}", field_name, field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("Score: {}", score.unwrap());

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a Value in HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("\n{:?}", scores);

    // Only Inserting a Value if the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("\n{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world hello world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
