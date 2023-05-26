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
    for_loop();println!("The value of the element at index {index} is : {element}, {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    if x > 5 {
        println!("x value {x}, is not allow on this function call");
    }
    x + 1
}

fn for_loop() {
    const ARRAY: [i32; 6] = [3, 2, 3, 42, 3, 23, ];
    for a in ARRAY {
        println!("the value of this array is {a}");
    }
}