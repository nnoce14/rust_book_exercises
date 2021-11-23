#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of rectangle 1 is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("Rectangle 1 has a nonzero width; it is {}.\n", rect1.width);
    }

    println!("Can rectangle 1 hold rectangle 2? {}.", rect1.can_hold(&rect2));
    println!("Can rectangle 1 hold rectangle 3? {}.\n", rect1.can_hold(&rect3));
    
    let square = Rectangle::square(5);

    println!(
        "My square has a width of {} and a height of also {}.",
        square.width, square.height
    );
}
