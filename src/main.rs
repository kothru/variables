use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("input");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed");

    let index: usize = index.trim().parse().expect("wrong num");
    let element = a[index];
    println!("value index {} is {}", index, element);

    // let y = {
    //     let x = 3;
    //     x + 1
    // };

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

    // immutable reference can twice at same time
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    // mutable reference
    let mut s = String::from("hello");
    change(&mut s);

    // mutable reference cannot twice reference same time
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // let r3 = &s;
    // println!("{}, {}", r1, r2);

    // slice string, &str is slice
    // let s = String::from("hello");
    // let len = s.len();
    // let slice = &s[0..len];
    // let slice = &s[..];

    // string literal is slice
    // let my_string_literal = "hello world";

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    dbg!(&rect1);

    // method: メソッド、最初のパラメータは常に&self
    // function: 関数

    // automatic referencing and dereferencing
    // same ref cause rust auto detect ownership
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // Rust dont have null
    // use Option<T>

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // rust default private
    // abs
    crate::a::b::c();
    // rel
    a::b::c();

    // function idiomatic use parent module
    use a::b;
    b::c();

    // Structs or enums use use full path
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(1, 2);

    // as alias
    use std::fmt::Result;
    use std::io::Result as IoResult;

    // pub use is re-exposing
    // pub use front_of_house;
}

mod a {
    pub mod b {
        pub fn c() {
            println!("dangle");
        }
    }
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

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// dangling pointer
// pointer return but empty...
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // alias of self: &Self (rectangle: &Rectangle)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Rectangle::square(3)
    // same as String::from('')
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

enum IpAddr {
    V4(String),
    V6(String),
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;
mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        // super::front_of_house::hosting::add_to_waitlist();
    }
}
