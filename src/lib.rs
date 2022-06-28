mod front_of_house;
use std::{
    fmt::Display,
    fs::{self, File},
    hash::Hash,
    io::{self, ErrorKind, Read},
};

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // init vector
    let v: Vec<i32> = Vec::new();
    // vec init macro
    let v = vec![1, 2, 3];
    match v.get(2) {
        Some(third) => println!("some {}", third),
        None => println!("none"),
    }
    let mut v = vec![100, 32, 57];
    // ref loop
    for i in &v {
        println!("i {}", i)
    }
    // mut loop
    for i in &mut v {
        *i += 50;
    }

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push str
    s1.push_str(s2);
    println!("s2 is {}", s2);
    // push char
    let mut s = String::from("lo");
    s.push('l');

    use std::collections::HashMap;
    let mut score: HashMap<String, i32> = HashMap::new();
    score.insert(String::from("test"), 1);
    let scoreval = score.get(&String::from("test"));
    println!("{:?}", score);

    let val = score.entry(String::from("test2")).or_insert(10);

    // Result<T, E>

    // panic!
    // unwind stack

    // panic = 'abort'
    // skip unwind stack, immediately abort (os release stack)

    // rust detect buffer overread

    // default
    // RUST_BACKTRACE=0 cargo run
    // trace(VSCode Run command?)
    // RUST_BACKTRACE=1 cargo run
    // detail
    // RUST_BACKTRACE=full cargo run

    // Debug symbol enable default
    // off debug symbol
    // cargo (build|run) --release

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // or alter unwrap_or_else and closure
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    // if Ok, return File
    // if Err, panic! good message
    let f = File::open("hello.txt").unwrap();
    // expect change panic! message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // panic! use invalid value
    // like impl type domain get invalid value
    // almost use Result

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// i32 and char has Copy trait cause using stack
// if use copy in generics, describe copy trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn read_username_from_file() -> Result<String, io::Error> {
    // ? operator is ReturnedError if error
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;

    // let mut s = String::new();
    // ? can chain
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // std shorthand
    fs::read_to_string("hello.txt")
}

// use where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone,
{
    1
}

// trait
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl trait for struct
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// function can return which has a trait
// can return single type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// cannot return multiple type
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
