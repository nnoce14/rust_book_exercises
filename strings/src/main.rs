fn main() {

    let s = String::new();

    let data = "initial contents";

    let s2 = data.to_string();

    // also works on string literal directly
    let s2 = "initial contents".to_string();

    // can also use String::from method to do the same thing
    let s3 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let hello = "hola";
    let s = &hello[0..2];

	for c in "ABCDE".chars() {
		println!("{}", c);
	}	   

    for b in "ABCDE".bytes() {
        println!("{}", b);
    }
}

