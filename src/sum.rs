use std::io;

fn main(){
    println!("Enter no1");
    let mut inp1: String = String::new();
    io::stdin().read_line(&mut inp1).expect("failed to read input");
    let num1: f64 = inp1.trim().parse().expect("enter valid number");

    println!("Enter no2");
    let mut inp2: String = String::new();
    io::stdin().read_line(&mut inp2).expect("failed to read input");
    let num2: f64 = inp2.trim().parse().expect("enter valid number");

    let sum: f64 = num1 + num2;
    println!("sum is {sum}");
}