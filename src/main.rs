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
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
