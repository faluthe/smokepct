use std::{collections::HashSet, fs::File, io::Read};

pub mod unit_tests;
pub mod knowns;
pub mod args;

pub fn permute(mut k: usize, mut string: Vec<char>) -> String {
    for i in 1..string.len() {
        string.swap(k % (i + 1), i);
        k = k / (i + 1);
    }
    string.into_iter().collect()
}

pub fn factorial(x: usize) -> usize {
    if x == 1 {
        x
    } else {
        x * factorial(x - 1)
    }
}

pub fn dump_manifest(manifest_path: String) -> HashSet<String> {
    let mut manifest = File::open(&manifest_path).expect(format!(
        "Manifest not found at path {}", 
        manifest_path).as_str()
    );
    let mut data = String::new();
    manifest.read_to_string(&mut data).unwrap();
    let mut sums: HashSet<String> = HashSet::new();
    
    for l in data.lines() {
        sums.insert(l.to_uppercase());
    }
    
    sums
}