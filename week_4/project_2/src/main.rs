// Rust program to determine employee annual incentive

use std::io;

fn main() {
    let mut input1 = String::new(); // Name 
    let mut input2 = String::new(); // Experience
    let mut input3 = String::new(); // Age

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let name = input1.trim();

    let exp: String;

    loop {
        println!("Is the employee experienced? (yes/no): ");
        input2.clear();
        io::stdin().read_line(&mut input2).expect("Not a valid input");
        let experience = input2.trim().to_lowercase();

        if experience == "yes" || experience == "no" {
            exp = experience;
            break;
        } else {
            println!("Please enter only 'yes' or 'no'.");
        }
    }

    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let age: i32 = input3.trim().parse().expect("Not a valid number");

    if exp == "yes" {
        if age >= 40 {
            println!("{}, the incentive is ₦1,560,000 per year.", name);
        } else if age >= 30 {
            println!("{}, the incentive is ₦1,480,000 per year.", name);
        } else {
            println!("{}, the incentive is ₦1,300,000 per year.", name);
        }
    } else {
        println!("{}, the incentive is ₦100,000 per year.", name);
    }
}
