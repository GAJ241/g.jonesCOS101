use std::io;

fn main() {
    println!("EY NIGERIA – EXPERIENCE CHECKER");
    println!("Find the candidate with the highest programming experience\n");

    // Vector to store (name, experience)
    let mut candidates: Vec<(String, i32)> = Vec::new();

    
    let mut input = String::new();
    println!("How many candidates are being interviewed?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let count: usize = input.trim().parse().expect("Enter a valid number");

    
    for i in 0..count {
    
        let mut name = String::new();
        println!("Enter name of candidate {}:", i + 1);
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim().to_string();

    
        let mut exp = String::new();
        println!("Enter {}'s years of programming experience:", name);
        io::stdin().read_line(&mut exp).expect("Failed to read input");
        let experience: i32 = exp.trim().parse().expect("Enter a number");

    
        candidates.push((name, experience));
    }

    
    let mut highest = &candidates[0];

    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = candidate;
        }
    }

    
    println!("Candidate with the highest experience:");
    println!("Name: {}", highest.0);
    println!("Years of Experience: {}", highest.1);
 
}
