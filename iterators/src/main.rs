use iterators::{shoes_in_size, Shoe};

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    /*
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    */

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    let shoes = vec![
        Shoe {
            size: 10,
            style: "sneaker".to_string(),
        },
        Shoe {
            size: 14,
            style: "dress shoe".to_string(),
        },
        Shoe {
            size: 9,
            style: "bowling shoe".to_string(),
        },
        Shoe {
            size: 10,
            style: "boots".to_string(),
        },
    ];

    let shoes_in_my_size = shoes_in_size(shoes, 10);

    println!("These are the shoes in my size!\n");
    for shoe in shoes_in_my_size {
        println!("{:?}\n", shoe);
    }
}
