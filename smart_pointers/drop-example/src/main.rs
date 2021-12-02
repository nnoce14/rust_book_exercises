struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomerSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomerSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomerSmartPointers created.");
    drop(c);
    println!("CustomerSmartPointer c dropped before the end of main");
}
