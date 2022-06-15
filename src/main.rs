use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("input");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed");

    let index: usize = index.trim().parse().expect("wrong num");
    let element = a[index];
    println!("value index {} is {}", index, element);

    let y = {
        let x = 3;
        x + 1
    };

    // Heap Datatype need clone to copy, if x steady use
    // let x = String::from("test");
    // let y = x.clone();

    // pass value to function is same
    // let y = takes_and_gives_back(y);

    // Stack Datatype only deep copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // ampersands represent references, no ownership
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
