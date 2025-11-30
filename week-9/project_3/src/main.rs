fn main() {
    // Each record: (S/N, Name, Ministry, Geopolitical Zone)
    let records = vec![
        (1, "Aigboqun Alamba Daudu", "Internal Affairs", "South West"),
        (2, "Murtala Afeez Bendu", "Justice", "North East"),
        (3, "Okorocha Calistus Ogbona", "Defense", "South South"),
        (4, "Adewale Jimoh Akanbi", "Power & Steel", "South West"),
        (5, "Osazuwa Faith Etiye", "Petroleum", "South East"),
    ];

    println!("EFCC Consolidated Convicted Ministers Dataset\n");
    println!("{: <5} | {: <25} | {: <18} | {: <15}", 
             "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE");
    println!("{}", "-".repeat(70));

    for record in &records {
        println!("{: <5} | {: <25} | {: <18} | {: <15}", 
                 record.0, record.1, record.2, record.3);
    }
}
