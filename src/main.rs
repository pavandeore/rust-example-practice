// use std::process::Command;
use std::io;

fn main(){


    // let a = 10;
    // let b = 2.4;
    // let char = 'a';
    // let verified = true;

    // println!("{a} {b} {char} {verified}");



    // let rust_version = Command::new("rustc").arg("--version").output()
    // .expect("Failed to execute");
    // println!("Rust version on system is: {:?}", rust_version);
    


    
    println!("Enter the first number");
    let mut input1:String = String::new();
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let num1: f64  = input1.trim().parse().expect("please enter valid number");

    println!("Enter the second number");
    let mut input2: String = String::new();
    io::stdin().read_line(&mut input2).expect("failed to read input2");
    let num2: f64 = input2.trim().parse().expect("please enter valid number");

    let sum: f64 = num1 + num2;
    println!("Sum is = {sum}");



}