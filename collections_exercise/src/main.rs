use std::collections::HashMap;

fn main() {

    let mut v = vec![20, 30, 45, 15, 20, 35, 65, 5, 20, 45, 20, 20];

    println!("\nThe vector contains {:?}.\n", v);

    println!("The mean of the vector is {:.2}", get_mean(&v));

    println!("The median of the vector is {}", get_median(&mut v));

    let (mode, frequency) = get_mode(&mut v);

    println!(
        "The mode of the vector is {} and it occurs {} times\n",
        mode, frequency
    );

}

fn get_mean(v: &Vec<u32>) -> f32 {
    let mut total: f32 = 0.0;
    for elem in v {
        total += *elem as f32;
    }
    total / v.len() as f32
}

fn get_median(v: &mut Vec<u32>) -> f32 {
    v.sort_unstable();
    if v.len() % 2 == 0 {
        let temp = vec![v[v.len()/2-1], v[v.len()/2]];
        get_mean(&temp)
    } else {
        v[v.len()/2] as f32
    }
}

fn get_mode(v: &mut Vec<u32>) -> (u32, u32) {
    let mut map = HashMap::<u32, u32>::new();        
    for elem in v {
        let count = map.entry(*elem).or_insert(0);
        *count += 1;
    }
    let key = get_key_with_maximum_value(&map);
    (key, *map.get(&key).unwrap())
}

fn get_key_with_maximum_value(map: &HashMap<u32, u32>) -> u32 {
    let mut elem = 0;
    let mut frequency = 0;
    for (key, value) in map.iter() {
        if value > &frequency {
            frequency = *value;
            elem = *key;
        }
    }
    elem
}
