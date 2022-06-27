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

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
}

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

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
