use ansi_term::Colour::{Red, Yellow, Blue};

use crate::DEBUG;

#[derive(Debug, Clone, Copy)]
pub struct KnownLetter {
    pub letter: char,
    pub pos: usize,
}

impl Default for KnownLetter {
    fn default() -> Self {
        KnownLetter { letter: '_', pos: 0 }
    }
}

impl KnownLetter {
    // Increment position of some KnownLetter
    pub fn increment_knowns(&mut self) {
        self.pos += 1;
    }
}

//////////////////////////////////////////////////////////////////////////////////
//      Take a "A_B__C____D___" formatted string 
//          -> produce a vector of KnownLetters
//
pub fn populate_knowns(string: &str) -> Vec<KnownLetter> {
    let mut knowns: Vec<KnownLetter> = Vec::new();
    let mut i: usize = 0;
    for (position, character) in string.char_indices() {
        if character != '_' {
            knowns.push(KnownLetter {letter: character, pos: position});
        }
    }
    knowns.shrink_to_fit();
    if DEBUG > 1 {
        println!("{}", Red.paint("@populate_knowns-------->"));
        println!("{} {}", 
            Blue.bold().paint("Capacity:"), 
            Yellow.bold().paint(knowns.capacity().to_string())
        );
        println!("{:?}", knowns);
        println!("\n{}", Blue.bold().paint("Success!"));
        println!("{}", Red.paint("<------------------------\n"));

    }
    knowns
}

//////////////////////////////////////////////////////////////////////////////////
//      Remove KnownLetters from key to give relevant letters to permute
//
pub fn remove_knowns(pzl_key: &mut String, knowns: &Vec<KnownLetter>) {
    for value in knowns {
        pzl_key.remove(pzl_key.find(value.letter)
            .expect(format!("Could not find letter '{}' in key '{}'", value.letter, pzl_key).as_str()));
    }
}

//////////////////////////////////////////////////////////////////////////////////
//      Insert KnownLetters into a string at their intended index
//
pub fn restore_knowns(s: &mut String, known_letters: &Vec<KnownLetter>) {
    for j in k {
        if DEBUG > 3 {
            println!("{}", Red.paint("@restore_knowns-------->"));
            println!("{} pos: {}, letter: {}", 
                Blue.bold().paint("Insert.."), 
                Blue.bold().paint(j.pos.to_string()), 
                Blue.bold().paint(j.letter.to_string())
            );
            
        }
        // Insert current letter

        s.insert(j.pos, j.letter);
        
        if DEBUG > 3 {
            println!("\n{}", Blue.bold().paint("Success!"));
            println!("{}", Red.paint("<------------------------\n"));
        }
    }
}
//////////////////////////////////////////////////////////////////////////////////
//      Generate STRING of KnownLetter in format "A_B___C__D____"
//          - uses restore_knowns() but with more functionality
//
pub fn generate_knowns(bank: &str, knowns: Option<&Vec<KnownLetter>>) -> String {
    // In default case, make empty vector and generate empty "___+n" string
    let empty = vec![KnownLetter::default()];

    let tmp_knowns: &Vec<KnownLetter> = knowns.unwrap_or(&empty);
    let knowns_len: usize = tmp_knowns.capacity();
    
    let length: usize = bank.chars().count();
    let mut knowns_str: String = String::new();
    for i in 0..(length - knowns_len) {
        knowns_str.insert(i, '_');
    }
    restore_knowns(&mut knowns_str, tmp_knowns);

    if DEBUG > 1 {
        println!("{}", Red.paint("@generate_knowns-------->"));
        println!("[{}] :: {}", 
            Yellow.bold().paint(&knowns_str), 
            Yellow.bold().paint(knowns_str.chars().count().to_string()) 
        );
        println!("\n{}", Blue.bold().paint("New String.. Success!"));
        println!("{}", Red.paint("<------------------------\n"));

    }

    knowns_str
}

//////////////////////////////////////////////////////////////////////////////////
//      Generate a vector of string possibilities with a sliding stride
//          ex: if (A < B < C)
//              string = "ABCDE"            (Letter Bank)
//              stride = "ABC"              (Stride)
//              actual_knowns = "___D_"     (N/A, coming soon!)
//
//              returns -> ["ABC__", "_ABC_", "__ABC"]
//  
pub fn run_stride(string: &str, stride: &str, actual_knowns: &str) -> Vec<String>{
    let mut current_knowns = populate_knowns(stride);
    let mut knowns_str: String;
    let mut str_set: Vec<String> = Vec::new();
    str_set.push(generate_knowns(string, Some(&current_knowns)));
    
    let top = string.chars().count() - stride.chars().count() - 1;
    for _ in 0..top - 1 {
        for k in current_knowns.as_mut_slice() {
            if k.pos < string.chars().count() {
                k.increment_knowns();
            } else {}
        }
        knowns_str = generate_knowns(string, Some(&current_knowns));
        str_set.push(knowns_str);
    }

    str_set
}
