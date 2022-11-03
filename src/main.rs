use std::{fs::File, io::Read};

fn main() {
    let mut manifest = File::open("MANIFEST.txt").expect("Manifest not found");
    let mut data = String::new();
    manifest.read_to_string(&mut data).unwrap();
    let data = data.replace("\r\n", "");
    let mut sums: Vec<String> = Vec::new();
    
    let mut i = 0;
    while i < data.len() {
        let s = data.chars().skip(i).take(32).collect();
        println!("{s}");
        sums.push(s);
        i += 32;
    }
    println!(" . . ");
}
