use std::io;

fn main() {
    // Company Information
    let company_name = "PAU CAFETERIA";

    println!("\nCompany Name: {}\n", company_name);

    // Menu Display
    println!("MENU LIST");
    println!("P = Poundo Yam / Edinkaiko Soup   - N3,200");
    println!("F = Fried Rice & Chicken          - N3,000");
    println!("A = Amala & Ewedu Soup            - N2,500");
    println!("E = Eba & Egusi Soup              - N2,000");
    println!("W = White Rice & Stew             - N2,500\n");
    println!("Note: Spend over N10,000 to receive a 5% discount.\n");

    // Get food type input
    let mut food_type = String::new();
    println!("Enter the type of food (P/F/A/E/W): ");
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");
    let food_type = food_type.trim().to_uppercase();

    // Get quantity input
    let mut quantity = String::new();
    println!("Enter quantity: ");
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");
    let quantity: i32 = quantity.trim().parse().expect("Please enter a valid number");

    // Determine price based on menu
    let price = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food type entered!");
            return;
        }
    };

    // Arithmetic calculation for total cost
    let mut total = price * quantity;

    // Check for discount
    if total > 10_000 {
        let discount = (5 * total) / 100;
        total -= discount;
        println!("\nCongratulations! You received a 5% discount of N{:2}", discount);
    }

    // Display results
    println!("\nFood Type: {}", food_type);
    println!("Unit Price: N{}", price);
    println!("Quantity: {}", quantity);
    println!("Total Amount to Pay: N{}", total);
    println!("\nThank you for your order!");
}
