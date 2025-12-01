struct Laptop {
    brand: &'static str,
    cost: u64,
}

impl Laptop {
    fn total_cost(&self, qty: u32) -> u64 {
        self.cost * qty as u64
    }
}

fn main() {
    let quantity = 3;

    let laptops = vec![
        Laptop { brand: "HP",      cost: 650_000 },
        Laptop { brand: "IBM",     cost: 755_000 },
        Laptop { brand: "Toshiba", cost: 550_000 },
        Laptop { brand: "Dell",    cost: 850_000 },
    ];

    println!("\n--- Laptop Purchase Cost Calculation ---");
    println!("Quantity purchased per brand: {}", quantity);
    println!("----------------------------------------");

    let total: u64 = laptops
        .iter()
        .map(|l| {
            let cost = l.total_cost(quantity);
            println!("{:<10} ₦{}", l.brand, cost);
            cost
        })
        .sum();

    println!("----------------------------------------");
    println!("Total Purchase Cost: ₦{}", total);
    println!("----------------------------------------");
}
