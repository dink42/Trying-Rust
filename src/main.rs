use std::{io};

fn main() {
    println!("Hello, world!");
    let name = "Hello";
    println!("{}", name);
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("How old are you?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).unwrap();
    println!("Hi {} you are {}!!", name, age);

    let sum = age + &*name;
    for s in sum.chars() {
        println!("{} - {}", sum, s);
    }
}
