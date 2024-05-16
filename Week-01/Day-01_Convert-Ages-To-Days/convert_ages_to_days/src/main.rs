use std::io;

fn main() {
    let mut age = String::new();
    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim().parse().expect("Please type a number!");
    let days = age * 365;
    println!("You are {} days old.", days);
}
