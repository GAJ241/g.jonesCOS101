fn main() {
    let toshiba_price: f64 = 450_000.0;
    let mac_price: f64 = 1_500_000.0;
    let hp_price: f64 = 750_000.0;
    let dell_price: f64 = 2_850_000.0;
    let acer_price: f64 = 250_000.0;

    let toshiba_qty: f64 = 2.0;
    let mac_qty: f64 = 1.0;
    let hp_qty: f64 = 3.0;
    let dell_qty: f64 = 3.0;
    let acer_qty: f64 = 1.0;

    let total = (toshiba_price * toshiba_qty) + (mac_price * mac_qty) + (hp_price * hp_qty)+ (dell_price * dell_qty)+ (acer_price * acer_qty);

    let total_items = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;
    let average = total / total_items;

    println!("The Total Cost is ₦{}", total);
    println!("The Average Cost is  ₦{}", average);
}
