use std::io;

fn main() {
    println!("Hello, world!");
    michael()
}


fn michael() {
    let a: [i32; 6] = [15, 23, 35, 42, 53, 69];
    println!("Pls enter an array index");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("failed to read line");

    let index: usize = index.trim().parse().expect("index entered was not a number");

    let element = a[index];
    let mut x = five();

    x = plus_one(x);


    println!("The value of the element at index {index} is : {element}, {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}