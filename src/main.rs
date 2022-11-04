use std::{fs::File, io::Read, collections::HashSet};

use b2sum_rust::Blake2bSum;

fn dump_manifest() -> HashSet<String> {
    let mut manifest = File::open("MANIFEST.txt").expect("Manifest not found");
    let mut data = String::new();
    manifest.read_to_string(&mut data).unwrap();
    let mut sums: HashSet<String> = HashSet::new();
    
    for l in data.lines() {
        sums.insert(l.to_uppercase());
    }
    sums
}

fn permute(mut k: usize, mut s: Vec<char>) -> String {
    for i in 1..s.len() {
        s.swap(k % (i + 1), i);
        k = k / (i + 1);
    }
    s.into_iter().collect()
}

fn factorial(x: usize) -> usize {
    if x == 1 {
        x
    } else {
        x * factorial(x - 1)
    }
}

fn main() {
    let sums = dump_manifest();
    let testkey = "CEGMNPRUVWX";
    let b2b = Blake2bSum::new(64);
    for k in 0..factorial(testkey.len()) {
        let x = permute(k, testkey.chars().collect());
        let check = b2b.read_str(x.clone() + "\n");
        if sums.contains(&check) {
            println!("Found solution: {}", x);
            break;
        }
    }
}
