use std::io;

// Function to check APS level
fn find_aps_level(role: &str, years: i32) -> String {

    // Vectors for each Public Servant level
    let aps1_2 = vec!["Intern", "-", "Paralegal", "Placement"];
    let aps3_5 = vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"];
    let aps5_8 = vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"];
    let el1_8_10 = vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"];
    let el2_10_13 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"];
    let ses = vec!["CEO", "Dean", "Partner", "Principal"];

    
    if aps1_2.contains(&role) && years >= 1 && years <= 2 {
        return "APS 1-2".to_string();
    }
    if aps3_5.contains(&role) && years >= 3 && years <= 5 {
        return "APS 3-5".to_string();
    }
    if aps5_8.contains(&role) && years >= 5 && years <= 8 {
        return "APS 5-8".to_string();
    }
    if el1_8_10.contains(&role) && years >= 8 && years <= 10 {
        return "EL1 8-10".to_string();
    }
    if el2_10_13.contains(&role) && years >= 10 && years <= 13 {
        return "EL2 10-13".to_string();
    }
    if ses.contains(&role) && years >= 14 {
        return "SES".to_string();
    }

    "Role and experience do not match any APS category".to_string()
}

fn main() {
    println!("PUBLIC SERVICE APS LEVEL CHECKER");


    let mut role = String::new();
    println!("Enter staff role (e.g., Associate, Administrator, Office Manager):");
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role = role.trim();

  
    let mut years_input = String::new();
    println!("Enter years of work experience:");
    io::stdin().read_line(&mut years_input).expect("Failed to read input");
    let years: i32 = years_input.trim().parse().expect("Enter a number");

    
    let level = find_aps_level(role, years);

 

    println!("Staff Role: {}", role);
    println!("Years of Experience: {}", years);
    println!("APS Level: {}", level);
}
