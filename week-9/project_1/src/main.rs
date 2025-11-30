use std::fs::File;
use std::io::Write;

fn main() {
    // Vectors for each drink category
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    // Create the file
    let mut file = File::create("drinks.txt")
        .expect("Could not create file");

    // Write contents into the file
    writeln!(file, "LAGER DRINKS:").unwrap();
    for drink in &lager {
        writeln!(file, "- {}", drink).unwrap();
    }

    writeln!(file, "\nSTOUT DRINKS:").unwrap();
    for drink in &stout {
        writeln!(file, "- {}", drink).unwrap();
    }

    writeln!(file, "\nNON-ALCOHOLIC DRINKS:").unwrap();
    for drink in &non_alcoholic {
        writeln!(file, "- {}", drink).unwrap();
    }

    println!("drinks.txt has been created successfully!");
}
