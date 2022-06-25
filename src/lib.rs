mod front_of_house;
use std::{fs::File, hash::Hash, io::ErrorKind};

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
}
