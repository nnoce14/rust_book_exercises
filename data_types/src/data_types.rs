fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let z: u8 = 255;

    println!("The value of z is {}", z);
    
    let t = true;

    let f: bool = false;

    let c = 'z';

    let z = 'Z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    
    println!("The value of y is {}", y);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [6, 7, 8, 9, 10];

    let c = [3; 5];

    println!("{:?}", c);

    let first = a[0];

    let second = a[1];
}

