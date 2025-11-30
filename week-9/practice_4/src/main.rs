use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");

    file.write_all(b"\nHello Class").expect("write failed");
    file.write_all(b"\nThis is the appendage to the document.").expect("write failed");

    println!("file append success");
}
