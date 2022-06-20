mod front_of_house;
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
}
